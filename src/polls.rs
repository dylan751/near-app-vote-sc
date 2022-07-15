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
        criteria_ids: Vec<CriteriaId>,
        poll_option_id: PollOptionId,
        created_by: UserId,
        img_url: Option<String>,
        title: String,
        description: String,
        start_at: Option<Timestamp>,
        end_at: Option<Timestamp>,
    ) -> Poll {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let poll_id = self.polls_by_id_counter;

        // Check if the all the criteria_ids exists or not
        for criteria_id in criteria_ids.clone() {
            assert!(
                self.criterias_by_id.get(&criteria_id).is_some(),
                "Some of the criterias does not exist"
            );
        }

        // Check if the pool_option_id exists or not
        let poll_option = self
            .poll_options_by_id
            .get(&poll_option_id)
            .expect("This poll option does not exist");

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
            criteria_ids: criteria_ids.clone(),
            poll_option_id,
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

        // --- Insert into IsUserVote (is_voted) table ---
        for user_id in poll_option.clone().user_ids {
            let is_user_vote_id = self.is_user_votes_by_id_counter;
            let new_is_user_vote = IsUserVote {
                user_id,
                poll_id,
                is_voted: false,
            };
            self.is_user_votes_by_id
                .insert(&is_user_vote_id, &new_is_user_vote);
            self.is_user_votes_by_id_counter += 1;
        }

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        // Insert data with total_vote = 0 into Result table
        for criteria_id in criteria_ids.clone() {
            for user_id in poll_option.clone().user_ids {
                self.create_result(criteria_id, poll_id, user_id);
            }
        }

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
        poll_option_id: PollOptionId,
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

        // Check if the pool_option_id exists or not
        assert!(
            self.poll_options_by_id.get(&poll_option_id).is_some(),
            "Poll Option does not exist"
        );

        let updated_poll = Poll {
            id: poll.id,
            criteria_ids: poll.criteria_ids,
            poll_option_id,
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
        self.polls_by_id
            .remove(&poll_id)
            .expect("This poll does not exists");
    }
}
