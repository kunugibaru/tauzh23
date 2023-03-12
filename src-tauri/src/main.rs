// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pinyin::{ToPinyin, ToPinyinMulti};
use chinese_dictionary::query;


fn pn(txt: &str, separator: &str) -> String {
    txt
        .to_pinyin()
        .filter_map(|x| x.to_owned())
        .map(|pn| pn.with_tone().to_owned())
        .collect::<Vec<String>>()
        .join(separator)
        .to_owned()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, separator: &str) -> String {
    format!("{}", pn(name, separator))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
