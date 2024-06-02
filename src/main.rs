mod models;
mod tests;

use chrono::prelude::*;
use chrono_tz::America::Sao_Paulo;
use models::{
    db::Database, expense_category::ExpenseCategory, expense_subcategory::ExpenseSubcategory,
    expenses::Expense,
};

fn main() -> rusqlite::Result<()> {
    let db = Database::new("expenses.db")?;
    db.init()?;

    let category = ExpenseCategory {
        id: 1,
        name: String::from("Food"),
    };

    let subcategory = ExpenseSubcategory {
        id: 1,
        name: String::from("Groceries"),
        category: category.clone(),
    };

    let expense = Expense {
        id: 0,
        amount: 150.0,
        description: String::from("Weekly Groceries"),
        date: Sao_Paulo.with_ymd_and_hms(2024, 5, 31, 12, 0, 0).unwrap(),
        category,
        subcategory,
    };

    match db.add_expense(&expense){
        Ok(_) => println!("Expense added successfully"),
        Err(e) => println!("Failed to add expense: {}", e)
    }
    let expenses = db.get_expenses()?;
    for exp in expenses {
        println!("{:?}", exp);
    }
    Ok(())
}
