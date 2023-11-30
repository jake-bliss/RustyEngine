#[cfg(test)]

mod data_creation_1_tests {
    use commission_engine::factory as ce_factory;
    use commission_engine::models as ce_models;
    use rand::Rng;

    #[test]

    fn test_data_generation() {
        //Create a Company
        let company: ce_models::Company = ce_factory::create_fake_company();

        //Create a Customer
        let master_customer: ce_models::Customer = ce_factory::create_fake_customer();

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

            if i > 1 {
                customer.enroller_id = Some(i - 1);
            }

            //Generate 1-3 orders for each customer
            for _j in 1..=Rng::gen_range(&mut rand::thread_rng(), 1..3) {
                let order = ce_factory::create_fake_order(customer.customer_id);

                orders.push(order);
            }

            // Add the customer to the vector
            customers.push(customer);
        }

        //Print Each CustomerID and their EnrollerID
        for customer in customers.iter() {
            println!(
                "Customer ID: {}, Enroller ID: {:?}",
                customer.customer_id, customer.enroller_id
            );
        }

        //Print Each OrderID and their CustomerID
        for order in orders.iter() {
            println!(
                "Order ID: {}, Customer ID: {}",
                order.order_id, order.customer_id
            );
        }

        assert!(company.company_id > 0);
        assert!(master_customer.customer_id > 0);
        assert!(tree.tree_id > 0);
        assert!(customers.len() == 10);
        //We need to test that some customers EnrollerID is < their CustomerID
        assert!(customers[2].enroller_id.unwrap() < customers[2].customer_id);
    }
}
