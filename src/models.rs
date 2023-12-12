// models.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::{clone, option::Option};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub company_id: i32,
    pub company_name: String,
    pub tree_types: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub company_id: i32,
    pub order_id: i32,
    pub customer_id: i32,
    pub order_status_id: i32,
    pub order_date: NaiveDateTime,
    pub currency_code: String,
    pub warehouse_id: i32,
    pub ship_method_id: i32,
    pub order_type_id: i32,
    pub price_type_id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub name_suffix: Option<String>,
    pub company: Option<String>,
    pub address1: String,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub county: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
    pub total: f64,
    pub sub_total: f64,
    pub tax_total: f64,
    pub shipping_total: f64,
    pub discount_total: f64,
    pub discount_percent: f64,
    pub weight_total: f64,
    pub business_volume_total: f64,
    pub commissionable_volume_total: f64,
    pub other1_total: Option<f64>,
    pub other2_total: Option<f64>,
    pub other3_total: Option<f64>,
    pub other4_total: Option<f64>,
    pub other5_total: Option<f64>,
    pub other6_total: Option<f64>,
    pub other7_total: Option<f64>,
    pub other8_total: Option<f64>,
    pub other9_total: Option<f64>,
    pub other10_total: Option<f64>,
    pub shipping_tax: f64,
    pub order_tax: f64,
    pub fed_tax_total: f64,
    pub state_tax_total: f64,
    pub fed_shipping_tax: f64,
    pub state_shipping_tax: f64,
    pub city_shipping_tax: f64,
    pub city_local_shipping_tax: f64,
    pub county_shipping_tax: f64,
    pub county_local_shipping_tax: f64,
    pub other11: Option<String>,
    pub other12: Option<String>,
    pub other13: Option<String>,
    pub other14: Option<String>,
    pub other15: Option<String>,
    pub other16: Option<String>,
    pub other17: Option<String>,
    pub other18: Option<String>,
    pub other19: Option<String>,
    pub other20: Option<String>,
    pub is_commissionable: bool,
    pub auto_order_id: Option<i32>,
    pub return_order_id: Option<i32>,
    pub replacement_order_id: Option<i32>,
    pub parent_order_id: Option<i32>,
    pub decline_count: Option<i32>,
    pub transfer_to_customer_id: Option<i32>,
    pub party_id: Option<i32>,
    pub shipped_date: Option<NaiveDateTime>,
    pub created_date: NaiveDateTime,
    pub locked_date: Option<NaiveDateTime>,
    pub modified_date: NaiveDateTime,
    pub created_by: String,
    pub modified_by: Option<String>,
    pub tax_integration_calculate: Option<bool>,
    pub tax_integration_commit: Option<bool>,
    pub handling_fee: Option<f64>,
    pub pickup_name: Option<String>,
    pub total_taxable: f64,
    pub order_sub_status_id: Option<i32>,
    pub referral_id: Option<i32>,
    pub order_details: Vec<OrderDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetail {
    pub order_id: i32,
    pub order_line: i32,
    pub order_detail_id: Option<i32>,
    pub parent_order_detail_id: Option<i32>,
    pub item_id: i32,
    pub item_code: String,
    pub item_description: String,
    pub quantity: i32,
    pub price_each: f64,
    pub price_total: f64,
    pub tax: f64,
    pub weight_each: f64,
    pub weight: f64,
    pub business_volume_each: f64,
    pub business_volume: f64,
    pub commissionable_volume_each: f64,
    pub commissionable_volume: f64,
    pub other1_each: Option<f64>,
    pub other1: Option<f64>,
    pub other2_each: Option<f64>,
    pub other2: Option<f64>,
    pub other3_each: Option<f64>,
    pub other3: Option<f64>,
    pub other4_each: Option<f64>,
    pub other4: Option<f64>,
    pub other5_each: Option<f64>,
    pub other5: Option<f64>,
    pub original_taxable_each: f64,
    pub original_business_volume_each: f64,
    pub original_commissionable_volume_each: f64,
    pub other6_each: Option<f64>,
    pub other6: Option<f64>,
    pub other7_each: Option<f64>,
    pub other7: Option<f64>,
    pub other8_each: Option<f64>,
    pub other8: Option<f64>,
    pub other9_each: Option<f64>,
    pub other9: Option<f64>,
    pub other10_each: Option<f64>,
    pub other10: Option<f64>,
    pub parent_item_id: Option<i32>,
    pub taxable: f64,
    pub fed_tax: f64,
    pub state_tax: f64,
    pub city_tax: f64,
    pub city_local_tax: f64,
    pub county_tax: f64,
    pub county_local_tax: f64,
    pub manual_tax: Option<f64>,
    pub is_state_tax_override: bool,
    pub reference1: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub customer_id: i32,
    pub company_id: i32,
    pub customer_type_id: i32,
    pub customer_status_id: i32,
    pub customer_sub_status_id: Option<i32>,
    pub enroller_id: Option<i32>,
    pub sponsor_id: Option<i32>,
    pub binary_placement_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub tree_id: i32,
    pub tree_name: String,
    pub tree_type_id: i32,
    pub company_id: i32,
    pub is_active: bool,
    pub created_date: NaiveDateTime,
    pub modified_date: NaiveDateTime,
    pub created_by: String,
    pub modified_by: Option<String>,
    pub top_node_customer_id: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TreeType {
    pub tree_type_id: i32,
    pub tree_type_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Period {
    pub period_id: i32,
    pub period_type_id: i32,
    pub period_name: String,
    pub period_start_date: NaiveDateTime,
    pub period_end_date: NaiveDateTime,
    pub period_status_id: i32,
    pub company_id: i32,
    pub created_date: NaiveDateTime,
    pub modified_date: NaiveDateTime,
    pub created_by: String,
    pub modified_by: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodStatus {
    pub period_status_id: i32,
    pub period_status_description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodType {
    pub period_type_id: i32,
    pub period_type_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bonus {
    pub bonus_id: i32,
    pub bonus_name: String,
    pub bonus_percentage: f64,
    pub bonus_amount: f64,
    pub to_customer_id: i32,
    pub source_customer_id: Option<i32>,
    pub source_order_id: Option<i32>,
}
