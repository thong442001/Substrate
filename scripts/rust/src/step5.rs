// kết hợp thông qua trait
// dựa vào chức năng của trait để quyết định cách triển khai
usr std::collections::HashMap;
use std::hash::Hash;

// *** Associated type ***
pub trait Config {
    type AccountId: Copy + Eq + Hash;
    type Balance: Copy + PartialOrd + std::ops::Add<Output=Self::Balance> + std::ops::Sub<Output=Self::Balance> + From<u32>;
    type VoteIndex: Copy + Eq + Hash + From<u32>;

    // mình ko định nghĩa hàm trong trait này
}

// *********************
// *** BalanceModule ***
// *********************
pub struct BalanceModule<T: Config> {
    balances: HashMap<T::AccountId, T::Balance>,
}

// *** sử dụng associated type từ trait Config ***
impl <T: Config> BalanceModule<T> {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.balances.insert(who, amount);
    }

    // trả về kiểu Balance từ associated type
    // tức là T::Balance
    pub fn get_balance(&self, who: T::AccountId) -> T::Balance {
        *self.balances.get(&who).unwrap_or(&T::Balance::from(0))
    }

    pub fn transfer(&mut self, from: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let from_balance = self.get_balance(from);
        if from_balance < amount {
            return Err("Insufficient balance");
        }

        let to_balance = self.get_balance(to);
        self.balances.insert(from, from_balance - amount);
        self.balances.insert(to, to_balance + amount);
        Ok(())
    }
}


// ******************
// *** VoteModule ***
// ******************
pub struct VoteModule<T: Config> {
    votes: HashMap<(T::AccountId, T::VoteIndex), bool>,
}

impl <T: Config> VoteModule<T> {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
        }
    }

    pub fn cast_vote(&mut self, who: T::AccountId, vote_index: T::VoteIndex, approve: bool) {
        self.votes.insert((who, vote_index), approve);
    }

    pub fn get_vote(&self, who: T::AccountId, vote_index: T::VoteIndex) -> Option<bool> {
        self.votes.get(&(who, vote_index)).cloned()
    }
}