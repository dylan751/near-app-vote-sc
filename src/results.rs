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
        assert!(
            self.polls_by_id.get(&poll_id).is_some(),
            "Poll does not exist"
        );
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

    // Update Criteria information (When a user vote)
    pub fn update_result(&mut self, result_id: ResultId) -> Result {
        let result = self
            .results_by_id
            .get(&result_id)
            .expect("This result does not exist");

        let updated_result = Result {
            id: result.id,
            criteria_id: result.criteria_id,
            poll_id: result.poll_id,
            user_id: result.user_id,
            total_vote: result.total_vote + 1, // Increase the number of votes by one
            created_at: result.created_at,
            updated_at: Some(env::block_timestamp()),
        };

        // Update results_by_id
        self.results_by_id.insert(&result_id, &updated_result);

        updated_result
    }

    // Delete Result from the Smart Contract
    pub fn delete_result(&mut self, result_id: PollOptionId) {
        self.results_by_id
            .remove(&result_id)
            .expect("This result does not exists");
    }
}
