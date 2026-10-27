# Classic 시스템 로그 스타일 (`classic`)

`imposter`의 모태가 된 원작 프로젝트 [`giacomo-b/rust-stakeholder`](https://github.com/giacomo-b/rust-stakeholder)의 고유한 시스템 엔지니어링 화면을 유지·발전시킨 레거시 모드입니다.

---

## 1. 주요 레이아웃 특징

### 프로그레스 바 부팅 화면
AI 에이전트와 대화하는 형태가 아니라, 대형 분산 시스템의 코어 엔진이 기동하는 듯한 프로그레스 바 애니메이션이 흐릅니다:
```text
 ▸ imposter · classic system log
 ████████░░░  73%  analyzing workspace
```

### 정밀한 시스템 엔지니어링 메트릭 로그
초당 수만 건의 요청을 처리하는 고성능 백엔드 서버의 내부 로그가 쉴 새 없이 기록됩니다:
- 코드 파일 분석: `✓ inspected src/worker.rs (194 lines)`
- 레이턴시 메트릭: `✓ p99 latency  38 ms · throughput 14.2k req/s`
- 메모리 및 네트워크 상태: `✓ GC sweep reclaimed 412 MB in 1.4 ms`

### 기술 전문 용어 (Jargon) 생성기
Jargon 레벨(`low`, `medium`, `high`, `extreme`)에 따라 엔터프라이즈 아키텍처 수준의 심오한 시스템 엔지니어링 용어가 무작위 셔플 덱을 통해 자연스럽게 이어집니다.

---

## 2. 실행 명령어

```bash
# Classic 스타일 실행
imposter --style classic

# 단축 플래그 사용
imposter --classic

# 극단적인 전문 용어 수준으로 실행
imposter --classic -j extreme
```
