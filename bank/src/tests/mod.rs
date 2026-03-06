use super::*;

#[test]
fn test_bank_creation() {
    let bank = Bank::new();
    assert_eq!(bank.accounts.len(), 0);
}

#[test]
fn test_account_creation() {
    let account = Account::new(1, String::from("Tomaz"));
    assert_eq!(account.balance, 0);
    assert_eq!(account.id, 1);
    assert_eq!(account.holder, "Tomaz");
}

#[test]
fn test_deposit() {
    let mut account = Account::new(1, String::from("Tomaz"));
    account.deposit(100);
    assert_eq!(account.balance, 100);
}

#[test]
fn test_withdraw() {
    let mut account = Account::new(1, String::from("Tomaz"));
    account.deposit(100);
    account.withraw(50); // Using the spelling from main.rs
    assert_eq!(account.balance, 50);
}

#[test]
fn test_account_summary() {
    let mut account = Account::new(1, String::from("Tomaz"));
    account.deposit(100);
    let summary = account.create_summary();
    assert_eq!(summary, "Account 1 has a balance of 100");
}

#[test]
fn test_bank_add_account() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("Tomaz"));
    bank.add_account(account);
    assert_eq!(bank.accounts.len(), 1);
}

#[test]
fn test_bank_total_balance() {
    let mut bank = Bank::new();
    let mut account1 = Account::new(1, String::from("Tomaz"));
    account1.deposit(150);

    let mut account2 = Account::new(2, String::from("Jerry"));
    account2.deposit(250);

    bank.add_account(account1);
    bank.add_account(account2);

    assert_eq!(bank.get_total_balance(), 400);
}
