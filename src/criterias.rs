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
    pub fn create_criteria(
        &mut self,
        user_id: UserId,
        description: String,
    ) -> Criteria {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let criteria_id = self.criterias_by_id_counter;

        // Create new User
        let new_criteria = Criteria {
            user_id,
            description,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new User into users_by_id (list of Users of this Smart Contract)
        self.criterias_by_id.insert(&criteria_id, &new_criteria);

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_criteria
    }
}
