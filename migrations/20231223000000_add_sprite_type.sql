-- Add sprite_type column to monster_definitions
-- This identifies the sprite folder name (e.g., 'slime', 'rat', 'goblin')
ALTER TABLE monster_definitions ADD COLUMN IF NOT EXISTS sprite_type VARCHAR(50);

-- Update existing monster data with sprite types
UPDATE monster_definitions SET sprite_type = 'rat' WHERE name = 'Giant Rat';
UPDATE monster_definitions SET sprite_type = 'bat' WHERE name = 'Vampire Bat';
UPDATE monster_definitions SET sprite_type = 'slime' WHERE name = 'Slime';
UPDATE monster_definitions SET sprite_type = 'wolf' WHERE name LIKE '%Fox%' OR name LIKE '%Wolf%';
UPDATE monster_definitions SET sprite_type = 'wolf' WHERE name = 'Corrupted Fox';
UPDATE monster_definitions SET sprite_type = 'wolf' WHERE name = 'Wolf';
UPDATE monster_definitions SET sprite_type = 'skeleton' WHERE name = 'Skeleton';
UPDATE monster_definitions SET sprite_type = 'goblin' WHERE name = 'Goblin';
UPDATE monster_definitions SET sprite_type = 'ghost' WHERE name = 'Ghost';
UPDATE monster_definitions SET sprite_type = 'skeleton' WHERE name = 'Dark Knight';
UPDATE monster_definitions SET sprite_type = 'ghost' WHERE name = 'Lich';
UPDATE monster_definitions SET sprite_type = 'dragon' WHERE name = 'Red Dragon';

-- Default to slime for any NULL values
UPDATE monster_definitions SET sprite_type = 'slime' WHERE sprite_type IS NULL;

-- Make sprite_type NOT NULL after setting defaults
ALTER TABLE monster_definitions ALTER COLUMN sprite_type SET NOT NULL;
ALTER TABLE monster_definitions ALTER COLUMN sprite_type SET DEFAULT 'slime';

-- Add sprite_size column (small, medium, large, boss)
ALTER TABLE monster_definitions ADD COLUMN IF NOT EXISTS sprite_size VARCHAR(20) DEFAULT 'small';

UPDATE monster_definitions SET sprite_size = 'small' WHERE level <= 10;
UPDATE monster_definitions SET sprite_size = 'medium' WHERE level > 10 AND level <= 50;
UPDATE monster_definitions SET sprite_size = 'large' WHERE level > 50 AND level < 99;
UPDATE monster_definitions SET sprite_size = 'boss' WHERE level >= 99;
