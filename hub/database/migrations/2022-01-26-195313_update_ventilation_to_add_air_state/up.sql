-- Create the `air_state` enum.
CREATE TYPE air_state AS ENUM ('paused', 'running');

-- Add the `state` column to the `air` table.
ALTER TABLE air ADD COLUMN state air_state;
