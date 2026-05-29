#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_event_once_per_cycle() {
        let mut state = DigestState::default();

        let input = SpendingSnapshotInput {
            total_spent: 500,
            remaining_balance: 1500,
            active_budgets: vec![],
        };

        let date = "2026-05-29".to_string();

        let first = run_daily_cycle(&mut state, "user1".to_string(), input.clone(), date.clone());
        assert!(first.is_some());

        let second = run_daily_cycle(&mut state, "user1".to_string(), input, date.clone());
        assert!(second.is_none());
    }

    #[test]
    fn emits_correct_values() {
        let mut state = DigestState::default();

        let input = SpendingSnapshotInput {
            total_spent: 200,
            remaining_balance: 800,
            active_budgets: vec![],
        };

        let event = run_daily_cycle(&mut state, "user1".to_string(), input, "2026-05-29".to_string())
            .unwrap();

        assert_eq!(event.total_spent, 200);
        assert_eq!(event.remaining_balance, 800);
    }
}