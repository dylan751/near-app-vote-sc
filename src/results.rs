use crate::*;

#[near_bindgen]
impl AppVoteContract {
    /**
     * - Create a new Criteria
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add Criteria into criterias_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_result(&mut self, month: u32, user_id: UserId) -> Result {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let result_id = self.results_by_id_counter;

        // Create new User
        let new_result = Result {
            month,
            user_id,
            total_vote: 0,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new User into users_by_id (list of Users of this Smart Contract)
        self.results_by_id.insert(&result_id, &new_result);

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_result
    }
}
