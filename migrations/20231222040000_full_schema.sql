-- Users (Login)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Classes (Static)
-- Already exists, but ensuring columns
-- id, name, etc.

-- Characters (Dynamic)
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    name VARCHAR(50) UNIQUE NOT NULL,
    class_id INT REFERENCES classes(id), -- Assuming classes has numerical ID or create enum mapping
    gender VARCHAR(10) NOT NULL, -- 'male', 'female'
    level INT DEFAULT 1,
    exp BIGINT DEFAULT 0,
    hp INT DEFAULT 100,
    mp INT DEFAULT 50,
    map_id VARCHAR(50) NOT NULL DEFAULT 'village',
    x INT DEFAULT 400,
    y INT DEFAULT 300,
    gold BIGINT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE
);

-- Character Stats (Dynamic - Normalized)
CREATE TABLE character_stats (
    character_id UUID PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    str INT DEFAULT 0,
    dex INT DEFAULT 0,
    con INT DEFAULT 0,
    int_stat INT DEFAULT 0, -- 'int' is reserved
    wis INT DEFAULT 0,
    stat_points_available INT DEFAULT 0
);

-- Item Definitions (Static - Metadata)
CREATE TABLE item_definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    item_type VARCHAR(20) NOT NULL, -- 'weapon', 'armor', 'consumable', 'etc'
    sub_type VARCHAR(20), -- 'sword', 'staff', 'helmet', 'potion'
    grade INT DEFAULT 1, -- 1 to 12
    req_level INT DEFAULT 1,
    req_class INT REFERENCES classes(id), -- NULL means all classes
    req_str INT DEFAULT 0,
    req_dex INT DEFAULT 0,
    req_con INT DEFAULT 0,
    req_int INT DEFAULT 0,
    req_wis INT DEFAULT 0,
    stats JSONB DEFAULT '{}', -- bonus stats { 'attack': 10, 'def': 5 }
    icon_path VARCHAR(255),
    price_buy BIGINT,
    price_sell BIGINT
);

-- Character Inventory (Dynamic)
CREATE TABLE character_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    item_def_id INT REFERENCES item_definitions(id),
    quantity INT DEFAULT 1,
    slot_index INT NOT NULL, -- Grid position
    is_equipped BOOLEAN DEFAULT FALSE,
    equipped_slot VARCHAR(20), -- 'head', 'armor', 'shoes', 'gloves', 'ring1', 'ring2', 'necklace', 'weapon', 'shield'
    acquired_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(character_id, slot_index) -- One item per slot
);

-- Monster Definitions (Static)
CREATE TABLE monster_definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    level INT NOT NULL,
    hp_max INT NOT NULL,
    mp_max INT NOT NULL,
    attack_min INT NOT NULL,
    attack_max INT NOT NULL,
    defense INT NOT NULL,
    exp_reward BIGINT NOT NULL,
    gold_min INT NOT NULL,
    gold_max INT NOT NULL,
    sprite_path VARCHAR(255),
    ai_type VARCHAR(20) DEFAULT 'passive'
);

-- Monster Drops (Static)
CREATE TABLE monster_drops (
    monster_id INT REFERENCES monster_definitions(id),
    item_def_id INT REFERENCES item_definitions(id),
    rate FLOAT NOT NULL, -- 0.0 to 1.0
    amount_min INT DEFAULT 1,
    amount_max INT DEFAULT 1,
    PRIMARY KEY (monster_id, item_def_id)
);

-- Initial Data Seeding (12 Grades of Weapons for 5 Classes)
-- Simplification for the migration file length, adding a few representative items
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, stats) VALUES
-- Warrior Swords
('나무검', 'weapon', 'sword', 1, 1, '{"attack": 5}'),
('철검', 'weapon', 'sword', 2, 1, '{"attack": 10}'),
('강철검', 'weapon', 'sword', 3, 1, '{"attack": 15}'),
-- ...
-- Mage Staffs
('나무지팡이', 'weapon', 'staff', 1, 3, '{"magic_attack": 5}'),
('고목나무지팡이', 'weapon', 'staff', 2, 3, '{"magic_attack": 10}'),
('마법사의지팡이', 'weapon', 'staff', 3, 3, '{"magic_attack": 15}');
