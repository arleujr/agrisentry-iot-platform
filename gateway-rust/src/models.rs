use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "dataqualitystatus", rename_all = "UPPERCASE")]
pub enum DataQualityStatus {
    Pending,
    Valid,
    AnomalyNoise,
    AnomalyCritical,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SensorPayload {
    pub device_id: String,
    pub reading_value: f64,
    pub timestamp: DateTime<Utc>,
}