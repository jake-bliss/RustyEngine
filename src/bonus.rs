use super::database as ce_database;
use super::models as ce_models;

pub async fn calculate_bonuses(orders: &Vec<ce_models::Order>, pool: &sqlx::MySqlPool) {
    println!("Starting bonus calculation loop...");
    for order in orders {
        retail_bonus(order, pool).await;
    }
}

pub async fn retail_bonus(order: &ce_models::Order, pool: &sqlx::MySqlPool) {
    // Bonus amount should be 20% of the order subtotal
    let bonus_amount = order.sub_total * 0.2;

    //Print order referrer
    println!("Referrer: {:?}", order.referral_id);

    // check if referral_id is set

    if order.referral_id.is_some() {
        // Create the bonus
        let bonus = ce_models::Bonus {
            bonus_id: 1,
            bonus_name: "Retail Bonus".to_string(),
            bonus_percentage: 20.0,
            bonus_amount: bonus_amount,
            to_customer_id: order.referral_id.unwrap(),
            source_customer_id: Some(order.customer_id),
            source_order_id: Some(order.order_id),
            company_id: order.company_id,
        };

        // Create the bonus in the database
        let result = ce_database::create_or_update_bonus(bonus.clone(), pool).await;
    }
}
