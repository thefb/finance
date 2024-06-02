use super::expense_category::*;

#[derive(Debug, Clone)]
pub struct ExpenseSubcategory {
    pub id: i32,
    pub name: String,
    pub category: ExpenseCategory,
}
