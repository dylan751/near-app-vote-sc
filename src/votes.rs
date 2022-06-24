use crate::*;

#[near_bindgen]
impl AppVoteContract {
    /**
     * - Create a new Vote
     * - Ask user to deposit an amount of NEAR to cover storage data fee
     * - Add Vote into votes_by_id
     * - Refund redundant deposited NEAR back to user
     */
    #[payable]
    pub fn create_vote(
        &mut self,
        criteria_id: CriteriaId,
        user_id: UserId,
        month: u32,
        start_at: Option<Timestamp>,
        end_at: Option<Timestamp>,
    ) -> Vote {
        let before_storage_usage = env::storage_usage(); // Used to calculate the amount of redundant NEAR when users deposit

        let vote_id = self.votes_by_id_counter;

        // Create new User
        let new_vote = Vote {
            criteria_id,
            user_id,
            status: Status::Done, // TODO: Xác nhận lại cơ chế của Status
            month,
            start_at,
            end_at,
            created_at: Some(env::block_timestamp()),
            updated_at: None,
        };

        // Insert new User into users_by_id (list of Users of this Smart Contract)
        self.votes_by_id.insert(&vote_id, &new_vote);

        // Update Vote Id Counter
        self.votes_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_vote
    }
}
