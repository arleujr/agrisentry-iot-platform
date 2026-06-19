-- Create custom data quality assessment type for strict data classification
CREATE TYPE dataqualitystatus AS ENUM ('PENDING', 'VALID', 'ANOMALY_NOISE', 'ANOMALY_CRITICAL');

-- Base hardware definitions table representing remote physical IoT nodes
CREATE TABLE "sensors" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "hardware_id" VARCHAR(64) UNIQUE NOT NULL,
    "sensor_type" VARCHAR(32) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Time-series telemetry tracking ledger 
CREATE TABLE "sensor_readings" (
    "id" UUID NOT NULL,
    "value" FLOAT8 NOT NULL,
    "sensor_id" UUID NOT NULL REFERENCES "sensors"("id") ON DELETE CASCADE,
    "status" dataqualitystatus NOT NULL DEFAULT 'PENDING',
    "ai_analysis_note" TEXT,
    "created_at" TIMESTAMPTZ NOT NULL,
    PRIMARY KEY ("id", "created_at")
);

-- System audit trail ledger for real-time observability pipelines
CREATE TABLE "system_events" (
    "id" BIGSERIAL PRIMARY KEY,
    "component" VARCHAR(32) NOT NULL,
    "message" TEXT NOT NULL,
    "level" VARCHAR(16) NOT NULL DEFAULT 'INFO',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed metadata records mimicking active telemetry sources on trucks/tractors
INSERT INTO "sensors" (hardware_id, sensor_type) VALUES 
('TRK-9402-SOIL', 'SoilMoisture'),
('TRK-9402-ENG',  'EngineTemperature'),
('TRK-9402-PH',   'SoilPH')
ON CONFLICT DO NOTHING;