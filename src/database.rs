use super::models as ce_models;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::mysql::MySqlPool;
use std::env;
use std::option::Option;

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
        .map(|row| ce_models::Period {
            period_id: row.period_id,
            period_type_id: row.period_type_id.unwrap(),
            period_name: row.period_name.unwrap_or_default(),
            period_start_date: row
                .period_start_date
                .map(|d| {
                    chrono::NaiveDate::from_ymd_opt(d.year(), d.month() as u32, d.day() as u32)
                        .expect("Invalid date")
                        .and_hms_opt(d.hour() as u32, d.minute() as u32, d.second() as u32)
                })
                .unwrap()
                .unwrap(),
            period_end_date: row
                .period_end_date
                .map(|d| {
                    chrono::NaiveDate::from_ymd_opt(d.year(), d.month() as u32, d.day() as u32)
                        .expect("Invalid date")
                        .and_hms_opt(d.hour() as u32, d.minute() as u32, d.second() as u32)
                })
                .unwrap()
                .unwrap(),
            period_status_id: row.period_status_id.unwrap(),
            company_id: row.company_id.unwrap(),
            created_date: row
                .created_date
                .map(|d| {
                    chrono::NaiveDate::from_ymd_opt(d.year(), d.month() as u32, d.day() as u32)
                        .expect("Invalid date")
                        .and_hms_opt(d.hour() as u32, d.minute() as u32, d.second() as u32)
                })
                .unwrap()
                .unwrap(),
            modified_date: row
                .modified_date
                .map(|d| {
                    chrono::NaiveDate::from_ymd_opt(d.year(), d.month() as u32, d.day() as u32)
                        .expect("Invalid date")
                        .and_hms_opt(d.hour() as u32, d.minute() as u32, d.second() as u32)
                })
                .unwrap()
                .unwrap(),
            created_by: row.created_by.unwrap_or_default(),
            modified_by: row.modified_by,
        })
        .fetch_all(&pool)
        .await?;

    //Return the periods
    Ok(periods)
}
