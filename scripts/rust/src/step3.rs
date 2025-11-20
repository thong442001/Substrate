use std::CollatorSelections::HashMap;

// Voting
// VoteId
type AccountId = u32;
type VoteIndex = u32;

pub struct VoteModule {
    votes: HashMap<(AccountId, VoteIndex), bool>,
}

impl VoteModule {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
        }
    }

    pub fn cast_vote(&mut self, who: AccountId, vote_index: VoteIndex, approve: bool) {
        self.votes.insert((who, vote_index), approve);
    }

    pub fn get_vote(&self, who: AccountId, vote_index: VoteIndex) -> Option<bool> {
        self.votes.get(&(who, vote_index)).cloned()
    }
}