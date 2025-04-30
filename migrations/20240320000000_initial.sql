-- Create continents table
CREATE TABLE IF NOT EXISTS continents_chirho (
    continent_id_chirho VARCHAR(36) PRIMARY KEY,
    name_chirho VARCHAR(255) NOT NULL UNIQUE,
    central_timezone_chirho VARCHAR(255) NOT NULL,
    created_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Create churches table
CREATE TABLE IF NOT EXISTS churches_chirho (
    church_id_chirho VARCHAR(36) PRIMARY KEY,
    name_chirho VARCHAR(255) NOT NULL,
    continent_id_chirho VARCHAR(36),
    church_timezone_chirho VARCHAR(255) NOT NULL,
    admin_details_note_chirho TEXT,
    internal_notes_chirho TEXT,
    member_access_token_chirho VARCHAR(255) NOT NULL UNIQUE,
    created_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (continent_id_chirho) REFERENCES continents_chirho(continent_id_chirho)
);

-- Create application administrators table
CREATE TABLE IF NOT EXISTS application_administrators_chirho (
    admin_id_chirho VARCHAR(36) PRIMARY KEY,
    username_chirho VARCHAR(255) NOT NULL UNIQUE,
    password_representation_chirho VARCHAR(255) NOT NULL,
    created_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create scheduled worship days table
CREATE TABLE IF NOT EXISTS scheduled_worship_days_chirho (
    schedule_id_chirho VARCHAR(36) PRIMARY KEY,
    church_id_chirho VARCHAR(36) NOT NULL,
    worship_date_chirho DATE NOT NULL,
    assigned_by_admin_id_chirho VARCHAR(36),
    created_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (church_id_chirho) REFERENCES churches_chirho(church_id_chirho),
    FOREIGN KEY (assigned_by_admin_id_chirho) REFERENCES application_administrators_chirho(admin_id_chirho),
    UNIQUE KEY unique_church_date_chirho (church_id_chirho, worship_date_chirho)
);

-- Create hourly signups table
CREATE TABLE IF NOT EXISTS hourly_signups_chirho (
    signup_id_chirho VARCHAR(36) PRIMARY KEY,
    schedule_id_chirho VARCHAR(36) NOT NULL,
    slot_hour_chirho INT NOT NULL CHECK (slot_hour_chirho BETWEEN 0 AND 23),
    participant_name_chirho VARCHAR(255) NOT NULL,
    signup_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (schedule_id_chirho) REFERENCES scheduled_worship_days_chirho(schedule_id_chirho)
); 