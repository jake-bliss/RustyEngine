// main.rs
mod api;
mod database;
mod factory;
mod models;

// use commission_engine::database as ce_database;
use commission_engine::bonus as ce_bonus;
use commission_engine::database as ce_database;
use commission_engine::factory as ce_factory;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //Generate Test Data
    let (dates, company, tree, customers, orders, periods) = ce_factory::generate_test_data(10, 5);

    // Iterate over Customers and create them in the database
    for customer in customers.iter() {
        let result = ce_database::create_customer(customer.clone()).await;
        // println!("{:#?}", result);
    }

    // Create Orders in the database
    // Iterate over orders and create them in the database
    for order in orders.iter() {
        let result = ce_database::create_order(order.clone()).await;
        // println!("{:#?}", result);
    }

    let routes = api::routes();

    // Start the bonus calculation loop
    tokio::spawn(async move {
        check_for_orders_and_calculate_bonuses().await;
    });

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn check_for_orders_and_calculate_bonuses() {
    loop {
        sleep(Duration::from_secs(30)).await; // Sleep for 5 minutes

        let orders = match ce_database::get_orders().await {
            Ok(orders) => orders,
            Err(e) => {
                eprintln!("Failed to get orders: {}", e);
                continue;
            }
        };

        ce_bonus::calculate_bonuses(&orders).await;
    }
}
