use crate::models::user::User;
use crate::schema::invoices;
use chrono::NaiveDateTime;
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Identifiable, Queryable, Insertable, Associations, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = invoices)]
pub struct Invoice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = invoices)]
pub struct NewInvoice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub created_at: NaiveDateTime,
}
