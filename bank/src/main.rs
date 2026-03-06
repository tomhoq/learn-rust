#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

#[derive(Debug)]
struct Account {
    balance: isize,
    id: usize,
    holder: String,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }

    fn get_total_balance(&self) -> isize {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn get_summaries(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.create_summary())
            .collect::<Vec<String>>()
    }
}

impl Account {
    fn new(id: usize, holder: String) -> Self {
        Account {
            balance: 0,
            id,
            holder,
        }
    }

    fn deposit(&mut self, amount: isize) {
        self.balance += amount;
    }

    fn withraw(&mut self, amount: isize) {
        self.balance -= amount;
    }

    fn create_summary(&self) -> String {
        format!("Account {} has a balance of {}", self.id, self.balance)
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Tomaz"));
    let account2 = Account::new(2, String::from("Tomaz"));

    bank.add_account(account);
    bank.add_account(account2);

    println!("{:#?}", bank.get_summaries())
}

#[cfg(test)]
mod tests;
