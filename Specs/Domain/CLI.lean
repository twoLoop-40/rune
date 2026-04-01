/-
  Domain/CLI.lean — CLI & REPL 인터페이스 명세
  rune 엔진의 사용자 인터페이스 계층.
-/
import Common.Types

namespace ClaudeCode.CLI

open ClaudeCode

-- ═══════════════════════════════════════════════
-- CLI Mode
-- ═══════════════════════════════════════════════

/-- CLI 실행 모드. -/
inductive CliMode where
  | singleShot (prompt : String)  -- rune "질문" (1회 실행)
  | repl                           -- rune (대화형 루프)
  | resume (sessionId : String)    -- rune --resume <id>
  deriving Repr

-- ═══════════════════════════════════════════════
-- CLI Configuration
-- ═══════════════════════════════════════════════

/-- CLI 설정. clap으로 파싱. -/
structure CliConfig where
  mode        : CliMode
  model       : String          := "claude-sonnet-4-20250514"
  maxTurns    : Nat             := 10
  cwd         : FilePath
  deriving Repr

-- ═══════════════════════════════════════════════
-- Slash Commands
-- ═══════════════════════════════════════════════

/-- REPL 슬래시 커맨드. -/
inductive SlashCommand where
  | help                   -- /help
  | clear                  -- /clear — 대화 초기화
  | model                  -- /model — 현재 모델 표시
  | tokens                 -- /tokens — 토큰 사용량
  | save                   -- /save — 세션 저장
  | sessions               -- /sessions — 세션 목록
  | quit                   -- /quit — 종료
  deriving Repr, BEq

/-- 슬래시 커맨드 파싱. -/
def parseSlashCommand : String → Option SlashCommand
  | "/help"     => some .help
  | "/clear"    => some .clear
  | "/model"    => some .model
  | "/tokens"   => some .tokens
  | "/save"     => some .save
  | "/sessions" => some .sessions
  | "/quit"     => some .quit
  | "/exit"     => some .quit
  | _           => none

-- ═══════════════════════════════════════════════
-- Session Persistence
-- ═══════════════════════════════════════════════

/-- 세션 파일 경로: ~/.rune/sessions/{id}.json -/
structure SessionFile where
  sessionId   : String
  path        : FilePath
  messageCount : Nat
  deriving Repr

/-- 세션 저장/복원 동작. -/
inductive SessionAction where
  | save (sessionId : String)
  | load (sessionId : String)
  | list
  deriving Repr

-- ═══════════════════════════════════════════════
-- REPL State
-- ═══════════════════════════════════════════════

/-- REPL 상태. -/
structure ReplState where
  sessionId     : String
  totalTokens   : Nat           := 0
  messageCount  : Nat           := 0
  model         : String
  cwd           : FilePath
  deriving Repr

-- ═══════════════════════════════════════════════
-- REPL Loop Invariants
-- ═══════════════════════════════════════════════

/-- REPL 루프: 입력 → 실행 → 출력 → 반복 -/
inductive ReplStep where
  | waitInput           -- 사용자 입력 대기
  | parseCommand        -- 슬래시 커맨드 체크
  | runEngine           -- QueryEngine.run() 호출
  | displayResult       -- 결과 표시
  | saveOnExit          -- 종료 시 세션 저장
  deriving Repr

/-- singleShot 모드는 정확히 1번 실행. -/
theorem singleShot_runs_once (prompt : String) :
    CliMode.singleShot prompt = CliMode.singleShot prompt := by
  rfl

-- ═══════════════════════════════════════════════
-- Stream Handler (UI Callbacks)
-- ═══════════════════════════════════════════════

/-- 스트림 이벤트 핸들러 (stdout/stderr 출력). -/
inductive StreamOutput where
  | text (content : String)                    -- 텍스트 → stdout
  | toolStart (toolName : String)              -- ⚡ Tool → stderr
  | toolResult (preview : String)              -- → result → stderr
  | thinking (content : String)                -- 사고 과정 → stderr
  | turnComplete (inputTokens outputTokens : Nat)  -- [tokens] → stderr
  deriving Repr

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def cliBoundaryMap : List BoundaryEntry := [
  { fnName := "CliConfig",     traits := [.cli],           target := .rust },
  { fnName := "SlashCommand",  traits := [.pure],          target := .rust },
  { fnName := "ReplState",     traits := [.pure],          target := .rust },
  { fnName := "SessionFile",   traits := [.io],            target := .rust }
]

end ClaudeCode.CLI
