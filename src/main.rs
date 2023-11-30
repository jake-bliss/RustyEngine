// main.rs
mod factory;
mod models;

use factory::create_fake_order;
fn main() {
    let order = create_fake_order();

    // Print the order and its details
    println!(
        "Order ID: {}, Company ID: {}",
        order.order_id, order.company_id
    );
    for detail in order.order_details.iter() {
        println!(
            "Order Detail - Item ID: {}, Item Code: {}, Quantity: {}",
            detail.item_id, detail.item_code, detail.quantity
        );
        // Add more fields to print as per your requirement
    }
}
