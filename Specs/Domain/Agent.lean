/-
  Domain/Agent.lean -- Agent Definition System
  Built-in, Custom, Plugin agent types. Subagent spawning & routing.
-/
import Common.Types

namespace ClaudeCode.Agent

open ClaudeCode

-- Agent Source
inductive AgentSource where
  | builtIn
  | custom
  | plugin (source : PluginSource)
  | policy
  deriving Repr, BEq

-- Agent Color
inductive AgentColorName where
  | blue | green | yellow | red | purple | orange | cyan | magenta
  deriving Repr, BEq

-- MCP Server Spec for Agents
structure AgentMcpServerSpec where
  serverName : String
  config     : JsonValue
  deriving Repr

-- Base Agent Definition
structure BaseAgentDef where
  name          : String
  description   : String
  source        : AgentSource
  model         : Option String      := none
  customPrompt  : Option String      := none
  appendPrompt  : Option String      := none
  allowedTools  : List String         := []
  disallowedTools : List String       := []
  mcpServers    : List AgentMcpServerSpec := []
  color         : Option AgentColorName   := none
  deriving Repr

-- Agent Definition Variants
inductive AgentDefinition where
  | builtIn (base : BaseAgentDef) (isDynamic : Bool)
  | custom (base : BaseAgentDef) (configPath : FilePath)
  | plugin (base : BaseAgentDef) (pluginName : String)
  | policy (base : BaseAgentDef)
  deriving Repr

-- Agent Definitions Result
structure AgentDefinitionsResult where
  agents : List AgentDefinition
  errors : List String := []
  deriving Repr

-- Forked Agent Parameters
structure ForkedAgentParams where
  agentType      : String
  prompt         : String
  agentId        : Option AgentId := none
  model          : Option String  := none
  permissionMode : Option String  := none
  isolation      : Option String  := none
  deriving Repr

-- Forked Agent Result
inductive ForkedAgentResult where
  | success (response : String) (agentId : AgentId)
  | failure (error : String)
  deriving Repr

-- Resolved Agent
structure ResolvedAgent where
  definition     : AgentDefinition
  resolvedModel  : String
  resolvedTools  : List String
  deriving Repr

-- Model Alias for Agents
inductive AgentModelAlias where
  | sonnet | opus | haiku
  deriving Repr, BEq

def agentBoundaryMap : List BoundaryEntry := [
  { fnName := "AgentDefinition",     traits := [.pure],     target := .typescript },
  { fnName := "ForkedAgentParams",   traits := [.io],       target := .typescript },
  { fnName := "ForkedAgentResult",   traits := [.io],       target := .typescript },
  { fnName := "ResolvedAgent",       traits := [.pure],     target := .typescript }
]

end ClaudeCode.Agent
