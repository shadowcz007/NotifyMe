use notifyme::notification::send_notification;

fn main() {
    println!("🧪 测试Mac通知功能...");
    
    match send_notification("测试通知", "这是一个来自Rust的测试通知！") {
        Ok(_) => println!("✅ 通知发送成功！"),
        Err(e) => println!("❌ 通知发送失败: {}", e),
    }
}
