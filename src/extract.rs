use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE}};
use serde::{Serialize, Deserialize};
use serde_json::Value;

pub struct Extract {
    base_url: String,
    project_id: String,
    bearer_token: String,
}

#[derive(Serialize)]
struct ExtractionData<'a> {
    documentId: &'a str,
    #[serde(flatten)]
    prompts: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct ExtractedData {
    // Define the structure of the extracted data
}

impl Extract {
    pub fn new(base_url: &str, project_id: &str, bearer_token: &str) -> Extract {
        Extract {
            base_url: base_url.to_string(),
            project_id: project_id.to_string(),
            bearer_token: bearer_token.to_string(),
        }
    }

    pub async fn extract_document(&self, extractor_id: &str, document_id: &str, prompts: Option<Value>) -> Option<ExtractedData> {
        // Define the API endpoint for document extraction
        let api_url = format!("{}/{}/extractors/{}/extraction?api-version=1", self.base_url, self.project_id, extractor_id);

        // Prepare request data
        let data = ExtractionData {
            documentId,
            prompts,
        };

        // Prepare request
        let client = Client::new();
        let response = client.post(&api_url)
            .header(AUTHORIZATION, format!("Bearer {}", self.bearer_token))
            .header(CONTENT_TYPE, "application/json")
            .json(&data)
            .send()
            .await;

        // Process response
        match response {
            Ok(response) => {
                match response.status() {
                    reqwest::StatusCode::OK => {
                        println!("Document successfully extracted!\n");
                        Some(response.json().await.unwrap())
                    },
                    _ => {
                        println!("Error: {} - {}", response.status(), response.text().await.unwrap());
                        None
                    }
                }
            },
            Err(e) => {
                println!("An error occurred during extraction: {}", e);
                None
            }
        }
    }
}
