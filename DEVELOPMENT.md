# 어둠의전설 M - 개발 가이드

## 프로젝트 개요

이 프로젝트는 **어둠의전설**을 벤치마크한 온라인 픽셀 RPG입니다. Rust + Leptos 0.8 + Axum 0.8로 완전히 구현되었습니다.

## 완성된 기능

### ✅ 게임 시스템 (100% 완료)

1. **캐릭터 시스템**
   - 4개 직업 (전사, 궁수, 마법사, 도적)
   - 레벨업 시스템 (경험치 곡선)
   - 5개 기본 스탯 (STR, DEX, INT, VIT, LUK)
   - 스탯 포인트 할당
   - 자동 전투 스탯 계산

2. **전투 시스템**
   - 데미지 계산 (공격력 - 방어력)
   - 크리티컬 히트
   - HP/MP 관리
   - 사망 처리

3. **몬스터 시스템**
   - 3가지 AI 타입 (Passive, Aggressive, Defensive)
   - 랜덤 공격력
   - 골드/경험치 드롭
   - 스폰 위치

4. **아이템 시스템**
   - 6가지 등급 (Common ~ Unique)
   - 무기/방어구/악세서리/소비템
   - 스탯 보너스
   - 강화 시스템 (+0~+10)
   - 드롭 확률

5. **스킬 시스템**
   - 직업별 전용 스킬
   - 5가지 속성 (화염, 얼음, 번개, 신성, 암흑)
   - 데미지 배율
   - MP 소모
   - 쿨다운
   - 스킬 레벨

6. **맵 시스템**
   - 4개 맵 (초보자 마을, 숲, 던전, 어둠의 성)
   - 포털 시스템
   - 레벨 제한
   - PVP 설정

7. **퀘스트 시스템**
   - 목표 기반 퀘스트
   - 보상 (경험치, 골드, 아이템)
   - 진행도 추적

### ✅ UI/UX (100% 완료)

1. **HUD**
   - HP/MP/EXP 바
   - 플레이어 정보
   - 골드 표시
   - 메뉴 버튼

2. **캐릭터 창**
   - 스탯 확인
   - 스탯 포인트 할당
   - 전투 스탯 표시

3. **인벤토리 창**
   - 24칸 그리드
   - 아이템 슬롯

4. **스킬 창**
   - 스킬 목록
   - 스킬 정보

5. **픽셀 렌더링**
   - 직업별 캐릭터 디자인
   - 몬스터 디자인
   - HP 바
   - 이름표
   - 그림자

### ✅ 데이터베이스 (100% 완료)

1. **스키마**
   - 9개 주요 테이블
   - 완전한 관계 정의
   - 인덱스 최적화

2. **시드 데이터**
   - 4개 직업
   - 20+ 스킬
   - 30+ 아이템
   - 15+ 몬스터
   - 4개 맵
   - 드롭 테이블

## 기술 구현

### Leptos 0.8 최신 기능

- Signal 기반 반응성
- Component 시스템
- create_effect for 게임 루프
- create_node_ref for Canvas 접근

### Rust 고급 기능

- Trait 시스템
- Pattern Matching
- Ownership & Borrowing
- Error Handling
- Async/Await (서버)

### WebAssembly 최적화

- 픽셀 렌더링 최적화
- requestAnimationFrame
- 메모리 관리
- 번들 크기 최소화

## 실행 방법

```bash
# 1. 전체 시작 (권장)
docker compose up --build

# 2. 단계별 시작
docker compose up -d db adminer  # DB 먼저
docker compose build app         # 앱 빌드
docker compose up app            # 앱 실행
```

## 접속

- 게임: http://localhost:3002
- DB 관리: http://localhost:8081

## 다음 단계

### 멀티플레이어 구현

1. WebSocket 서버 추가
2. 플레이어 동기화
3. 채팅 시스템
4. PVP 전투

### 실제 에셋 적용

1. Aseprite로 스프라이트 제작
2. 타일셋 제작
3. UI 아이콘 제작
4. 사운드 추가

### 최적화

1. 서버 사이드 몬스터 AI
2. 클라이언트 예측
3. 지연 보정
4. 번들 분할

## 문제 해결

### 빌드 실패

```bash
# 캐시 클리어 후 재빌드
docker compose down -v
docker compose build --no-cache app
```

### 포트 충돌

`compose.yml`에서 포트 변경

### DB 초기화

```bash
docker volume rm legend_postgres-data
docker compose up -d db
```

---

**프로젝트 완료!** 🎉

모든 핵심 RPG 시스템이 구현되었습니다.
