use crate::utils::lib::time_stamp;
use ring::hmac;
use std::str;
// mod utils {
//     pub mod lib;
// }

// SHA512 HASH
fn hex_signature(secret_key: &str, payload: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA512, secret_key.as_bytes());
    let signature = hmac::sign(&key, payload.as_bytes());
    signature
        .as_ref()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

// Create Gateio signature
pub fn generate_signature(method: &str, url: &str, query_string: &str, secret: &str) -> String {
    let format_str = format!(
        "{}\n{}\n{}\n{}\n{}",
        method,
        url,
        query_string,
        "",
        time_stamp()
    );
    hex_signature(&secret, &format_str[..])
}
