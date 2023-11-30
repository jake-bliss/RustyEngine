#[cfg(test)]

mod customer_tests {
    use commission_engine::models::Customer;

    #[test]

    fn test_customer_creation() {
        let customer = Customer {
            customer_id: 1,
            company_id: 1,
            customer_type_id: 1,
            customer_status_id: 1,
            customer_sub_status_id: None,
            enroller_id: None,
            sponsor_id: None,
            binary_placement_id: None,
        };

        assert_eq!(customer.customer_id, 1);
        assert_eq!(customer.company_id, 1);
        assert_eq!(customer.customer_type_id, 1);
        assert_eq!(customer.customer_status_id, 1);
        assert_eq!(customer.customer_sub_status_id, None);
        assert_eq!(customer.enroller_id, None);
        assert_eq!(customer.sponsor_id, None);
        assert_eq!(customer.binary_placement_id, None);
    }
}
