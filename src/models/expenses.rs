use super::expense_category::ExpenseCategory;
use super::expense_subcategory::*;
use chrono::DateTime;
use chrono_tz::Tz;

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: i32,
    pub amount: f32,
    pub description: String,
    pub date: DateTime<Tz>,
    pub category: ExpenseCategory,
    pub subcategory: ExpenseSubcategory,
}
