use serde::Deserialize;

#[allow(non_camel_case_types)]
pub type null = ();

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub token: String,
    pub expiresAt: String,
    pub user: UserResponse,
    pub company: usize,
    pub settings: SettingsResponse,
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