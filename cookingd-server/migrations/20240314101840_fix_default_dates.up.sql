ALTER TABLE post ALTER COLUMN created_at SET DEFAULT now();
ALTER TABLE c_user ALTER COLUMN created_at SET DEFAULT now();
ALTER TABLE list ALTER COLUMN created_at SET DEFAULT now();
ALTER TABLE tag ALTER COLUMN created_at SET DEFAULT now();
