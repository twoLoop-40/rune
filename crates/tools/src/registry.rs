// Tool registry — stores and looks up tools by name.

use std::collections::HashMap;
use std::sync::Arc;

use claude_code_core::tool::ToolDef;

use crate::Tool;

/// Registry of available tools.
/// The engine queries this to find tools when the LLM makes a tool_use call.
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool. Also registers aliases.
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let def = tool.definition();
        self.tools.insert(def.name.clone(), Arc::clone(&tool));
        for alias in &def.aliases {
            self.tools.insert(alias.clone(), Arc::clone(&tool));
        }
    }

    /// Look up a tool by name or alias.
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// List all unique tool definitions (for sending to LLM).
    pub fn definitions(&self) -> Vec<&ToolDef> {
        let mut seen = std::collections::HashSet::new();
        let mut defs = Vec::new();
        for tool in self.tools.values() {
            let name = &tool.definition().name;
            if seen.insert(name.clone()) {
                defs.push(tool.definition());
            }
        }
        defs
    }

    /// Number of unique tools.
    pub fn len(&self) -> usize {
        self.definitions().len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
