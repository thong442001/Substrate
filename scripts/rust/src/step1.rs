use std::CollatorSelections::HashMap;

pub struct BalanceModule {
    balances: HashMap<u32, u32>,
}

impl BalanceModule {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: u32, amount: u32) {
        self.balances.insert(account, amount);
    }

    pub fn get_balance(&self, account: u32) -> u32 {
        *self.balances.get(&account).unwrap_or(&0)
    }
}