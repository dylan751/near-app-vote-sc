use crate::*;

#[near_bindgen]
impl AppVoteContract {
    // ----------------------------------------- CREATE -----------------------------------------
    /**
     * - Create a new Result
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add Result into results_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_result(
        &mut self,
        criteria_id: CriteriaId,
        poll_id: PollId,
        user_id: UserId,
    ) -> Result {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let result_id = self.results_by_id_counter;

        // Check if the criteria_id exists or not
        assert!(
            self.criterias_by_id.get(&criteria_id).is_some(),
            "Criteria does not exist"
        );

        // Check if the poll_id exists or not
        let poll = self.polls_by_id.get(&poll_id).expect("Poll does not exist");
        let mut is_belongs = 0;

        // Check if this Criteria belongs to this Poll or not
        for poll_criteria_id in poll.criteria_ids {
            if poll_criteria_id == criteria_id {
                is_belongs = 1;
            }
        }
        assert_eq!(is_belongs, 1, "This Criteria does not belongs to this Poll");

        // Check if the user_id exists or not
        assert!(
            self.users_by_id.get(&user_id).is_some(),
            "User does not exist"
        );

        // Create new Result
        let new_result = Result {
            id: result_id,
            criteria_id,
            poll_id,
            user_id,
            total_vote: 0,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new Result into results_by_id (list of Results of this Smart Contract)
        self.results_by_id.insert(&result_id, &new_result);

        // Update Result Id Counter
        self.results_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_result
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get list of all Results in this Smart Contract (with pagination)
    pub fn get_all_results(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Result> {
        self.results_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(result_id, _result)| self.results_by_id.get(&result_id).unwrap())
            .collect()
    }

    // Get 1 Result by id
    pub fn get_result_by_id(&self, result_id: ResultId) -> Result {
        self.results_by_id
            .get(&result_id)
            .expect("Result does not exist")
    }

    // Voting
    // Update Result information (When a user vote)
    pub fn vote(
        &mut self,
        voted_user_id: UserId,
        poll_id: PollId,
        criteria_user_array: Vec<CriteriaUser>,
    ) {
        // Check voted User exists or not
        assert!(
            self.users_by_id.get(&voted_user_id).is_some(),
            "The user who votes do not exist"
        );

        // Check Criteria, Poll, User exists or not
        let poll = self
            .polls_by_id
            .get(&poll_id)
            .expect("Related poll does not exist");

        for criteria_user in criteria_user_array.clone() {
            assert!(
                self.criterias_by_id
                    .get(&criteria_user.criteria_id)
                    .is_some(),
                "Some of the Criteria does not exist"
            );
            assert!(
                self.users_by_id.get(&criteria_user.user_id).is_some(),
                "Some of the User does not exist"
            );
        }

        // Check if User has voted for this Poll or not
        assert_eq!(
            self.is_voted(voted_user_id, poll_id),
            false,
            "This User has voted for this Poll"
        );

        // Check current time is between poll.start_time and poll.end_time or not
        let vote_timestamp = env::block_timestamp(); // Voting timestamp

        let base: u64 = 10;
        let poll_start_at = poll.start_at.unwrap_or(0);
        let poll_end_at = poll.end_at.unwrap_or(0);

        let end_at_nano = poll_end_at * base.pow(6); // Voting timestamp in milliseconds
        let start_at_nano = poll_start_at * base.pow(6); // Voting timestamp in milliseconds

        // --------------------------- CHECK VOTING TIME ---------------------------
        if poll_start_at != 0 {
            if poll_end_at == 0 {
                // If poll_end_at == 0 -> Only check poll_start_at
                assert!(
                    vote_timestamp > start_at_nano,
                    "Cannot vote this Poll during this time"
                );
            } else {
                // If poll_end_at != 0 -> Check both poll_start_at and poll_end_at
                assert!(
                    vote_timestamp < end_at_nano && vote_timestamp > start_at_nano,
                    "Cannot vote this Poll during this time"
                );
            }
        } else {
            // If poll_end_at == 0 -> Don't have to check anything

            // If poll_end_at != -> Only check poll_end_at
            if poll_end_at != 0 {
                assert!(
                    vote_timestamp < end_at_nano,
                    "Cannot vote this Poll during this time"
                )
            }
        }
        // -------------------------------------------------------------------------
        for criteria_user in criteria_user_array {
            // Get result_id
            let mut match_result_id = 0; // Default value
            let mut match_result = Result {
                // Default value
                id: 0,
                criteria_id: 0,
                poll_id: 0,
                user_id: 0,
                total_vote: 0,
                created_at: None,
                updated_at: None,
            };

            for (result_id, result) in self.results_by_id.iter() {
                if result.criteria_id == criteria_user.criteria_id
                    && result.poll_id == poll_id
                    && result.user_id == criteria_user.user_id
                {
                    match_result_id = result_id;
                    match_result = result;
                    break;
                }
            }

            assert!(match_result_id != 0, "Not found result"); // If match_result_id == 0 -> No appropriate result

            let updated_result = Result {
                id: match_result.id,
                criteria_id: criteria_user.criteria_id,
                poll_id,
                user_id: criteria_user.user_id,
                total_vote: match_result.total_vote + 1, // Increase the number of votes by one
                created_at: match_result.created_at,
                updated_at: Some(env::block_timestamp()),
            };

            // Update results_by_id
            self.results_by_id.insert(&match_result_id, &updated_result);
        }

        // Find the voted user -> Mark this User as has voted for this Pol
        let is_user_vote_id = self.is_user_votes_by_id_counter; // Default value
        let is_user_vote = IsUserVote {
            // Default value
            user_id: voted_user_id,
            poll_id,
            is_voted: true,
        };
        self.is_user_votes_by_id
            .insert(&is_user_vote_id, &is_user_vote);
        self.is_user_votes_by_id_counter += 1;
    }

    pub fn get_all_results_by_poll_id(&self, poll_id: PollId) -> Vec<ResultByPoll> {
        // Check Poll exists or not
        assert!(
            self.polls_by_id.get(&poll_id).is_some(),
            "Poll does not exist"
        );

        // Get the Array of Result of this Poll
        let mut result_by_poll_id_set: Vec<Result> = vec![];
        let mut return_set: Vec<ResultByPoll> = vec![];
        for result in self.results_by_id.values() {
            if result.poll_id == poll_id {
                result_by_poll_id_set.push(result);
            }
        }

        let poll = self
            .polls_by_id
            .get(&poll_id)
            .expect("Poll does not exists");
        let poll_option = self
            .poll_options_by_id
            .get(&poll.poll_option_id)
            .expect("Poll Option does not exists");
        for user_id in poll_option.user_ids {
            let mut vote_count = 0;
            for result in result_by_poll_id_set.clone() {
                if result.user_id == user_id {
                    vote_count += result.total_vote;
                }
            }
            let new_result: ResultByPoll = ResultByPoll {
                poll_id: poll_id,
                user_id: user_id,
                total_vote: vote_count,
            };

            return_set.push(new_result);
        }

        // Sort ranking desc
        return_set.sort_by(|a, b| b.total_vote.cmp(&a.total_vote));

        return_set
    }

    // Delete Result from the Smart Contract
    pub fn delete_result(&mut self, result_id: PollOptionId) {
        self.results_by_id
            .remove(&result_id)
            .expect("This result does not exists");
    }
}
