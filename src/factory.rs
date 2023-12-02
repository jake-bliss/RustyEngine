use super::factory as ce_factory;
use super::models as ce_models;
use chrono::Datelike;
use chrono::{Local, NaiveDate, NaiveDateTime};
use faker_rand::en_us::addresses::{Address, CityName, PostalCode, SecondaryAddress};
use faker_rand::en_us::internet::Email;
use faker_rand::en_us::names::{FirstName, LastName};
use faker_rand::en_us::phones::PhoneNumber;
use faker_rand::lorem::{Sentence, Word};
use rand::Rng;

pub fn create_fake_order(customer_id: i32) -> ce_models::Order {
    //Set an orderID for the order
    let order_id = Rng::gen_range(&mut rand::thread_rng(), 1..1000);

    ce_models::Order {
        company_id: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        order_id: order_id,
        customer_id: customer_id,
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
        total: generate_random_float_two_decimals(1.0, 500.0),
        sub_total: generate_random_float_two_decimals(1.0, 500.0),
        tax_total: generate_random_float_two_decimals(1.0, 500.0),
        shipping_total: generate_random_float_two_decimals(1.0, 500.0),
        discount_total: generate_random_float_two_decimals(1.0, 500.0),
        discount_percent: generate_random_float_two_decimals(1.0, 500.0),
        weight_total: generate_random_float_two_decimals(1.0, 500.0),
        business_volume_total: generate_random_float_two_decimals(1.0, 500.0),
        commissionable_volume_total: generate_random_float_two_decimals(1.0, 500.0),
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
        shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
        order_tax: generate_random_float_two_decimals(1.0, 500.0),
        fed_tax_total: generate_random_float_two_decimals(1.0, 500.0),
        state_tax_total: generate_random_float_two_decimals(1.0, 500.0),
        fed_shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
        state_shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
        city_local_shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
        city_shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
        county_shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
        county_local_shipping_tax: generate_random_float_two_decimals(1.0, 500.0),
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
        total_taxable: generate_random_float_two_decimals(1.0, 500.0),
        order_sub_status_id: None,
        referral_id: None,
        //Random number of order details
        order_details: (0..Rng::gen_range(&mut rand::thread_rng(), 1..10))
            .enumerate()
            .map(|(index, _)| create_fake_order_detail(order_id, index as i32))
            .collect(),
    }
}

pub fn create_fake_order_detail(order_id: i32, order_line: i32) -> ce_models::OrderDetail {
    ce_models::OrderDetail {
        order_id,
        order_line,
        order_detail_id: None,
        parent_order_detail_id: None,
        item_id: Rng::gen_range(&mut rand::thread_rng(), 1..1000),
        item_code: rand::random::<Word>().to_string(),
        item_description: rand::random::<Sentence>().to_string(),
        quantity: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        price_each: generate_random_float_two_decimals(1.0, 500.0),
        price_total: generate_random_float_two_decimals(1.0, 500.0),
        tax: generate_random_float_two_decimals(1.0, 500.0),
        weight_each: generate_random_float_two_decimals(1.0, 500.0),
        weight: generate_random_float_two_decimals(1.0, 500.0),
        business_volume_each: generate_random_float_two_decimals(1.0, 500.0),
        business_volume: generate_random_float_two_decimals(1.0, 500.0),
        commissionable_volume_each: generate_random_float_two_decimals(1.0, 500.0),
        commissionable_volume: generate_random_float_two_decimals(1.0, 500.0),
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
        original_taxable_each: generate_random_float_two_decimals(1.0, 500.0),
        original_business_volume_each: generate_random_float_two_decimals(1.0, 500.0),
        original_commissionable_volume_each: generate_random_float_two_decimals(1.0, 500.0),
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
        taxable: generate_random_float_two_decimals(1.0, 500.0),
        fed_tax: generate_random_float_two_decimals(1.0, 500.0),
        state_tax: generate_random_float_two_decimals(1.0, 500.0),
        city_tax: generate_random_float_two_decimals(1.0, 500.0),
        city_local_tax: generate_random_float_two_decimals(1.0, 500.0),
        county_tax: generate_random_float_two_decimals(1.0, 500.0),
        county_local_tax: generate_random_float_two_decimals(1.0, 500.0),
        manual_tax: None,
        is_state_tax_override: false,
        reference1: Some("Ref001".to_string()),
    }
}

pub fn create_fake_customer() -> ce_models::Customer {
    ce_models::Customer {
        customer_id: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        company_id: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        customer_type_id: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        customer_status_id: Rng::gen_range(&mut rand::thread_rng(), 1..10),
        customer_sub_status_id: None,
        enroller_id: None,
        sponsor_id: None,
        binary_placement_id: None,
    }
}

pub fn create_fake_tree(company_id: i32) -> ce_models::Tree {
    ce_models::Tree {
        tree_id: Rng::gen_range(&mut rand::thread_rng(), 1..1000),
        tree_type: ce_models::TreeType::Unilevel,
        tree_name: "Main".to_string(),
        company_id: company_id,
        is_active: true,
        created_date: NaiveDate::from_ymd_opt(2023, 4, 30)
            .expect("Invalid date")
            .and_hms_opt(12, 0, 0)
            .expect("Invalid time"),
        modified_date: NaiveDate::from_ymd_opt(2023, 4, 30)
            .expect("Invalid date")
            .and_hms_opt(12, 0, 0)
            .expect("Invalid time"),
        created_by: "Jake Test".to_string(),
        modified_by: None,
        top_node_customer_id: Rng::gen_range(&mut rand::thread_rng(), 1..1000),
    }
}

pub fn create_fake_company() -> ce_models::Company {
    let company_id: i32 = Rng::gen_range(&mut rand::thread_rng(), 1..2);

    ce_models::Company {
        company_id: company_id,
        company_name: format!("Test Company {}", company_id),
        tree_types: vec![ce_models::TreeType::Unilevel],
    }
}

// Params should be start and end date
pub fn create_fake_period(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    period_id: i32,
    period_type: ce_models::PeriodType,
) -> ce_models::Period {
    let now = Local::now();

    ce_models::Period {
        period_id: period_id,
        period_type: period_type,
        period_name: start_date.format("%B %Y").to_string(),
        period_start_date: start_date,
        period_end_date: end_date,
        period_status: ce_models::PeriodStatus::Open,
        company_id: 1,
        created_date: now.naive_local(),
        modified_date: now.naive_local(),
        created_by: "Jake Test".to_string(),
        modified_by: None,
    }
}

fn generate_random_float_two_decimals(start: f64, end: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen_range(start..end);
    let rounded_number = (random_number * 100.0).round() / 100.0;
    rounded_number
}

// Return all Vectors it creates
pub fn _generate_test_data(
    number_of_customers: i32,
    orders_per_customer: i32,
) -> (
    Vec<chrono::NaiveDateTime>,
    ce_models::Company,
    ce_models::Tree,
    Vec<ce_models::Customer>,
    Vec<ce_models::Order>,
    Vec<ce_models::Period>,
) {
    //Create a vector of 364 Date Times
    let mut dates: Vec<chrono::NaiveDateTime> = Vec::new();

    //Set the start date of Jan 1st 2023
    let mut date = chrono::NaiveDate::from_ymd_opt(2023, 1, 1)
        .expect("Invalid date")
        .and_hms_opt(12, 0, 0)
        .expect("Invalid time");

    //Add all dates to the vector
    for _i in 1..365 {
        dates.push(date);

        //Add 1 day to the date
        date = date
            .checked_add_signed(chrono::Duration::days(1))
            .expect("Invalid date");
    }

    //Create a Company
    let company: ce_models::Company = ce_factory::create_fake_company();

    //Create a Tree
    let mut tree: ce_models::Tree = ce_factory::create_fake_tree(company.company_id);

    //Create Vector for Customers
    let mut customers: Vec<ce_models::Customer> = Vec::new();

    //Create Vector for Orders
    let mut orders: Vec<ce_models::Order> = Vec::new();

    //Generate customers and Orders for each customer
    for i in 1..number_of_customers {
        let mut customer = ce_factory::create_fake_customer();

        customer.customer_id = i;

        //Set the EnrollerID for the customer
        if i > 1 {
            customer.enroller_id = Some(i - 1);
        }

        //Set the EnrollerID for the customer randomly as more customers enter the tree
        if i > 2 {
            customer.enroller_id = Some(Rng::gen_range(&mut rand::thread_rng(), 1..(i - 1)));
        }

        //Generate 1-3 orders for each customer
        for _j in 1..=Rng::gen_range(&mut rand::thread_rng(), 1..orders_per_customer) {
            let mut order = ce_factory::create_fake_order(customer.customer_id);

            //Get a random date from the dates vector
            let date = dates[Rng::gen_range(&mut rand::thread_rng(), 0..(dates.len() - 1))];

            //Set the order date and created date to the random date
            order.created_date = date;
            order.order_date = date;

            orders.push(order);
        }

        // Add the customer to the vector
        customers.push(customer);
    }

    //Set Tree Top Node Customer ID
    tree.top_node_customer_id = customers.first().unwrap().customer_id;

    // Create a Vector of Periods
    let mut periods: Vec<ce_models::Period> = Vec::new();

    // Add 12 monthly periods to the vector
    for i in 1..13 {
        // First start date should be jan 1 2023
        let start_date = chrono::NaiveDate::from_ymd_opt(2023, i, 1)
            .expect("Invalid date")
            .and_hms_opt(12, 0, 0)
            .expect("Invalid time");

        // End date should be start date Adding 1 month and then subtracting 1 day
        let mut end_date = start_date
            .checked_add_signed(chrono::Duration::days(35))
            .expect("Invalid date");

        // Set day to 1 and subtract 1 second
        end_date = end_date
            .with_day(1)
            .expect("Invalid date")
            .checked_sub_signed(chrono::Duration::days(1))
            .expect("Invalid date")
            .checked_sub_signed(chrono::Duration::seconds(1))
            .expect("Invalid date");

        // Conver i to i32
        let i_32 = i as i32;

        let period = ce_factory::create_fake_period(
            start_date,
            end_date,
            i_32,
            ce_models::PeriodType::Monthly,
        );

        periods.push(period);
    }

    //Return all Vectors
    (dates, company, tree, customers, orders, periods)
}
