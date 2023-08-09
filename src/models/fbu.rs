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
    pub id: Value, // Changed to Value to avoid panic,
    pub name: String,
    pub sort_order: Value, // Changed to Value to avoid panic,
    pub icon_id: Value,
    pub page_id: Value, // Changed to Value to avoid panic,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    pub id: Value, // Changed to Value to avoid panic,
    pub name: String,
    pub page_id: Value, // Changed to Value to avoid panic,
    pub page_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: Value, // Changed to Value to avoid panic,
    pub categories: Vec<Category2>,
    pub category_id: Value, // Changed to Value to avoid panic,
    pub name: String,
    pub image_url: String,
    pub gross_price: Value, // Changed to Value to avoid panic,
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
    pub default_quantity: Value, // Changed to Value to avoid panic,
    pub hide_price_on_tile: bool,
    pub quantity_step: Value, // Changed to Value to avoid panic,
}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FbuJSON {
    pub fresh: Fresh,
    pub street: Street,
    pub flow: Flow,
    pub allergens: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fresh {
    pub name: String,
    pub info: String,
    pub menu: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Street {
    pub name: String,
    pub info: String,
    pub menu: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flow {
    pub name: String,
    pub info: String,
    pub menu: String,
}
