/-
  Domain/Message.lean — 메시지 타입 시스템
  세션 트랜스크립트, API 프로토콜, 큐 관리.
-/
import Common.Types

namespace ClaudeCode.Message

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Message Role & Type
-- ═══════════════════════════════════════════════

/-- 메시지 역할. -/
inductive MessageRole where
  | user
  | assistant
  | system
  deriving Repr, BEq

/-- 메시지 유형 (discriminated union). -/
inductive MessageType where
  | user
  | assistant
  | system
  | attachment
  | progress
  | tombstone       -- 삭제/압축된 메시지
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Message Origin
-- ═══════════════════════════════════════════════

/-- 메시지 출처. -/
inductive MessageOrigin where
  | userTyped           -- 사용자 직접 입력
  | slashCommand        -- /명령어
  | queue               -- 큐에서 자동
  | hook                -- 훅에서 생성
  | subagent            -- 서브에이전트
  | bridge              -- 리모트 브릿지
  | sdk                 -- SDK API
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Serialized Message (디스크 저장)
-- ═══════════════════════════════════════════════

/-- 직렬화된 메시지. 세션 로그에 저장. -/
structure SerializedMessage where
  uuid      : UUID
  type      : MessageType
  role      : MessageRole
  content   : List ContentBlock
  cwd       : FilePath
  sessionId : SessionId
  timestamp : Timestamp
  version   : Nat
  deriving Repr

/-- 트랜스크립트 메시지 (UI 표시용, 트리 구조). -/
structure TranscriptMessage extends SerializedMessage where
  parentUuid  : Option UUID   := none
  isSidechain : Bool          := false
  agentId     : Option AgentId := none
  deriving Repr

-- ═══════════════════════════════════════════════
-- Metadata Messages
-- ═══════════════════════════════════════════════

/-- 메타데이터 메시지 유형. -/
inductive MetadataType where
  | summary
  | customTitle
  | aiTitle
  | lastPrompt
  | taskSummary
  | tag
  | agentName
  | agentColor
  | agentSetting
  | prLink
  | fileHistorySnapshot
  | attributionSnapshot
  | speculationAccept
  | contextCollapseCommit
  | contextCollapseSnapshot
  deriving Repr, BEq

/-- PR 링크 메타데이터. -/
structure PRLinkMeta where
  prNumber     : Nat
  prUrl        : String
  prRepository : String
  deriving Repr

-- ═══════════════════════════════════════════════
-- Message Queue
-- ═══════════════════════════════════════════════

/-- 큐 우선순위. -/
inductive QueuePriority where
  | now    -- 즉시 실행
  | next   -- 현재 턴 후
  | later  -- 나중에
  deriving Repr, BEq, Ord

/-- 큐 모드. -/
inductive QueueMode where
  | bash                 -- 쉘 명령
  | prompt               -- 프롬프트
  | orphanedPermission   -- 고아 권한
  | taskNotification     -- 태스크 알림
  deriving Repr, BEq

/-- 큐에 넣은 명령. -/
structure QueuedCommand where
  value    : String
  mode     : QueueMode
  priority : QueuePriority
  origin   : MessageOrigin
  agentId  : AgentId
  deriving Repr

-- ═══════════════════════════════════════════════
-- Message Normalization (내부 ↔ API 변환)
-- ═══════════════════════════════════════════════

/--
  메시지 정규화 흐름:
  Internal Message → normalizeMessagesForAPI → ContentBlockParam[]
  ContentBlockParam[] → createUserMessage → Internal Message
  Internal Message → recordTranscript → 디스크 저장
-/
inductive NormalizationStep where
  | toApi           -- 내부 → API 형식
  | fromApi         -- API → 내부 형식
  | toTranscript    -- 내부 → 디스크 저장
  deriving Repr

-- ═══════════════════════════════════════════════
-- Invariants
-- ═══════════════════════════════════════════════

/-- 모든 메시지는 유효한 UUID를 가진다. -/
structure MessageInvariant (m : SerializedMessage) : Prop where
  nonEmptyUuid : m.uuid.val ≠ ""
  validVersion : m.version > 0

/-- 트랜스크립트의 부모 참조가 유효. -/
structure TranscriptTreeInvariant (messages : List TranscriptMessage) : Prop where
  rootExists : messages.any (fun m => m.parentUuid.isNone) = true

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def messageBoundaryMap : List BoundaryEntry := [
  { fnName := "SerializedMessage",  traits := [.pure],     target := .typescript },
  { fnName := "TranscriptMessage",  traits := [.pure],     target := .typescript },
  { fnName := "QueuedCommand",      traits := [.pure, .io], target := .typescript },
  { fnName := "MessageOrigin",      traits := [.pure],     target := .typescript }
]

end ClaudeCode.Message
