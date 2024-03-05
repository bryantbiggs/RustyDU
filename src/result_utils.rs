use crate::extract::ExtractionResults;
use crate::validate::ValidatedResults;
use csv::{Position, ReaderBuilder, Writer};
use std::fs::File;
use std::io::BufReader;
use std::{
  fs, io,
  path::{Path, PathBuf},
};

pub struct CSVWriter;

impl CSVWriter {
  pub fn write_extraction_results_to_csv(
    extraction_results: &ExtractionResults,
    document_path: &PathBuf,
    output_directory: &PathBuf,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let fields_to_extract = ["FieldName", "Value", "OcrConfidence", "Confidence", "IsMissing"];

    let file_name = Path::new(&document_path)
      .file_stem()
      .unwrap()
      .to_string_lossy()
      .to_string();
    let output_dir_path = Path::new(output_directory);
    fs::create_dir_all(&output_dir_path)?;

    let output_file = output_dir_path.join(file_name + ".csv");

    let mut writer = Writer::from_path(output_file)?;

    writer.write_record(&fields_to_extract)?;

    if let Some(fields) = &extraction_results.results_document.fields {
      for field in fields {
        let field_name = field.field_name.clone();
        let value = field.values[0].value.clone();
        let confidence = field.values[0].confidence.clone();
        let ocr_confidence = field.values[0].ocr_confidence.clone().to_string();
        let is_missing = field.is_missing.clone();

        writer.write_record(&[
          field_name,
          value,
          ocr_confidence,
          confidence.to_string(),
          is_missing.to_string(),
        ])?;
      }
    } else {
      println!("No fields found in extraction results.");
    }

    writer.flush()?;
    Ok(())
  }

  // pub fn write_validated_results_to_csv(
  //   validated_results: &ValidatedResults,
  //   extraction_results: &ExtractionResults,
  //   document_path: &PathBuf,
  //   output_directory: &PathBuf,
  // ) -> Option<Result<(), Box<dyn std::error::Error>>> {
  //   let file_name = Path::new(&document_path)
  //       .file_stem()
  //       .unwrap()
  //       .to_string_lossy()
  //       .to_string();
  //
  //   let output_dir_path = { Path::new(output_directory) };
  //
  //   let output_file = output_dir_path.join(file_name + ".csv");
  //
  //   let mut writer = Writer::from_path(output_file)?;
  //
  //   let fields_to_extract = [
  //     "FieldName",
  //     "Value",
  //     "OcrConfidence",
  //     "Confidence",
  //     "IsMissing",
  //     "ActualValue",
  //     "OperatorConfirmed",
  //     "IsCorrect",
  //   ];
  //
  //   writer.write_record(&fields_to_extract)?;
  //
  //   if let Some(validated_fields) = &validated_results
  //       .result
  //       .validated_extraction_results
  //       .results_document
  //       .fields
  //   {
  //     for validated_field in validated_fields {
  //       let field_name = &validated_field.field_name;
  //
  //       if let Some(extraction_field) = extraction_results
  //           .results_document
  //           .fields
  //           .as_ref()
  //           .and_then(|fields| fields.iter().find(|&field| field.field_name == *field_name))
  //       {
  //         let extracted_value = extraction_field
  //             .values
  //             .get(0)
  //             .and_then(|value| value.value.and_then(|v| v.as_str()))
  //             .unwrap_or_default();
  //         let confidence = extraction_field
  //             .values
  //             .get(0)
  //             .and_then(|value| value.confidence.and_then(|v| v.as_str()))
  //             .unwrap_or_default();
  //         let ocr_confidence = extraction_field
  //             .values
  //             .get(0)
  //             .and_then(|value| value.ocr_confidence.and_then(|v| v.as_str()))
  //             .unwrap_or_default();
  //         let is_missing = extraction_field
  //             .is_missing
  //             .then(|v| v.as_bool())
  //             .unwrap_or_default();
  //
  //         let validated_value = validated_field
  //             .values
  //             .get(0)
  //             .and_then(|value| value.value.and_then(|v| v.as_str()))
  //             .unwrap_or_default();
  //         let operator_confirmed = validated_field
  //             .operator_confirmed
  //             .and_then(|v| v.as_bool())
  //             .unwrap_or_default();
  //         let is_correct =
  //             validated_value.is_empty() && extracted_value.is_empty() || validated_value == extracted_value;
  //
  //         writer.write_record(&[
  //           field_name,
  //           extracted_value,
  //           ocr_confidence,
  //           confidence,
  //           &is_missing.to_string(),
  //           validated_value,
  //           &operator_confirmed.to_string(),
  //           &is_correct.to_string(),
  //         ])?;
  //       }
  //     }
  //   }
  // }

  pub fn print_csv_results(document_path: &PathBuf, output_directory: &PathBuf) -> Result<(), io::Error> {
    // Extract file name without extension
    let file_name = std::path::Path::new(document_path)
      .file_stem()
      .unwrap_or_default()
      .to_string_lossy();

    // Construct output directory path
    let output_dir_path = std::path::Path::new(output_directory);

    // Construct output file path with .csv extension
    let output_file = output_dir_path.join(format!("{}.csv", file_name));

    let file = File::open(&output_file)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    // Get headers
    let headers = csv_reader.headers()?.clone();

    // Get maximum widths of headers
    let mut max_widths = headers.iter().map(|header| header.len()).collect::<Vec<_>>();

    // Iterate over rows to find maximum widths
    for result in csv_reader.records() {
      let record = result?;
      for (index, field) in record.iter().enumerate() {
        max_widths[index] = max_widths[index].max(field.len());
      }
    }

    // Print headers
    let header_format = headers
      .iter()
      .zip(max_widths.iter())
      .map(|(header, &width)| format!("{:<width$}", header, width = width))
      .collect::<Vec<_>>()
      .join("|");
    println!("{}", header_format);
    println!("{}", "-".repeat(header_format.len()));

    // Reset reader to the beginning of the file
    let pos = Position::new();
    // Seek to the specified position (start of the file)
    csv_reader.seek(pos)?;
    // Print rows
    for result in csv_reader.records() {
      let record = result?;
      let row_format = record
        .iter()
        .zip(max_widths.iter())
        .map(|(field, &width)| format!("{:<width$}", field, width = width))
        .collect::<Vec<_>>()
        .join("|");
      println!("{}", row_format);
    }

    Ok(())
  }
}
