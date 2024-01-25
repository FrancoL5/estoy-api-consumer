use serde::Deserialize;

pub fn checks_request(
    token: String,
    url: &str,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client.get(url).bearer_auth(token).send()
}

pub type Checks = Vec<Check>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub id: usize,
    #[serde(rename = "type")]
    pub check_type: CheckType,
    pub date: String,
    pub home_office: usize,
    pub user_location: String,
    pub employee_id: usize,
    pub company_id: usize,
    pub location_id: usize,
    pub created_at_origin: String,
    pub updated_at_origin: String,
    pub employee: String,
    pub created_at: String,
    pub updated_at: String,
    pub colaborador: usize,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum CheckType {
    In,
    Out,
}
