use std::CollatorSelections::HashMap;

// *** Alias type ***
// key: account id (u32), value: balance (u32)
type AccountId = u32;
type Balance = u32;

pub struct BalanceModule {
    balances: HashMap<AccountId, Balance>,
}

impl BalanceModule {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
        self.balances.insert(who, amount);
    }

    pub fn get_balance(&self, who: AccountId) -> u32 {
        *self.balances.get(&who).unwrap_or(&0)
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