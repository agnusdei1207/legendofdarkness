# 어둠의전설 M (Legend of Darkness M)

어둠의전설 스타일의 **완전한 온라인 픽셀 RPG 게임**

**Rust + Leptos 0.8 + Axum 0.8 + SQLx + PostgreSQL**

## 🎮 게임 특징

### 핵심 시스템
- ✅ **4개 직업**: 전사, 궁수, 마법사, 도적
- ✅ **레벨업 시스템**: 경험치 획득 및 레벨업
- ✅ **스탯 시스템**: STR, DEX, INT, VIT, LUK
- ✅ **스킬 시스템**: 직업별 20+ 스킬
- ✅ **전투 시스템**: 데미지 계산, 크리티컬
- ✅ **몬스터 AI**: Passive, Aggressive, Defensive
- ✅ **아이템 시스템**: 무기, 방어구, 소비 아이템
- ✅ **등급 시스템**: Common ~ Legendary
- ✅ **강화 시스템**: +0 ~ +10
- ✅ **드롭 시스템**: 몬스터별 아이템 드롭
- ✅ **맵 시스템**: 4개 맵 + 포털
- ✅ **퀘스트 시스템**: 목표 달성 퀘스트

### UI/UX
- 🎨 픽셀 아트 그래픽 (16x16, 32x32)
- 🎨 다크 판타지 테마
- 🎨 골드 강조 색상
- 🎨 반응형 디자인
- 🎨 Press Start 2P 폰트

## 🛠 기술 스택

| 분류 | 기술 | 버전 |
|------|------|------|
| **프론트엔드** | Leptos | 0.8 |
| **백엔드** | Axum | 0.8 |
| **데이터베이스** | PostgreSQL | 16 |
| **ORM** | SQLx | 0.8 |
| **언어** | Rust | 1.85+ (nightly) |
| **개발 환경** | Docker Compose | - |

## 📋 필수 요구사항

- Docker
- Docker Compose

**로컬에 아무것도 설치하지 않아도 됩니다!**

## 🚀 빠른 시작

### 1. 환경 변수 설정

```bash
cp .env.example .env
```

### 2. 전체 스택 시작

```bash
# 전체 서비스 시작 (DB + 게임 서버)
docker compose up --build
```

### 3. 게임 접속

- **게임**: http://localhost:3002
- **DB 관리**: http://localhost:8081

### 서비스별 실행

```bash
# DB만 시작
docker compose up -d db adminer

# 개발 서버 빌드
docker compose build app

# 개발 서버 실행
docker compose up app
```

## 🎯 게임 조작법

### 이동
- `W` 또는 `↑` - 위로 이동
- `S` 또는 `↓` - 아래로 이동
- `A` 또는 `←` - 왼쪽으로 이동
- `D` 또는 `→` - 오른쪽으로 이동

### UI 토글
- `C` - 캐릭터 창 (스탯 할당)
- `I` - 인벤토리 창
- `K` - 스킬 창

## 📁 프로젝트 구조

```
legend/
├── src/
│   ├── main.rs                 # 서버 엔트리  (SSR)
│   ├── lib.rs                  # 라이브러리 엔트리 (WASM)
│   ├── app.rs                  # Leptos 앱 루트
│   │
│   ├── models/                 # 게임 데이터 모델
│   │   ├── mod.rs              # 공통 타입 (Position, Stats)
│   │   ├── player.rs           # 플레이어 모델
│   │   ├── monster.rs          # 몬스터 모델
│   │   ├── item.rs             # 아이템 모델
│   │   ├── skill.rs            # 스킬 모델
│   │   └── map.rs              # 맵 모델
│   │
│   ├── game/                   # 게임 로직
│   │   ├── mod.rs              # 게임 메인 뷰
│   │   ├── canvas.rs           # 캔버스 렌더링
│   │   └── systems/            # 게임 시스템
│   │       ├── combat.rs       # 전투 시스템
│   │       └── movement.rs     # 이동 시스템
│   │
│   └── components/             # UI 컴포넌트
│       ├── mod.rs
│       ├── hud.rs              # HUD (HP/MP/EXP 바)
│       ├── character_window.rs # 캐릭터 창
│       ├── inventory_window.rs # 인벤토리 창
│       └── skill_window.rs     # 스킬 창
│
├── migrations/                 # 데이터베이스 마이그레이션
│   ├── *_initial_schema.sql   # 스키마 정의
│   └── *_seed_data.sql         # 시드 데이터
│
├── style/
│   └── main.css                # 게임 스타일
│
├── public/
│   ├── index.html              # HTML 템플릿
│   ├── manifest.json           # PWA 매니페스트
│   └── assets/                 # 게임 에셋
│       ├── items/              # 아이템 스프라이트
│       ├── monsters/           # 몬스터 스프라이트
│       ├── characters/         # 캐릭터 스프라이트
│       └── tiles/              # 타일셋
│
├── Cargo.toml                  # Rust 의존성
├── compose.yml                 # Docker Compose 설정
├── Dockerfile.dev              # 개발용 Dockerfile
└── README.md
```

## 🗄️ 데이터베이스 스키마

### 주요 테이블

#### `classes` - 직업
- 전사, 궁수, 마법사, 도적
- 기본 스탯 정의

#### `players` - 플레이어
- 레벨, 경험치, 스탯
- HP/MP, 공격력, 방어력
- 위치, 골드

#### `skills` - 스킬
- 직업별 스킬
- 데미지, MP 소모, 쿨다운
- 속성 (무, 화염, 얼음, 번개, 신성, 암흑)

#### `items` - 아이템
- 무기, 방어구, 악세서리, 소비 아이템
- 등급 (Common ~ Legendary)
- 스탯 보너스, 강화 가능 여부

#### `monsters` - 몬스터
- 레벨, HP, 공격력
- AI 타입 (Passive, Aggressive, Defensive)
- 경험치, 골드 보상

#### `monster_drops` - 몬스터 드롭
- 아이템 드롭 확률
- 수량 범위

#### `maps` - 맵
- 크기, 타일 데이터
- 레벨 제한, PVP 설정

#### `quests` - 퀘스트
- 목표, 보상
- 요구 레벨

### DB 접속 정보 (Adminer)

```
시스템: PostgreSQL
서버: db
사용자: legend
암호: legend
데이터베이스: legend
```

## 🎨 직업별 특징

### 전사 (Warrior)
- **색상**: 빨강 (#ff4444)
- **무기**: 검
- **특징**: 높은 HP, 강력한 물리 공격
- **기본 스탯**: STR 20, VIT 18

### 궁수 (Archer)
- **색상**: 초록 (#44ff44)
- **무기**: 활
- **특징**: 원거리 공격, 높은 회피율
- **기본 스탯**: DEX 20, LUK 15

### 마법사 (Mage)
- **색상**: 파랑 (#4444ff)
- **무기**: 스태프
- **특징**: 강력한 마법 공격, 높은 MP
- **기본 스탯**: INT 25

### 도적 (Rogue)
- **색상**: 보라 (#ff44ff)
- **무기**: 단검
- **특징**: 높은 크리티컬, 빠른 속도
- **기본 스탯**: DEX 18, LUK 20

## 💡 개발 팁

### 컨테이너 내부 접속

```bash
docker exec -it legend-dev bash
```

### 데이터베이스 마이그레이션

```bash
# 컨테이너 내부에서
sqlx migrate run
```

### 로그 확인

```bash
docker compose logs -f app
```

### 빌드만 (프로덕션)

```bash
cargo leptos build --release
```

## 🔧 포트 설정

| 서비스 | 포트 | 설명 |
|--------|------|------|
| 게임 서버 | 3002 | Leptos SSR |
| 핫 리로드 | 3003 | 개발 모드 자동 새로고침 |
| PostgreSQL | 5433 | 데이터베이스 |
| Adminer | 8081 | DB 관리 도구 |

## 🎯 로드맵

### 다음 업데이트

- [ ] **멀티플레이어**: WebSocket 실시간 동기화
- [ ] **채팅 시스템**: 플레이어 간 채팅
- [ ] **파티 시스템**: 파티 구성 및 공유 경험치
- [ ] **길드 시스템**: 길드 생성 및 길드전
- [ ] **PVP 시스템**: 플레이어 간 전투
- [ ] **랭킹 시스템**: 레벨, 재화 랭킹
- [ ] **거래 시스템**: 플레이어 간 아이템 거래
- [ ] **이벤트 시스템**: 시간 제한 이벤트
- [ ] **업적 시스템**: 업적 달성시 보상
- [ ] **펫 시스템**: 동반 펫

### 기술 개선

- [ ] **에셋 로딩**: 실제 픽셀 스프라이트 시트
- [ ] **사운드**: BGM 및 효과음
- [ ] **파티클**: 스킬 이펙트
- [ ] **애니메이션**: 캐릭터 이동/공격 애니메이션
- [ ] **맵 에디터**: 맵 제작 도구
- [ ] **모바일 최적화**: 터치 컨트롤

## 🤝 기여

이슈 및 PR은 언제나 환영합니다!

## 📝 라이선스

이 프로젝트는 학습 및 포트폴리오 목적으로 제작되었습니다.

---

**Made with 🦀 Rust + ⚡ Leptos 0.8 + 🚀 Axum 0.8 + 🐘 PostgreSQL**

**어둠의전설의 감성을 현대적인 웹 기술로 재현**
# legend
