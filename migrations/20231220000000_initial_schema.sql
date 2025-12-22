-- 직업 (Class) 테이블
CREATE TABLE IF NOT EXISTS classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    base_str INTEGER DEFAULT 10,
    base_dex INTEGER DEFAULT 10,
    base_int INTEGER DEFAULT 10,
    base_vit INTEGER DEFAULT 10,
    base_luk INTEGER DEFAULT 10,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 플레이어 테이블 (확장)
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    
    -- 기본 정보
    class_id INTEGER REFERENCES classes(id),
    level INTEGER DEFAULT 1,
    exp BIGINT DEFAULT 0,
    
    -- 스탯
    strength INTEGER DEFAULT 10,
    dexterity INTEGER DEFAULT 10,
    intelligence INTEGER DEFAULT 10,
    vitality INTEGER DEFAULT 10,
    luck INTEGER DEFAULT 10,
    stat_points INTEGER DEFAULT 0,
    
    -- 전투 스탯 (계산됨)
    hp INTEGER DEFAULT 100,
    max_hp INTEGER DEFAULT 100,
    mp INTEGER DEFAULT 50,
    max_mp INTEGER DEFAULT 50,
    attack_min INTEGER DEFAULT 5,
    attack_max INTEGER DEFAULT 10,
    defense INTEGER DEFAULT 5,
    magic_attack INTEGER DEFAULT 5,
    magic_defense INTEGER DEFAULT 5,
    hit_rate INTEGER DEFAULT 80,
    avoid_rate INTEGER DEFAULT 10,
    critical_rate INTEGER DEFAULT 5,
    
    -- 위치
    current_map VARCHAR(100) DEFAULT 'starting_zone',
    x DOUBLE PRECISION DEFAULT 400.0,
    y DOUBLE PRECISION DEFAULT 300.0,
    
    -- 재화
    gold BIGINT DEFAULT 100,
    
    -- 상태
    is_online BOOLEAN DEFAULT FALSE,
    last_login TIMESTAMP WITH TIME ZONE,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 스킬 테이블
CREATE TABLE IF NOT EXISTS skills (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    class_id INTEGER REFERENCES classes(id),
    icon_path VARCHAR(255),
    
    -- 스킬 타입
    skill_type VARCHAR(50) NOT NULL, -- active, passive, buff
    element VARCHAR(50), -- fire, ice, lightning, holy, dark, none
    
    -- 요구사항
    required_level INTEGER DEFAULT 1,
    required_skill_id INTEGER REFERENCES skills(id),
    required_skill_level INTEGER DEFAULT 1,
    
    -- 스킬 효과
    damage_multiplier DECIMAL(5,2) DEFAULT 1.0,
    mp_cost INTEGER DEFAULT 10,
    cooldown_ms INTEGER DEFAULT 1000,
    cast_time_ms INTEGER DEFAULT 500,
    range INTEGER DEFAULT 100,
    area_of_effect INTEGER DEFAULT 0,
    
    -- 최대 레벨
    max_level INTEGER DEFAULT 10,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 플레이어 스킬 테이블
CREATE TABLE IF NOT EXISTS player_skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    skill_id INTEGER NOT NULL REFERENCES skills(id),
    skill_level INTEGER DEFAULT 1,
    current_exp INTEGER DEFAULT 0,
    
    -- 퀵슬롯
    hotkey_slot INTEGER,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(player_id, skill_id)
);

-- 아이템 테이블 (확장)
CREATE TABLE IF NOT EXISTS items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    icon_path VARCHAR(255),
    
    -- 아이템 타입
    item_type VARCHAR(50) NOT NULL, -- weapon, armor, accessory, consumable, material, quest
    sub_type VARCHAR(50), -- sword, bow, staff, helmet, armor, boots, ring, potion, etc
    
    -- 등급
    rarity VARCHAR(20) NOT NULL, -- common, uncommon, rare, epic, legendary, unique
    
    -- 요구사항
    level_requirement INTEGER DEFAULT 1,
    class_requirement INTEGER REFERENCES classes(id),
    str_requirement INTEGER DEFAULT 0,
    dex_requirement INTEGER DEFAULT 0,
    int_requirement INTEGER DEFAULT 0,
    
    -- 스탯 보너스 (JSON)
    stat_bonuses JSONB DEFAULT '{}',
    
    -- 가격
    buy_price INTEGER DEFAULT 0,
    sell_price INTEGER DEFAULT 0,
    
    -- 스택
    max_stack INTEGER DEFAULT 1,
    
    -- 강화 가능 여부
    can_be_enhanced BOOLEAN DEFAULT FALSE,
    max_enhancement INTEGER DEFAULT 10,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 플레이어 인벤토리 테이블
CREATE TABLE IF NOT EXISTS player_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL REFERENCES items(id),
    
    -- 수량 및 상태
    quantity INTEGER DEFAULT 1,
    enhancement_level INTEGER DEFAULT 0,
    
    -- 장착 여부
    equipped BOOLEAN DEFAULT FALSE,
    equipment_slot VARCHAR(50), -- weapon, helmet, armor, boots, gloves, ring1, ring2, necklace
    
    -- 인벤토리 위치
    inventory_slot INTEGER,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 몬스터 테이블 (확장)
CREATE TABLE IF NOT EXISTS monsters (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    sprite_path VARCHAR(255),
    
    -- 레벨 및 스탯
    level INTEGER DEFAULT 1,
    hp INTEGER NOT NULL,
    attack_min INTEGER DEFAULT 5,
    attack_max INTEGER DEFAULT 10,
    defense INTEGER DEFAULT 5,
    magic_defense INTEGER DEFAULT 5,
    
    -- 보상
    exp_reward INTEGER DEFAULT 10,
    gold_min INTEGER DEFAULT 5,
    gold_max INTEGER DEFAULT 15,
    
    -- AI
    ai_type VARCHAR(50) DEFAULT 'passive', -- passive, aggressive, defensive
    detection_range INTEGER DEFAULT 200,
    attack_range INTEGER DEFAULT 50,
    move_speed INTEGER DEFAULT 100,
    
    -- 스폰 정보
    spawn_map VARCHAR(100),
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 몬스터 드롭 테이블
CREATE TABLE IF NOT EXISTS monster_drops (
    id SERIAL PRIMARY KEY,
    monster_id INTEGER NOT NULL REFERENCES monsters(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL REFERENCES items(id),
    drop_rate DECIMAL(5,2) NOT NULL, -- 0.01 ~ 100.00 (%)
    min_quantity INTEGER DEFAULT 1,
    max_quantity INTEGER DEFAULT 1,
    UNIQUE(monster_id, item_id)
);

-- 맵 테이블 (확장)
CREATE TABLE IF NOT EXISTS maps (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    background_music VARCHAR(255),
    
    -- 맵 크기
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    tile_size INTEGER DEFAULT 32,
    
    -- 맵 데이터
    tile_data JSONB NOT NULL,
    
    -- 레벨 제한
    min_level INTEGER DEFAULT 1,
    max_level INTEGER DEFAULT 999,
    
    -- PVP 설정
    pvp_enabled BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 맵 포털 테이블
CREATE TABLE IF NOT EXISTS map_portals (
    id SERIAL PRIMARY KEY,
    from_map_id INTEGER NOT NULL REFERENCES maps(id),
    to_map_id INTEGER NOT NULL REFERENCES maps(id),
    from_x INTEGER NOT NULL,
    from_y INTEGER NOT NULL,
    to_x INTEGER NOT NULL,
    to_y INTEGER NOT NULL,
    portal_type VARCHAR(50) DEFAULT 'normal' -- normal, hidden, quest
);

-- 퀘스트 테이블
CREATE TABLE IF NOT EXISTS quests (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    
    -- 요구사항
    required_level INTEGER DEFAULT 1,
    required_quest_id INTEGER REFERENCES quests(id),
    
    -- 목표 (JSON)
    objectives JSONB NOT NULL,
    
    -- 보상
    exp_reward INTEGER DEFAULT 0,
    gold_reward INTEGER DEFAULT 0,
    item_rewards JSONB DEFAULT '[]',
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 플레이어 퀘스트 테이블
CREATE TABLE IF NOT EXISTS player_quests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    quest_id INTEGER NOT NULL REFERENCES quests(id),
    
    status VARCHAR(50) DEFAULT 'in_progress', -- in_progress, completed, failed
    progress JSONB DEFAULT '{}',
    
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    
    UNIQUE(player_id, quest_id)
);

-- 인덱스 생성
CREATE INDEX idx_players_username ON players(username);
CREATE INDEX idx_players_level ON players(level);
CREATE INDEX idx_players_class ON players(class_id);
CREATE INDEX idx_players_map ON players(current_map);
CREATE INDEX idx_players_online ON players(is_online);

CREATE INDEX idx_player_inventory_player ON player_inventory(player_id);
CREATE INDEX idx_player_inventory_equipped ON player_inventory(player_id, equipped);

CREATE INDEX idx_player_skills_player ON player_skills(player_id);

CREATE INDEX idx_items_type ON items(item_type);
CREATE INDEX idx_items_rarity ON items(rarity);

CREATE INDEX idx_monsters_spawn_map ON monsters(spawn_map);

CREATE INDEX idx_monster_drops_monster ON monster_drops(monster_id);

CREATE INDEX idx_player_quests_player ON player_quests(player_id);
CREATE INDEX idx_player_quests_status ON player_quests(player_id, status);
