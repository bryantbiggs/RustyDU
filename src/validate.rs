use std::time::Duration;

use reqwest::{
  header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error};

use crate::{classify::ClassificationResults, extract::ExtractionResults};

pub struct Validate {
  base_url: String,
  project_id: String,
  bearer_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OperationResponse {
  operation_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedResults {
  pub result: ValidationResult,
  pub status: String,
  pub created_at: String,
  pub last_updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
  pub action_data: ActionData,
  pub validated_extraction_results: ValidatedExtractionResults,
  pub action_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionData {
  pub action_data_type: String,
  pub id: i32,
  pub status: String,
  pub title: String,
  pub priority: String,
  pub task_catalog_name: String,
  pub task_url: String,
  pub folder_path: String,
  pub folder_id: i32,
  pub data: ActionDataDetail,
  pub action: String,
  pub is_deleted: bool,
  pub assigned_to_user: Option<User>,
  pub creator_user: User,
  pub deleter_user: Option<User>,
  pub last_modifier_user: User,
  pub completed_by_user: User,
  pub creation_time: String,
  pub last_assigned_time: String,
  pub completion_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDataDetail {
  pub validated_extraction_results_path: String,
  pub document_rejection_details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub email_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedExtractionResults {
  pub document_id: String,
  pub results_version: i32,
  pub results_document: ResultsDocument,
  pub extractor_payloads: Option<serde_json::Value>,
  pub business_rules_results: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsDocument {
  pub bounds: Bounds,
  pub language: String,
  pub document_group: String,
  pub document_category: String,
  pub document_type_id: String,
  pub document_type_name: String,
  pub document_type_data_version: i32,
  pub data_version: i32,
  pub document_type_source: String,
  pub document_type_field: DocumentTypeField,
  pub fields: Option<Vec<Field>>,
  pub tables: Option<Vec<Table>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
  pub start_page: i32,
  pub page_count: i32,
  pub text_start_index: i32,
  pub text_length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentTypeField {
  pub components: Vec<String>,
  pub value: String,
  pub unformatted_value: String,
  pub reference: Reference,
  pub derived_fields: Vec<String>,
  pub confidence: f64,
  pub operator_confirmed: bool,
  pub ocr_confidence: f64,
  pub text_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
  pub text_start_index: i32,
  pub text_length: i32,
  pub tokens: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub text_start_index: i32,
  pub text_length: i32,
  pub page: i32,
  pub page_width: f64,
  pub page_height: f64,
  pub boxes: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
  pub field_id: String,
  pub field_name: String,
  pub field_type: String,
  pub is_missing: bool,
  pub data_source: String,
  pub values: Vec<Value>,
  pub data_version: i32,
  pub operator_confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
  pub components: Vec<String>,
  pub value: String,
  pub unformatted_value: String,
  pub reference: Reference,
  pub derived_fields: Vec<String>,
  pub confidence: f64,
  pub operator_confirmed: bool,
  pub ocr_confidence: f64,
  pub text_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
  pub field_id: String,
  pub field_name: String,
  pub is_missing: bool,
  pub data_source: String,
  pub data_version: i32,
  pub operator_confirmed: bool,
  pub values: Vec<TableValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableValue {
  pub operator_confirmed: bool,
  pub confidence: f64,
  pub ocr_confidence: f64,
  pub cells: Vec<Cell>,
  pub column_info: Vec<ColumnInfo>,
  pub number_of_rows: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
  pub row_index: i32,
  pub column_index: i32,
  pub is_header: bool,
  pub is_missing: bool,
  pub operator_confirmed: bool,
  pub data_source: String,
  pub data_version: i32,
  pub values: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
  pub field_id: String,
  pub field_name: String,
  pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateClassification {
  pub result: ClassificationResult,
  pub status: String,
  pub created_at: String,
  pub last_updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationResult {
  pub action_data: ActionData,
  pub validated_classification_results: Vec<ValidatedClassificationResult>,
  pub action_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionDataDetails {
  pub validated_classification_results_path: String,
  pub document_rejection_details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedClassificationResult {
  pub document_type_id: String,
  pub document_id: String,
  pub confidence: f64,
  pub ocr_confidence: f64,
  pub reference: Reference,
  pub document_bounds: DocumentBounds,
  pub classifier_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentBounds {
  pub start_page: usize,
  pub page_count: usize,
  pub text_start_index: usize,
  pub text_length: usize,
}

impl Validate {
  pub fn new(base_url: &str, project_id: &str, bearer_token: &str) -> Self {
    Validate {
      base_url: base_url.to_string(),
      project_id: project_id.to_string(),
      bearer_token: bearer_token.to_string(),
    }
  }

  pub async fn validate_extraction_results(
    &self,
    document_type_id: &str,
    document_id: &str,
    extraction_results: &ExtractionResults,
  ) -> Option<ValidatedResults> {
    let client = Client::new();

    let api_url = format!(
      "{}/{}/extractors/{}/validation/start?api-version=1",
      self.base_url, self.project_id, document_type_id
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", self.bearer_token).parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let payload = json!({
        "documentId": document_id,
        "actionTitle": format!("Validate - {}", document_type_id),
        "actionPriority": "Medium",
        "actionCatalog": "default_du_actions",
        "actionFolder": "Shared",
        "storageBucketName": "du_storage_bucket",
        "storageBucketDirectoryPath": "du_storage_bucket",
        "extractionResult": extraction_results.clone(),
    });

    match client.post(&api_url).headers(headers).json(&payload).send().await {
      Ok(response) => {
        if response.status().is_success() {
          println!("Extraction Validation request sent!");
          let response_json: OperationResponse = response.json().await.ok()?;
          let operation_id = response_json.operation_id;
          Some(operation_id.clone());
          return self
            .submit_extraction_validation_request(&document_type_id, &operation_id)
            .await;
        } else {
          println!(
            "Error: {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
          );
        }
      }
      Err(err) => println!("An error occurred during validation: {}", err),
    }
    None
  }

  async fn submit_extraction_validation_request(
    &self,
    document_type_id: &str,
    operation_id: &str,
  ) -> Option<ValidatedResults> {
    let url = format!(
      "{}/{}/extractors/{}/validation/result/{}?api-version=1",
      self.base_url, self.project_id, document_type_id, operation_id
    );

    let client = Client::new();

    loop {
      match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", self.bearer_token))
        .send()
        .await
      {
        Ok(response) => {
          let response_data: serde_json::Value = match response.json().await {
            Ok(data) => data,
            Err(err) => {
              eprintln!("Error parsing JSON response: {}", err);
              return None;
            }
          };

          match response_data["status"].as_str() {
            Some("Succeeded") => {
              println!("Extraction Validation request submitted successfully!");
              loop {
                match client
                  .get(&url)
                  .header("Authorization", format!("Bearer {}", self.bearer_token))
                  .send()
                  .await
                {
                  Ok(response) => {
                    let response_data: serde_json::Value = match response.json().await {
                      Ok(data) => data,
                      Err(err) => {
                        eprintln!("Error parsing JSON response: {}", err);
                        return None;
                      }
                    };

                    match response_data["result"]["actionData"]["status"].as_str() {
                      Some("Unassigned") => {
                        println!("Validation Document Extraction is unassigned. Waiting...")
                      }
                      Some("Pending") => {
                        println!("Validate Document Extraction in progress. Waiting...")
                      }
                      Some("Completed") => {
                        println!("Validate Document Extraction is completed.");
                        let validated_results: Result<ValidatedResults, _> =
                          serde_json::from_value(response_data.clone());
                        match validated_results {
                          Ok(validated_results) => {
                            return Some(validated_results);
                          }
                          _ => {}
                        }
                      }
                      Some(status) => println!("Unknown validation action status: {}", status),
                      None => {
                        println!("No status found in actionData");
                        return None;
                      }
                    }
                    tokio::time::sleep(Duration::from_secs(5)).await; // Wait for 5 seconds before
                                                                      // checking again
                  }
                  Err(err) => {
                    eprintln!("Error sending request: {}", err);
                    return None;
                  }
                }
              }
            }
            Some("NotStarted") | Some("Running") | Some("Unassigned") => {
              println!("Validation request status: {}", response_data["status"])
            }
            Some(status) => println!("Unknown validation request status: {}", status),
            None => {
              println!("No status found in response");
              return None;
            }
          }
          tokio::time::sleep(Duration::from_secs(5)).await; // Wait for 5 seconds before checking again
        }
        Err(err) => {
          eprintln!("Error sending request: {}", err);
          return None;
        }
      }
    }
  }

  pub async fn validate_classification_results(
    &self,
    document_id: &str,
    classification_results: &ClassificationResults,
  ) -> Option<String> {
    let client = Client::new();

    let api_url = format!(
      "{}/{}/classifiers/ml-classification/validation/start?api-version=1",
      self.base_url, self.project_id
    );

    let document_type_id = classification_results
      .classification_results
      .get(0) // Get the first element
      .map(|result| result.document_type_id.clone());

    let action_title = match document_type_id {
      Some(id) => format!("Validate - {}", id),
      None => "Validate - Unknown".to_string(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", self.bearer_token).parse().unwrap());
    headers.insert(ACCEPT, "text/plain".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let payload = serde_json::json!({
        "documentId": document_id,
        "actionTitle": action_title,
        "actionPriority": "Medium",
        "actionCatalog": "default_du_actions",
        "actionFolder": "Shared",
        "storageBucketName": "du_storage_bucket",
        "storageBucketDirectoryPath": "du_storage_bucket",
        "classificationResults": classification_results,
    });

    match client.post(&api_url).headers(headers).json(&payload).send().await {
      Ok(response) => {
        if response.status().is_success() {
          println!("Classification Validation request sent!");
          let response_json: OperationResponse = response.json().await.ok()?;
          let operation_id = response_json.operation_id;
          Some(operation_id.clone());
          return self.submit_classification_validation_request(&operation_id).await;
        } else {
          println!(
            "Error: {} - {}",
            response.status(),
            response.text().await.unwrap_or_default()
          );
        }
      }
      Err(err) => println!("An error occurred during validation: {}", err),
    }
    None
  }

  async fn submit_classification_validation_request(&self, operation_id: &str) -> Option<String> {
    let api_url = format!(
      "{}/{}/classifiers/ml-classification/validation/result/{}?api-version=1",
      self.base_url, self.project_id, operation_id
    );
    let client = Client::new();

    loop {
      match client
        .get(&api_url)
        .header("Authorization", format!("Bearer {}", self.bearer_token))
        .send()
        .await
      {
        Ok(response) => {
          let response_data: serde_json::Value = match response.json().await {
            Ok(data) => data,
            Err(err) => {
              eprintln!("Error parsing JSON response: {}", err);
              return None;
            }
          };

          match response_data["status"].as_str() {
            Some("Succeeded") => {
              println!("Classification Validation request submitted successfully!");
              if let Some(document_type_id) = self.get_document_type_id(&operation_id).await {
                return Some(document_type_id);
              }
            }
            Some("NotStarted") | Some("Running") | Some("Unassigned") => {
              println!("Validation request status: {}", response_data["status"])
            }
            Some(status) => println!("Unknown validation request status: {}", status),
            None => {
              println!("No status found in response");
              return None;
            }
          }
          tokio::time::sleep(Duration::from_secs(5)).await; // Wait for 5 seconds before checking again
        }
        Err(err) => {
          eprintln!("Error sending request: {}", err);
          return None;
        }
      }
    }
  }

  async fn get_document_type_id(&self, operation_id: &str) -> Option<String> {
    let api_url = format!(
      "{}/{}/classifiers/ml-classification/validation/result/{}?api-version=1",
      self.base_url, self.project_id, operation_id
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", self.bearer_token).parse().unwrap());
    headers.insert(ACCEPT, "text/plain".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    loop {
      let mut request = Client::new().get(&api_url);
      for (name, value) in &headers {
        request = request.header(name.clone(), value.clone());
      }

      match request.send().await {
        Ok(response) => {
          let response_data: serde_json::Value = match response.json().await {
            Ok(data) => data,
            Err(err) => {
              eprintln!("Error parsing JSON response: {}", err);
              return None;
            }
          };

          match response_data["status"].as_str() {
            Some("Succeeded") => {
              println!("Classification Validation request submitted successfully!");
              if let Some(document_type_id) =
                response_data["result"]["validatedClassificationResults"][0]["DocumentTypeId"].as_str()
              {
                return Some(document_type_id.to_string());
              }
            }
            Some("NotStarted") | Some("Running") | Some("Unassigned") => {
              println!("Validation request status: {}", response_data["status"])
            }
            Some(status) => println!("Unknown validation request status: {}", status),
            None => {
              println!("No status found in response");
              return None;
            }
          }
          tokio::time::sleep(Duration::from_secs(5)).await; // Wait for 5 seconds before checking again
        }
        Err(err) => {
          eprintln!("Error sending request: {}", err);
          return None;
        }
      }
    }
  }
}
