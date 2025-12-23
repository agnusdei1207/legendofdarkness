-- Add description and metadata to monster_definitions
ALTER TABLE monster_definitions 
ADD COLUMN description TEXT,
ADD COLUMN metadata JSONB DEFAULT '{}';

-- Update some descriptions
UPDATE monster_definitions 
SET description = 'A slimy creature that lives in damp places.' 
WHERE name = 'Slime';

UPDATE monster_definitions 
SET description = 'A small but aggressive green goblin.' 
WHERE name = 'Goblin';

UPDATE monster_definitions 
SET description = 'An animated skeleton of a fallen warrior.' 
WHERE name = 'Skeleton';
