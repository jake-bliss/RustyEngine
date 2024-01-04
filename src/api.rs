use commission_engine::database as ce_database;
use commission_engine::models as ce_models;
use dotenv::dotenv;
use std::env;
use std::error::Error as StdError;
use warp::reject::Reject;
use warp::Filter;

// Define your API endpoints
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello_route()
        .or(create_customer_route())
        .or(order_route())
        .or(company_route())
        .or(get_bonuses_route())
        .or(get_bonuses_by_customer_route())
}

//Handle Custom Errors

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

fn hello_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello" / String).map(|name| format!("Hello, {}!", name))
}

// Authenticated Route
fn authenticate() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Copy {
    dotenv().ok(); // Load the .env file

    warp::header::optional::<String>("Authorization").and_then(
        |header_value: Option<String>| async move {
            match header_value {
                Some(token) => {
                    // Get the token value from the .env file
                    let expected_token = env::var("TOKEN").unwrap_or_default();

                    // Validate the token
                    if token == expected_token {
                        Ok(())
                    } else {
                        Err(warp::reject::custom(CustomError {}))
                    }
                }
                None => Err(warp::reject::custom(CustomError {})),
            }
        },
    )
}

//This Route will recieve a JSON object and return a JSON object
fn create_customer_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate()
        .and(warp::path!("create_customer"))
        .and(warp::post())
        .and(warp::body::json())
        .map(|_, customer: ce_models::Customer| {
            // Modify the closure to take two arguments
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

fn order_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate()
        .and(warp::path!("order"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|_, order: ce_models::Order| async move {
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

fn company_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate()
        .and(warp::path!("company"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|_, company: ce_models::Company| async move {
            println!("Company: {:?}", company);

            // Perform the database operation here
            match ce_database::create_company(company.clone()).await {
                Ok(_) => Ok(warp::reply::json(&company)),
                Err(e) => {
                    eprintln!("Error creating company: {}", e);
                    Err(warp::reject::custom(CustomError {}))
                }
            }
        })
}

#[derive(serde::Deserialize)]
struct GetBonusesRequest {
    company_id: i32,
    bonus_id: Option<i32>,
}

fn get_bonuses_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate()
        .and(warp::path!("bonuses"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|_, request: GetBonusesRequest| async move {
            // Perform the database operation here
            match ce_database::get_bonuses(request.company_id, request.bonus_id).await {
                Ok(bonuses) => Ok(warp::reply::json(&bonuses)),
                Err(e) => {
                    eprintln!("Error retrieving bonuses: {}", e);
                    Err(warp::reject::custom(CustomError {}))
                }
            }
        })
}

#[derive(serde::Deserialize)]

struct GetBonusesByCustomerRequest {
    company_id: i32,
    customer_id: i32,
}

fn get_bonuses_by_customer_route(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    authenticate()
        .and(warp::path!("bonuses_by_customer"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|_, request: GetBonusesByCustomerRequest| async move {
            // Perform the database operation here
            match ce_database::get_bonuses_by_customer(request.company_id, request.customer_id)
                .await
            {
                Ok(bonuses) => Ok(warp::reply::json(&bonuses)),
                Err(e) => {
                    eprintln!("Error retrieving bonuses: {}", e);
                    Err(warp::reject::custom(CustomError {}))
                }
            }
        })
}
