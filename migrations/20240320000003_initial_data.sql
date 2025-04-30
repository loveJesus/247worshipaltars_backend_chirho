-- For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

-- Insert sample continents
INSERT INTO continents_chirho (continent_id_chirho, name_chirho, central_timezone_chirho)
VALUES 
    ('00000000-0000-0000-0000-000000000001', 'North America', 'America/New_York'),
    ('00000000-0000-0000-0000-000000000002', 'South America', 'America/Sao_Paulo'),
    ('00000000-0000-0000-0000-000000000003', 'Europe', 'Europe/London'),
    ('00000000-0000-0000-0000-000000000004', 'Asia', 'Asia/Tokyo'),
    ('00000000-0000-0000-0000-000000000005', 'Africa', 'Africa/Cairo'),
    ('00000000-0000-0000-0000-000000000006', 'Oceania', 'Australia/Sydney');

-- Insert sample churches
INSERT INTO churches_chirho (
    church_id_chirho,
    name_chirho,
    continent_id_chirho,
    church_timezone_chirho,
    admin_details_note_chirho,
    internal_notes_chirho,
    member_access_token_chirho
)
VALUES 
    ('00000000-0000-0000-0000-000000000101', 'New York Worship Center', '00000000-0000-0000-0000-000000000001', 'America/New_York', 'Main worship center', 'Active congregation', 'nywc-token-123'),
    ('00000000-0000-0000-0000-000000000102', 'Sao Paulo Chapel', '00000000-0000-0000-0000-000000000002', 'America/Sao_Paulo', 'Brazilian branch', 'Growing community', 'spc-token-456'),
    ('00000000-0000-0000-0000-000000000103', 'London Prayer House', '00000000-0000-0000-0000-000000000003', 'Europe/London', 'UK headquarters', 'Established ministry', 'lph-token-789'),
    ('00000000-0000-0000-0000-000000000104', 'Tokyo Sanctuary', '00000000-0000-0000-0000-000000000004', 'Asia/Tokyo', 'Japanese branch', 'New location', 'ts-token-012'),
    ('00000000-0000-0000-0000-000000000105', 'Cairo Worship Center', '00000000-0000-0000-0000-000000000005', 'Africa/Cairo', 'African headquarters', 'Strong presence', 'cwc-token-345'),
    ('00000000-0000-0000-0000-000000000106', 'Sydney Prayer House', '00000000-0000-0000-0000-000000000006', 'Australia/Sydney', 'Oceania branch', 'Growing ministry', 'sph-token-678');

-- Insert sample administrator (password: admin123)
INSERT INTO application_administrators_chirho (
    admin_id_chirho,
    username_chirho,
    password_representation_chirho
)
VALUES 
    ('00000000-0000-0000-0000-000000000201', 'admin', '$2y$10$8TWgvOYV8hdych.JOSXFBe4IByGe6oRB2/IxaBfbqoU.V0UfF82qK');

-- Insert sample scheduled worship days
INSERT INTO scheduled_worship_days_chirho (
    schedule_id_chirho,
    church_id_chirho,
    worship_date_chirho,
    assigned_by_admin_id_chirho
)
VALUES 
    ('00000000-0000-0000-0000-000000000301', '00000000-0000-0000-0000-000000000101', '2024-03-25', '00000000-0000-0000-0000-000000000201'),
    ('00000000-0000-0000-0000-000000000302', '00000000-0000-0000-0000-000000000102', '2024-03-26', '00000000-0000-0000-0000-000000000201'),
    ('00000000-0000-0000-0000-000000000303', '00000000-0000-0000-0000-000000000103', '2024-03-27', '00000000-0000-0000-0000-000000000201');

-- Insert sample hourly signups
INSERT INTO hourly_signups_chirho (
    signup_id_chirho,
    schedule_id_chirho,
    slot_hour_chirho,
    participant_name_chirho
)
VALUES 
    ('00000000-0000-0000-0000-000000000401', '00000000-0000-0000-0000-000000000301', 9, 'John Smith'),
    ('00000000-0000-0000-0000-000000000402', '00000000-0000-0000-0000-000000000301', 10, 'Mary Johnson'),
    ('00000000-0000-0000-0000-000000000403', '00000000-0000-0000-0000-000000000302', 11, 'David Brown'),
    ('00000000-0000-0000-0000-000000000404', '00000000-0000-0000-0000-000000000303', 12, 'Sarah Wilson'); 