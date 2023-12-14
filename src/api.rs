use commission_engine::database as ce_database;
use commission_engine::models as ce_models;
use std::error::Error as StdError;
use warp::reject::Reject;
use warp::Filter;

// Define your API endpoints
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello_route().or(create_customer_route()).or(order_route())
}

fn hello_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello" / String).map(|name| format!("Hello, {}!", name))
}

//This Route will recieve a JSON object and return a JSON object
fn create_customer_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create_customer")
        .and(warp::post())
        .and(warp::body::json())
        .map(|customer: ce_models::Customer| {
            println!("Customer: {:?}", customer);
            warp::reply::json(&customer)
        })
}

async fn order_handler(order: ce_models::Order) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Order: {:?}", order);

    // Create order in database
    ce_database::create_order(order.clone()).await;

    Ok(warp::reply::json(&order))
}

#[derive(Debug)]
struct CustomError {
    // Your error fields here
}

impl StdError for CustomError {}

impl Reject for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the formatting logic for CustomError here
        write!(f, "CustomError")
    }
}

fn order_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("order")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|order: ce_models::Order| async move {
            println!("Order: {:?}", order);

            // Perform the database operation here
            match ce_database::create_order(order.clone()).await {
                Ok(_) => Ok(warp::reply::json(&order)),
                Err(e) => {
                    eprintln!("Error creating order: {}", e);
                    Err(warp::reject::custom(CustomError {}))
                }
            }
        })
}
