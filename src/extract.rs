use reqwest::{
  header::{AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Extract {
  base_url: String,
  project_id: String,
  bearer_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ExtractionData<'a> {
  document_id: &'a str,
  #[serde(flatten)]
  prompts: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractionResults {
  pub document_id: String,
  pub results_version: i32,
  pub results_document: ResultsDocument,
  pub extractor_payloads: Option<String>,
  pub business_rules_results: Option<String>,
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
  pub values: Vec<FieldValue>,
  pub data_version: i32,
  pub operator_confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldValue {
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

impl Extract {
  pub fn new(base_url: &str, project_id: &str, bearer_token: &str) -> Extract {
    Extract {
      base_url: base_url.to_string(),
      project_id: project_id.to_string(),
      bearer_token: bearer_token.to_string(),
    }
  }

  pub async fn extract_document(
    &self,
    extractor_id: &str,
    document_id: &str,
    prompts: Option<Value>,
  ) -> Option<ExtractionResults> {
    // Define the API endpoint for document extraction
    let api_url = format!(
      "{}/{}/extractors/{}/extraction?api-version=1",
      self.base_url, self.project_id, extractor_id
    );

    // Prepare request data
    let data = ExtractionData { document_id, prompts };

    // Prepare request
    let client = Client::new();
    let response = client
      .post(&api_url)
      .header(AUTHORIZATION, format!("Bearer {}", self.bearer_token))
      .header(CONTENT_TYPE, "application/json")
      .json(&data)
      .send()
      .await;

    // Process response
    match response {
      Ok(response) => match response.status() {
        reqwest::StatusCode::OK => {
          println!("Document successfully extracted!\n");
          let extraction_results: ExtractionResults = response.json().await.unwrap();
          Some(extraction_results)
        }
        _ => {
          println!("Error: {} - {}", response.status(), response.text().await.unwrap());
          None
        }
      },
      Err(e) => {
        println!("An error occurred during extraction: {}", e);
        None
      }
    }
  }
}
