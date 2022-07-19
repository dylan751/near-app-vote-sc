use crate::*;

#[near_bindgen]
impl AppVoteContract {
    pub fn is_voted(&self, user_id: UserId, poll_id: PollId) -> bool {
        let mut is_voted = false; // Default value

        for (_is_user_vote_id, is_user_vote) in self.is_user_votes_by_id.iter() {
            if is_user_vote.user_id == user_id && is_user_vote.poll_id == poll_id {
                is_voted = is_user_vote.is_voted;
            }
        }
        
        is_voted
    }

    pub fn get_all_is_user_votes(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<IsUserVote> {
        self.is_user_votes_by_id
            .iter()
            .skip(from_index.unwrap_or(0) as usize)
            .take(limit.unwrap_or(PAGINATION_SIZE) as usize)
            .map(|(is_user_vote_id, _is_user_vote)| {
                self.is_user_votes_by_id.get(&is_user_vote_id).unwrap()
            })
            .collect()
    }
}
