#[cfg(target_os = "windows")]
use winrt_notification::{Toast, Sound};
#[cfg(target_os = "windows")]
use std::path::Path;
use std::process::Command;

pub fn send_notification(title: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_notification_with_duration(title, message, None)
}

pub fn send_notification_with_duration(
    title: &str, 
    message: &str, 
    duration_seconds: Option<u32>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔔 发送通知: '{}' - '{}'", title, message);
    if let Some(duration) = duration_seconds {
        println!("⏱️ 设置显示时长: {}秒", duration);
    }
    
    #[cfg(target_os = "macos")]
    {
        // 使用AppleScript发送通知（最可靠的方法）
        let mut apple_script = format!(
            "display notification \"{}\" with title \"{}\" sound name \"Glass\"",
            message.replace("\"", "\\\""),
            title.replace("\"", "\\\"")
        );
        
        // 如果指定了时长，添加到AppleScript中
        if let Some(duration) = duration_seconds {
            apple_script = format!(
                "display notification \"{}\" with title \"{}\" sound name \"Glass\" subtitle \"显示时长: {}秒\"",
                message.replace("\"", "\\\""),
                title.replace("\"", "\\\""),
                duration
            );
        }
        
        let result = Command::new("osascript")
            .arg("-e")
            .arg(&apple_script)
            .output();
            
        match result {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ 通知发送成功");
                    return Ok(());
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    println!("❌ 通知发送失败: {}", error_msg);
                    return Err(format!("AppleScript执行失败: {}", error_msg).into());
                }
            }
            Err(e) => {
                println!("❌ 无法执行AppleScript: {}", e);
                return Err(format!("无法执行AppleScript: {}", e).into());
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        let mut toast = Toast::new(Toast::POWERSHELL_APP_ID)
            .title(title)
            .text1(message);
            
        // Windows支持设置显示时长
        if let Some(duration) = duration_seconds {
            toast = toast.duration(duration);
        }
        
        toast.show()?;
        return Ok(());
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        return Err("不支持的操作系统".into());
    }
}

pub fn set_notification_sound(sound_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In a real implementation, you would configure the system to use this sound for notifications
    println!("设置通知声音: {}", sound_path);
    Ok(())
}