use serde::Deserialize;

use crate::{get_token, ApiConsumer};

pub fn get_checks(
    consumer: &mut ApiConsumer,
    url: &str,
) -> Result<Vec<Check>, Box<dyn std::error::Error>> {
    let response = checks_request(consumer.get_token().to_owned(), url);
    let checks: Vec<Check> = match response {
        Ok(v) => v.json::<Vec<Check>>()?,
        Err(err) => {
            if err.status().unwrap() == 401 {
                let new_token = get_token()?.token;

                consumer.set_token(new_token);

                let new_response = checks_request(consumer.get_token().to_owned(), url)?;

                new_response.json::<Vec<Check>>()?
            } else {
                panic!("unexpected error when trying to get new token")
            }
        }
    };

    Ok(checks)
}
fn checks_request(token: String, url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client.get(url).bearer_auth(token).send()
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
