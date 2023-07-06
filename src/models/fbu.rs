use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FbuResponse {
    pub category: Category,
    pub article: Article,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub sort_order: i64,
    pub icon_id: Value,
    pub page_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    pub id: i64,
    pub name: String,
    pub page_id: i64,
    pub page_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i64,
    pub categories: Vec<Category2>,
    pub category_id: i64,
    pub name: String,
    pub image_url: String,
    pub gross_price: i64,
    pub description: String,
    pub description_short: String,
    pub allergens: String,
    pub force_item_options: bool,
    pub is_read_only: bool,
    pub is_deposit: bool,
    pub dynamic_status: Value,
    pub price_group_name: Value,
    pub child_article_groups: Value,
    pub reward_campaigns: Vec<Value>,
    pub default_quantity: i64,
    pub hide_price_on_tile: bool,
    pub quantity_step: i64,
}