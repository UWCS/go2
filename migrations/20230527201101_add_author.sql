-- Adds an author column to the table
ALTER TABLE redirects_new 
ADD COLUMN IF NOT EXISTS author text;