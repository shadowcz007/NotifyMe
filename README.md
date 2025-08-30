# NotifyMe

一个基于 MCP (Model Context Protocol) 的通知服务，提供系统通知和声音设置功能。

## 功能特性

- 🔔 **系统通知**: 发送跨平台系统通知
- ⏱️ **通知时长**: 设置通知显示时长
- 🔊 **通知声音**: 设置自定义通知声音
- 🌐 **MCP 协议**: 支持 Model Context Protocol
- 🖥️ **跨平台**: 支持 macOS 和 Windows

## 可用工具

### 1. send_notification
发送系统通知

**参数:**
- `title` (String): 通知标题
- `message` (String): 通知消息内容

**示例:**
```json
{
  "title": "提醒",
  "message": "这是一个测试通知"
}
```

### 2. send_notification_with_duration
发送带时长的系统通知

**参数:**
- `title` (String): 通知标题
- `message` (String): 通知消息内容
- `duration_seconds` (Number, 可选): 显示时长（秒）

**示例:**
```json
{
  "title": "定时提醒",
  "message": "这个通知会显示5秒",
  "duration_seconds": 5
}
```

### 3. set_notification_sound
设置通知声音

**参数:**
- `sound_path` (String): 声音文件路径

**示例:**
```json
{
  "sound_path": "/path/to/sound.wav"
}
```

## 安装和运行

### 环境要求
- Rust 1.70+
- macOS 或 Windows

### 编译
```bash
cargo build --release
```

### 运行
```bash
# 使用默认端口 (6656)
cargo run

# 或设置自定义端口
MCP_PORT=8080 cargo run
```

## 配置

通过环境变量配置：

- `MCP_PORT`: MCP 服务器端口号 (默认: 6656)

## MCP 服务器信息

- **协议版本**: 2024-11-05
- **服务器地址**: `127.0.0.1:6656` (默认)
- **SSE 端点**: `/sse`
- **POST 端点**: `/message`
- **CORS**: 已启用

## 使用示例

### 通过 MCP 客户端调用

1. 连接到 MCP 服务器
2. 调用 `send_notification` 工具发送普通通知
3. 调用 `send_notification_with_duration` 工具发送带时长的通知
4. 调用 `set_notification_sound` 工具设置声音

### 工具调用示例

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "send_notification_with_duration",
    "arguments": {
      "title": "任务完成",
      "message": "您的任务已经完成！",
      "duration_seconds": 10
    }
  }
}
```

## 测试

### 测试通知功能
```bash
cargo run --bin test_notification
```

这个测试程序会发送三种类型的通知：
1. 普通通知
2. 5秒时长的通知
3. 2秒时长的通知

## 开发

### 项目结构
```
src/
├── main.rs          # 主程序入口
├── lib.rs           # 库模块定义
├── config.rs        # 配置管理
├── mcp_service.rs   # MCP 服务实现
├── notification.rs  # 通知功能实现
└── models.rs        # 数据模型
```

### 添加新工具

1. 在 `mcp_service.rs` 中定义工具参数结构体
2. 实现工具函数
3. 添加 `#[tool]` 宏标记
4. 更新服务器信息中的工具列表

## 许可证

MIT License
