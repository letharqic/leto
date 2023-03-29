#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::fs::metadata;
use std::time::SystemTime;
use std::process::Command;
use std::path::Path;
use std::path::PathBuf;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_edit_time, add, commit, push, pull, move_to, rename])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn rename(old_path : &str, new_path : &str) {
    _ = fs::rename(old_path, new_path);
}

#[tauri::command]
fn move_to(old_path : &str, new_path : &str) {
    let src = Path::new(old_path);
    let dst = Path::new(new_path);
    if src.is_dir() {
      move_dir_recursive(src, dst).map_err(|err| println!("{:?}", err)).ok();
    } else {
      let path_buf = PathBuf::from(dst);
      let folder = path_buf.parent().unwrap();
      fs::create_dir_all(folder).expect("");
      fs::rename(&src, &dst).expect("");
    }
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
      } else {
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
  let mut command = String::from("cd /d ");
  command.push_str(_path);
  command.push_str(" && git add --all");
  let status = Command::new("cmd")
            .args(["/C", &command])
            .status()
            .expect("failed to execute process");
  return status.to_string();
}

#[tauri::command]
fn commit(_path : &str) -> String {
  let mut command = String::from("cd /d ");
  command.push_str(_path);
  command.push_str(" && git commit -m leto_backup");
  let output = Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process");
  return String::from_utf8(output.stdout).unwrap();
}

#[tauri::command]
fn push(_path : &str) -> String {
  let mut command = String::from("cd /d ");
  command.push_str(_path);
  command.push_str(" && git push");
  let output = Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process");
  return String::from_utf8(output.stderr).unwrap();
}

#[tauri::command]
fn pull(_path : &str) -> String {
  let output = Command::new("cmd")
            .args(["/C", "cd /d E:/Obsidian && git pull"])
            .output()
            .expect("failed to execute process");
  return String::from_utf8(output.stdout).unwrap();
}
