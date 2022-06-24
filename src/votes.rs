use crate::*;

#[near_bindgen]
impl AppVoteContract {
    // ----------------------------------------- CREATE -----------------------------------------
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

        // Check if the criteria_id exists or not
        assert!(self.criterias_by_id.get(&criteria_id).is_some(), "Criteria does not exist");
        // Check if the user_id exists or not
        assert!(self.users_by_id.get(&user_id).is_some(), "User does not exist");
        // Check if month is valid or not

        // Create new Vote
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

        // Insert new Vote into votes_by_id (list of Votes of this Smart Contract)
        self.votes_by_id.insert(&vote_id, &new_vote);

        // Update Vote Id Counter
        self.votes_by_id_counter += 1;

        // Used data storage = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);

        new_vote
    }

    // ----------------------------------------- READ -----------------------------------------
    // Get list of all Votes in this Smart Contract (with pagination)
    pub fn get_all_votes(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Vote> {
        self.votes_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(vote_id, _vote)| self.votes_by_id.get(&vote_id).unwrap())
            .collect()
    }

    // Get 1 Vote by id
    pub fn get_vote_by_id(&self, vote_id: VoteId) -> Vote {
        self.votes_by_id.get(&vote_id).expect("Vote does not exist")
    }
}
