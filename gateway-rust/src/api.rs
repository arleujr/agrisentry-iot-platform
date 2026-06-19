use actix_web::{post, web, HttpResponse};
use crate::models::SensorPayload;
use crate::db::DbClient;
use crate::error::GatewayError;

#[post("/telemetry")]
pub async fn ingest_telemetry(
    db: web::Data<DbClient>,
    payload: web::Json<SensorPayload>,
) -> Result<HttpResponse, GatewayError> {
    db.insert_reading(&payload.into_inner()).await?;
    Ok(HttpResponse::Accepted().json(serde_json::json!({ "status": "QUEUED_FOR_AI_EVALUATION" })))
}
