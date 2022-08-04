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
    // Get total number of User in the Smart Contract
    pub fn user_total_supply(&self) -> u64 {
        // Count the number of user_id in users_by_id
        self.users_by_id.len()
    }

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

    // ----------------------------------------- UPDATE -----------------------------------------
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
        // TODO: Check only admin can call this function

        // Check if this User is a foreign key in Criteria or not
        for (_criteria_id, criteria) in self.criterias_by_id.iter() {
            assert!(
                criteria.created_by != user_id,
                "Cannot delete this User! This User is linked to a Criteria record!"
            );
        }
        // Check if this User is a foreign key in Poll or not
        for (_poll_id, poll) in self.polls_by_id.iter() {
            assert!(
                poll.created_by != user_id,
                "Cannot delete this User! This User is linked to a Poll record!"
            );
        }
        // Check if this User is a foreign key in PollOption or not
        for (_poll_option_id, poll_option) in self.poll_options_by_id.iter() {
            assert!(
                poll_option.created_by != user_id,
                "Cannot delete this User! This User is linked to a Poll Option record!"
            );
        }

        // Delete Result belongs to this User
        let mut remove_result_set = vec![]; // Vector of result_ids that need to be deleted
        for (result_id, result) in self.results_by_id.iter() {
            if result.user_id == user_id {
                remove_result_set.push(result_id);
            }
        }
        log!("Remove result set: {:?}", remove_result_set);
        for result_id in remove_result_set {
            self.results_by_id.remove(&result_id).unwrap();
        }

        // Delete this User from IsUserVote
        let mut remove_is_user_vote_set = vec![]; // Vector of is_user_vote_ids that need to be deleted
        for (is_user_vote_id, is_user_vote) in self.is_user_votes_by_id.iter() {
            if is_user_vote.user_id == user_id {
                remove_is_user_vote_set.push(is_user_vote_id);
            }
        }
        log!("Remove is user vote set: {:?}", remove_is_user_vote_set);
        for is_user_vote_id in remove_is_user_vote_set {
            self.is_user_votes_by_id.remove(&is_user_vote_id).unwrap();
        }

        // Delete User
        self.users_by_id
            .remove(&user_id)
            .expect("This user does not exists");
    }
}
