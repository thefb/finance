use crate::models::{
    expense_category::ExpenseCategory, expense_subcategory::ExpenseSubcategory, expenses::Expense,
};
use chrono::prelude::*;
use chrono_tz::America::Sao_Paulo;
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;
        Ok(Database { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS expense_category (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS expense_subcategory (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS expense (
                id INTEGER PRIMARY KEY,
                amount REAL NOT NULL,
                description TEXT,
                date TEXT NOT NULL,
                category_id INTEGER NOT NULL,
                subcategory_id INTEGER,
                FOREIGN KEY(category_id) REFERENCES expense_category(id),
                FOREIGN KEY(subcategory_id) REFERENCES expense_subcategory(id)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_expense(&self, expense: &Expense) -> Result<()> {
        self.conn.execute(
            "INSERT INTO expense (amount, description, date, category_id, subcategory_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                expense.amount,
                expense.description,
                expense.date.to_rfc3339(),
                expense.category.id,
                expense.subcategory.id,
            ]
        )?;
        Ok(())
    }

    pub fn get_expenses(&self) -> Result<Vec<Expense>> {
        let mut stmt = self.conn.prepare(
            "SELECT e.id, e.amount, e.description, e.date, ec.id, ec.name, es.id, es.name
             FROM expense e
             JOIN expense_category ec ON e.category_id = ec.id
             LEFT JOIN expense_subcategory es ON e.subcategory_id = es.id",
        )?;
        let expense_iter = stmt.query_map([], |row| {
            let date: String = row.get(3)?;
            let date_time = date
                .parse::<DateTime<FixedOffset>>()
                .map(|dt| dt.with_timezone(&Sao_Paulo))
                .unwrap();

            let category = ExpenseCategory {
                id: row.get(4)?,
                name: row.get(5)?,
            };

            let subcategory = ExpenseSubcategory {
                id: row.get(6)?,
                name: row.get(7)?,
                category: ExpenseCategory {
                    id: row.get(8)?,
                    name: row.get(9)?,
                }
            };

            Ok(Expense {
                id: row.get(0)?,
                amount: row.get(1)?,
                description: row.get(2)?,
                date: date_time,
                category,
                subcategory
            })
        })?;

        let mut expenses = Vec::new();
        for expense in expense_iter {
            expenses.push(expense?);
        }
        Ok(expenses)
    }
}
