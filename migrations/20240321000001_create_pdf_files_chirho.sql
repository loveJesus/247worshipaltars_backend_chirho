-- Create pdf_files_chirho table
CREATE TABLE IF NOT EXISTS pdf_files_chirho (
    pdf_file_id_chirho VARCHAR(36) PRIMARY KEY,
    church_id_chirho VARCHAR(36) NOT NULL,
    file_name_chirho VARCHAR(255) NOT NULL,
    file_path_chirho VARCHAR(255) NOT NULL,
    description_chirho TEXT,
    is_public_chirho BOOLEAN DEFAULT false,
    created_timestamp_chirho TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_timestamp_chirho TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (church_id_chirho) REFERENCES churches_chirho(church_id_chirho)
); 