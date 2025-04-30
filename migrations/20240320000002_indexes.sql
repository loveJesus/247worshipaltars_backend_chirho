-- For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.

-- Add indexes for continents table
CREATE INDEX idx_continents_name_chirho ON continents_chirho(name_chirho);

-- Add indexes for churches table
CREATE INDEX idx_churches_continent_chirho ON churches_chirho(continent_id_chirho);
CREATE INDEX idx_churches_name_chirho ON churches_chirho(name_chirho);
CREATE INDEX idx_churches_token_chirho ON churches_chirho(member_access_token_chirho);

-- Add indexes for application administrators table
CREATE INDEX idx_admins_username_chirho ON application_administrators_chirho(username_chirho);

-- Add indexes for scheduled worship days table
CREATE INDEX idx_schedules_church_chirho ON scheduled_worship_days_chirho(church_id_chirho);
CREATE INDEX idx_schedules_date_chirho ON scheduled_worship_days_chirho(worship_date_chirho);
CREATE INDEX idx_schedules_admin_chirho ON scheduled_worship_days_chirho(assigned_by_admin_id_chirho);
CREATE INDEX idx_schedules_church_date_chirho ON scheduled_worship_days_chirho(church_id_chirho, worship_date_chirho);

-- Add indexes for hourly signups table
CREATE INDEX idx_signups_schedule_chirho ON hourly_signups_chirho(schedule_id_chirho);
CREATE INDEX idx_signups_hour_chirho ON hourly_signups_chirho(slot_hour_chirho);
CREATE INDEX idx_signups_schedule_hour_chirho ON hourly_signups_chirho(schedule_id_chirho, slot_hour_chirho); 