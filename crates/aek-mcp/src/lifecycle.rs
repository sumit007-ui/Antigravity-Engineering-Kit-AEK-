//! MCP server lifecycle management and state machine.

use crate::capabilities::{ServerCapabilities, ServerInfo};
use crate::errors::McpError;
use serde::{Deserialize, Serialize};

/// High-level lifecycle states of an MCP server session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerState {
    Disconnected,
    Connecting,
    Initializing,
    Ready {
        server_info: ServerInfo,
        capabilities: ServerCapabilities,
    },
    Degraded {
        reason: String,
    },
    Failed {
        error: String,
    },
    Stopping,
}

impl std::fmt::Display for ServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disconnected => write!(f, "DISCONNECTED"),
            Self::Connecting => write!(f, "CONNECTING"),
            Self::Initializing => write!(f, "INITIALIZING"),
            Self::Ready { .. } => write!(f, "READY"),
            Self::Degraded { reason } => write!(f, "DEGRADED ({reason})"),
            Self::Failed { error } => write!(f, "FAILED ({error})"),
            Self::Stopping => write!(f, "STOPPING"),
        }
    }
}

/// Lifecycle supervisor managing states and transition validation.
#[derive(Debug, Clone)]
pub struct ServerLifecycle {
    server_id: String,
    state: ServerState,
}

impl ServerLifecycle {
    /// Creates a new lifecycle supervisor initialized to `Disconnected`.
    #[must_use]
    pub fn new(server_id: impl Into<String>) -> Self {
        Self {
            server_id: server_id.into(),
            state: ServerState::Disconnected,
        }
    }

    /// Current server state.
    #[must_use]
    pub fn state(&self) -> &ServerState {
        &self.state
    }

    /// Server identifier.
    #[must_use]
    pub fn server_id(&self) -> &str {
        &self.server_id
    }

    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        matches!(self.state, ServerState::Disconnected)
    }

    #[must_use]
    pub fn is_connecting(&self) -> bool {
        matches!(self.state, ServerState::Connecting)
    }

    #[must_use]
    pub fn is_initializing(&self) -> bool {
        matches!(self.state, ServerState::Initializing)
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        matches!(self.state, ServerState::Ready { .. })
    }

    #[must_use]
    pub fn is_degraded(&self) -> bool {
        matches!(self.state, ServerState::Degraded { .. })
    }

    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self.state, ServerState::Failed { .. })
    }

    #[must_use]
    pub fn is_stopping(&self) -> bool {
        matches!(self.state, ServerState::Stopping)
    }

    /// Transition to `Connecting`. Valid from `Disconnected` or `Failed`.
    pub fn transition_to_connecting(&mut self) -> Result<(), McpError> {
        match &self.state {
            ServerState::Disconnected | ServerState::Failed { .. } => {
                self.state = ServerState::Connecting;
                Ok(())
            }
            other => Err(McpError::ConnectionFailed(
                self.server_id.clone(),
                format!("Cannot transition to Connecting from {other}"),
            )),
        }
    }

    /// Transition to `Initializing`. Valid from `Connecting`.
    pub fn transition_to_initializing(&mut self) -> Result<(), McpError> {
        match &self.state {
            ServerState::Connecting => {
                self.state = ServerState::Initializing;
                Ok(())
            }
            other => Err(McpError::InitializationFailed(
                self.server_id.clone(),
                format!("Cannot transition to Initializing from {other}"),
            )),
        }
    }

    /// Transition to `Ready`. Valid from `Initializing`.
    pub fn transition_to_ready(
        &mut self,
        server_info: ServerInfo,
        capabilities: ServerCapabilities,
    ) -> Result<(), McpError> {
        match &self.state {
            ServerState::Initializing => {
                self.state = ServerState::Ready {
                    server_info,
                    capabilities,
                };
                Ok(())
            }
            other => Err(McpError::InitializationFailed(
                self.server_id.clone(),
                format!("Cannot transition to Ready from {other}"),
            )),
        }
    }

    /// Transition to `Degraded`. Valid from `Ready`.
    pub fn transition_to_degraded(&mut self, reason: impl Into<String>) -> Result<(), McpError> {
        match &self.state {
            ServerState::Ready { .. } => {
                self.state = ServerState::Degraded {
                    reason: reason.into(),
                };
                Ok(())
            }
            other => Err(McpError::ConnectionFailed(
                self.server_id.clone(),
                format!("Cannot transition to Degraded from {other}"),
            )),
        }
    }

    /// Transition to `Failed`. Valid from any non-stopping state.
    pub fn transition_to_failed(&mut self, error: impl Into<String>) {
        self.state = ServerState::Failed {
            error: error.into(),
        };
    }

    /// Transition to `Stopping`. Valid from any active state.
    pub fn transition_to_stopping(&mut self) -> Result<(), McpError> {
        match &self.state {
            ServerState::Disconnected => Ok(()),
            _ => {
                self.state = ServerState::Stopping;
                Ok(())
            }
        }
    }

    /// Transition to `Disconnected`. Completes shutdown.
    pub fn transition_to_disconnected(&mut self) -> Result<(), McpError> {
        self.state = ServerState::Disconnected;
        Ok(())
    }
}
