use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePdfFileChirho {
    pub church_id_chirho: String,
    pub file_name_chirho: String,
    pub description_chirho: Option<String>,
    pub is_public_chirho: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfFileChirho {
    pub pdf_file_id_chirho: String,
    pub church_id_chirho: String,
    pub file_name_chirho: String,
    pub file_path_chirho: String,
    pub description_chirho: Option<String>,
    pub is_public_chirho: bool,
    pub created_timestamp_chirho: DateTime<Utc>,
    pub updated_timestamp_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfFileResponseChirho {
    pub pdf_file_id_chirho: String,
    pub church_id_chirho: String,
    pub file_name_chirho: String,
    pub description_chirho: Option<String>,
    pub is_public_chirho: bool,
    pub created_timestamp_chirho: DateTime<Utc>,
    pub updated_timestamp_chirho: DateTime<Utc>,
}

impl PdfFileChirho {
    pub fn new(create_pdf_file_chirho: CreatePdfFileChirho, file_path_chirho: String) -> Self {
        Self {
            pdf_file_id_chirho: Uuid::new_v4().to_string(),
            church_id_chirho: create_pdf_file_chirho.church_id_chirho,
            file_name_chirho: create_pdf_file_chirho.file_name_chirho,
            file_path_chirho,
            description_chirho: create_pdf_file_chirho.description_chirho,
            is_public_chirho: create_pdf_file_chirho.is_public_chirho,
            created_timestamp_chirho: Utc::now(),
            updated_timestamp_chirho: Utc::now(),
        }
    }
}

impl From<PdfFileChirho> for PdfFileResponseChirho {
    fn from(pdf_file_chirho: PdfFileChirho) -> Self {
        Self {
            pdf_file_id_chirho: pdf_file_chirho.pdf_file_id_chirho,
            church_id_chirho: pdf_file_chirho.church_id_chirho,
            file_name_chirho: pdf_file_chirho.file_name_chirho,
            description_chirho: pdf_file_chirho.description_chirho,
            is_public_chirho: pdf_file_chirho.is_public_chirho,
            created_timestamp_chirho: pdf_file_chirho.created_timestamp_chirho,
            updated_timestamp_chirho: pdf_file_chirho.updated_timestamp_chirho,
        }
    }
} 