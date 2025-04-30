-- Create hourly signups table
CREATE TABLE IF NOT EXISTS hourly_signups_chirho (
    signup_id_chirho VARCHAR(36) PRIMARY KEY,
    schedule_id_chirho VARCHAR(36) NOT NULL,
    user_id_chirho VARCHAR(36) NOT NULL,
    status_chirho VARCHAR(50) NOT NULL,
    created_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_timestamp_chirho TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (schedule_id_chirho) REFERENCES schedules_chirho(schedule_id_chirho),
    FOREIGN KEY (user_id_chirho) REFERENCES users_chirho(user_id_chirho)
); 