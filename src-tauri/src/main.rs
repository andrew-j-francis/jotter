#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use jwt_simple::prelude::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_jwt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn generate_jwt(connected_app_consumer_key: String, org_username: String, pem_file_contents: String) {
    /*
    Change SSL to create a PEM instead of a pkcs8?
     */

    println!("Consumer Key: {}", connected_app_consumer_key);
    println!("Org Username: {}", org_username);
    println!("PEM File: {}", pem_file_contents);

    let mut key_pair = RS256KeyPair::from_pem(pem_file_contents.as_str()).unwrap();
    let claims = Claims::create(Duration::from_hours(1))
        .with_issuer(connected_app_consumer_key)
        .with_subject(org_username)
        .with_audience("https://login.salesforce.com");

    let token = key_pair.sign(claims);

    println!("JWT: {}", &token.unwrap());
}