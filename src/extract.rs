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
    DocumentId: &'a str,
    #[serde(flatten)]
    prompts: Option<Value>,
}

#[derive(Deserialize)]
struct ExtractedData {
    // Define the structure of the extracted data
}

#[derive(Debug, Serialize, Deserialize)]
struct ExtractionResult {
    document_id: String,
    results_version: i32,
    results_document: ResultsDocument,
    extractor_payloads: Option<String>,
    business_rules_results: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultsDocument {
    bounds: Bounds,
    language: String,
    document_group: String,
    document_category: String,
    document_type_id: String,
    document_type_name: String,
    document_type_data_version: i32,
    data_version: i32,
    document_type_source: String,
    document_type_field: DocumentTypeField,
    fields: Vec<Field>,
    tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bounds {
    start_page: i32,
    page_count: i32,
    text_start_index: i32,
    text_length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentTypeField {
    components: Vec<String>,
    value: String,
    unformatted_value: String,
    reference: Reference,
    derived_fields: Vec<String>,
    confidence: f64,
    operator_confirmed: bool,
    ocr_confidence: f64,
    text_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Reference {
    text_start_index: i32,
    text_length: i32,
    tokens: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    text_start_index: i32,
    text_length: i32,
    page: i32,
    page_width: f64,
    page_height: f64,
    boxes: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Field {
    field_id: String,
    field_name: String,
    field_type: String,
    is_missing: bool,
    data_source: String,
    values: Vec<Value>,
    data_version: i32,
    operator_confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldValue {
    components: Vec<String>,
    value: String,
    unformatted_value: String,
    reference: Reference,
    derived_fields: Vec<String>,
    confidence: f64,
    operator_confirmed: bool,
    ocr_confidence: f64,
    text_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    field_id: String,
    field_name: String,
    is_missing: bool,
    data_source: String,
    data_version: i32,
    operator_confirmed: bool,
    values: Vec<TableValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableValue {
    operator_confirmed: bool,
    confidence: f64,
    ocr_confidence: f64,
    cells: Vec<Cell>,
    column_info: Vec<ColumnInfo>,
    number_of_rows: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cell {
    row_index: i32,
    column_index: i32,
    is_header: bool,
    is_missing: bool,
    operator_confirmed: bool,
    data_source: String,
    data_version: i32,
    values: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnInfo {
    field_id: String,
    field_name: String,
    field_type: String,
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
            DocumentId: document_id,
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
