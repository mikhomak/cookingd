ALTER TABLE post ALTER COLUMN created_at SET DEFAULT 'NOW'::timestamptz;
ALTER TABLE c_user ALTER COLUMN created_at SET DEFAULT 'NOW'::timestamptz;
ALTER TABLE list ALTER COLUMN created_at SET DEFAULT 'NOW'::timestamptz;
ALTER TABLE tag ALTER COLUMN created_at SET DEFAULT 'NOW'::timestamptz;