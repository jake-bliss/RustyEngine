#[cfg(test)]

mod tree_tests {
    use commission_engine::models::{Tree, TreeType};

    #[test]

    fn test_tree_creation() {
        let tree = Tree {
            tree_id: 1,
            tree_type: TreeType::Unilevel,
            tree_name: "Main".to_string(),
            company_id: 1,
            is_active: true,
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
            top_node_customer_id: 1,
        };

        assert_eq!(tree.tree_id, 1);
        assert_eq!(tree.tree_type, TreeType::Unilevel);
        assert_eq!(tree.tree_name, "Main".to_string());
        assert_eq!(tree.company_id, 1);
        assert_eq!(tree.is_active, true);
        assert_eq!(tree.created_by, "Jake Test".to_string());
        assert_eq!(tree.modified_by, None);
        assert_eq!(tree.top_node_customer_id, 1);
    }
}
