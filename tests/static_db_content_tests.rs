#[cfg(test)]

mod customer_tests {
    use commission_engine::database as ce_database;
    use commission_engine::models as ce_models;

    #[test]

    fn test_period_statuses() {
        let result = ce_database::get_period_statuses();

        assert!(result.is_ok());

        let period_statuses = result.unwrap();

        assert_eq!(period_statuses[0].period_status_id, 1);
        assert_eq!(period_statuses[0].period_status_description, "Open");

        assert_eq!(period_statuses[1].period_status_id, 2);
        assert_eq!(period_statuses[1].period_status_description, "Closed");
    }

    #[test]

    fn test_period_types() {
        let result = ce_database::get_period_types();

        assert!(result.is_ok());

        let period_types = result.unwrap();

        assert_eq!(period_types[0].period_type_id, 1);
        assert_eq!(period_types[0].period_type_description, "Monthly");

        assert_eq!(period_types[1].period_type_id, 2);
        assert_eq!(period_types[1].period_type_description, "Weekly");
    }
}
