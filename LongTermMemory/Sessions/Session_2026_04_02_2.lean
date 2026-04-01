-- Session_2026_04_02_2: Milestone 1 vertical slice 완성 + E2E 돌파 + 6 tools + REPL
-- ClaudeProvider SSE, 6 builtin tools, interactive REPL, session save/restore. First E2E run 성공.
set_option autoImplicit true
import LongTermMemory.MemorySchema
open LongTermMemory.MemorySchema

namespace LongTermMemory.Sessions.Session_2026_04_02_2

def sessionInfo : SessionMeta :=
  { date := "2026-04-02"
    commits := []
    files := 20 }

def ctx : Context :=
  { project := some "rune"
    domain := some "rust-engine"
    technology := ["Rust", "SSE", "Claude API", "tokio"]
    tags := ["milestone-1", "e2e", "tools", "repl", "rune"] }

-- ============================================================================
-- 1. Milestone 1 Vertical Slice: ClaudeProvider SSE + Tools + CLI
-- First E2E: rune reading Cargo.toml and answering
-- ============================================================================

inductive MilestoneComponent where
  | claudeProviderSSE   -- SSE streaming, tool_use block parsing
  | bashTool            -- shell command execution
  | fileReadTool        -- file read
  | fileWriteTool       -- file write
  | fileEditTool        -- string replacement edit
  | grepTool            -- regex search
  | globTool            -- file pattern matching
  | cliBinary           -- cargo run entry point
  deriving BEq, DecidableEq, Repr

def MilestoneComponent.implemented : MilestoneComponent -> Bool
  | .claudeProviderSSE => true
  | .bashTool          => true
  | .fileReadTool      => true
  | .fileWriteTool     => true
  | .fileEditTool      => true
  | .grepTool          => true
  | .globTool          => true
  | .cliBinary         => true

def allComponents : List MilestoneComponent :=
  [.claudeProviderSSE, .bashTool, .fileReadTool, .fileWriteTool,
   .fileEditTool, .grepTool, .globTool, .cliBinary]

theorem all_8_components_done :
    allComponents.all MilestoneComponent.implemented = true := by decide

def builtinTools : List MilestoneComponent :=
  [.bashTool, .fileReadTool, .fileWriteTool, .fileEditTool, .grepTool, .globTool]

theorem six_builtin_tools : builtinTools.length = 6 := by native_decide

-- ============================================================================
-- 2. SSE Stream Processing: tool_use block parsing
-- Bug: content_block_start carries id+name, deltas carry partial_json
-- ============================================================================

inductive SSEEvent where
  | messageStart       -- carries usage info (input_tokens)
  | contentBlockStart  -- carries tool id + name (for tool_use blocks)
  | contentBlockDelta  -- carries partial text or partial json
  | contentBlockStop   -- block finished
  | messageDelta       -- carries stop_reason + output usage
  | messageStop        -- stream done
  deriving BEq, DecidableEq, Repr

def SSEEvent.carriesToolMeta : SSEEvent -> Bool
  | .contentBlockStart => true   -- id + name here
  | _ => false

def SSEEvent.carriesPartialJson : SSEEvent -> Bool
  | .contentBlockDelta => true   -- partial_json accumulated here
  | _ => false

-- Key insight: tool_use blocks split across events
-- contentBlockStart has {id, name}, deltas accumulate {partial_json}
-- Must buffer and parse JSON only after contentBlockStop
inductive StreamParseStrategy where
  | naive       -- parse each delta independently (BROKEN)
  | buffered    -- accumulate partial_json, parse on stop (CORRECT)
  deriving BEq, DecidableEq, Repr

def StreamParseStrategy.correct : StreamParseStrategy -> Bool
  | .naive    => false
  | .buffered => true

theorem buffered_is_correct :
    StreamParseStrategy.buffered.correct = true := by decide

-- ============================================================================
-- 3. Token Usage: parsed from SSE message_start / message_delta
-- 6-turn conversation test: 18,956 tokens
-- ============================================================================

structure TokenUsage where
  inputTokens  : Nat
  outputTokens : Nat
  deriving Repr, BEq

def SSEEvent.carriesUsage : SSEEvent -> Bool
  | .messageStart => true   -- input_tokens
  | .messageDelta => true   -- output_tokens
  | _ => false

def testConversation : TokenUsage :=
  { inputTokens := 14000   -- approximate
    outputTokens := 4956 }  -- approximate

-- 6-turn, ~18956 total tokens
theorem test_reasonable :
    testConversation.inputTokens + testConversation.outputTokens < 20000 := by
  native_decide

-- ============================================================================
-- 4. Interactive REPL + Session Management + Slash Commands
-- ============================================================================

inductive SlashCommand where
  | help | clear | model | tokens | save | sessions | quit
  deriving BEq, DecidableEq, Repr

def allSlashCommands : List SlashCommand :=
  [.help, .clear, .model, .tokens, .save, .sessions, .quit]

theorem seven_slash_commands : allSlashCommands.length = 7 := by native_decide

inductive REPLFeature where
  | multiTurnConversation  -- conversation context preserved across turns
  | sessionSaveRestore     -- ~/.rune/sessions/{id}.json
  | slashCommands          -- /help /clear /model /tokens /save /sessions /quit
  | tokenDisplay           -- show token usage per turn
  deriving BEq, DecidableEq, Repr

def REPLFeature.implemented : REPLFeature -> Bool
  | .multiTurnConversation => true
  | .sessionSaveRestore    => true
  | .slashCommands         => true
  | .tokenDisplay          => true

theorem all_repl_features :
    [REPLFeature.multiTurnConversation, .sessionSaveRestore,
     .slashCommands, .tokenDisplay].all REPLFeature.implemented = true := by decide

-- ============================================================================
-- 5. GitHub repo + tomorrow plan
-- ============================================================================

structure RepoInfo where
  org     : String
  name    : String
  commits : Nat
  deriving Repr

def repo : RepoInfo :=
  { org := "twoLoop-40"
    name := "rune"
    commits := 6 }

-- Tomorrow: test all tools, make them fully usable
-- Agent tool + ratatui TUI moved to Milestone 3
inductive NextStep where
  | testAllTools        -- verify 6 tools work end-to-end
  | makeToolsUsable     -- polish tool behavior, error handling
  | agentTool           -- moved to Milestone 3
  | ratatuiTUI          -- moved to Milestone 3
  deriving BEq, DecidableEq, Repr

def NextStep.milestone : NextStep -> Nat
  | .testAllTools    => 1
  | .makeToolsUsable => 1
  | .agentTool       => 3
  | .ratatuiTUI      => 3

def NextStep.tomorrow : NextStep -> Bool
  | .testAllTools    => true
  | .makeToolsUsable => true
  | _                => false

-- ============================================================================
-- 에피소드 기억
-- ============================================================================

def episodes : List EpisodicMemory := [
  { what := "Milestone 1 vertical slice 완성. ClaudeProvider SSE streaming, Bash/FileRead/FileWrite 도구, CLI binary. First E2E: rune이 Cargo.toml 읽고 답변."
    when_ := "2026-04-02T10:00"
    context := ctx
    emotionalValence := some 9
    outcome := some "첫 E2E 성공. 사용자 비전 'LLM만 넣으면 돌아가는 엔진' 실현" },
  { what := "SSE stream processing 버그 수정. tool_use 블록이 content_block_start(id+name) + delta(partial_json)로 분리됨. 버퍼링 파싱으로 해결."
    when_ := "2026-04-02T12:00"
    context := ctx
    outcome := some "buffered parsing strategy로 tool_use 블록 정상 파싱 및 실행" },
  { what := "FileEdit, Grep, Glob 도구 추가. 총 6개 builtin tools 완성."
    when_ := "2026-04-02T14:00"
    context := ctx
    outcome := some "6 tools: Bash, FileRead, FileWrite, FileEdit, Grep, Glob" },
  { what := "Token usage 파싱 (SSE message_start/message_delta). 6-turn 대화 테스트 18,956 토큰."
    when_ := "2026-04-02T15:00"
    context := ctx
    outcome := some "토큰 사용량 추적 동작 확인" },
  { what := "Interactive REPL + multi-turn conversation + session save/restore (~/.rune/sessions/{id}.json) + 7 slash commands."
    when_ := "2026-04-02T17:00"
    context := ctx
    outcome := some "완전한 대화형 CLI 환경 완성" },
  { what := "Specs/Domain/CLI.lean formal specification 작성. GitHub twoLoop-40/rune 6 commits 푸시."
    when_ := "2026-04-02T18:00"
    context := ctx
    outcome := some "프로젝트 공개. rune = Rust + engine" }
]

-- ============================================================================
-- 의미 기억
-- ============================================================================

def semantics : List SemanticMemory := [
  { fact := "SSE tool_use 파싱: content_block_start가 id+name, delta가 partial_json 운반. 반드시 버퍼링 후 contentBlockStop에서 JSON 파싱해야 함."
    domain := some "sse-parsing"
    source := some "stream processing 버그 수정"
    stability := ⟨95, by omega⟩
    maturity := .consolidated },
  { fact := "사용자 비전 'LLM만 넣으면 돌아가는 엔진' E2E로 확인됨. rune이 Cargo.toml 읽고 답변하는 것으로 증명."
    domain := some "product-vision"
    source := some "first E2E run"
    stability := ⟨90, by omega⟩
    maturity := .consolidated },
  { fact := "사용자는 GDD(Goal-Driven Development) 워크플로우로 구조화된 진행을 선호함."
    domain := some "user-preference"
    source := some "세션 관찰"
    stability := ⟨80, by omega⟩
    maturity := .consolidated },
  { fact := "내일 계획: 6개 도구 모두 테스트, fully usable하게 만들기. Agent tool + ratatui TUI는 Milestone 3로 이동."
    domain := some "planning"
    source := some "세션 종료 시 정리"
    stability := ⟨70, by omega⟩
    maturity := .raw }
]

theorem episode_count : episodes.length = 6 := by native_decide
theorem semantic_count : semantics.length = 4 := by native_decide

end LongTermMemory.Sessions.Session_2026_04_02_2
