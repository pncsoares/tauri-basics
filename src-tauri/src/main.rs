#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit));
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

fn main() {

    tauri::Builder::default()
    .setup(|app| {
      
      let id = app.listen_global("event-name", |event| {
        println!("got event-name {:?}", event.payload());
      });

      app.unlisten(id);

      app.emit_all("event-name", Payload {
        message: "Tauri rocks!".into()
      }).unwrap();

      Ok(())

    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn say_hi() -> String {
  "Hello world!".into()
}