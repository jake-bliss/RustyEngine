#[cfg(test)]
mod order_tests {
    use commission_engine::models::Order;
    use commission_engine::models::OrderDetail;

    // use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_order_creation() {
        let order = Order {
            order_id: 1,
            customer_id: 10,
            order_status_id: 2,
            order_date: NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            currency_code: "USD".to_string(),
            warehouse_id: 3,
            ship_method_id: 1,
            order_type_id: 1,
            price_type_id: 2,
            first_name: "John".to_string(),
            middle_name: Some("A.".to_string()),
            last_name: "Doe".to_string(),
            name_suffix: None,
            company: None,
            address1: "123 Main St.".to_string(),
            address2: None,
            address3: None,
            city: "Anytown".to_string(),
            state: "NY".to_string(),
            zip: "12345".to_string(),
            country: "USA".to_string(),
            county: None,
            email: None,
            phone: None,
            notes: None,
            total: 0.0,
            sub_total: 0.0,
            tax_total: 0.0,
            shipping_total: 0.0,
            discount_total: 0.0,
            discount_percent: 0.0,
            weight_total: 0.0,
            business_volume_total: 0.0,
            commissionable_volume_total: 0.0,
            other1_total: None,
            other2_total: None,
            other3_total: None,
            other4_total: None,
            other5_total: None,
            other6_total: None,
            other7_total: None,
            other8_total: None,
            other9_total: None,
            other10_total: None,
            shipping_tax: 0.0,
            order_tax: 0.0,
            fed_tax_total: 0.0,
            state_tax_total: 0.0,
            fed_shipping_tax: 0.0,
            state_shipping_tax: 0.0,
            city_shipping_tax: 0.0,
            city_local_shipping_tax: 0.0,
            county_shipping_tax: 0.0,
            county_local_shipping_tax: 0.0,
            other11: None,
            other12: None,
            other13: None,
            other14: None,
            other15: None,
            other16: None,
            other17: None,
            other18: None,
            other19: None,
            other20: None,
            is_commissionable: true,
            auto_order_id: None,
            return_order_id: None,
            replacement_order_id: None,
            parent_order_id: None,
            decline_count: None,
            transfer_to_customer_id: None,
            party_id: None,
            shipped_date: None,
            created_date: NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            locked_date: None,
            modified_date: NaiveDate::from_ymd_opt(2023, 4, 30)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
            created_by: "Jake".to_string(),
            modified_by: None,
            tax_integration_calculate: None,
            tax_integration_commit: None,
            handling_fee: None,
            pickup_name: None,
            total_taxable: 0.0,
            order_sub_status_id: None,
            referral_id: None,
            order_details: vec![OrderDetail {
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
            }],
        };

        assert_eq!(order.order_id, 1);
        assert_eq!(order.customer_id, 10);
        assert!(order.middle_name.is_some());
        assert_eq!(order.middle_name.unwrap(), "A.");
        assert!(order.referral_id.is_none());
        assert!(order.order_details.len() == 1);
        assert_eq!(order.order_details[0].order_id, 1);
        // ... additional assertions for other fields ...
    }

    // Additional tests can be added here...
}
