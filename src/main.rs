mod auth;
mod digitize;
mod classify;
mod extract;
mod validate;
mod result_utils;

use std::env;
use std::ffi::OsStr;
use clap::{Arg, ArgAction, Command};
use auth::Authentication;
use digitize::Digitize;
use classify::Classify;
use extract::Extract;
use validate::Validate;
use result_utils::CSVWriter;

// Load environment variables
fn load_env_vars() -> (String, String, String, String, String) {
    (
        env::var("APP_ID").unwrap(),
        env::var("APP_SECRET").unwrap(),
        env::var("AUTH_URL").unwrap(),
        env::var("BASE_URL").unwrap(),
        env::var("PROJECT_ID").unwrap(),
    )
}

// Function to load prompts from a JSON file based on the document type ID
fn load_prompts(document_type_id: &str) -> Option<serde_json::Value> {
    let prompts_directory = "Generative Prompts";
    let prompts_file = format!("{}/{}_prompts.json", prompts_directory, document_type_id);
    match std::fs::read_to_string(prompts_file) {
        Ok(contents) => {
            match serde_json::from_str(&contents) {
                Ok(prompts) => Some(prompts),
                Err(err) => {
                    eprintln!("Error parsing JSON prompts: {}", err);
                    None
                }
            }
        },
        Err(err) => {
            eprintln!("Error reading prompts file: {}", err);
            None
        }
    }
}

// Main function to process documents in the folder
fn process_documents_in_folder(folder_path: &str, validate_classification: bool, validate_extraction: bool, generative_classification: bool, generative_extraction: bool) {
    // Load environment variables
    let (app_id, app_secret, auth_url, base_url, project_id) = load_env_vars();

    // Initialize Authentication
    let auth = Authentication::new(&app_id, &app_secret, &auth_url);
    let bearer_token = auth.get_bearer_token();

    // Initialize API clients
    let digitize_client = Digitize::new(&base_url, &project_id, &bearer_token);
    let classify_client = Classify::new(&base_url, &project_id, &bearer_token);
    let extract_client = Extract::new(&base_url, &project_id, &bearer_token);
    let validate_client = Validate::new(&base_url, &project_id, &bearer_token);

    // Load classification prompts if generative_classification is enabled
    let classifier = if generative_classification { "generative_classifier" } else { "ml-classification" };
    let classification_prompts = if generative_classification { load_prompts("classification") } else { None };

    // Iterate through files in the specified folder
    for entry in std::fs::read_dir(folder_path).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let extension = extension.to_string_lossy().to_lowercase();
                if extension == "png" || extension == "jpe" || extension == "jpg" || extension == "jpeg" || extension == "tiff" || extension == "tif" || extension == "bmp" || extension == "pdf" {
                    println!("Processing document: {:?}", path);
                    match digitize_client.start(&path) {
                        Some(document_id) => {
                            match classify_client.classify_document(&document_id, classifier, classification_prompts.clone(), validate_classification) {
                                Some(document_type_id) => {
                                    if validate_classification {
                                        if let Some(classification_results) = validate_client.validate_classification_results(&document_id, &document_type_id) {
                                            let extraction_prompts = if generative_extraction { load_prompts(&document_type_id) } else { None };
                                            let classification_results = if generative_extraction { "generative_extractor" } else { classification_results };
                                            if let Some(extraction_results) = extract_client.extract_document(classification_results, &document_id, extraction_prompts) {
                                                if !validate_extraction {
                                                    if let Err(err) = CSVWriter::write_extraction_results_to_csv(&extraction_results, &path) {
                                                        eprintln!("Error writing extraction results to CSV: {}", err);
                                                    }
                                                    CSVWriter::pprint_csv_results(&path);
                                                } else {
                                                    if let Some(validated_results) = validate_client.validate_extraction_results(&document_type_id, &document_id, &extraction_results) {
                                                        if let Err(err) = CSVWriter::write_validated_results_to_csv(&validated_results, &extraction_results, &path) {
                                                            eprintln!("Error writing validated results to CSV: {}", err);
                                                        }
                                                        CSVWriter::pprint_csv_results(&path);
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        let classification_results = document_type_id.clone();
                                        if let Some(extraction_prompts) = if generative_extraction { load_prompts(&document_type_id) } else { None } {
                                            let classification_results = if generative_extraction { "generative_extractor" } else { classification_results };
                                            if let Some(extraction_results) = extract_client.extract_document(classification_results, &document_id, extraction_prompts) {
                                                if !validate_extraction {
                                                    if let Err(err) = CSVWriter::write_extraction_results_to_csv(&extraction_results, &path) {
                                                        eprintln!("Error writing extraction results to CSV: {}", err);
                                                    }
                                                    CSVWriter::pprint_csv_results(&path);
                                                } else {
                                                    if let Some(validated_results) = validate_client.validate_extraction_results(&document_type_id, &document_id, &extraction_results) {
                                                        if let Err(err) = CSVWriter::write_validated_results_to_csv(&validated_results, &extraction_results, &path) {
                                                            eprintln!("Error writing validated results to CSV: {}", err);
                                                        }
                                                        CSVWriter::pprint_csv_results(&path);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                None => println!("Error classifying document {:?}", path),
                            }
                        },
                        None => println!("Error digitizing document {:?}", path),
                    }
                }
            }
        }
    }
}

fn main() {
    // Define command-line arguments using clap
    let matches = Command::new("Document Processor")
        .version("1.0")
        .author("Your Name")
        .about("Process documents in a folder")
        .arg(Arg::new("folder")
            .short("f")
            .long("folder")
            .value_name("FOLDER")
            .help("Sets the folder path containing documents to process")
            .action(clap::ArgAction::Set)
            .required(true))
        .arg(Arg::new("validate_classification")
            .long("validate-classification")
            .help("Enables classification validation"))
        .arg(Arg::new("validate_extraction")
            .long("validate-extraction")
            .help("Enables extraction validation"))
        .arg(Arg::new("generative_classification")
            .long("generative-classification")
            .help("Enables generative classification"))
        .arg(Arg::new("generative_extraction")
            .long("generative-extraction")
            .help("Enables generative extraction"))
        .get_matches();

    let folder_path = matches.try_get_one("folder");
    let validate_classification = matches.get_flag("validate_classification");
    let validate_extraction = matches.get_flag("validate_extraction");
    let generative_classification = matches.get_flag("generative_classification");
    let generative_extraction = matches.get_flag("generative_extraction");


    // Call the main processing function with the parsed arguments
    process_documents_in_folder(folder_path, validate_classification, validate_extraction,
                                generative_classification, generative_extraction);
}
