use super::models as ce_models;
use chrono::NaiveDateTime;
use futures::future::join_all;
use sqlx::mysql::MySqlPool;
use sqlx::types::time::PrimitiveDateTime;
use std::env;
use std::option::Option; // Add this import at the top

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

pub async fn get_periods(
    period_id: Option<i32>,
) -> Result<Vec<ce_models::Period>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

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

    //If the PeriodID is specified, filter the periods
    if let Some(period_id) = period_id {
        return Ok(periods
            .into_iter()
            .filter(|period| period.period_id == period_id)
            .collect());
    }

    //Return the periods
    Ok(periods)
}

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

pub async fn get_orders() -> Result<Vec<ce_models::Order>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the orders
    let orders = sqlx::query!("SELECT * FROM Orders;",)
        .map(|row| {
            Box::pin(async move {
                let order_date = row.order_date.map(convert_to_naive_date_time).unwrap();
                let shipped_date: Option<NaiveDateTime> =
                    row.shipped_date.map(convert_to_naive_date_time);
                let created_date = row.created_date.map(convert_to_naive_date_time).unwrap();
                let locked_date: Option<NaiveDateTime> =
                    row.locked_date.map(convert_to_naive_date_time);
                let modified_date = row.modified_date.map(convert_to_naive_date_time).unwrap();

                //Get OrderDetails for the order

                let order_details = get_order_details(row.order_id).await;

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
                    commissionable_volume_total: f64::from(
                        row.commissionable_volume_total.unwrap(),
                    ),
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
                    tax_integration_calculate: Some(
                        row.tax_integration_calculate.unwrap_or(0) != 0,
                    ),
                    tax_integration_commit: Some(row.tax_integration_commit.unwrap_or(0) != 0),
                    handling_fee: Some(f64::from(row.handling_fee.unwrap() as f64)),
                    pickup_name: row.pickup_name,
                    total_taxable: f64::from(row.total_taxable.unwrap() as f64),
                    order_sub_status_id: row.order_sub_status_id,
                    referral_id: row.referral_id,
                    order_details: order_details.unwrap(),
                }
            })
        })
        .fetch_all(&pool)
        .await?;

    //Return the orders

    let orders: Vec<ce_models::Order> = join_all(orders).await.into_iter().collect();

    Ok(orders)
}

pub async fn get_orders_in_period(
    period_id: i32,
) -> Result<Vec<ce_models::Order>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let period = get_periods(Some(period_id)).await?;

    // If vec is not empty, get the first element
    let period = if !period.is_empty() {
        period.first().unwrap()
    } else {
        // If vec is empty, return an error and exit
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Period not found",
        )));
    };

    // Get start and end dates for the period
    let start_date = period.period_start_date;
    let end_date = period.period_end_date;

    // Convert start and end dates to strings with time
    let start_date = start_date.format("%Y-%m-%d %H:%M:%S").to_string();
    let end_date = end_date.format("%Y-%m-%d %H:%M:%S").to_string();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Get the orders
    let orders = sqlx::query!(
        "SELECT * FROM Orders WHERE order_date >= ? AND order_date <= ?;",
        start_date,
        end_date
    )
    .map(|row| {
        Box::pin(async move {
            let order_date = row.order_date.map(convert_to_naive_date_time).unwrap();
            let shipped_date: Option<NaiveDateTime> =
                row.shipped_date.map(convert_to_naive_date_time);
            let created_date = row.created_date.map(convert_to_naive_date_time).unwrap();
            let locked_date: Option<NaiveDateTime> =
                row.locked_date.map(convert_to_naive_date_time);
            let modified_date = row.modified_date.map(convert_to_naive_date_time).unwrap();

            //Get OrderDetails for the order

            let order_details = get_order_details(row.order_id).await;

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
                modified_by: match row.modified_by {
                    Some(value) => Some(value),
                    None => None,
                },
                tax_integration_calculate: match row.tax_integration_calculate {
                    Some(value) => Some(value != 0),
                    None => None,
                },
                tax_integration_commit: match row.tax_integration_commit {
                    Some(value) => Some(value != 0),
                    None => None,
                },
                handling_fee: match row.handling_fee {
                    Some(value) => Some(f64::from(value as f64)),
                    None => None,
                },
                pickup_name: row.pickup_name,
                total_taxable: f64::from(row.total_taxable.unwrap() as f64),
                order_sub_status_id: match row.order_sub_status_id {
                    Some(value) => Some(value),
                    None => None,
                },
                referral_id: match row.referral_id {
                    Some(value) => Some(value),
                    None => None,
                },
                order_details: order_details.unwrap(),
            }
        })
    })
    .fetch_all(&pool)
    .await?;

    //Return the orders

    let orders: Vec<ce_models::Order> = join_all(orders).await.into_iter().collect();

    Ok(orders)
}

pub async fn create_order(order: ce_models::Order) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Create the order
    let order_id = sqlx::query(
        r#"
        INSERT INTO Orders
        SET order_id = ?, 
        company_id = ?,
            order_date = ?,
            customer_id = ?,
            order_status_id = ?,
            currency_code = ?,
            warehouse_id = ?,
            ship_method_id = ?,
            order_type_id = ?,
            price_type_id = ?,
            first_name = ?,
            middle_name = ?,
            last_name = ?,
            name_suffix = ?,
            company = ?,
            address1 = ?,
            address2 = ?,
            address3 = ?,
            city = ?,
            state = ?,
            zip = ?,
            country = ?,
            county = ?,
            email = ?,
            phone = ?,
            notes = ?,
            total = ?,
            sub_total = ?,
            tax_total = ?,
            shipping_total = ?,
            discount_total = ?,
            discount_percent = ?,
            weight_total = ?,
            business_volume_total = ?,
            commissionable_volume_total = ?,
            other1_total = ?,
            other2_total = ?,
            other3_total = ?,
            other4_total = ?,
            other5_total = ?,
            other6_total = ?,
            other7_total = ?,
            other8_total = ?,
            other9_total = ?,
            other10_total = ?,
            shipping_tax = ?,
            order_tax = ?,
            fed_tax_total = ?,
            state_tax_total = ?,
            fed_shipping_tax = ?,
            state_shipping_tax = ?,
            city_shipping_tax = ?,
            city_local_shipping_tax = ?,
            county_shipping_tax = ?,
            county_local_shipping_tax = ?,
            other11 = ?,
            other12 = ?,
            other13 = ?,
            other14 = ?,
            other15 = ?,
            other16 = ?,
            other17 = ?,
            other18 = ?,
            other19 = ?,
            other20 = ?,
            is_commissionable = ?,
            auto_order_id = ?,
            return_order_id = ?,
            replacement_order_id = ?,
            parent_order_id = ?,
            decline_count = ?,
            transfer_to_customer_id = ?,
            party_id = ?,
            shipped_date = ?,
            created_date = ?,
            locked_date = ?,
            modified_date = ?,
            created_by = ?,
            modified_by = ?,
            tax_integration_calculate = ?,
            tax_integration_commit = ?,
            handling_fee = ?,
            pickup_name = ?,
            total_taxable = ?,
            order_sub_status_id = ?,
            referral_id = ?;
        "#,
    )
    .bind(order.order_id)
    .bind(order.company_id)
    .bind(order.order_date.format("%Y-%m-%d %H:%M:%S").to_string())
    .bind(order.customer_id)
    .bind(order.order_status_id)
    .bind(order.currency_code)
    .bind(order.warehouse_id)
    .bind(order.ship_method_id)
    .bind(order.order_type_id)
    .bind(order.price_type_id)
    .bind(order.first_name)
    .bind(order.middle_name)
    .bind(order.last_name)
    .bind(order.name_suffix)
    .bind(order.company)
    .bind(order.address1)
    .bind(order.address2)
    .bind(order.address3)
    .bind(order.city)
    .bind(order.state)
    .bind(order.zip)
    .bind(order.country)
    .bind(order.county)
    .bind(order.email)
    .bind(order.phone)
    .bind(order.notes)
    .bind(order.total)
    .bind(order.sub_total)
    .bind(order.tax_total)
    .bind(order.shipping_total)
    .bind(order.discount_total)
    .bind(order.discount_percent)
    .bind(order.weight_total)
    .bind(order.business_volume_total)
    .bind(order.commissionable_volume_total)
    .bind(order.other1_total)
    .bind(order.other2_total)
    .bind(order.other3_total)
    .bind(order.other4_total)
    .bind(order.other5_total)
    .bind(order.other6_total)
    .bind(order.other7_total)
    .bind(order.other8_total)
    .bind(order.other9_total)
    .bind(order.other10_total)
    .bind(order.shipping_tax)
    .bind(order.order_tax)
    .bind(order.fed_tax_total)
    .bind(order.state_tax_total)
    .bind(order.fed_shipping_tax)
    .bind(order.state_shipping_tax)
    .bind(order.city_shipping_tax)
    .bind(order.city_local_shipping_tax)
    .bind(order.county_shipping_tax)
    .bind(order.county_local_shipping_tax)
    .bind(order.other11)
    .bind(order.other12)
    .bind(order.other13)
    .bind(order.other14)
    .bind(order.other15)
    .bind(order.other16)
    .bind(order.other17)
    .bind(order.other18)
    .bind(order.other19)
    .bind(order.other20)
    .bind(order.is_commissionable)
    .bind(order.auto_order_id)
    .bind(order.return_order_id)
    .bind(order.replacement_order_id)
    .bind(order.parent_order_id)
    .bind(order.decline_count)
    .bind(order.transfer_to_customer_id)
    .bind(order.party_id)
    .bind(
        order
            .shipped_date
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
    )
    .bind(order.created_date.format("%Y-%m-%d %H:%M:%S").to_string())
    .bind(
        order
            .locked_date
            .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string()),
    )
    .bind(order.modified_date.format("%Y-%m-%d %H:%M:%S").to_string())
    .bind(order.created_by)
    .bind(order.modified_by)
    .bind(order.tax_integration_calculate)
    .bind(order.tax_integration_commit)
    .bind(order.handling_fee)
    .bind(order.pickup_name)
    .bind(order.total_taxable)
    .bind(order.order_sub_status_id)
    .bind(order.referral_id)
    .execute(&pool)
    .await?
    .last_insert_id();

    // Create the order details
    for order_detail in order.order_details {
        create_order_detail(order_detail).await?;
    }

    // Return Ok

    Ok(())
}

pub async fn create_order_detail(
    order_detail: ce_models::OrderDetail,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Create the order detail
    let order_detail_id = sqlx::query(
        r#"
        INSERT INTO OrderDetail
        SET order_id = ?,
            order_line = ?,
            parent_order_detail_id = ?,
            item_id = ?,
            item_code = ?,
            item_description = ?,
            quantity = ?,
            price_each = ?,
            price_total = ?,
            tax = ?,
            weight_each = ?,
            weight = ?,
            business_volume_each = ?,
            business_volume = ?,
            commissionable_volume_each = ?,
            commissionable_volume = ?,
            other1_each = ?,
            other1 = ?,
            other2_each = ?,
            other2 = ?,
            other3_each = ?,
            other3 = ?,
            other4_each = ?,
            other4 = ?,
            other5_each = ?,
            other5 = ?,
            original_taxable_each = ?,
            original_business_volume_each = ?,
            original_commissionable_volume_each = ?,
            other6_each = ?,
            other6 = ?,
            other7_each = ?,
            other7 = ?,
            other8_each = ?,
            other8 = ?,
            other9_each = ?,
            other9 = ?,
            other10_each = ?,
            other10 = ?,
            parent_item_id = ?,
            taxable = ?,
            fed_tax = ?,
            state_tax = ?,
            city_tax = ?,
            city_local_tax = ?,
            county_tax = ?,
            county_local_tax = ?,
            manual_tax = ?,
            is_state_tax_override = ?,
            reference1 = ?;
        "#,
    )
    .bind(order_detail.order_id)
    .bind(order_detail.order_line)
    .bind(order_detail.parent_order_detail_id)
    .bind(order_detail.item_id)
    .bind(order_detail.item_code)
    .bind(order_detail.item_description)
    .bind(order_detail.quantity)
    .bind(order_detail.price_each)
    .bind(order_detail.price_total)
    .bind(order_detail.tax)
    .bind(order_detail.weight_each)
    .bind(order_detail.weight)
    .bind(order_detail.business_volume_each)
    .bind(order_detail.business_volume)
    .bind(order_detail.commissionable_volume_each)
    .bind(order_detail.commissionable_volume)
    .bind(order_detail.other1_each)
    .bind(order_detail.other1)
    .bind(order_detail.other2_each)
    .bind(order_detail.other2)
    .bind(order_detail.other3_each)
    .bind(order_detail.other3)
    .bind(order_detail.other4_each)
    .bind(order_detail.other4)
    .bind(order_detail.other5_each)
    .bind(order_detail.other5)
    .bind(order_detail.original_taxable_each)
    .bind(order_detail.original_business_volume_each)
    .bind(order_detail.original_commissionable_volume_each)
    .bind(order_detail.other6_each)
    .bind(order_detail.other6)
    .bind(order_detail.other7_each)
    .bind(order_detail.other7)
    .bind(order_detail.other8_each)
    .bind(order_detail.other8)
    .bind(order_detail.other9_each)
    .bind(order_detail.other9)
    .bind(order_detail.other10_each)
    .bind(order_detail.other10)
    .bind(order_detail.parent_item_id)
    .bind(order_detail.taxable)
    .bind(order_detail.fed_tax)
    .bind(order_detail.state_tax)
    .bind(order_detail.city_tax)
    .bind(order_detail.city_local_tax)
    .bind(order_detail.county_tax)
    .bind(order_detail.county_local_tax)
    .bind(order_detail.manual_tax)
    .bind(order_detail.is_state_tax_override)
    .bind(order_detail.reference1)
    .execute(&pool)
    .await?
    .last_insert_id();

    // Return Ok

    Ok(())
}

pub async fn create_customer(
    customer: ce_models::Customer,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //Create the customer
    let customer_id = sqlx::query(
        r#"
        INSERT INTO Customer
        SET company_id = ?,
            customer_type_id = ?,
            customer_status_id = ?,
            customer_sub_status_id = ?,
            enroller_id = ?,
            sponsor_id = ?,
            binary_placement_id = ?;
        "#,
    )
    .bind(customer.company_id)
    .bind(customer.customer_type_id)
    .bind(customer.customer_status_id)
    .bind(customer.customer_sub_status_id)
    .bind(customer.enroller_id)
    .bind(customer.sponsor_id)
    .bind(customer.binary_placement_id)
    .execute(&pool)
    .await?
    .last_insert_id();

    // Return Ok

    Ok(())
}
