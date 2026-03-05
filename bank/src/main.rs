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
}

impl Account {
    fn new(id: usize, holder: String) -> Self {
        Account {
            balance: 0,
            id,
            holder,
        }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_accounts(accounts: Vec<Account>) {
    for account in accounts {
        println!("{:#?}", account);
    }
}

fn print_bank(bank: Bank) {
    println!("{:#?}", bank);
}

fn print_num_accounts(bank: &Bank) {
    println!("{:#?}", bank.accounts.len());
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("Tomaz"));
    let account2 = Account::new(2, String::from("Tomaz"));

    let account_ref = &account;
    print_account(&account_ref);

    bank.accounts.push(account);
    bank.accounts.push(account2);

    print_num_accounts(&bank);
    print_bank(bank);
}
