use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, CustomMenuItem};
use tauri_plugin_positioner::{Position, WindowExt};
use std::process::Command;


fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayBottomRight);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_current_running_ports, kill_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn get_current_running_ports() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("lsof -i -P -n | grep LISTEN")
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let open_ports: Vec<&str> = stdout.trim().split('\n').collect();
        let json = serde_json::to_string(&open_ports).expect("Failed to convert to JSON");
        json
    } else {
        serde_json::to_string(&"Failed to get open ports").expect("Failed to convert to JSON")
    }
}

#[tauri::command]
async fn kill_port(pid: u32)-> bool{
    let output = Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        true
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
        false
    }
}