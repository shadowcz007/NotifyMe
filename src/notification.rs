#[cfg(target_os = "macos")]
use cocoa::base::nil;
#[cfg(target_os = "macos")]
use cocoa::foundation::{NSAutoreleasePool, NSString};
#[cfg(target_os = "macos")]
use objc::runtime::{Class, Object};
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};
#[cfg(target_os = "windows")]
use winrt_notification::{Toast, Sound};
#[cfg(target_os = "windows")]
use std::path::Path;

pub fn send_notification(title: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);
            
            let title_str = NSString::alloc(nil).init_str(title);
            let message_str = NSString::alloc(nil).init_str(message);
            
            // 尝试使用 UNUserNotificationCenter (iOS 10+ / macOS 10.14+)
            let notification_class = Class::get("UNUserNotificationCenter");
            if let Some(notification_class) = notification_class {
                // 使用现代的通知 API
                let center: *mut Object = msg_send![notification_class, currentNotificationCenter];
                if !center.is_null() {
                    // 创建通知内容
                    let content_class = Class::get("UNMutableNotificationContent").unwrap();
                    let content: *mut Object = msg_send![content_class, new];
                    let _: () = msg_send![content, setTitle: title_str];
                    let _: () = msg_send![content, setBody: message_str];
                    
                    // 创建通知请求
                    let request_class = Class::get("UNNotificationRequest").unwrap();
                    let identifier = NSString::alloc(nil).init_str(&format!("notifyme_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
                    let request: *mut Object = msg_send![request_class, requestWithIdentifier: identifier content: content trigger: nil];
                    
                    // 添加通知请求
                    let _: () = msg_send![center, addNotificationRequest: request withCompletionHandler: nil];
                }
            } else {
                // 回退到旧的 NSUserNotification API
                let notification_class = Class::get("NSUserNotification");
                if let Some(notification_class) = notification_class {
                    let notification: *mut Object = msg_send![notification_class, new];
                    if !notification.is_null() {
                        let _: () = msg_send![notification, setTitle: title_str];
                        let _: () = msg_send![notification, setInformativeText: message_str];
                        
                        let center_class = Class::get("NSUserNotificationCenter").unwrap();
                        let center: *mut Object = msg_send![center_class, defaultUserNotificationCenter];
                        if !center.is_null() {
                            let _: () = msg_send![center, deliverNotification: notification];
                        }
                    }
                }
            }
            
            let _: () = msg_send![pool, drain];
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(title)
            .text1(message)
            .show()?;
    }
    
    Ok(())
}

pub fn set_notification_sound(sound_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In a real implementation, you would configure the system to use this sound for notifications
    println!("Setting notification sound to: {}", sound_path);
    Ok(())
}