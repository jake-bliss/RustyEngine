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

    #[test]

    fn test_tree_types() {
        let result = ce_database::get_tree_types();

        assert!(result.is_ok());

        let tree_types = result.unwrap();

        assert_eq!(tree_types[0].tree_type_id, 1);
        assert_eq!(tree_types[0].tree_type_description, "Enroller");

        assert_eq!(tree_types[1].tree_type_id, 2);
        assert_eq!(tree_types[1].tree_type_description, "Unilevel");

        assert_eq!(tree_types[2].tree_type_id, 3);
        assert_eq!(tree_types[2].tree_type_description, "Binary");
    }

    #[test]

    fn test_get_periods() {
        let result = ce_database::get_periods();

        assert!(result.is_ok());

        //Unwrap the result
        let periods = result.unwrap();

        //Assert that the first period is correct
        assert_eq!(periods[0].period_id, 1);
        assert_eq!(periods[0].period_type_id, 1);
        assert_eq!(periods[0].period_status_id, 1);
        assert_eq!(periods[0].company_id, 1);
    }

    #[test]

    fn test_get_trees() {
        let result = ce_database::get_trees();

        assert!(result.is_ok());

        //Unwrap the result
        let trees = result.unwrap();

        //Assert that the first tree is correct
        assert_eq!(trees[0].tree_id, 1);
        assert_eq!(trees[0].tree_name, "Enroller Tree");
        assert_eq!(trees[0].tree_type_id, 1);
        assert_eq!(trees[0].company_id, 1);
        assert_eq!(trees[0].is_active, true);
        assert_eq!(trees[0].created_by, "System");
        assert_eq!(trees[0].modified_by, None);
        assert_eq!(trees[0].top_node_customer_id, 1);
    }
}
