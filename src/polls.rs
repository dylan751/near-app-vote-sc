use crate::*;

#[near_bindgen]
impl AppVoteContract {
    // ----------------------------------------- CREATE -----------------------------------------
    /**
     * - Create a new Poll
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add Vote into polls_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_poll(
        &mut self,
        criteria_id: CriteriaId,
        user_id: UserId,
        title: String,
        description: String,
        start_at: Option<Timestamp>,
        end_at: Option<Timestamp>,
    ) -> Poll {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let poll_id = self.polls_by_id_counter;

        // Check if the criteria_id exists or not
        assert!(
            self.criterias_by_id.get(&criteria_id).is_some(),
            "Criteria does not exist"
        );
        // Check if the user_id exists or not
        assert!(
            self.users_by_id.get(&user_id).is_some(),
            "User does not exist"
        );
        // Check if month is valid or not

        // Create new Poll
        let new_poll = Poll {
            criteria_id,
            user_id,
            title,
            description,
            start_at,
            end_at,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new Poll into polls_by_id (list of Votes of this Smart Contract)
        self.polls_by_id.insert(&poll_id, &new_poll);

        // Update Poll Id Counter
        self.polls_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_poll
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get list of all Polls in this Smart Contract (with pagination)
    pub fn get_all_polls(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Poll> {
        self.polls_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(poll_id, _poll)| self.polls_by_id.get(&poll_id).unwrap())
            .collect()
    }

    // Get 1 Poll by id
    pub fn get_poll_by_id(&self, poll_id: PollId) -> Poll {
        self.polls_by_id.get(&poll_id).expect("Poll does not exist")
    }

    // Update Poll information
    pub fn update_poll(
        &mut self,
        poll_id: PollId,
        title: String,
        description: String,
        start_at: Option<Timestamp>,
        end_at: Option<Timestamp>,
    ) -> Poll {
        let poll = self
            .polls_by_id
            .get(&poll_id)
            .expect("This poll does not exist");

        let updated_poll = Poll {
            criteria_id: poll.criteria_id,
            user_id: poll.user_id,
            title: title,
            description: description,
            start_at,
            end_at,
            created_at: poll.created_at,
            updated_at: Some(env::block_timestamp()),
        };

        // Update polls_by_id
        self.polls_by_id.insert(&poll_id, &updated_poll);

        updated_poll
    }

    // Delete Poll from the Smart Contract
    pub fn delete_poll(&mut self, poll_id: PollId) {
        self.polls_by_id
            .remove(&poll_id)
            .expect("This poll does not exists");
    }
}
