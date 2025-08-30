use crate::Config;
use crate::notification::{send_notification, set_notification_sound, send_notification_with_duration};
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::wrapper::Parameters,
    handler::server::router::tool::ToolRouter,
    model::*,
    tool, tool_handler, tool_router, schemars,
};
use serde::Deserialize;
use std::error::Error;
use tokio_util::sync::CancellationToken;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SendNotificationArgs {
    pub title: String,
    pub message: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SendNotificationWithDurationArgs {
    pub title: String,
    pub message: String,
    pub duration_seconds: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetNotificationSoundArgs {
    pub sound_path: String,
}

#[derive(Clone)]
pub struct NotifyMeService {
    config: Config,
    tool_router: ToolRouter<NotifyMeService>,
}

#[tool_router]
impl NotifyMeService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "发送系统通知 - 参数: title(标题), message(消息内容)")]
    async fn send_notification(&self, Parameters(args): Parameters<SendNotificationArgs>) -> Result<CallToolResult, McpError> {
        match send_notification(&args.title, &args.message) {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("通知发送成功")])),
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!("通知发送失败: {}", e))])),
        }
    }

    #[tool(description = "发送带时长的系统通知 - 参数: title(标题), message(消息内容), duration_seconds(显示时长，可选)")]
    async fn send_notification_with_duration(&self, Parameters(args): Parameters<SendNotificationWithDurationArgs>) -> Result<CallToolResult, McpError> {
        match send_notification_with_duration(&args.title, &args.message, args.duration_seconds) {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("带时长通知发送成功")])),
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!("带时长通知发送失败: {}", e))])),
        }
    }

    #[tool(description = "设置通知声音 - 参数: sound_path(声音文件路径)")]
    async fn set_notification_sound(&self, Parameters(args): Parameters<SetNotificationSoundArgs>) -> Result<CallToolResult, McpError> {
        match set_notification_sound(&args.sound_path) {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("通知声音设置成功")])),
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!("通知声音设置失败: {}", e))])),
        }
    }
}

#[tool_handler]
impl ServerHandler for NotifyMeService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "NotifyMe".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("NotifyMe MCP 服务器: tools=send_notification, send_notification_with_duration, set_notification_sound".to_string()),
        }
    }
}

pub async fn run_mcp_server(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let bind_address = format!("127.0.0.1:{}", config.mcp_port);
    println!("🌐 启动 MCP SSE 服务器，地址: {}", bind_address);

    let server_config = SseServerConfig {
        bind: bind_address.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: CancellationToken::new(),
        sse_keep_alive: None,
    };
    
    let (sse_server, router) = SseServer::new(server_config);
    
    // 添加 CORS 中间件
    use tower_http::cors::{Any, CorsLayer};
    use axum::http::HeaderName;
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_credentials(false);
    
    let router_with_cors = router.layer(cors);
    
    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;
    let ct = sse_server.config.ct.child_token();

    let http = axum::serve(listener, router_with_cors).with_graceful_shutdown(async move {
        ct.cancelled().await;
    });

    tokio::spawn(async move {
        if let Err(e) = http.await {
            eprintln!("sse server shutdown with error: {}", e);
        }
    });

    let cfg = config.clone();
    let cancel_token = sse_server.with_service(move || NotifyMeService::new(cfg.clone()));

    println!("✅ MCP 服务器启动成功！ SSE: /sse, POST: /message");
    println!("🔧 可用工具: send_notification, send_notification_with_duration, set_notification_sound");
    println!("🌐 CORS 已启用，支持跨域访问");
    println!("按 Ctrl+C 停止服务器...");

    tokio::signal::ctrl_c().await?;
    cancel_token.cancel();
    Ok(())
}