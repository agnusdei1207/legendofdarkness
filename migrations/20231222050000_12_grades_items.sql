-- Add 12 grades of weapons for each class (Warrior, Rogue, Mage, Cleric, MartialArtist)
-- This is a large insert, using a loop logic or just bulk insert.
-- Writing explicit values for clarity.

-- Warrior (Sword)
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, req_str, stats, price_buy) VALUES
('목검', 'weapon', 'sword', 1, 0, 10, '{"attack": 5}', 100),
('철검', 'weapon', 'sword', 2, 0, 15, '{"attack": 12}', 500),
('강철검', 'weapon', 'sword', 3, 0, 20, '{"attack": 20}', 1500),
('미스릴검', 'weapon', 'sword', 4, 0, 30, '{"attack": 32}', 4000),
('오러검', 'weapon', 'sword', 5, 0, 45, '{"attack": 50}', 10000),
('화염검', 'weapon', 'sword', 6, 0, 60, '{"attack": 75}', 25000),
('얼음검', 'weapon', 'sword', 7, 0, 80, '{"attack": 105}', 60000),
('번개검', 'weapon', 'sword', 8, 0, 100, '{"attack": 140}', 150000),
('용사의검', 'weapon', 'sword', 9, 0, 130, '{"attack": 190}', 400000),
('영웅의검', 'weapon', 'sword', 10, 0, 160, '{"attack": 250}', 1000000),
('전설의검', 'weapon', 'sword', 11, 0, 200, '{"attack": 330}', 3000000),
('신의검', 'weapon', 'sword', 12, 0, 250, '{"attack": 450}', 10000000);

-- Rogue (Dagger) - Class ID 1
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, req_dex, stats, price_buy) VALUES
('녹슨단검', 'weapon', 'dagger', 1, 1, 10, '{"attack": 3, "critical_rate": 2}', 100),
('철단검', 'weapon', 'dagger', 2, 1, 15, '{"attack": 8, "critical_rate": 3}', 500),
('강철단검', 'weapon', 'dagger', 3, 1, 20, '{"attack": 14, "critical_rate": 5}', 1500),
('암살자의단검', 'weapon', 'dagger', 4, 1, 30, '{"attack": 22, "critical_rate": 7}', 4000),
('쉐도우대거', 'weapon', 'dagger', 5, 1, 45, '{"attack": 35, "critical_rate": 10}', 10000),
('독단검', 'weapon', 'dagger', 6, 1, 60, '{"attack": 50, "critical_rate": 12}', 25000),
('바람의단검', 'weapon', 'dagger', 7, 1, 80, '{"attack": 70, "critical_rate": 15}', 60000),
('폭풍의단검', 'weapon', 'dagger', 8, 1, 100, '{"attack": 95, "critical_rate": 18}', 150000),
('그림자검', 'weapon', 'dagger', 9, 1, 130, '{"attack": 125, "critical_rate": 22}', 400000),
('환영검', 'weapon', 'dagger', 10, 1, 160, '{"attack": 160, "critical_rate": 26}', 1000000),
('악몽의단검', 'weapon', 'dagger', 11, 1, 200, '{"attack": 210, "critical_rate": 30}', 3000000),
('사신의낫', 'weapon', 'dagger', 12, 1, 250, '{"attack": 280, "critical_rate": 35}', 10000000);

-- Mage (Staff) - Class ID 2
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, req_int, stats, price_buy) VALUES
('나무지팡이', 'weapon', 'staff', 1, 2, 10, '{"magic_attack": 5}', 100),
('고목나무지팡이', 'weapon', 'staff', 2, 2, 15, '{"magic_attack": 12}', 500),
('마법사의지팡이', 'weapon', 'staff', 3, 2, 20, '{"magic_attack": 20}', 1500),
('현자의지팡이', 'weapon', 'staff', 4, 2, 30, '{"magic_attack": 32}', 4000),
('크리스탈지팡이', 'weapon', 'staff', 5, 2, 45, '{"magic_attack": 50}', 10000),
('루비지팡이', 'weapon', 'staff', 6, 2, 60, '{"magic_attack": 75}', 25000),
('사파이어지팡이', 'weapon', 'staff', 7, 2, 80, '{"magic_attack": 105}', 60000),
('에메랄드지팡이', 'weapon', 'staff', 8, 2, 100, '{"magic_attack": 140}', 150000),
('다이아몬드지팡이', 'weapon', 'staff', 9, 2, 130, '{"magic_attack": 190}', 400000),
('대마법사의지팡이', 'weapon', 'staff', 10, 2, 160, '{"magic_attack": 250}', 1000000),
('원소의지팡이', 'weapon', 'staff', 11, 2, 200, '{"magic_attack": 330}', 3000000),
('드래곤지팡이', 'weapon', 'staff', 12, 2, 250, '{"magic_attack": 450}', 10000000);

-- Cleric (Mace/Wand) - Using Staff for now or Mace logic? Let's use Mace. Class ID 3
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, req_wis, stats, price_buy) VALUES
('나무메이스', 'weapon', 'mace', 1, 3, 10, '{"attack": 3, "magic_attack": 3}', 100),
('철메이스', 'weapon', 'mace', 2, 3, 15, '{"attack": 8, "magic_attack": 8}', 500),
('신성한메이스', 'weapon', 'mace', 3, 3, 20, '{"attack": 12, "magic_attack": 15}', 1500),
('사제의메이스', 'weapon', 'mace', 4, 3, 30, '{"attack": 20, "magic_attack": 25}', 4000),
('축복받은메이스', 'weapon', 'mace', 5, 3, 45, '{"attack": 30, "magic_attack": 40}', 10000),
('빛의메이스', 'weapon', 'mace', 6, 3, 60, '{"attack": 45, "magic_attack": 60}', 25000),
('천사의메이스', 'weapon', 'mace', 7, 3, 80, '{"attack": 60, "magic_attack": 85}', 60000),
('심판의메이스', 'weapon', 'mace', 8, 3, 100, '{"attack": 80, "magic_attack": 110}', 150000),
('성녀의메이스', 'weapon', 'mace', 9, 3, 130, '{"attack": 110, "magic_attack": 150}', 400000),
('교황의메이스', 'weapon', 'mace', 10, 3, 160, '{"attack": 140, "magic_attack": 200}', 1000000),
('신의메이스', 'weapon', 'mace', 11, 3, 200, '{"attack": 180, "magic_attack": 260}', 3000000),
('천상의메이스', 'weapon', 'mace', 12, 3, 250, '{"attack": 240, "magic_attack": 350}', 10000000);

-- Martial Artist (Knuckle/Glove) - Class ID 4
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, req_str, stats, price_buy) VALUES
('가죽장갑', 'weapon', 'knuckle', 1, 4, 10, '{"attack": 4}', 100),
('징박은장갑', 'weapon', 'knuckle', 2, 4, 15, '{"attack": 10}', 500),
('강철너클', 'weapon', 'knuckle', 3, 4, 20, '{"attack": 18}', 1500),
('가시너클', 'weapon', 'knuckle', 4, 4, 30, '{"attack": 28}', 4000),
('파괴의너클', 'weapon', 'knuckle', 5, 4, 45, '{"attack": 42}', 10000),
('화염너클', 'weapon', 'knuckle', 6, 4, 60, '{"attack": 65}', 25000),
('강철주먹', 'weapon', 'knuckle', 7, 4, 80, '{"attack": 90}', 60000),
('타이탄의장갑', 'weapon', 'knuckle', 8, 4, 100, '{"attack": 120}', 150000),
('챔피언너클', 'weapon', 'knuckle', 9, 4, 130, '{"attack": 160}', 400000),
('패왕의너클', 'weapon', 'knuckle', 10, 4, 160, '{"attack": 210}', 1000000),
('투신의너클', 'weapon', 'knuckle', 11, 4, 200, '{"attack": 280}', 3000000),
('신의주먹', 'weapon', 'knuckle', 12, 4, 250, '{"attack": 380}', 10000000);
