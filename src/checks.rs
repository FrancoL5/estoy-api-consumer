use serde::Deserialize;

pub fn checks_request(token: String, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client.get(url).bearer_auth(token).send()
}
#[derive(Debug, Deserialize, PartialEq)]
pub enum CheckType {
    In,
    Out,
}
#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Check {
    pub id: usize,
    pub r#type: CheckType,
    pub date: String,
    pub picture: String,
    pub hash: Option<String>,
    pub homeOffice: u8,
    pub userLocation: UserLocation,
    pub employeeId: u16,
    pub companyId: u8,
    pub locationId: Option<u16>,
    pub recordType: Option<String>,
    pub cafeteriaType: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub employee: Employee,
    pub longitude: Option<String>,
    pub latitude: Option<String>,
}
#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserLocation {
    pub r#type: String,
    pub coordinates: Vec<f32>,
}
#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Employee {
    pub firstName: String,
    pub lastName: String,
}
