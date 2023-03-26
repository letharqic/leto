#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::fs::metadata;
use std::time::SystemTime;
use std::process::Command;
use std::path::Path;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_edit_time, add, commit, push, move_dir])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn move_dir(old_path : &str, new_path : &str) {
    let src = Path::new(old_path);
    let dst = Path::new(new_path);
    move_dir_recursive(src, dst).map_err(|err| println!("{:?}", err)).ok();
}

fn move_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
  if src.is_dir() {
    fs::create_dir_all(dst)?;

    for entry in src.read_dir()? {
      let entry = entry?;
      let src_path = entry.path();
      let dst_path = dst.join(entry.file_name());
      if src_path.is_dir() {
        move_dir_recursive(&src_path, &dst_path)?;
      }
      else {
        fs::rename(&src_path, &dst_path)?;
      }
    }
  } else {
    fs::rename(src, dst)?;
  }

  fs::remove_dir_all(src)?;
  Ok(())
}

#[tauri::command]
fn get_edit_time(path : &str) -> i32 {
  let metadata = metadata(path).expect("Error while trying to get metadata");
  let time_modified = metadata.modified().expect("Error while parcing metadata");
  let since_the_epoch = time_modified.duration_since(SystemTime::UNIX_EPOCH).expect("Error while parsing duration");
  since_the_epoch.as_secs() as i32
}

#[tauri::command]
fn add(_path : &str) -> String {
  let status = Command::new("cmd")
            .args(["/C", "cd /d E:/Obsidian && git add --all"])
            .status()
            .expect("failed to execute process");
  return status.to_string();
}

#[tauri::command]
fn commit(_path : &str) -> String {
  let output = Command::new("cmd")
            .args(["/C", "cd /d E:/Obsidian && git commit -m leto_backup"])
            .output()
            .expect("failed to execute process");
  return String::from_utf8(output.stdout).unwrap();
}

#[tauri::command]
fn push(_path : &str) -> String {
  let output = Command::new("cmd")
            .args(["/C", "cd /d E:/Obsidian && git push"])
            .output()
            .expect("failed to execute process");
          //return String::from_utf8(output.stdout).unwrap();
          //return std::str::from_utf8(output.stdout).unwrap().to_string();
  return String::from_utf8(output.stderr).unwrap();
}
