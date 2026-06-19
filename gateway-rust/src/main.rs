use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors; // ✅ Import CORS middleware
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use tokio::signal;
use tokio::sync::watch;

mod models;
mod error;
mod api;
mod db;

/// Utility function to convert string status into enum
fn status_from_str(status: &str) -> Option<models::DataQualityStatus> {
    match status {
        "PENDING" => Some(models::DataQualityStatus::Pending),
        "VALID" => Some(models::DataQualityStatus::Valid),
        "ANOMALY_NOISE" => Some(models::DataQualityStatus::AnomalyNoise),
        "ANOMALY_CRITICAL" => Some(models::DataQualityStatus::AnomalyCritical),
        _ => None,
    }
}

/// Basic health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy", "engine": "rust-gateway" }))
}

/// Dashboard metrics aggregation (last 24h)
async fn get_dashboard_metrics(db_client: web::Data<db::DbClient>) -> impl Responder {
    match sqlx::query!(
        r#"
        SELECT 
            status::text as "status!",
            COUNT(*)::bigint as "count!"
        FROM "sensor_readings"
        WHERE created_at > NOW() - INTERVAL '24 hours'
        GROUP BY status;
        "#
    )
    .fetch_all(&db_client.pool)
    .await 
    {
        Ok(records) => {
            let metrics: Vec<serde_json::Value> = records
                .into_iter()
                .map(|rec| serde_json::json!({ "status": rec.status, "count": rec.count }))
                .collect();
            HttpResponse::Ok().json(serde_json::json!({ "timeframe": "24h", "metrics": metrics }))
        }
        Err(e) => {
            tracing::error!("Metrics aggregation failed: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": "Metrics failed" }))
        }
    }
}

/// Retrieve latest system logs
async fn get_system_logs(db_client: web::Data<db::DbClient>) -> impl Responder {
    match sqlx::query!(
        r#"SELECT component, message, level, created_at FROM "system_events" ORDER BY id DESC LIMIT 25"#
    )
    .fetch_all(&db_client.pool)
    .await {
        Ok(rows) => {
            let logs: Vec<serde_json::Value> = rows.into_iter().map(|r| serde_json::json!({
                "component": r.component,
                "message": r.message,
                "level": r.level,
                "created_at": r.created_at
            })).collect();
            HttpResponse::Ok().json(logs)
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging/tracing
    tracing_subscriber::fmt::init();
    tracing::info!("🚀 Initializing AgriSentry Core Engine...");

    // Load environment variables
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let ai_api_url = env::var("AI_API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8000/v1/analyze".to_string());

    // Setup PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(database_url.parse().unwrap())
        .await
        .expect("CRITICAL: Failed to attach PostgreSQL core pool");

    let db_client = web::Data::new(db::DbClient::new(pool.clone()));
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    // Background worker for AI processing
    let worker_client = db_client.get_ref().clone();
    let http_client = reqwest::Client::new();
    tokio::spawn(async move {
        worker_client.log_event("WORKER-RUST", "AI background processing line operational", "INFO").await;
        loop {
            tokio::select! {
                res = shutdown_rx.changed() => { if res.is_ok() && *shutdown_rx.borrow() { break; } }
                _ = tokio::time::sleep(Duration::from_secs(5)) => {
                    if let Ok(readings) = worker_client.fetch_pending_readings(50).await {
                        if !readings.is_empty() {
                            worker_client.log_event("WORKER-RUST", &format!("Dispatching batch of {} telemetry points to AI", readings.len()), "INFO").await;
                            
                            let telemetry_json: Vec<serde_json::Value> = readings.iter().map(|(id, val, time)| serde_json::json!({
                                "id": id, "value": val, "created_at": time
                            })).collect();

                            let payload = serde_json::json!({ "readings": telemetry_json });
                            if let Ok(resp) = http_client.post(&ai_api_url).json(&payload).send().await {
                                if resp.status().is_success() {
                                    if let Ok(ai_res) = resp.json::<serde_json::Value>().await {
                                        if let Some(results) = ai_res.get("results").and_then(|r| r.as_array()) {
                                            for res_item in results {
                                                let id = uuid::Uuid::parse_str(res_item["id"].as_str().unwrap()).unwrap();
                                                let created_at = chrono::DateTime::parse_from_rfc3339(res_item["created_at"].as_str().unwrap()).unwrap().with_timezone(&chrono::Utc);
                                                let status_enum = status_from_str(res_item["status"].as_str().unwrap()).unwrap();
                                                let note = res_item["note"].as_str().unwrap();
                                                let _ = worker_client.update_reading_status(id, created_at, status_enum, note).await;
                                            }
                                            worker_client.log_event("DATABASE", &format!("Successfully resolved and updated batch of {} evaluations", results.len()), "INFO").await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    // ✅ Pega a porta dinamicamente do Render, ou usa 8080 localmente
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a valid u16 integer");

    // Configure HTTP server
    let server = HttpServer::new(move || {
        // ✅ CORS configuration (permissive for local development and production)
        let cors = Cors::permissive();

        App::new()
            .wrap(cors) // ✅ Inject CORS middleware
            .app_data(db_client.clone())
            // Rotas globais obrigatórias de monitoramento do Render (baterão na raiz sem o prefixo)
            .route("/", web::get().to(health_check))
            .route("/health", web::get().to(health_check))
            // 🚀 Escopo centralizado injetando /api/v1 automaticamente para o Vue 3
            .service(
                web::scope("/api/v1")
                    .service(api::ingest_telemetry) // Escopo: /api/v1/telemetry (verifique se em api.rs está #[post("/telemetry")])
                    .route("/dashboard/metrics", web::get().to(get_dashboard_metrics)) // Escopo: /api/v1/dashboard/metrics
                    .route("/dashboard/logs", web::get().to(get_system_logs)) // Escopo: /api/v1/dashboard/logs
            )
    })
    .bind(("0.0.0.0", port))?
    .run();

    // Graceful shutdown handling
    let handle = server.handle();
    tokio::select! {
        _ = server => { tracing::warn!("Web node interrupted"); }
        _ = signal::ctrl_c() => { tracing::info!("SIGINT captured. Processing safe termination routine..."); }
    }

    let _ = shutdown_tx.send(true);
    handle.stop(true).await;
    pool.close().await;
    Ok(())
}
