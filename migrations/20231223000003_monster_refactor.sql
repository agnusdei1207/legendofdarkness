-- Add monster metadata and remove hardcoding
ALTER TABLE monster_definitions 
ADD COLUMN detection_range FLOAT DEFAULT 150.0,
ADD COLUMN attack_range FLOAT DEFAULT 40.0,
ADD COLUMN move_speed FLOAT DEFAULT 80.0,
ADD COLUMN sprite_type VARCHAR(50) DEFAULT 'slime',
ADD COLUMN sprite_size VARCHAR(20) DEFAULT 'Small';

-- Monster loot table (Items drop from monsters)
CREATE TABLE monster_drops (
    id SERIAL PRIMARY KEY,
    monster_id INT REFERENCES monster_definitions(id) ON DELETE CASCADE,
    item_id INT REFERENCES item_definitions(id) ON DELETE CASCADE,
    probability FLOAT NOT NULL, -- 0.0 to 1.0
    min_quantity INT DEFAULT 1,
    max_quantity INT DEFAULT 1
);

-- Update existing data with more diverse values
UPDATE monster_definitions SET 
    detection_range = 250.0, attack_range = 55.0, move_speed = 110.0, sprite_type = 'rat' 
WHERE name = 'Giant Rat';

UPDATE monster_definitions SET 
    detection_range = 250.0, attack_range = 55.0, move_speed = 120.0, sprite_type = 'bat' 
WHERE name = 'Vampire Bat';

UPDATE monster_definitions SET 
    detection_range = 150.0, attack_range = 40.0, move_speed = 60.0, sprite_type = 'slime' 
WHERE name = 'Slime';

-- Seed some drops
INSERT INTO monster_drops (monster_id, item_id, probability) VALUES
(1, 1, 0.1), -- Rat drops Red Potion (10%)
(2, 1, 0.15), -- Bat drops Red Potion (15%)
(3, 1, 0.2),  -- Slime drops Red Potion (20%)
(3, 2, 0.05); -- Slime drops Blue Potion (5%)
