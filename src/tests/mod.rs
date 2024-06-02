use crate::models::{
    expense_category::ExpenseCategory, expense_subcategory::ExpenseSubcategory, expenses::Expense,
};
use crate::Database;
use chrono::prelude::*;
use chrono_tz::America::Sao_Paulo;
use tempfile::NamedTempFile;
use rusqlite::Result;

fn create_test_db() -> Database {
    let temp_file: NamedTempFile = NamedTempFile::new().unwrap();
    let db = Database::new(temp_file.path().to_str().unwrap()).unwrap();
    db.init().unwrap();
}

fn create_test_category() -> ExpenseCategory {
    ExpenseCategory {
        id: 1,
        name: String::from("Test Category"),
    }
}

fn create_test_subcategory() -> ExpenseSubcategory {
    ExpenseSubcategory {
        id: 1,
        name: String::from("Test Subcategory"),
        category: create_test_category(),
    }
}

fn create_test_expense() -> Expense {
    Expense {
        id: 1,
        amount: 22.22,
        description: String::from("Test Expense"),
        date: Sao_Paulo.with_ymd_and_hms(2024, 5, 31, 12, 00, 00).unwrap(),
        category: create_test_category(),
        subcategory: create_test_subcategory(),
    }
}

#[test]
fn test_create_expense() {
    let expense = create_test_expense();

    assert_eq!(expense.id, 1);
    assert_eq!(expense.amount, 22.22);
    assert_eq!(expense.description, String::from("Test Expense"));
    assert_eq!(
        expense.date,
        Sao_Paulo.with_ymd_and_hms(2024, 5, 31, 12, 00, 00).unwrap()
    );
    assert_eq!(expense.category.name, String::from("Test Category"));
    assert_eq!(expense.subcategory.name, String::from("Test Subcategory"));
    assert_eq!(expense.date.timezone(), Sao_Paulo);
}

#[test]
fn test_update_expense_amount() {
    let mut expense = create_test_expense();
    expense.amount = 200.0;

    assert_eq!(expense.amount, 200.0);
}

#[test]
fn test_update_expense_description() {
    let mut expense = create_test_expense();
    expense.description = String::from("Update description");
    assert_eq!(expense.description, String::from("Update description"));
}
