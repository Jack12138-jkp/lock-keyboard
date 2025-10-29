use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use rdev::{grab, Event, EventType};
use tauri::{
    AppHandle,
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};

#[cfg(target_os = "macos")]
use std::process::Command;

const TRAY_ID: &str = "lockkeyboard-tray";
const MENU_LOCK_ID: &str = "lock-keyboard";
const MENU_UNLOCK_ID: &str = "unlock-keyboard";
const MENU_QUIT_ID: &str = "quit-app";

#[cfg(target_os = "macos")]
fn check_and_request_accessibility_permission() -> bool {
    let check_script = r#"
        tell application "System Events"
            return get UI elements enabled
        end tell
    "#;
    
    if let Ok(result) = Command::new("osascript").arg("-e").arg(check_script).output() {
        if String::from_utf8_lossy(&result.stdout).trim().eq_ignore_ascii_case("true") {
            return true;
        }
    }
    
    // 打开系统偏好设置
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn();
    
    false
}

#[cfg(not(target_os = "macos"))]
fn check_and_request_accessibility_permission() -> bool {
    true
}

#[derive(Clone)]
struct KeyboardState {
    locked: Arc<AtomicBool>,
}

impl KeyboardState {
    fn new() -> Self {
        Self {
            locked: Arc::new(AtomicBool::new(false)),
        }
    }

    fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }

    fn set_locked(&self, locked: bool) {
        self.locked.store(locked, Ordering::SeqCst);
    }
}

fn spawn_keyboard_grabber(lock_flag: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        loop {
            let flag_clone = Arc::clone(&lock_flag);
            
            let result: std::result::Result<(), rdev::GrabError> = grab(move |event: Event| {
                if flag_clone.load(Ordering::SeqCst)
                    && matches!(event.event_type, EventType::KeyPress(_) | EventType::KeyRelease(_))
                {
                    return None;
                }
                Some(event)
            });

            if let Err(err) = result {
                eprintln!("键盘监听异常: {err:?}，1秒后重启");
            }
            
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn update_tray_ui(app: &AppHandle, locked: bool) -> tauri::Result<()> {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let tooltip = if locked { "🔒 键盘已锁定" } else { "🔓 键盘已解锁" };
        tray.set_tooltip(Some(tooltip))?;
    }
    Ok(())
}

fn handle_menu_action(app: &AppHandle, action: &str) {
    let state = app.state::<KeyboardState>();
    match action {
        MENU_LOCK_ID => {
            if !state.is_locked() {
                state.set_locked(true);
                let _ = update_tray_ui(app, true);
            }
        }
        MENU_UNLOCK_ID => {
            if state.is_locked() {
                state.set_locked(false);
                let _ = update_tray_ui(app, false);
            }
        }
        MENU_QUIT_ID => {
            app.exit(0);
        }
        _ => {}
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    check_and_request_accessibility_permission();
    
    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            
            let keyboard_state = KeyboardState::new();
            spawn_keyboard_grabber(Arc::clone(&keyboard_state.locked));

            let lock_item = MenuItemBuilder::with_id(MENU_LOCK_ID, "🔒 锁定键盘").build(app)?;
            let unlock_item = MenuItemBuilder::with_id(MENU_UNLOCK_ID, "🔓 解锁键盘").build(app)?;
            let quit_item = MenuItemBuilder::with_id(MENU_QUIT_ID, "❌ 退出").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&lock_item, &unlock_item, &quit_item])
                .build()?;

            TrayIconBuilder::with_id(TRAY_ID)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("键盘锁定工具")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    handle_menu_action(app, event.id().as_ref());
                })
                .build(app)?;

            app.manage(keyboard_state);
            update_tray_ui(app.handle(), false)?;

            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
