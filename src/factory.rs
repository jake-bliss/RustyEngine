use chrono::NaiveDate;
use commission_engine::models::{Order, OrderDetail};
use faker_rand::en_us::addresses::{Address, CityName, PostalCode, SecondaryAddress};
use faker_rand::en_us::internet::Email;
use faker_rand::en_us::names::{FirstName, LastName};
use faker_rand::en_us::phones::PhoneNumber;
use faker_rand::lorem::{Sentence, Word};
use rand::Rng;

pub fn create_fake_order() -> Order {
    //Set an orderID for the order
    let order_id = Rng::gen_range(&mut rand::thread_rng(), 1..1000);

    Order {
        order_id: order_id,
        customer_id: Rng::gen_range(&mut rand::thread_rng(), 1..1000),
        order_status_id: Rng::gen_range(&mut rand::thread_rng(), 1..1000),
        order_date: NaiveDate::from_ymd_opt(2023, 4, 30)
            .expect("Invalid date")
            .and_hms_opt(12, 0, 0)
            .expect("Invalid time"),
        currency_code: "USD".to_string(),
        warehouse_id: Rng::gen_range(&mut rand::thread_rng(), 1..2),
        ship_method_id: Rng::gen_range(&mut rand::thread_rng(), 1..5),
        order_type_id: Rng::gen_range(&mut rand::thread_rng(), 1..14),
        price_type_id: Rng::gen_range(&mut rand::thread_rng(), 1..2),
        first_name: rand::random::<FirstName>().to_string(),
        middle_name: None,
        last_name: rand::random::<LastName>().to_string(),
        name_suffix: None,
        company: None,
        address1: rand::random::<Address>().to_string(),
        address2: Some(rand::random::<SecondaryAddress>().to_string()),
        address3: None,
        city: rand::random::<CityName>().to_string(),
        state: "NY".to_string(),
        zip: rand::random::<PostalCode>().to_string(),
        country: "US".to_string(),
        county: None,
        email: Some(rand::random::<Email>().to_string()),
        phone: Some(rand::random::<PhoneNumber>().to_string()),
        notes: Some(rand::random::<Sentence>().to_string()),
        total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        sub_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        tax_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        shipping_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        discount_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        discount_percent: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        weight_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        business_volume_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        commissionable_volume_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
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
        shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        order_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        fed_tax_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        state_tax_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        fed_shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        state_shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        city_local_shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        city_shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        county_shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        county_local_shipping_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
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
        shipped_date: Some(
            NaiveDate::from_ymd_opt(2023, 6, 6)
                .expect("Invalid date")
                .and_hms_opt(12, 0, 0)
                .expect("Invalid time"),
        ),
        created_date: NaiveDate::from_ymd_opt(2023, 5, 5)
            .expect("Invalid date")
            .and_hms_opt(12, 0, 0)
            .expect("Invalid time"),
        locked_date: None,
        modified_date: NaiveDate::from_ymd_opt(2023, 5, 5)
            .expect("Invalid date")
            .and_hms_opt(12, 0, 0)
            .expect("Invalid time"),
        created_by: "Factory".to_string(),
        modified_by: None,
        tax_integration_calculate: None,
        tax_integration_commit: None,
        handling_fee: None,
        pickup_name: None,
        total_taxable: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        order_sub_status_id: None,
        referral_id: None,
        //Random number of order details
        order_details: (0..Rng::gen_range(&mut rand::thread_rng(), 1..10))
            .enumerate()
            .map(|(index, _)| create_fake_order_detail(order_id, index as i32))
            .collect(),
    }
}

pub fn create_fake_order_detail(order_id: i32, order_line: i32) -> OrderDetail {
    OrderDetail {
        order_id,
        order_line,
        order_detail_id: None,
        parent_order_detail_id: None,
        item_id: Rng::gen_range(&mut rand::thread_rng(), 1..1000),
        item_code: rand::random::<Word>().to_string(),
        item_description: rand::random::<Sentence>().to_string(),
        quantity: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        price_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        price_total: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        weight_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        weight: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        business_volume_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        business_volume: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        commissionable_volume_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        commissionable_volume: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
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
        original_taxable_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        original_business_volume_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        original_commissionable_volume_each: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
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
        taxable: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        fed_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        state_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        city_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        city_local_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        county_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        county_local_tax: Rng::gen_range(&mut rand::thread_rng(), 1.0..500.0),
        manual_tax: None,
        is_state_tax_override: false,
        reference1: Some("Ref001".to_string()),
    }
}
