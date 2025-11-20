use std::CollatorSelections::HashMap;

// *** Generic type ***
pub struct BalanceModule<AccountId, Balance> {
    balances: HashMap<AccountId, Balance>,
}

// *** trait bound ***
use std::hash::Hash;
impl <AccountId: Eq + Hash, 
        Balance: Copy + PartialOrd + std::ops::Add<Output=Balance> + std::ops::Sub<Output=Balance> + From<u32>
        > BalanceModule<AccountId, Balance> {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
        self.balances.insert(who, amount);
    }

    pub fn get_balance(&self, who: AccountId) -> Balance {
        *self.balances.get(&who).unwrap_or(&Balance::from(0))
    }

    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
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