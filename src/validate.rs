use std::time::Duration;

use reqwest::{
  header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  Client,
};

pub struct Validate {
  base_url: String,
  project_id: String,
  bearer_token: String,
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
    extractor_id: &str,
    document_id: &str,
    extraction_results: &serde_json::Value,
  ) -> Option<serde_json::Value> {
    let client = Client::new();
    let api_url = format!(
      "{}/{}/extractors/{}/validation/start?api-version=1",
      self.base_url, self.project_id, extractor_id
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", self.bearer_token).parse().unwrap());
    headers.insert(ACCEPT, "text/plain".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let payload = serde_json::json!({
        "documentId": document_id,
        "actionTitle": format!("Validate - {}", extractor_id),
        "actionPriority": "Medium",
        "actionCatalog": "default_du_actions",
        "actionFolder": "Shared",
        "storageBucketName": "du_storage_bucket",
        "storageBucketDirectoryPath": "du_storage_bucket",
        "extractionResult": extraction_results["extractionResult"].clone(),
    });

    match client.post(&api_url).headers(headers).json(&payload).send().await {
      Ok(response) => {
        if response.status().is_success() {
          println!("Validation request sent!");
          if let Some(operation_id) = self
            .submit_extraction_validation_request(extractor_id, &(response))
            .await
          {
            return Some(operation_id);
          }
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
    _extractor_id: &str,
    response: &reqwest::Response,
  ) -> Option<serde_json::Value> {
    let response_data: serde_json::Value = match response.json().await {
      Ok(data) => data,
      Err(err) => {
        eprintln!("Error parsing JSON response: {}", err);
        return None;
      }
    };
    let operation_id = response_data.get("operationId").cloned()?;
    let extractor_id = match response_data["classificationResults"][0]["DocumentTypeId"].as_str() {
      Some(id) => id,
      None => return None,
    };

    let url = format!(
      "{}/{}/extractors/{}/validation/result/{}?api-version=1",
      self.base_url, self.project_id, extractor_id, operation_id
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
                        return Some(response_data);
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
    classification_results: &serde_json::Value,
  ) -> Option<String> {
    let api_url = format!(
      "{}/{}/classifiers/ml-classification/validation/start?api-version=1",
      self.base_url, self.project_id
    );

    let document_type_id = match classification_results["classificationResults"][0]["DocumentTypeId"].as_str() {
      Some(id) => id.to_string(),
      None => return None,
    };

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", self.bearer_token).parse().unwrap());
    headers.insert(ACCEPT, "text/plain".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let payload = serde_json::json!({
        "documentId": document_id,
        "actionTitle": format!("Validate - {}", document_type_id),
        "actionPriority": "Medium",
        "actionCatalog": "default_du_actions",
        "actionFolder": "Shared",
        "storageBucketName": "du_storage_bucket",
        "storageBucketDirectoryPath": "du_storage_bucket",
        "classificationResults": classification_results["classificationResults"].clone(),
    });

    match Client::new()
      .post(&api_url)
      .headers(headers)
      .json(&payload)
      .send()
      .await
    {
      Ok(response) => {
        if response.status().is_success() {
          println!("Classification Validation request sent!");
          if let Some(operation_id) = self.submit_classification_validation_request(&response).await {
            if let Some(document_type_id) = self.get_document_type_id(&operation_id).await {
              return Some(document_type_id);
            }
          }
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

  async fn submit_classification_validation_request(&self, response: &reqwest::Response) -> Option<String> {
    let response_data: serde_json::Value = match response.json().await {
      Ok(data) => data,
      Err(err) => {
        eprintln!("Error parsing JSON response: {}", err);
        return None;
      }
    };

    let operation_id = match response_data.get("operationId").and_then(|id| id.as_str()) {
      Some(id) => id.to_string(),
      None => return None,
    };

    let url = format!(
      "{}/{}/classifiers/ml-classification/validation/result/{}?api-version=1",
      self.base_url, self.project_id, operation_id
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
