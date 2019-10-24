
pub mod types;


pub fn login(username: &str, password: &str) {
    let req = format!("https://bullionvault.com/secure/j_security_check?j_username={}&j_password={}", username, password);
}
