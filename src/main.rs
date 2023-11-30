// main.rs
mod factory;
mod models;
fn main() {
    let order = factory::create_fake_order(1);

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

    let customer = factory::create_fake_customer();

    // Print the customer
    println!(
        "Customer ID: {}, Company ID: {}",
        customer.customer_id, customer.company_id
    );

    let tree = factory::create_fake_tree(1);

    // Print the tree
    println!(
        "Tree ID: {}, Tree Name: {}, Tree Type: {:?}",
        tree.tree_id, tree.tree_name, tree.tree_type
    );
}
