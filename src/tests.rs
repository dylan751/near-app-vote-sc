#[allow(unused_imports)]
use crate::*;

#[cfg(all(test, not(target_arch = "wasm-32")))]
mod tests {
    use super::*;

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    const CREATE_STORAGE_COST: u128 = 100_000_000_000_000_000_000_000;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_crud() {
        // ------------------------------------------------------------------------------------------------
        // ----------------------------------- INIT TESTING ENVIRONMENT -----------------------------------
        // ------------------------------------------------------------------------------------------------
        let mut context = get_context(false);
        testing_env!(context.build());

        // Init contract
        let mut contract = AppVoteContract::new(accounts(0).to_string());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(CREATE_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        // ----------------------------------------------------------------------------
        // ----------------------------------- USER -----------------------------------
        // ----------------------------------------------------------------------------
        let name = "Zuong".to_string();
        let role = Role::Admin;
        let email = "admin@gmail.com".to_string();
        let near_account_id = "duongnh.testnet".to_string();

        // --- Create the first user ---
        contract.create_user(name, role, email, near_account_id);

        let first_user_id = 0; // Id of the newly created user
        let mut first_user = contract.get_user_by_id(first_user_id);
        assert_eq!(first_user.name, "Zuong".to_string());

        // --- Create the second user ---
        contract.create_user(
            "Hoang".to_string(),
            Role::Employee,
            "hoangnv@gmail.com".to_string(),
            "hoangnv.testnet".to_string(),
        );
        let mut all_users = contract.get_all_users(Some(0), Some(10));
        assert_eq!(all_users.len(), 2);
        assert_eq!(all_users[1].name, "Hoang".to_string());

        // --- Update the first user ---
        contract.update_user(
            first_user_id,
            "Updated name".to_string(),
            Role::Admin,
            "updatedmail@gmail.com".to_string(),
            "duongnh.testnet".to_string(),
        );

        // Check updated information
        first_user = contract.get_user_by_id(first_user_id);
        assert_eq!(first_user.name, "Updated name".to_string());

        // --- Delete the first user ---
        contract.delete_user(first_user_id);

        // Check if the first user has been deleted
        all_users = contract.get_all_users(Some(0), Some(10));
        assert_eq!(all_users.len(), 1);

        // --------------------------------------------------------------------------------
        // ----------------------------------- CRITERIA -----------------------------------
        // --------------------------------------------------------------------------------
        let user_id = 1; // User id 1 create this criteria
        let description = "The most handsome employee".to_string();

        // --- Create the first criteria ---
        contract.create_criteria(user_id, description);

        let first_criteria_id = 0; // Id of the newly created Criteria
        let mut first_criteria = contract.get_criteria_by_id(first_criteria_id);
        assert_eq!(first_criteria.user_id, 1);
        assert_eq!(
            first_criteria.description,
            "The most handsome employee".to_string()
        );

        // --- Create the second criteria ---
        contract.create_criteria(1, "The most creative employee".to_string());
        let mut all_criterias = contract.get_all_criterias(Some(0), Some(10));
        assert_eq!(all_criterias.len(), 2);
        assert_eq!(
            all_criterias[1].description,
            "The most creative employee".to_string()
        );

        // --- Update the first Criteria ---
        contract.update_criteria(first_criteria_id, "Updated description".to_string());

        // Check updated information
        first_criteria = contract.get_criteria_by_id(first_criteria_id);
        assert_eq!(
            first_criteria.description,
            "Updated description".to_string()
        );

        // --- Delete the first Criteria ---
        contract.delete_criteria(first_criteria_id);

        // Check if the first Criteria has been deleted
        all_criterias = contract.get_all_criterias(Some(0), Some(10));
        assert_eq!(all_criterias.len(), 1);

        // ----------------------------------------------------------------------------
        // ----------------------------------- POLL -----------------------------------
        // ----------------------------------------------------------------------------
        let user_id = 1; // User id 1 create this Poll
        let criteria_id = 1; // This Poll belongs to Critetia id 1
        let title = "Test poll".to_string();
        let description = "Test poll description".to_string();
        let start_at = Some(0);
        let end_at = Some(0);

        // --- Create the first criteria ---
        contract.create_poll(criteria_id, user_id, title, description, start_at, end_at);

        let first_poll_id = 0; // Id of the newly created Poll
        let mut first_poll = contract.get_poll_by_id(first_poll_id);
        assert_eq!(first_poll.user_id, 1);
        assert_eq!(first_poll.criteria_id, 1);
        assert_eq!(first_poll.title, "Test poll".to_string());
        assert_eq!(first_poll.description, "Test poll description".to_string());

        // --- Create the second Poll ---
        contract.create_poll(
            1,
            1,
            "Test poll 2".to_string(),
            "Test poll description 2".to_string(),
            Some(0),
            Some(0),
        );
        let mut all_polls = contract.get_all_polls(Some(0), Some(10));
        assert_eq!(all_polls.len(), 2);
        assert_eq!(all_polls[1].title, "Test poll 2".to_string());

        // --- Update the first Poll ---
        contract.update_poll(
            first_poll_id,
            "Updated Poll title".to_string(),
            "Updated Poll description".to_string(),
            Some(0),
            Some(0),
        );

        // Check updated information
        first_poll = contract.get_poll_by_id(first_poll_id);
        assert_eq!(first_poll.title, "Updated Poll title".to_string());
        assert_eq!(
            first_poll.description,
            "Updated Poll description".to_string()
        );

        // --- Delete the first Poll ---
        contract.delete_poll(first_poll_id);

        // Check if the first Poll has been deleted
        all_polls = contract.get_all_polls(Some(0), Some(10));
        assert_eq!(all_polls.len(), 1);
    }
}
