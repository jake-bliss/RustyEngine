// main.rs
mod factory;
mod models;

use chrono::Datelike;
use commission_engine::factory as ce_factory;
use commission_engine::models as ce_models;
use rand::Rng;

fn main() {
    //Create a Company
    let company: ce_models::Company = ce_factory::create_fake_company();

    //Create a Customer
    // let master_customer: ce_models::Customer = ce_factory::create_fake_customer();

    //Create a Tree
    let tree: ce_models::Tree = ce_factory::create_fake_tree(company.company_id);

    //Create Vector for Customers
    let mut customers: Vec<ce_models::Customer> = Vec::new();

    //Create Vector for Orders
    let mut orders: Vec<ce_models::Order> = Vec::new();

    //Generate 10 customers and Orders for each customer
    for i in 1..11 {
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
        for _j in 1..=Rng::gen_range(&mut rand::thread_rng(), 1..3) {
            let order = ce_factory::create_fake_order(customer.customer_id);

            orders.push(order);
        }

        // Add the customer to the vector
        customers.push(customer);
    }

    // //Print Each CustomerID and their EnrollerID
    // for customer in customers.iter() {
    //     println!(
    //         "Customer ID: {}, Enroller ID: {:?}",
    //         customer.customer_id, customer.enroller_id
    //     );
    // }

    // //Print Each OrderID and their CustomerID
    // for order in orders.iter() {
    //     println!(
    //         "Order ID: {}, Customer ID: {}",
    //         order.order_id, order.customer_id
    //     );
    //     // Print Business Volume and Commissionable Volume
    //     println!(
    //         "Business Volume: {}, Commissionable Volume: {}",
    //         order.business_volume_total, order.commissionable_volume_total
    //     );
    // }

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

    //Print Each PeriodID and their PeriodType, Start Date, End Date, and Period Status
    for period in periods.iter() {
        println!(
            "Period ID: {}, Period Type: {:?}, Start Date: {}, End Date: {}, Period Status: {:?}",
            period.period_id,
            period.period_type,
            period.period_start_date,
            period.period_end_date,
            period.period_status
        );

        //print period name
        println!("Period Name: {}", period.period_name);
    }
}
