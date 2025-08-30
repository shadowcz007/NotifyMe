use notifyme::{run_mcp_server, Config};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = Config::from_env();
    
    println!("🚀 启动 NotifyMe MCP 服务器");
    println!("📋 配置信息:");
    println!("  - MCP 端口: {}", config.mcp_port);
    println!("🔧 可用工具:");
    println!("  - send_notification: 发送系统通知");
    println!("  - set_notification_sound: 设置通知声音");
    println!();
    
    run_mcp_server(config).await
}