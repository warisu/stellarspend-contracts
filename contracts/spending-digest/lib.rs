pub struct SpendingSnapshotInput {
    pub total_spent: u128,
    pub remaining_balance: u128,
    pub active_budgets: Vec<BudgetSnapshot>,
}