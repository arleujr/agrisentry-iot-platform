use sqlx::{PgPool, Postgres};
use crate::models::{SensorPayload, DataQualityStatus};
use crate::error::GatewayError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct DbClient {
    pub pool: PgPool,
}

impl DbClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn log_event(&self, component: &str, message: &str, level: &str) {
        let _ = sqlx::query(
            r#"INSERT INTO "system_events" (component, message, level) VALUES ($1, $2, $3)"#
        )
        .bind(component)
        .bind(message)
        .bind(level)
        .execute(&self.pool)
        .await;
    }

    pub async fn insert_reading(&self, payload: &SensorPayload) -> Result<u64, GatewayError> {
        let sensor_check = sqlx::query!(
            r#"SELECT id FROM "sensors" WHERE hardware_id = $1"#,
            payload.device_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let s_id = match sensor_check {
            Some(row) => row.id,
            None => return Err(GatewayError::HardwareNotFound),
        };

        let result = sqlx::query(
            r#"
            INSERT INTO "sensor_readings" (id, value, sensor_id, status, created_at)
            VALUES (gen_random_uuid(), $1, $2, $3, $4)
            "#,
        )
        .bind(payload.reading_value)
        .bind(s_id)
        .bind(DataQualityStatus::Pending)
        .bind(payload.timestamp)
        .execute(&self.pool)
        .await?;

        self.log_event(
            "GATEWAY-RUST",
            &format!("Ingested value {} from device {}", payload.reading_value, payload.device_id),
            "INFO"
        ).await;

        Ok(result.rows_affected())
    }

    pub async fn fetch_pending_readings(&self, limit: i64) -> Result<Vec<(Uuid, f64, DateTime<Utc>)>, GatewayError> {
        let rows = sqlx::query_as::<Postgres, (Uuid, f64, DateTime<Utc>)>(
            r#"
            SELECT id, value, created_at 
            FROM "sensor_readings" 
            WHERE status = 'PENDING'
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn update_reading_status(&self, id: Uuid, created_at: DateTime<Utc>, status: DataQualityStatus, note: &str) -> Result<(), GatewayError> {
        sqlx::query(
            r#"
            UPDATE "sensor_readings"
            SET status = $1, ai_analysis_note = $2
            WHERE id = $3 AND created_at = $4
            "#,
        )
        .bind(status)
        .bind(note)
        .bind(id)
        .bind(created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}