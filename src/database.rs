use super::models as ce_models;
use sqlx::mysql::MySqlPool;
use std::env;

#[tokio::main(flavor = "current_thread")]
pub async fn get_periods() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    //print pool
    println!("{:#?}", pool);

    //Print each row of the table
    let period_types = sqlx::query!("SELECT * FROM PeriodType;",)
        .map(|row| ce_models::PeriodType {
            period_type_id: row.period_type_id,
            period_type_description: row.type_description.unwrap_or_default(),
        })
        .fetch_all(&pool)
        .await?;

    for period_type in period_types.iter() {
        println!(
            "Period Type ID: {}, Period Type Description: {}",
            period_type.period_type_id, period_type.period_type_description
        );
    }

    Ok(())
}
