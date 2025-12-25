# 어둠의전설 M (Legend of Darkness M)

넥슨 어둠의전설 스타일의 **아이소메트릭 쿼터뷰 픽셀 RPG**

**Rust + Bevy 0.15 + Axum 0.8 + PostgreSQL 18**

---

## 🎮 게임 특징

### 아이소메트릭 쿼터뷰 (Isometric Quarter View)

어둠의전설의 가장 큰 특징은 **45° 사선 카메라**입니다.

```
카메라 시점 (Isometric 2:1)
     ↘
      ╲
       ╲  캐릭터를 45° 각도로 바라봄
        ╲
         ◆──────◆
        ╱        ╲
       ╱  마름모   ╲
      ╱    타일     ╲
     ◆──────────────◆
```

### 핵심 시스템

- **Paper Doll 시스템**: 장비에 따른 실시간 외형 변화
- **5서클 성장 시스템**: 레벨 1-99, 5단계 지역/컨텐츠
- **5대 클래스**: 전사, 도적, 마법사, 성직자, 무도가
- **다크 판타지 세계관**: 중세 DND 스타일

---

## 🚀 빠른 시작

```bash
# 환경 변수 설정
cp .env.example .env

# 서비스 실행
docker compose up -d
```

| 서비스 | URL |
|--------|-----|
| 🎮 웹 게임 | http://localhost:8080 |
| 🔌 API | http://localhost:3000 |
| 🗄️ DB 관리 | http://localhost:8081 |

---

## 📊 5서클 컨텐츠

| 서클 | 레벨 | 지역 | 보스 |
|------|------|------|------|
| **1서클** | 1-20 | 밀레스 마을, 평원, 늑대숲 | 늑대 대장 |
| **2서클** | 21-40 | 사라크 오아시스, 사막, 피라미드 | 스콜피온 킹 |
| **3서클** | 41-60 | 프로스트 헤이븐, 빙산, 얼음동굴 | 아이스 골렘 |
| **4서클** | 61-80 | 엠버 전초기지, 화산, 악마소굴 | 염마 |
| **5서클** | 81-99 | 그림자 성역, 어둠의 성 | **어둠의 군주** |

### 클래스별 스킬 (서클당 2-3개)

| 클래스 | 1서클 | 2서클 | 3서클 | 4서클 | 5서클 |
|--------|-------|-------|-------|-------|-------|
| 전사 | Bash, Crash, Iron Will | Whirlwind, Battle Cry | Ground Slam, Shield Wall | Berserk, Earthquake | Titan Strike, Immortal |
| 도적 | Double Stab, Ambush, Evasion | Poison Blade, Shadow Step | Fan of Knives, Vanish | Deadly Poison, Shadow Dance | Assassinate, Death Mark |
| 마법사 | Fireball, Thunder Bolt, Ice Shield | Flame Wave, Teleport | Blizzard, Mana Shield | Meteor, Time Stop | Armageddon, Arcane Mastery |
| 성직자 | Heal, Holy Bolt, Great Heal | Blessing, Sanctuary | Mass Heal, Holy Armor | Resurrection, Divine Judgment | Guardian Angel, Divine Intervention |
| 무도가 | Punch, Power Kick, Inner Peace | Dragon Fist, Iron Body | Tiger Palm, Flying Kick | Pressure Point, Chi Burst | Hundred Fists, Enlightenment |

---

## 📁 프로젝트 구조

```
legend/
├── src/
│   ├── game_main.rs          # Bevy 게임 클라이언트
│   ├── server_main.rs        # Axum API 서버
│   ├── client/               # 게임 클라이언트
│   │   ├── game.rs           # 월드, 전투, AI
│   │   ├── animation.rs      # 스프라이트 애니메이션
│   │   ├── equipment.rs      # Paper Doll 시스템
│   │   └── ui.rs             # 메뉴, HUD
│   ├── server/               # REST API
│   │   ├── auth.rs           # 인증
│   │   ├── monsters.rs       # 몬스터 API
│   │   └── skills.rs         # 스킬 API
│   └── shared/               # 공용 모듈
│       ├── constants.rs      # 게임 상수 (타일, 스프라이트 크기)
│       ├── domain/           # 도메인 모델
│       └── data/             # 정적 데이터 (5서클 컨텐츠)
│           ├── skills.rs     # 55개 스킬 (클래스당 11개)
│           ├── monsters.rs   # 25개 몬스터 (서클당 5개)
│           ├── maps.rs       # 15개 맵 (서클당 3개)
│           ├── items.rs      # 장비/소모품
│           └── characters.rs # 클래스, 경험치 테이블
│
├── public/assets/            # 게임 에셋
│   ├── characters/base/      # Paper Doll 기본 캐릭터
│   ├── equipment/            # Paper Doll 장비 레이어
│   ├── monsters/             # 몬스터 스프라이트
│   ├── skills/               # 스킬 아이콘 (64×64)
│   ├── items/                # 아이템 아이콘 (64×64)
│   └── fonts/                # Cinzel (타이틀), NanumGothic (본문)
│
└── ASSETS.md                 # 에셋 및 기술 사양
```

---

## 🎨 에셋 규격 요약

> 📖 상세 내용: **[ASSETS.md](./ASSETS.md)**

| 항목 | 규격 |
|------|------|
| **파일 포맷** | PNG (투명 배경, RGBA) |
| **투영 방식** | Isometric 2:1 (45° 쿼터뷰) |
| **타일** | 64×32 (마름모) |
| **캐릭터/장비** | 256×256 시트, 64×64 프레임, 4×4 그리드 |
| **몬스터** | Small 32², Medium 48², Large 64², Boss 128² |
| **아이콘** | 64×64 |

---

## 🛠 기술 스택

| 분류 | 기술 | 버전 |
|------|------|------|
| 게임 엔진 | Bevy | 0.15 |
| 백엔드 | Axum | 0.8 |
| DBMS | PostgreSQL | 18 |
| ORM | SQLx | 0.8 |
| 언어 | Rust | 1.85+ |

---

## 🎯 조작법

| 키 | 동작 |
|----|------|
| WASD / 화살표 | 이동 |
| Space | 공격 |
| 1-5 | 스킬 사용 |
| E / Enter | 상호작용 |
| C | 캐릭터 창 |
| I | 인벤토리 |
| K | 스킬 창 |

---

## 📋 개발 명령어

```bash
# 웹 개발 로그
docker compose logs -f web

# 네이티브 빌드
docker compose run --rm game cargo build --bin legend-game --features client

# DB 마이그레이션
docker compose run --rm api sqlx migrate run

# 정적 빌드 (배포용)
docker compose run --rm game trunk build --release
```

---

## 📈 개발 현황

### 완료
- [x] Bevy 0.15 마이그레이션
- [x] 아이소메트릭 렌더링 시스템
- [x] 기본 이동/전투
- [x] 5서클 스킬 체계 (55개)
- [x] 5서클 몬스터 체계 (25개)
- [x] 5서클 맵 체계 (15개)
- [x] Paper Doll 시스템 기반
- [x] 경험치 테이블 (Lv 1-99)

### 진행 중
- [ ] 스프라이트 애니메이션 완성
- [ ] Paper Doll 렌더링
- [ ] 인벤토리 시스템
- [ ] 스킬 UI
- [ ] 멀티플레이어 (WebSocket)

---

**Made with 🦀 Rust + 🎮 Bevy + 🚀 Axum + 🐘 PostgreSQL**
