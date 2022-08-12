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
        criteria_option_id_array: Vec<CriteriaOptionId>,
        created_by: UserId,
        img_url: Option<String>,
        title: String,
        description: String,
        start_at: Option<Timestamp>,
        end_at: Option<Timestamp>,
    ) -> Poll {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let poll_id = self.polls_by_id_counter;

        for criteria_option_id in criteria_option_id_array.clone() {
            // Check if the all the criteria_ids exists or not
            assert!(
                self.criterias_by_id
                    .get(&criteria_option_id.criteria_id)
                    .is_some(),
                "Some of the criterias does not exist"
            );
            // Check if the pool_option_id exists or not
            assert!(
                self.poll_options_by_id
                    .get(&criteria_option_id.poll_option_id)
                    .is_some(),
                "Some of the poll options does not exist"
            );
        }

        // Check if the user_id exists or not
        let user = self
            .users_by_id
            .get(&created_by)
            .expect("User does not exist");

        // Check if the User who create poll is Admin or not?
        assert!(
            matches!(user.role, Role::Admin),
            "Only Admin can create polls"
        );

        // Create new Poll
        let new_poll = Poll {
            id: poll_id,
            criteria_option_id_array: criteria_option_id_array.clone(),
            created_by,
            img_url,
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

        // Insert data with total_vote = 0 into Result table
        for criteria_option_id in criteria_option_id_array.clone() {
            let poll_option = self
                .poll_options_by_id
                .get(&criteria_option_id.poll_option_id)
                .expect("This Poll does not exists");
            for option in poll_option.clone().options {
                self.create_result(criteria_option_id.criteria_id, poll_id, option);
            }
        }

        new_poll
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get total number of User in the Smart Contract
    pub fn poll_total_supply(&self) -> u64 {
        // Count the number of poll_id in polls_by_id
        self.polls_by_id.len()
    }

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

    // ----------------------------------------- UPDATE -----------------------------------------
    // Update Poll information
    pub fn update_poll(
        &mut self,
        poll_id: PollId,
        img_url: Option<String>,
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
            id: poll.id,
            criteria_option_id_array: poll.criteria_option_id_array,
            created_by: poll.created_by,
            img_url,
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
        // Delete Result belongs to this Poll
        let mut remove_result_set = vec![]; // Vector of result_ids that need to be deleted
        for (result_id, result) in self.results_by_id.iter() {
            if result.poll_id == poll_id {
                remove_result_set.push(result_id);
            }
        }
        log!("Remove result set: {:?}", remove_result_set);
        for result_id in remove_result_set {
            self.results_by_id.remove(&result_id).unwrap();
        }

        // Delete this Poll from IsUserVote
        let mut remove_is_user_vote_set = vec![]; // Vector of is_user_vote_ids that need to be deleted
        for (is_user_vote_id, is_user_vote) in self.is_user_votes_by_id.iter() {
            if is_user_vote.poll_id == poll_id {
                remove_is_user_vote_set.push(is_user_vote_id);
            }
        }
        log!("Remove is user vote set: {:?}", remove_is_user_vote_set);
        for is_user_vote_id in remove_is_user_vote_set {
            self.is_user_votes_by_id.remove(&is_user_vote_id).unwrap();
        }

        // Delete Poll
        self.polls_by_id
            .remove(&poll_id)
            .expect("This poll does not exists");
    }
}
