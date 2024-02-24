# Document Understanding Cloud APIs Example

This code snippet demonstrates how to digitize, classify, and extract documents using UiPath Document Understanding API's.

## Official Documentation

UiPath Document Understanding offers standalone capabilities, allowing integration with external tools and systems through APIs. This release includes APIs for Discovery, Digitization, Classification, Extraction, and Validation. Please take a look at the [Official Documentation](https://docs.uipath.com/document-understanding/automation-cloud/latest/api-guide/example).

## Requirements

- Rust 1.74.1

## Setup

1. Clone the repository to your local machine:

    ```bash
    git clone https://github.com/nvpnathan/RustyDU.git
    ```

2. Navigate to the project directory:

    ```bash
    cd RustyDU
    ```

3. Install the required dependencies:

    ```bash
    cargo build
    ```

4. Set up your environment variables by creating a `.env` file in the root directory and adding the following variables:

  ```env
  APP_ID=
  APP_SECRET=
  AUTH_URL=https://cloud.uipath.com/identity_/connect/token
  BASE_URL=https://cloud.uipath.com/<Cloud Org>/<Cloud Tenant>/du_/api/framework/projects/
  PROJECT_ID=00000000-0000-0000-0000-000000000000
  ```

## Usage

### Processing Documents

1. Place the documents you want to process in the specified folder (`example_documents` by default).

2. Run the main script `main.rs` to process the documents:


3. Monitor the console output for processing status and any errors.

4. Extracted results will be printed to the console and saved in CSV format in `output_results` folder.

## File Structure

The project structure is organized as follows:
```bash
RustyDU/
│
├── src/
│   ├── main.rs         # Main entry point for the application
│   ├── auth.rs         # Authentication module for obtaining bearer token
│   ├── digitize.rs     # Digitize module for initiating document digitization
│   ├── classify.rs     # Classify module for document classification
│   ├── extract.rs      # Extract module for document extraction
│   ├── validate.rs     # Validate module for document validation
│   └── result_utils.rs # Utility module for printing and writing extraction results
│
├── .env.example         # Example environment variables file
├── Cargo.toml           # Rust package configuration file
├── example_documents/   # Folder containing example documents
├── generative_prompts/ # Folder containing Extraction and Classification Prompt Templates
└── output_results/      # Folder containing the CSV's of the Document Extraction Results

```

## TODO

* Everything
