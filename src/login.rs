use serde::Deserialize;

#[allow(non_camel_case_types)]
pub type null = ();

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    userId: usize,
    username: String,
    roles:Vec<String>
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserResponse {
    id: usize,
    nationalId: String,
    firstName: String,
    lastName: String,
    email: String,
    superadmin: bool,
    financeAdmin: bool,
}
#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct SettingsResponse {
    logo: String,
    mainColor: String,
    accentColor: null,
    language: String,
    homeOfficeMode: String,
    financeStatus: String,
}