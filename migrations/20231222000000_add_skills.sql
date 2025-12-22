-- Create skills table
CREATE TABLE IF NOT EXISTS skills (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    icon_path VARCHAR(255),
    class_req VARCHAR(50), -- 'warrior', 'mage', etc. or NULL for all
    level_req INTEGER DEFAULT 1,
    mp_cost INTEGER DEFAULT 0,
    cooldown INTEGER DEFAULT 0, -- in seconds
    damage_min INTEGER DEFAULT 0,
    damage_max INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Seed initial skills
INSERT INTO skills (name, description, icon_path, class_req, level_req, mp_cost, cooldown, damage_min, damage_max) VALUES
('파워 스트라이크', '강력한 베기 공격', '/assets/skills/power_strike.png', '전사', 1, 10, 5, 10, 20),
('슬래시 블러스트', '주변 적들을 동시에 공격', '/assets/skills/slash_blast.png', '전사', 5, 20, 8, 15, 25),
('아이언 바디', '방어력을 일시적으로 증가', '/assets/skills/iron_body.png', '전사', 3, 15, 60, 0, 0),

('매직 클로', '마력의 손톱으로 공격', '/assets/skills/magic_claw.png', '마법사', 1, 10, 0, 15, 30),
('에너지 볼트', '마법 구체를 발사', '/assets/skills/energy_bolt.png', '마법사', 1, 5, 0, 10, 20),
('파이어볼', '화염 구를 발사', '/assets/skills/fireball.png', '마법사', 5, 25, 3, 30, 50),
('힐', '자신과 주변 아군의 HP 회복', '/assets/skills/heal.png', '마법사', 8, 40, 10, 0, 0),

('더블 샷', '화살을 두 번 연속 발사', '/assets/skills/double_shot.png', '궁수', 1, 10, 0, 10, 15),
('애로우 블로우', '강력한 화살 한 발', '/assets/skills/arrow_blow.png', '궁수', 3, 15, 2, 20, 35),

('럭키 세븐', '행운을 담은 두 번의 공격', '/assets/skills/lucky_seven.png', '도적', 1, 10, 0, 10, 15),
('헤이스트', '이동 속도와 점프력 증가', '/assets/skills/haste.png', '도적', 5, 20, 60, 0, 0);

-- Character skills (acquired skills)
CREATE TABLE IF NOT EXISTS character_skills (
    character_id INTEGER REFERENCES characters(id) ON DELETE CASCADE,
    skill_id INTEGER REFERENCES skills(id) ON DELETE CASCADE,
    level INTEGER DEFAULT 1,
    assigned_slot INTEGER, -- 0-9 hotkeys, NULL if not assigned
    PRIMARY KEY (character_id, skill_id)
);
