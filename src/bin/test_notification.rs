use notifyme::notification::{send_notification, send_notification_with_duration};

fn main() {
    println!("🧪 测试Mac通知功能...");
    
    // 测试普通通知
    println!("\n📢 测试1: 普通通知");
    match send_notification("测试通知", "这是一个来自Rust的测试通知！") {
        Ok(_) => println!("✅ 普通通知发送成功！"),
        Err(e) => println!("❌ 普通通知发送失败: {}", e),
    }
    
    // 等待2秒
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // 测试带时长的通知
    println!("\n📢 测试2: 带时长的通知（5秒）");
    match send_notification_with_duration("定时通知", "这个通知会显示5秒", Some(5)) {
        Ok(_) => println!("✅ 定时通知发送成功！"),
        Err(e) => println!("❌ 定时通知发送失败: {}", e),
    }
    
    // 等待2秒
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // 测试短时长通知
    println!("\n📢 测试3: 短时长通知（2秒）");
    match send_notification_with_duration("短时通知", "这个通知会显示2秒", Some(2)) {
        Ok(_) => println!("✅ 短时通知发送成功！"),
        Err(e) => println!("❌ 短时通知发送失败: {}", e),
    }
    
    println!("\n🎉 所有测试完成！");
}
