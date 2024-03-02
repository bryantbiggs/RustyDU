use reqwest::{
  header::{AUTHORIZATION, CONTENT_TYPE},
  Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct Extract {
  base_url: String,
  project_id: String,
  bearer_token: String,
}

#[derive(Serialize)]
struct ExtractionData<'a> {
  DocumentId: &'a str,
  #[serde(flatten)]
  prompts: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExtractionResult {
  DocumentId: String,
  ResultsVersion: i32,
  ResultsDocument: ResultsDocument,
  ExtractorPayloads: Option<String>,
  BusinessRulesResults: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultsDocument {
  Bounds: Bounds,
  Language: String,
  DocumentGroup: String,
  DocumentCategory: String,
  DocumentTypeId: String,
  DocumentTypeName: String,
  DocumentTypeDataVersion: i32,
  DataVersion: i32,
  DocumentTypeSource: String,
  DocumentTypeField: DocumentTypeField,
  Fields: Vec<Field>,
  Tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bounds {
  StartPage: i32,
  PageCount: i32,
  TextStartIndex: i32,
  TextLength: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentTypeField {
  Components: Vec<String>,
  Value: String,
  UnformattedValue: String,
  Reference: Reference,
  DerivedFields: Vec<String>,
  Confidence: f64,
  OperatorConfirmed: bool,
  OcrConfidence: f64,
  TextType: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reference {
  TextStartIndex: i32,
  TextLength: i32,
  Tokens: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
  TextStartIndex: i32,
  TextLength: i32,
  Page: i32,
  PageWidth: f64,
  PageHeight: f64,
  Boxes: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Field {
  FieldId: String,
  FieldName: String,
  FieldType: String,
  IsMissing: bool,
  DataSource: String,
  Values: Vec<Value>,
  DataVersion: i32,
  OperatorConfirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldValue {
  Components: Vec<String>,
  Value: String,
  UnformattedValue: String,
  Reference: Reference,
  DerivedFields: Vec<String>,
  Confidence: f64,
  OperatorConfirmed: bool,
  OcrConfidence: f64,
  TextType: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
  FieldId: String,
  FieldName: String,
  IsMissing: bool,
  DataSource: String,
  DataVersion: i32,
  OperatorConfirmed: bool,
  Values: Vec<TableValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableValue {
  OperatorConfirmed: bool,
  Confidence: f64,
  OcrConfidence: f64,
  Cells: Vec<Cell>,
  ColumnInfo: Vec<ColumnInfo>,
  NumberOfRows: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cell {
  RowIndex: i32,
  ColumnIndex: i32,
  IsHeader: bool,
  IsMissing: bool,
  OperatorConfirmed: bool,
  DataSource: String,
  DataVersion: i32,
  Values: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnInfo {
  FieldId: String,
  FieldName: String,
  FieldType: String,
}

impl Extract {
  pub fn new(base_url: &str, project_id: &str, bearer_token: &str) -> Extract {
    Extract {
      base_url: base_url.to_string(),
      project_id: project_id.to_string(),
      bearer_token: bearer_token.to_string(),
    }
  }

  pub async fn extract_document(&self, extractor_id: &str, document_id: &str, prompts: Option<Value>) -> Option<Value> {
    // Define the API endpoint for document extraction
    let api_url = format!(
      "{}/{}/extractors/{}/extraction?api-version=1",
      self.base_url, self.project_id, extractor_id
    );

    // Prepare request data
    let data = ExtractionData {
      DocumentId: document_id,
      prompts,
    };

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
          let extraction_results: ExtractionResult = response.json().await.unwrap();
          Some(json!(extraction_results).unwrap())
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
