use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GptZeroPredictRequest {
    pub document: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GptZeroSentenceResult {
    #[serde(default)]
    pub sentence: String,
    #[serde(default)]
    pub perplexity: Option<f64>,
    #[serde(default)]
    pub generated_prob: Option<f64>,
    #[serde(default)]
    pub highlight_sentence_for_ai: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GptZeroClassProbabilities {
    #[serde(default)]
    pub ai: Option<f64>,
    #[serde(default)]
    pub human: Option<f64>,
    #[serde(default)]
    pub mixed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GptZeroDocumentResult {
    #[serde(default)]
    pub completely_generated_prob: Option<f64>,
    #[serde(default)]
    pub overall_burstiness: Option<f64>,
    #[serde(default)]
    pub document_classification: Option<String>,
    #[serde(default)]
    pub confidence_category: Option<String>,
    #[serde(default)]
    pub class_probabilities: Option<GptZeroClassProbabilities>,
    #[serde(default)]
    pub sentences: Vec<GptZeroSentenceResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GptZeroPredictResponse {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub scan_id: Option<String>,
    #[serde(default)]
    pub documents: Vec<GptZeroDocumentResult>,
}

/// Client for interacting with official GPTZero v2 Prediction API.
pub struct GptZeroClient {
    api_key: String,
    endpoint: String,
    timeout: Duration,
}

impl GptZeroClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            endpoint: "https://api.gptzero.me/v2/predict/text".to_string(),
            timeout: Duration::from_secs(30),
        }
    }

    /// Evaluates text using GPTZero prediction endpoint.
    pub fn predict_text(&self, text: &str) -> Result<GptZeroPredictResponse, String> {
        let request_body = GptZeroPredictRequest {
            document: text.to_string(),
        };

        let response = ureq::post(&self.endpoint)
            .set("x-api-key", &self.api_key)
            .set("Content-Type", "application/json")
            .set("Accept", "application/json")
            .timeout(self.timeout)
            .send_json(serde_json::to_value(&request_body).map_err(|e| e.to_string())?);

        match response {
            Ok(resp) => {
                let parsed: GptZeroPredictResponse = resp
                    .into_json()
                    .map_err(|e| format!("Failed to parse GPTZero response: {}", e))?;
                Ok(parsed)
            }
            Err(ureq::Error::Status(code, resp)) => {
                let err_text = resp
                    .into_string()
                    .unwrap_or_else(|_| "Unknown error".into());
                Err(format!("GPTZero API error (HTTP {}): {}", code, err_text))
            }
            Err(e) => Err(format!("Network error contacting GPTZero API: {}", e)),
        }
    }
}
