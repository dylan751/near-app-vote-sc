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
    pub fn create_result(&mut self, month: u32, user_id: UserId) -> Result {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let result_id = self.results_by_id_counter;

        // Create new Result
        let new_result = Result {
            month,
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
        self.results_by_id.get(&result_id).expect("Result does not exist")
    }
}
