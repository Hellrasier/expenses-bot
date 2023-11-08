-- Add migration script here

ALTER TABLE new_expenses
ALTER COLUMN user_id TYPE BIGINT;