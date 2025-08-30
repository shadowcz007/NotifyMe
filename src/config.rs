use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// MCP 服务器端口号
    pub mcp_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            mcp_port: env::var("MCP_PORT").map(|v| v.parse().unwrap_or(6656)).unwrap_or(6656),
        }
    }
}