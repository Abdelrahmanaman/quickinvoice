use std::str::FromStr;

use crate::db::DbPool;
use crate::models::invoice::NewInvoice;
use crate::models::webhook::{NewWebhook, WebhookPayload};
// src/handlers/webhook.rs
use crate::schema::{invoices, webhooks};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use diesel::RunQueryDsl;
use rust_decimal::Decimal;
use uuid::Uuid;

#[post("/webhook")]
pub async fn handle_webhook(
    payload: web::Json<WebhookPayload>,
    db_pool: web::Data<DbPool>, // Database connection pool
) -> impl Responder {
    println!("Received webhook payload: {:?}", payload);

    // Step 1: Get a database connection
    let mut connection = match db_pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get a database connection",
            }));
        }
    };

    // Step 2: Parse the amount into a Decimal
    let amount = match Decimal::from_str(&payload.amount) {
        Ok(amount) => amount,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid amount format",
            }));
        }
    };

    // Step 3: Save the webhook to the database
    let new_webhook = match serde_json::to_value(&payload) {
        Ok(payload_value) => NewWebhook {
            id: Uuid::new_v4(),
            user_id: payload.user_id,
            event_type: payload.event_type.clone(),
            payload: payload_value,
            created_at: Utc::now().naive_utc(),
        },
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to serialize webhook payload",
            }));
        }
    };

    if let Err(_) = diesel::insert_into(webhooks::table)
        .values(&new_webhook)
        .execute(&mut connection)
    {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to save webhook to the database",
        }));
    }

    // Step 4: Generate and save the invoice
    let new_invoice = NewInvoice {
        id: Uuid::new_v4(),
        user_id: payload.user_id,
        amount,
        currency: payload.currency.clone(),
        created_at: Utc::now().naive_utc(),
    };

    if let Err(_) = diesel::insert_into(invoices::table)
        .values(&new_invoice)
        .execute(&mut connection)
    {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to save invoice to the database",
        }));
    }

    // Step 5: Return a success response
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Webhook processed successfully",
    }))
}
