#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use jwt_simple::prelude::*;

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_jwt(connected_app_consumer_key: String, org_username: String, pkcs8_key_path: String) {
    /*
    Change SSL to create a PEM instead of a pkcs8?
     */

    //PEM file needs to be converted to string?
    let pem_file_contents = "";
    let mut key_pair = RS256KeyPair::from_pem(pem_file_contents);
    let claims = Claims::create(Duration::from_hours(1))
        .with_issuer(connected_app_consumer_key)
        .with_subject(org_username)
        .with_audience("https://login.salesforce.com");

    let token = key_pair.sign(claims);

    /*
    Create Signature

    1. Create Private Key
        1. Encode PKCS8 into a PKCS8 Key Spec
        2. Create a private RSA key from the PKCS8 key spec
    2. Create Signature
        1. Create signature with SHA256withRSA algorithm
        2. Sign with RSA Key from step 1
     */
}