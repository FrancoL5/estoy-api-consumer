use serde::Deserialize;

pub fn get_checks(token: String, url: &str) -> Result<Vec<Check>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let checks = client
        .get(url)
        .bearer_auth(token)
        .send()?
        .json::<Vec<Check>>()?;

    Ok(checks)
}

#[derive(Debug, Deserialize)]
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
    pub locationId: u16,
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
