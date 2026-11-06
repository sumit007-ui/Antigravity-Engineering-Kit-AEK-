//! Agent adapters for all supported AI coding agent platforms.

pub mod claude_code;
pub mod cursor;
pub mod opencode;

pub use claude_code::ClaudeCodeAdapter;
pub use cursor::CursorAdapter;
pub use opencode::OpenCodeAdapter;

// Re-export AntigravityAdapter from root antigravity module
pub use crate::antigravity::AntigravityAdapter;
