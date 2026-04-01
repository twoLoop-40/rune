/-
  Domain/Command.lean — 명령어 시스템
  슬래시 커맨드 레지스트리. 100+ 명령어의 분류 체계.
-/
import Common.Types

namespace ClaudeCode.Command

open ClaudeCode

-- ═══════════════════════════════════════════════
-- Command Type (Discriminated Union)
-- ═══════════════════════════════════════════════

/-- 명령어 유형. -/
inductive CommandType where
  | prompt     -- AI 프롬프트 실행
  | local_     -- 로컬 함수 실행
  | localJsx   -- 로컬 JSX 컴포넌트
  deriving Repr, BEq

/-- 명령어 가용성 컨텍스트. -/
inductive CommandAvailability where
  | claudeAi   -- claude.ai 웹
  | console     -- CLI 터미널
  deriving Repr, BEq

/-- 프롬프트 실행 컨텍스트. -/
inductive PromptContext where
  | inline   -- 현재 대화에서 실행
  | fork     -- 서브에이전트로 실행
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Command Definition
-- ═══════════════════════════════════════════════

/-- 명령어 기본 정의. -/
structure CommandBase where
  name          : String
  aliases       : List String := []
  description   : String
  availability  : List CommandAvailability := [.console]
  isEnabled     : Bool := true
  isHidden      : Bool := false
  isMcp         : Bool := false
  argumentHint  : String := ""
  whenToUse     : String := ""
  version       : String := ""
  userInvocable : Bool := true
  immediate     : Bool := false      -- 큐 우회
  isSensitive   : Bool := false      -- 인자 숨김
  kind          : Option String := none  -- "workflow" | none
  deriving Repr

/-- 프롬프트 명령어 확장. -/
structure PromptCommandDef extends CommandBase where
  type             : CommandType := .prompt
  progressMessage  : String := ""
  contentLength    : Nat := 0
  argNames         : List String := []
  allowedTools     : List String := []
  model            : Option String := none
  source           : SettingSource := .userSettings
  context          : PromptContext := .inline
  agent            : String := ""
  effort           : EffortValue := .medium
  paths            : List GlobPattern := []
  deriving Repr

/-- 로컬 명령어 확장. -/
structure LocalCommandDef extends CommandBase where
  type                   : CommandType := .local_
  supportsNonInteractive : Bool := false
  deriving Repr

-- ═══════════════════════════════════════════════
-- Command Categories (100+ commands)
-- ═══════════════════════════════════════════════

/-- 명령어 카테고리. -/
inductive CommandCategory where
  | session       -- /clear, /compact, /resume
  | git           -- /commit, /pr, /diff
  | config        -- /config, /permissions, /model
  | agent         -- /agents, /tasks, /team
  | navigation    -- /cd, /ls
  | help          -- /help, /shortcuts
  | skill         -- /skill (동적 로드)
  | debug         -- /debug, /verbose
  | mcp           -- /mcp (MCP 도구)
  | plugin        -- /plugin (플러그인 관리)
  deriving Repr, BEq

-- ═══════════════════════════════════════════════
-- Command Registry
-- ═══════════════════════════════════════════════

/-- 명령어 레지스트리. -/
structure CommandRegistry where
  builtinCommands : List CommandBase
  pluginCommands  : List CommandBase := []
  mcpCommands     : List CommandBase := []
  deriving Repr

/-- 이름으로 명령어 조회. 별칭 포함. -/
def CommandRegistry.findByName (reg : CommandRegistry) (name : String) : Option CommandBase :=
  let all := reg.builtinCommands ++ reg.pluginCommands ++ reg.mcpCommands
  all.find? fun cmd => cmd.name == name || cmd.aliases.contains name

-- ═══════════════════════════════════════════════
-- Invariants
-- ═══════════════════════════════════════════════

/-- 명령어 이름 유일성. -/
structure CommandNameUniqueness (cmds : List CommandBase) : Prop where
  unique : ∀ (i j : Nat) (hi : i < cmds.length) (hj : j < cmds.length),
    i ≠ j → (cmds.get ⟨i, hi⟩).name ≠ (cmds.get ⟨j, hj⟩).name

-- ═══════════════════════════════════════════════
-- BoundaryMap
-- ═══════════════════════════════════════════════

def commandBoundaryMap : List BoundaryEntry := [
  { fnName := "CommandBase",       traits := [.pure],     target := .typescript },
  { fnName := "PromptCommandDef",  traits := [.pure, .io], target := .typescript },
  { fnName := "CommandRegistry",   traits := [.pure],     target := .typescript }
]

end ClaudeCode.Command
