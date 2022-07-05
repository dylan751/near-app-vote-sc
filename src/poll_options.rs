use crate::*;

#[near_bindgen]
impl AppVoteContract {
    // ----------------------------------------- CREATE -----------------------------------------
    /**
     * - Create a new Poll Option
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add Vote into polls_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_poll_option(
        &mut self,
        poll_id: PollId,
        created_by: UserId,
        title: String,
        description: String,
        user_ids: Vec<UserId>
    ) -> PollOption {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let poll_option_id = self.poll_options_by_id_counter;

        // Check if the user_id exists or not
        assert!(
            self.users_by_id.get(&created_by).is_some(),
            "User who created this Option does not exist"
        );

        // Check if the poll_id exists or not
        assert!(
            self.polls_by_id.get(&poll_id).is_some(),
            "The poll this Option belongs to does not exist"
        );

        // Create new Poll
        let new_poll_option = PollOption {
            id: poll_option_id,
            poll_id,
            created_by,
            title,
            description,
            user_ids,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new Poll into polls_by_id (list of Votes of this Smart Contract)
        self.poll_options_by_id
            .insert(&poll_option_id, &new_poll_option);

        // Update Poll Id Counter
        self.poll_options_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_poll_option
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get list of all Poll Options in this Smart Contract (with pagination)
    pub fn get_all_poll_options(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<PollOption> {
        self.poll_options_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(poll_option_id, _poll_option)| {
                self.poll_options_by_id.get(&poll_option_id).unwrap()
            })
            .collect()
    }

    // Get 1 Poll Option by id
    pub fn get_poll_option_by_id(&self, poll_option_id: PollOptionId) -> PollOption {
        self.poll_options_by_id
            .get(&poll_option_id)
            .expect("Poll Option does not exist")
    }

    // // Update Poll Option information
    // pub fn update_poll_option(&mut self, poll_option_id: PollOptionId) -> PollOption {}

    // Delete Poll Option from the Smart Contract
    pub fn delete_poll_option(&mut self, poll_option_id: PollOptionId) {
        self.poll_options_by_id
            .remove(&poll_option_id)
            .expect("This poll_option does not exists");
    }
}
