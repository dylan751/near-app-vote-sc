use crate::*;

#[near_bindgen]
impl AppVoteContract {
    // ----------------------------------------- CREATE -----------------------------------------
    /**
     * - Create a new Criteria
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add Criteria into criterias_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_criteria(&mut self, user_id: UserId, description: String) -> Criteria {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let criteria_id = self.criterias_by_id_counter;

        // Check if the user_id exists or not
        assert!(self.users_by_id.get(&user_id).is_some(), "User does not exist");

        // Create new Criteria
        let new_criteria = Criteria {
            user_id,
            description,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new Criteria into criterias_by_id (list of Criterias of this Smart Contract)
        self.criterias_by_id.insert(&criteria_id, &new_criteria);

        // Update Criteria Id Counter
        self.criterias_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_criteria
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get list of all Criterias in this Smart Contract (with pagination)
    pub fn get_all_criterias(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Criteria> {
        self.criterias_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(criteria_id, _criteria)| self.criterias_by_id.get(&criteria_id).unwrap())
            .collect()
    }

    // Get 1 Criteria by id
    pub fn get_criteria_by_id(&self, criteria_id: CriteriaId) -> Criteria {
        self.criterias_by_id.get(&criteria_id).expect("Criteria does not exist")
    }
}
