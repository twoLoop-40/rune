import Lake
open Lake DSL

package «claude-code» where
  leanOptions := #[
    ⟨`autoImplicit, false⟩
  ]

@[default_target]
lean_lib Specs where
  srcDir := "Specs"
  roots := #[
    `Common.Types,
    `Domain.Tool,
    `Domain.Task,
    `Domain.Permission,
    `Domain.Message,
    `Domain.Query,
    `Domain.State,
    `Domain.Hook,
    `Domain.Plugin,
    `Domain.Command,
    `Domain.Bridge,
    `Domain.Agent,
    `Domain.Session,
    `Domain.Model,
    `Domain.VimMode,
    `Domain.Worktree,
    `Domain.FileHistory,
    `Domain.Voice,
    `Domain.TextInput,
    `Domain.MCPServer,
    `Domain.Compaction,
    `Domain.RemoteSession,
    `Domain.Cost,
    `Project.ProjectGoalSpec
  ]

lean_lib LongTermMemory where
  roots := #[
    `LongTermMemory.MemorySchema,
    `LongTermMemory.Sessions,
    `LongTermMemory.Domain,
    `LongTermMemory.Days,
    `LongTermMemory.LongTermMemory
  ]
