use crate::*;

#[near_bindgen]
impl AppVoteContract {
    // ----------------------------------------- CREATE -----------------------------------------
    /**
     * - Create a new User
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add User into users_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_user(
        &mut self,
        name: AccountId,
        role: Role,
        email: String,
        near_account_id: AccountId,
    ) -> User {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let user_id = self.users_by_id_counter;

        // TODO: Check duplicate email

        // Create new User
        let new_user = User {
            name,
            role,
            email,
            near_account_id,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new User into users_by_id (list of Users of this Smart Contract)
        self.users_by_id.insert(&user_id, &new_user);

        // Update User Id Counter
        self.users_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_user
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get list of all Users in this Smart Contract (with pagination)
    pub fn get_all_users(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<User> {
        self.users_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(user_id, _user)| self.users_by_id.get(&user_id).unwrap())
            .collect()
    }

    // Get 1 User by id
    pub fn get_user_by_id(&self, user_id: UserId) -> User {
        self.users_by_id.get(&user_id).expect("User does not exist")
    }

    // Update User information
    pub fn update_user(
        &mut self,
        user_id: UserId,
        name: AccountId,
        role: Role,
        email: String,
        near_account_id: AccountId,
    ) -> User {
        let user = self
            .users_by_id
            .get(&user_id)
            .expect("This user does not exist");

        let updated_user = User {
            name,
            role,
            email,
            near_account_id,
            created_at: user.created_at,
            updated_at: Some(env::block_timestamp()),
        };

        // Update users_by_id
        self.users_by_id.insert(&user_id, &updated_user);

        updated_user
    }

    // Delete User from the Smart Contract
    pub fn delete_user(&mut self, user_id: UserId) {
        self.users_by_id
            .remove(&user_id)
            .expect("This user does not exists");
    }
}
