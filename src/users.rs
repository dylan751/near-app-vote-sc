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
        blockchain_type: BlockchainType,
        wallet_address: String,
    ) -> User {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let user_id = self.users_by_id_counter;

        // Check duplicate wallet_address of the same blockchain_type
        // Check only SC's owner / user with role Admin can create_user
        let mut create_account_id = 0; // Default value
        for (_user_id, user) in self.users_by_id.iter() {
            if matches!(user.user_wallet.blockchain_type, _blockchain_type) {
                assert!(
                    user.user_wallet.wallet_address != wallet_address,
                    "Duplicate wallet address for this blockchain!"
                );
            }
            if user.user_wallet.wallet_address == env::predecessor_account_id() {
                create_account_id = user.id;
            }
        }
        if env::predecessor_account_id() != self.owner_id {
            let create_account = self
                .users_by_id
                .get(&create_account_id)
                .expect("User does not exist");
            assert!(
                matches!(create_account.role, Role::Admin)
                    || self.owner_id == create_account.user_wallet.wallet_address,
                "Only Admin or Smart Contract's owner can create new user"
            );
        }

        // Create new User
        let new_user = User {
            id: user_id,
            name,
            role,
            email,
            user_wallet: UserWallet {
                blockchain_type,
                wallet_address,
            },
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

    // Get User's id by Wallet Account
    pub fn get_user_by_wallet_address(&self, wallet_address: AccountId) -> Option<User> {
        for user in self.users_by_id.values() {
            if user.user_wallet.wallet_address == wallet_address {
                return Some(user);
            }
        }
        return None;
    }

    // Update User information
    pub fn update_user(
        &mut self,
        user_id: UserId,
        name: AccountId,
        role: Role,
        email: String,
        blockchain_type: BlockchainType,
        wallet_address: String,
    ) -> User {
        let user = self
            .users_by_id
            .get(&user_id)
            .expect("This user does not exist");

        let updated_user = User {
            id: user.id,
            name,
            role,
            email,
            user_wallet: UserWallet {
                blockchain_type,
                wallet_address,
            },
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
