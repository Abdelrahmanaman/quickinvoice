// src/models.rs
use crate::models::user::User;
use crate::schema::webhooks;
use chrono::NaiveDateTime;
use diesel::prelude::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Associations, Debug, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = webhooks)]
pub struct Webhook {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = webhooks)]
pub struct NewWebhook {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WebhookPayload {
    pub user_id: Uuid,
    pub event_type: String,
    pub amount: String,
    pub currency: String,
    pub timestamp: String,
}
