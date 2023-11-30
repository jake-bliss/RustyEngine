#[cfg(test)]

mod company_tests {
    use commission_engine::models::Company;

    #[test]

    fn test_company_creation() {
        let company = Company {
            company_id: 1,
            company_name: "Test Company".to_string(),
            tree_types: vec![],
        };

        assert_eq!(company.company_id, 1);
        assert_eq!(company.company_name, "Test Company".to_string());
        assert_eq!(company.tree_types, vec![]);
    }
}
