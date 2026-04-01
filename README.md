# Claude Code CLI
Claude Code’s source code appears to have leaked because an early npm release was shipped with source-map data still attached. A developer inspecting the bundled cli.mjs file noticed --enable-source-maps and then found a huge sourceMappingURL block at the bottom, which suggested the package still contained enough mapping data to reconstruct the original source. Anthropic later removed the source map from newer updates, but by then the earlier package had already been downloaded and inspected, so the code could still be recovered from what was already published.



## Overview

Claude Code CLI is a sophisticated terminal application that provides an interactive interface for developers to collaborate with Claude (Anthropic's AI assistant) directly from their terminal. It combines the power of large language models with comprehensive file system operations, tool calling, and extensible architecture.

## Features

### Core Capabilities

- **Interactive AI Chat** - Natural language coding assistance with real-time streaming responses
- **File Operations** - Read, edit, write, search, and analyze files with AI assistance
- **Bash Execution** - Execute shell commands with safety controls and sandboxing
- **Web Integration** - Search and fetch web content directly from the CLI
- **Agent Swarms** - Spawn sub-agents for parallel task execution
- **Task Management** - Track and manage background tasks and operations

### Advanced Features

- **Multiple Execution Modes**
  - Interactive REPL mode for conversational coding
  - Headless mode (`-p/--print`) for scripting and CI/CD pipelines
  - Remote session support via SSH and direct connections

- **MCP (Model Context Protocol)**
  - Connect to external tool servers
  - Dynamic tool discovery and execution
  - Enterprise-grade security with allowlist/denylist policies

- **Context Management**
  - Automatic context compaction for long conversations
  - Session persistence and resume capability
  - Project-specific memory and context

- **Developer Tools**
  - Git integration (commit, diff, branch management)
  - LSP (Language Server Protocol) support
  - Voice mode for speech-to-text input
  - Chrome/IDE integration

## Installation

### Prerequisites

- [Bun](https://bun.sh) runtime (latest stable version recommended)
- Git (for repository operations)

### Install Options

```bash
# Install globally via npm
npm install -g @anthropic-ai/claude-code

# Or install via Bun
bun install -g @anthropic-ai/claude-code

# Or use the native installer (macOS/Linux)
curl -fsSL https://claude.ai/install.sh | sh
```

### Post-Installation Setup

```bash
# Run the setup wizard
claude

# Or initialize with a specific directory
claude /path/to/your/project

# Authenticate with your Anthropic account
claude login
```

## Quick Start

```bash
# Start interactive mode
claude

# Run in headless mode with a prompt
claude -p "Explain this codebase"

# Execute a command and exit
claude -p "Fix the bug in src/utils.ts" --allowedTools BashTool,FileEditTool

# Resume a previous session
claude --resume
```

## Available Commands

Type `/` in the REPL to see all available commands. Here are the key ones:

### Core Commands
| Command | Description |
|---------|-------------|
| `/help` | Show help and available commands |
| `/exit` or `/quit` | Exit the REPL |
| `/clear` or `/reset` | Clear conversation history |
| `/compact` | Summarize conversation to save context |
| `/init` | Initialize CLAUDE.md file with codebase documentation |

### Development Commands
| Command | Description |
|---------|-------------|
| `/commit` | Create a git commit |
| `/review` | Review a pull request |
| `/diff` | Show git diff interface |
| `/branch` | Branch management |
| `/doctor` | Run diagnostics and troubleshooting |

### Configuration Commands
| Command | Description |
|---------|-------------|
| `/config` or `/settings` | Open configuration panel |
| `/theme` | Change terminal theme |
| `/model` | Set the AI model |
| `/mcp` | Manage MCP servers |
| `/skills` | List available skills |

### Context & Memory
| Command | Description |
|---------|-------------|
| `/context` | Visualize current context usage |
| `/memory` | Edit Claude memory files |
| `/files` | List tracked files |
| `/tasks` | List background tasks |

### Session Management
| Command | Description |
|---------|-------------|
| `/session` or `/remote` | Show remote session URL |
| `/resume` | Resume previous session |
| `/share` | Share session |
| `/cost` | Show session cost and duration |

## Built-in Tools

Claude Code comes with 40+ built-in tools for file operations, execution, and integration:

### File Operations
- **FileReadTool** - Read files, images, PDFs with smart pagination
- **FileEditTool** - Edit files with sed-like functionality
- **FileWriteTool** - Create or overwrite files
- **GlobTool** - Find files using glob patterns
- **GrepTool** - Search file contents with regex
- **NotebookEditTool** - Edit Jupyter notebooks

### Execution
- **BashTool** - Execute shell commands with sandboxing
- **PowerShellTool** - Windows PowerShell execution

### Web & Search
- **WebFetchTool** - Fetch web pages content
- **WebSearchTool** - Search the web

### AI & Agents
- **AgentTool** - Spawn sub-agents for parallel work
- **TaskCreateTool/TaskStopTool/TaskListTool** - Background task management

### MCP Integration
- **MCPTool** - Execute tools from MCP servers
- **ListMcpResourcesTool/ReadMcpResourceTool** - MCP resource management

## Configuration

### Global Configuration (~/.claude.json)

```json
{
  "theme": "dark",
  "autoCompactEnabled": true,
  "editorMode": "vim",
  "verbose": false,
  "mcpServers": {
    "my-server": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path"]
    }
  }
}
```

### Project Configuration (.claude.json)

```json
{
  "allowedTools": ["BashTool", "FileEditTool", "FileReadTool"],
  "mcpServers": {
    "project-server": {
      "type": "sse",
      "url": "https://api.example.com/mcp"
    }
  }
}
```

### MCP Configuration (.mcp.json)

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "."]
    },
    "github": {
      "type": "sse",
      "url": "https://api.github.com/mcp",
      "oauth": {
        "clientId": "your-client-id"
      }
    }
  }
}
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `CLAUDE_CODE_DEBUG` | Enable debug logging |
| `CLAUDE_CONFIG_DIR` | Override config directory |
| `DISABLE_AUTOUPDATER` | Disable auto-updater |
| `CLAUDE_CODE_BUBBLEWRAP` | Enable sandbox mode |

## Usage Examples

### Code Review

```bash
# Review the current PR
/review

# Security-focused review
/security-review

# Deep bug-finding review
/ultrareview
```

### File Operations

```bash
# Read a file with specific range
Please read src/main.ts lines 50-100

# Edit a file
Please update the config in package.json to add a new script

# Search across files
Find all TODO comments in the codebase
```

### Git Operations

```bash
# Stage and commit changes
/commit

# Commit, push, and open PR
/commit-push-pr

# Show diff
/diff
```

### Task Management

```bash
# Create a background task
Please create a task to run the test suite in background

# List tasks
/tasks

# Stop a task
Please stop the test task
```

### MCP Server Usage

```bash
# Add an MCP server
/mcp add filesystem npx -y @modelcontextprotocol/server-filesystem .

# Use MCP tools
Please use the filesystem server to list all TypeScript files
```

## Architecture

### High-Level Flow

```
User Input → REPL.tsx → QueryEngine → Claude API → Tool Execution → Response
```

### Key Components

- **main.tsx** - CLI entry point and argument parsing
- **entrypoints/init.ts** - System initialization and configuration
- **screens/REPL.tsx** - Interactive terminal UI
- **QueryEngine.ts** - Message submission and query coordination
- **services/tools/** - Tool execution engine with concurrency control
- **services/mcp/** - MCP client and server management

### Technology Stack

| Component | Technology |
|-----------|------------|
| Runtime | Bun |
| Language | TypeScript (TSX) |
| UI Framework | React 18+ |
| Terminal UI | Ink (custom fork) |
| Schema Validation | Zod v4 |
| CLI Parser | C
