// @spec Specs/Domain/VimMode.lean ClaudeCode.VimMode

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VimMode {
    Insert,
    #[default]
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Operator {
    Delete,
    Change,
    Yank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FindType {
    Find,
    FindBack,
    Till,
    TillBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextObjScope {
    Inner,
    Around,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandState {
    pub operator: Option<Operator>,
    pub count: Option<u32>,
    pub find_type: Option<FindType>,
    pub find_char: Option<char>,
    pub text_obj_scope: Option<TextObjScope>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersistentState {
    pub last_find: Option<(FindType, char)>,
    pub registers: Vec<(char, String)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VimState {
    pub mode: VimMode,
    pub command: CommandState,
    pub persistent: PersistentState,
    pub cursor_pos: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedChange {
    pub keys: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimInputState {
    pub mode: VimMode,
    pub vim: VimState,
    #[serde(default)]
    pub buffer: String,
    #[serde(default)]
    pub cursor: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimTransition {
    InsertToNormal,
    NormalToInsert,
    StayInNormal,
    StayInInsert,
}
