use warp::Filter;

// Define your API endpoints
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello_route()
}

fn hello_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello" / String).map(|name| format!("Hello, {}!", name))
}
