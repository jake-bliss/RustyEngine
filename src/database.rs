use super::models as ce_models;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::mysql::MySqlPool;
use sqlx::types::time::PrimitiveDateTime;
use std::env;
use std::option::Option;

fn convert_to_naive_date_time(d: PrimitiveDateTime) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(
            d.date().year(),
            d.date().month() as u32,
            d.date().day() as u32,
        )
        .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
        chrono::NaiveTime::from_hms_opt(
            d.time().hour() as u32,
            d.time().minute() as u32,
            d.time().second() as u32,
        )
        .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
    )
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_period_types() -> Result<Vec<ce_models::PeriodType>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the period types
    let period_types = sqlx::query!("SELECT * FROM PeriodType;",)
        .map(|row| ce_models::PeriodType {
            period_type_id: row.period_type_id,
            period_type_description: row.type_description.unwrap_or_default(),
        })
        .fetch_all(&pool)
        .await?;

    //Return the period types
    Ok(period_types)
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_period_statuses(
) -> Result<Vec<ce_models::PeriodStatus>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the period statuses
    let period_statuses = sqlx::query!("SELECT * FROM PeriodStatus;",)
        .map(|row| ce_models::PeriodStatus {
            period_status_id: row.period_status_id,
            period_status_description: row.status_description.unwrap_or_default(),
        })
        .fetch_all(&pool)
        .await?;

    //Return the period statuses
    Ok(period_statuses)
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_tree_types() -> Result<Vec<ce_models::TreeType>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the tree types
    let tree_types = sqlx::query!("SELECT * FROM TreeType;",)
        .map(|row| ce_models::TreeType {
            tree_type_id: row.tree_type_id,
            tree_type_description: row.type_description.unwrap_or_default(),
        })
        .fetch_all(&pool)
        .await?;

    //Return the tree types
    Ok(tree_types)
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_periods() -> Result<Vec<ce_models::Period>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //print pool
    println!("{:#?}", pool);

    //Get the periods
    let periods = sqlx::query!("SELECT * FROM Period;",)
        .map(|row| {
            let period_start_date = row
                .period_start_date
                .map(convert_to_naive_date_time)
                .unwrap();
            let period_end_date = row.period_end_date.map(convert_to_naive_date_time).unwrap();
            let created_date = row.created_date.map(convert_to_naive_date_time).unwrap();
            let modified_date = row.modified_date.map(convert_to_naive_date_time).unwrap();

            ce_models::Period {
                period_id: row.period_id,
                period_type_id: row.period_type_id.unwrap(),
                period_name: row.period_name.unwrap_or_default(),
                period_start_date,
                period_end_date,
                period_status_id: row.period_status_id.unwrap(),
                company_id: row.company_id.unwrap(),
                created_date,
                modified_date,
                created_by: row.created_by.unwrap_or_default(),
                modified_by: row.modified_by,
            }
        })
        .fetch_all(&pool)
        .await?;

    //Return the periods
    Ok(periods)
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_trees() -> Result<Vec<ce_models::Tree>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the trees
    let trees = sqlx::query!("SELECT * FROM Tree;",)
        .map(|row| {
            let created_date = row.created_date.map(convert_to_naive_date_time).unwrap();
            let modified_date = row.modified_date.map(convert_to_naive_date_time).unwrap();

            ce_models::Tree {
                tree_id: row.tree_id,
                tree_name: row.tree_name.unwrap_or_default(),
                tree_type_id: row.tree_type_id.unwrap(),
                company_id: row.company_id.unwrap(),
                is_active: row.is_active.unwrap_or(0) != 0,
                created_date,
                modified_date,
                created_by: row.created_by.unwrap_or_default(),
                modified_by: row.modified_by,
                top_node_customer_id: row.top_node_customer_id.unwrap(),
            }
        })
        .fetch_all(&pool)
        .await?;

    //Return the trees
    Ok(trees)
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_order_details(
    order_id: i32,
) -> Result<Vec<ce_models::OrderDetail>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the order details
    let order_details = sqlx::query!("SELECT * FROM OrderDetail WHERE order_id = ?;", order_id)
        .map(|row| ce_models::OrderDetail {
            order_id: row.order_id.unwrap_or_default(),
            order_line: row.order_line.unwrap_or_default(),
            order_detail_id: row.order_detail_id,
            parent_order_detail_id: row.parent_order_detail_id,
            item_id: row.item_id.unwrap_or_default(),
            item_code: row.item_code.unwrap_or_default(),
            item_description: row.item_description.unwrap_or_default(),
            quantity: row.quantity.unwrap_or_default(),
            price_each: row.price_each.unwrap_or_default() as f64,

            price_total: row.price_total.unwrap_or_default() as f64,
            tax: row.tax.unwrap_or_default() as f64,
            weight_each: row.weight_each.unwrap_or_default() as f64,
            weight: row.weight.unwrap_or_default() as f64,
            business_volume_each: row.business_volume_each.unwrap_or_default() as f64,
            business_volume: row.business_volume.unwrap_or_default() as f64,
            commissionable_volume_each: row.commissionable_volume_each.unwrap_or_default() as f64,
            commissionable_volume: row.commissionable_volume.unwrap_or_default() as f64,
            other1_each: row.other1_each.map(|value| value as f64),

            other1: row.other1.map(|value| value as f64),
            other2_each: row.other2_each.map(|value| value as f64),
            other2: row.other2.map(|value| value as f64),
            other3_each: row.other3_each.map(|value| value as f64),
            other3: row.other3.map(|value| value as f64),
            other4_each: row.other4_each.map(|value| value as f64),
            other4: row.other4.map(|value| value as f64),
            other5_each: row.other5_each.map(|value| value as f64),
            other5: row.other5.map(|value| value as f64),
            original_taxable_each: row.original_taxable_each.unwrap_or_default() as f64,
            original_business_volume_each: row.original_business_volume_each.unwrap_or_default()
                as f64,
            original_commissionable_volume_each: row
                .original_commissionable_volume_each
                .unwrap_or_default() as f64,
            other6_each: row.other6_each.map(|value| value as f64),
            other6: row.other6.map(|value| value as f64),
            other7_each: row.other7_each.map(|value| value as f64),
            other7: row.other7.map(|value| value as f64),
            other8_each: row.other8_each.map(|value| value as f64),
            other8: row.other8.map(|value| value as f64),
            other9_each: row.other9_each.map(|value| value as f64),
            other9: row.other9.map(|value| value as f64),
            other10_each: row.other10_each.map(|value| value as f64),
            other10: row.other10.map(|value| value as f64),
            parent_item_id: row.parent_item_id,
            taxable: row.taxable.unwrap_or_default() as f64,
            fed_tax: row.fed_tax.unwrap_or_default() as f64,
            state_tax: row.state_tax.unwrap_or_default() as f64,
            city_tax: row.city_tax.unwrap_or_default() as f64,
            city_local_tax: row.city_local_tax.unwrap_or_default() as f64,
            county_tax: row.county_tax.unwrap_or_default() as f64,
            county_local_tax: row.county_local_tax.unwrap_or_default() as f64,
            manual_tax: Some(row.manual_tax.unwrap_or_default() as f64),
            is_state_tax_override: row.is_state_tax_override.unwrap_or(0) != 0,
            reference1: row.reference1,
        })
        .fetch_all(&pool)
        .await?;

    //Return the order details
    Ok(order_details)
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_orders() -> Result<Vec<ce_models::Order>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the orders
    let orders = sqlx::query!("SELECT * FROM Orders;",)
        .map(|row| {
            let order_date = row.order_date.map(convert_to_naive_date_time).unwrap();
            let shipped_date: Option<NaiveDateTime> =
                row.shipped_date.map(convert_to_naive_date_time);
            let created_date = row.created_date.map(convert_to_naive_date_time).unwrap();
            let locked_date: Option<NaiveDateTime> =
                row.locked_date.map(convert_to_naive_date_time);
            let modified_date = row.modified_date.map(convert_to_naive_date_time).unwrap();

            //Get OrderDetails for the order

            let order_details = get_order_details(row.order_id).unwrap();

            ce_models::Order {
                order_id: row.order_id,
                company_id: row.company_id.unwrap(),
                order_date,
                customer_id: row.customer_id.unwrap(),
                order_status_id: row.order_status_id.unwrap(),
                currency_code: row.currency_code.unwrap_or_default(),
                warehouse_id: row.warehouse_id.unwrap(),
                ship_method_id: row.ship_method_id.unwrap(),
                order_type_id: row.order_type_id.unwrap(),
                price_type_id: row.price_type_id.unwrap(),
                first_name: row.first_name.unwrap_or_default(),
                middle_name: row.middle_name,
                last_name: row.last_name.unwrap_or_default(),
                name_suffix: row.name_suffix,
                company: row.company,
                address1: row.address1.unwrap_or_default(),
                address2: row.address2,
                address3: row.address3,
                city: row.city.unwrap_or_default(),
                state: row.state.unwrap_or_default(),
                zip: row.zip.unwrap_or_default(),
                country: row.country.unwrap_or_default(),
                county: row.county,
                email: row.email,
                phone: row.phone,
                notes: row.notes,
                total: f64::from(row.total.unwrap()),
                sub_total: f64::from(row.sub_total.unwrap()),
                tax_total: f64::from(row.tax_total.unwrap()),
                shipping_total: f64::from(row.shipping_total.unwrap()),
                discount_total: f64::from(row.discount_total.unwrap()),
                discount_percent: f64::from(row.discount_percent.unwrap()),
                weight_total: f64::from(row.weight_total.unwrap()),
                business_volume_total: f64::from(row.business_volume_total.unwrap()),
                commissionable_volume_total: f64::from(row.commissionable_volume_total.unwrap()),
                other1_total: Some(row.other1_total.unwrap_or_default() as f64),
                other2_total: Some(row.other2_total.unwrap_or_default() as f64),
                other3_total: Some(row.other3_total.unwrap_or_default() as f64),
                other4_total: Some(row.other4_total.unwrap_or_default() as f64),
                other5_total: Some(row.other5_total.unwrap_or_default() as f64),
                other6_total: Some(row.other6_total.unwrap_or_default() as f64),
                other7_total: Some(row.other7_total.unwrap_or_default() as f64),
                other8_total: Some(row.other8_total.unwrap_or_default() as f64),
                other9_total: Some(row.other9_total.unwrap_or_default() as f64),
                other10_total: Some(row.other10_total.unwrap_or_default() as f64),
                shipping_tax: f64::from(row.shipping_tax.unwrap()),
                order_tax: f64::from(row.order_tax.unwrap()),
                fed_tax_total: f64::from(row.fed_tax_total.unwrap()),
                state_tax_total: f64::from(row.state_tax_total.unwrap()),
                fed_shipping_tax: f64::from(row.fed_shipping_tax.unwrap()),
                state_shipping_tax: f64::from(row.state_shipping_tax.unwrap()),
                city_shipping_tax: f64::from(row.city_shipping_tax.unwrap()),
                city_local_shipping_tax: f64::from(row.city_local_shipping_tax.unwrap()),
                county_shipping_tax: f64::from(row.county_shipping_tax.unwrap()),
                county_local_shipping_tax: f64::from(row.county_local_shipping_tax.unwrap()),
                other11: row.other11,
                other12: row.other12,
                other13: row.other13,
                other14: row.other14,
                other15: row.other15,
                other16: row.other16,
                other17: row.other17,
                other18: row.other18,
                other19: row.other19,
                other20: row.other20,
                is_commissionable: row.is_commissionable.unwrap_or(0) != 0,
                auto_order_id: row.auto_order_id,
                return_order_id: row.return_order_id,
                replacement_order_id: row.replacement_order_id,
                parent_order_id: row.parent_order_id,
                decline_count: row.decline_count,
                transfer_to_customer_id: row.transfer_to_customer_id,
                party_id: row.party_id,
                shipped_date,
                created_date,
                locked_date,
                modified_date,
                created_by: row.created_by.unwrap_or_default(),
                modified_by: row.modified_by,
                tax_integration_calculate: Some(row.tax_integration_calculate.unwrap_or(0) != 0),
                tax_integration_commit: Some(row.tax_integration_commit.unwrap_or(0) != 0),
                handling_fee: Some(f64::from(row.handling_fee.unwrap() as f64)),
                pickup_name: row.pickup_name,
                total_taxable: f64::from(row.total_taxable.unwrap() as f64),
                order_sub_status_id: row.order_sub_status_id,
                referral_id: row.referral_id,
                order_details: order_details,
            }
        })
        .fetch_all(&pool)
        .await?;

    //Return the orders
    Ok(orders)
}
