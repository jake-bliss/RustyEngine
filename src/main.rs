// main.rs
mod database;
mod factory;
mod models;

// use commission_engine::database as ce_database;
use commission_engine::factory as ce_factory;
use commission_engine::models as ce_models;
use sqlx::mysql::MySqlPool;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate Test Data
    // let (dates, company, tree, customers, orders, periods) =
    //     ce_factory::generate_test_data(100, 10);

    // // Find all orders that are in the first period
    // // These are references to Orders so we can't mutate them
    // let first_period = periods.first().unwrap();

    // let orders_in_period: Vec<&ce_models::Order> = orders
    //     .iter()
    //     .filter(|&order| {
    //         order.order_date >= first_period.period_start_date
    //             && order.order_date <= first_period.period_end_date
    //     })
    //     .collect();

    // //Print Each order in the first period and their OrderID, CustomerID, and Order Date
    // for order in orders_in_period.iter() {
    //     println!(
    //         "Order ID: {}, Customer ID: {}, Order Date: {}",
    //         order.order_id, order.customer_id, order.order_date
    //     );
    // }

    // //Create a Vector of Bonuses
    // let mut bonuses: Vec<ce_models::Bonus> = Vec::new();

    // //Iterate over orders in the first period
    // //We will want to create a bonus for each order if applicable
    // for order in orders_in_period.iter() {
    //     //Find the customer that placed the order
    //     let customer = customers
    //         .iter()
    //         .find(|&customer| customer.customer_id == order.customer_id)
    //         .unwrap();

    //     //See if the customer has an enroller
    //     if let Some(enroller_id) = customer.enroller_id {
    //         //Create a bonus for the enroller
    //         let bonus = ce_models::Bonus {
    //             bonus_id: 1,
    //             bonus_name: "Retail Bonus".to_string(),
    //             bonus_percentage: 20.0,
    //             bonus_amount: order.commissionable_volume_total * 0.2,
    //             to_customer_id: enroller_id,
    //             source_customer_id: Some(customer.customer_id),
    //             source_order_id: Some(order.order_id),
    //         };

    //         //Add the bonus to the vector
    //         bonuses.push(bonus);
    //     }
    // }

    // //Print Each BonusID and their BonusName, BonusPercentage, BonusAmount, ToCustomerID, SourceCustomerID, and SourceOrderID
    // for bonus in bonuses.iter() {
    //     println!(
    //         "Bonus ID: {}, Bonus Name: {}, Bonus Percentage: {}, Bonus Amount: {}, To Customer ID: {}, Source Customer ID: {:?}, Source Order ID: {:?}",
    //         bonus.bonus_id,
    //         bonus.bonus_name,
    //         bonus.bonus_percentage,
    //         bonus.bonus_amount,
    //         bonus.to_customer_id,
    //         bonus.source_customer_id,
    //         bonus.source_order_id
    //     );
    // }

    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //print pool
    println!("{:#?}", pool);

    //Print each row of the table
    let period_types = sqlx::query!("SELECT * FROM PeriodType;",)
        .map(|row| ce_models::PeriodType {
            period_type_id: row.period_type_id,
            period_type_description: row.type_description.unwrap_or_default(),
        })
        .fetch_all(&pool)
        .await?;

    for period_type in period_types.iter() {
        println!(
            "Period Type ID: {}, Period Type Description: {}",
            period_type.period_type_id, period_type.period_type_description
        );
    }

    Ok(())
}
