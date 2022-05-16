#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use clipboard::{ClipboardContext, ClipboardProvider};
use jwt_simple::prelude::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_jwt, copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn generate_jwt(connected_app_consumer_key: String, org_username: String, pem_file_contents: String) -> String {
    let mut key_pair = RS256KeyPair::from_pem(pem_file_contents.as_str()).unwrap();
    let claims = Claims::create(Duration::from_hours(1))
        .with_issuer(connected_app_consumer_key)
        .with_subject(org_username)
        .with_audience("https://login.salesforce.com");

    let token = key_pair.sign(claims);
    let jwt = token.unwrap();

    return jwt;
}

#[tauri::command]
fn copy_to_clipboard(jwt: String) {
    println!("jwt to copy: {}", jwt.as_str());
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(String::from(jwt.as_str()));
}

