import logging
from typing import List
from fastapi import FastAPI, status
from pydantic import BaseModel
import numpy as np

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("AgriSentryAI")

app = FastAPI(title="AgriSentry AI Engine", version="1.0.0")

class TelemetryItem(BaseModel):
    id: str
    value: float
    created_at: str

class AnalysisRequest(BaseModel):
    readings: List[TelemetryItem]

class AnalysisResultItem(BaseModel):
    id: str
    created_at: str
    status: str
    note: str

class AnalysisResponse(BaseModel):
    results: List[AnalysisResultItem]

@app.post("/v1/analyze", response_model=AnalysisResponse, status_code=status.HTTP_200_OK)
async def analyze_telemetry_batch(payload: AnalysisRequest):
    logger.info(f"Received data matrix containing {len(payload.readings)} elements for quality evaluation")
    if not payload.readings:
        return AnalysisResponse(results=[])

    values = [item.value for item in payload.readings]
    mean = np.mean(values) if len(values) > 0 else 0.0
    std = np.std(values) if len(values) > 1 else 1.0

    analysis_results = []
    for item in payload.readings:
        # Dynamic Multi-Sigma Anomaly Detection Contingency Matrix (3-Sigma Rule)
        z_score = abs(item.value - mean) / std if std > 0 else 0.0

        if z_score > 3.0 or item.value > 120.0:
            classification = "ANOMALY_CRITICAL"
            note = f"Mathematical Outlier Detected: Signal outside standard variance boundaries (Z-Score: {z_score:.2f})"
        elif item.value <= 0.0:
            classification = "ANOMALY_NOISE"
            note = "Signal drop detected: Amplitude evaluation indicates sensor offline dead state"
        else:
            classification = "VALID"
            note = f"Evaluated safely within statistical deviation standards (Z-Score: {z_score:.2f})"

        analysis_results.append(
            AnalysisResultItem(id=item.id, created_at=item.created_at, status=classification, note=note)
        )

    return AnalysisResponse(results=analysis_results)