#[cfg(test)]
mod order_detail_tests {
    use commission_engine::models::OrderDetail;

    #[test]
    fn test_order_detail_creation() {
        let order_detail = OrderDetail {
            order_id: 1,
            order_line: 1,
            order_detail_id: Some(100),
            parent_order_detail_id: None,
            item_id: 50,
            item_code: "ITEM001".to_string(),
            item_description: "Test Item".to_string(),
            quantity: 1,
            price_each: 10.0,
            price_total: 10.0,
            tax: 0.0,
            weight_each: 0.0,
            weight: 0.0,
            business_volume_each: 0.0,
            business_volume: 0.0,
            commissionable_volume_each: 0.0,
            commissionable_volume: 0.0,
            other1_each: None,
            other1: None,
            other2_each: None,
            other2: None,
            other3_each: None,
            other3: None,
            other4_each: None,
            other4: None,
            other5_each: None,
            other5: None,
            original_taxable_each: 0.0,
            original_business_volume_each: 0.0,
            original_commissionable_volume_each: 0.0,
            other6_each: None,
            other6: None,
            other7_each: None,
            other7: None,
            other8_each: None,
            other8: None,
            other9_each: None,
            other9: None,
            other10_each: None,
            other10: None,
            parent_item_id: None,
            taxable: 0.0,
            fed_tax: 0.0,
            state_tax: 0.0,
            city_tax: 0.0,
            city_local_tax: 0.0,
            county_tax: 0.0,
            county_local_tax: 0.0,
            manual_tax: None,
            is_state_tax_override: false,
            reference1: Some("Ref001".to_string()),
        };

        assert_eq!(order_detail.order_id, 1);
        assert_eq!(order_detail.order_line, 1);
        assert!(order_detail.order_detail_id.is_some());
        assert_eq!(order_detail.order_detail_id.unwrap(), 100);
        assert!(order_detail.parent_order_detail_id.is_none());
        assert!(order_detail.reference1.is_some());
        // ... additional assertions for other fields ...
    }

    // Additional tests can be added here...
}
