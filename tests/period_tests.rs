#[cfg(test)]

mod tree_tests {
    use commission_engine::models::{Period, PeriodStatus, PeriodType};

    #[test]

    fn test_period_creation() {
        let period = Period {
            period_id: 1,
            period_type_id: PeriodType::Weekly,
            period_name: "Main".to_string(),
            period_start_date: chrono::NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            period_end_date: chrono::NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            period_status_id: PeriodStatus::Open,
            company_id: 1,
            created_date: chrono::NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            modified_date: chrono::NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            created_by: "Jake Test".to_string(),
            modified_by: None,
        };

        assert_eq!(period.period_id, 1);
        assert_eq!(period.period_type_id, PeriodType::Weekly);
        assert_eq!(period.period_name, "Main".to_string());
        assert_eq!(period.company_id, 1);
        assert_eq!(period.period_status_id, PeriodStatus::Open);
        assert_eq!(period.created_by, "Jake Test".to_string());
        assert_eq!(period.modified_by, None);
    }
}
