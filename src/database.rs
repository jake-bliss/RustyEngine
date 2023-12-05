use super::models as ce_models;
use sqlx::mysql::MySqlPool;
use std::env;

#[tokio::main(flavor = "current_thread")]
pub async fn get_period_types() -> Result<Vec<ce_models::PeriodType>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //print pool
    println!("{:#?}", pool);

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

    //print pool
    println!("{:#?}", pool);

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
