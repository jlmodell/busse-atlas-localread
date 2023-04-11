use mongodb::bson;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Rebate {
    pub _id: bson::oid::ObjectId,
    pub period: String,
    pub name: String,
    pub addr: String,
    pub city: String,
    pub state: String,
    pub gpo: String,
    pub license: String,
    #[serde(rename = "searchScore")]
    pub search_score: f64,
    pub contract: String,
    pub claim_nbr: String,
    pub order_nbr: String,
    pub invoice_nbr: String,
    pub invoice_date: bson::DateTime,
    pub part: String,
    pub unit_rebate: f64,
    pub ship_qty: f64,
    pub uom: String,
    pub ship_qty_as_cs: f64,
    pub rebate: f64,
    pub cost: f64,
    pub check_license: Option<bool>,
    pub postal: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sale {
    pub _id: bson::oid::ObjectId,
    pub key: String,
    pub distribution: String,
    pub rep: String,
    pub item: String,
    pub sale: f64,
    pub quantity: f64,
    pub uom: String,
    pub date: bson::DateTime,
    pub customer: String,
    pub ship_to_name: String,
    pub addr1: String,
    pub addr2: String,
    pub city: String,
    pub state: String,
    pub postal: String,
    pub country: String,
    pub contract: String,
    pub cust_nbr: String,
    pub notes: bson::Document,
    pub gpo: String,
    pub rebate: f64,
}