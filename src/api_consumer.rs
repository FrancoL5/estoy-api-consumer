use std::{collections::HashMap, thread};

use parser::files::File;
fn get_token() -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let mut body_json = HashMap::new();

    body_json.insert("id", "404745");
    body_json.insert("password", "DiMe2022%");
    body_json.insert("company", "404745");

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.estoy.com.ar/admin/login")
        .json(&body_json)
        .send()?
        .json::<LoginResponse>()?;

    Ok(response)
}

fn generate_log(err: Box<dyn std::error::Error>) {
    let data = format!(
        "------------API------------\n
fecha: {}\n
error: {}\n
---------------------------",
        chrono::offset::Local::now(),
        err.to_string()
    );
    File::write_to(&*data, "./log.txt", true).unwrap();
}

use crate::{
    checks::{self, checks_request, Check},
    create_report::generate_report,
    login::LoginResponse,
    parser::ParsedStruct,
};

#[derive(Debug)]
pub struct ApiConsumer {
    token: String,
}
impl ApiConsumer {
    /// Creates the ApiConsumer struct and asings a token
    /// can fail when requesting the token  
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ApiConsumer {
            token: get_token()?.token,
        })
    }
     fn set_token(&mut self, token: String) {
        self.token = token
    }

     fn get_token(&mut self) -> String {
        self.token.clone()
    }
    /// Returns the struct representing the parsed data
    pub fn get_parsed_struct(&mut self) -> Option<Vec<ParsedStruct>> {
        match self.parse() {
            Ok(v) => Some(v),
            Err(err) => {
                println!("error |{}|", err);
                generate_log(err);
                None
            }
        }
    }
    fn parse(&mut self) -> Result<Vec<ParsedStruct>, Box<dyn std::error::Error>> {
        let url = "https://api.estoy.com.ar/admin/company/404745/check?";
        let param = "offset=0&limit=300&orderBy=createdAt&order=desc&tz=-180";
        let checks = self.get_checks(&format!("{}{}", url, param)).unwrap();

        let parsed = checks
            .iter()
            .map(|v: &Check| ParsedStruct::parse_checks(v).unwrap())
            .collect();
        Ok(parsed)
    }

    fn get_checks(&mut self, url: &str) -> Result<Vec<Check>, Box<dyn std::error::Error>> {
        let mut checks: Vec<Check> = vec![];
        thread::scope(|s| {
            s.spawn(|| {
                let response = checks_request(self.get_token(), url);
                checks = match response {
                    Ok(v) => {
                        if v.status() == 401 {
                            println!("got token");
                            let new_token = get_token().unwrap().token;

                            self.set_token(new_token);

                            let new_response = checks_request(self.get_token(), url).unwrap();

                            new_response.json::<Vec<Check>>().unwrap()
                        } else {
                            v.json::<Vec<Check>>().unwrap()
                        }
                    }
                    Err(err) => {
                        generate_log(Box::new(err));
                        panic!("unexpected error")
                    }
                };
            });
        });

        Ok(checks)
    }
    /// Tries to create a file with the direction you provide
    ///
    /// Returns true if succeed or false if fail
    ///
    /// Automatically creates a log if fails
    pub fn write_parse_file(&mut self, dir: &str) -> bool {
        match self.parse() {
            Ok(values) => {
                let result: String = values.iter().fold("".into(), |acc, v| format!("{acc}{v}"));
                match File::write_to(&*result, &*format!("{dir}result.txt"), false) {
                    Ok(()) => true,
                    Err(err) => {
                        generate_log(Box::new(err));
                        false
                    }
                }
            }
            Err(err) => {
                generate_log(err);
                false
            }
        }
    }
    pub fn get_unparsed_data(
        &mut self,
        limit: u32,
    ) -> Result<Vec<checks::Check>, Box<dyn std::error::Error>> {
        let url = "https://api.estoy.com.ar/admin/company/404745/check?";
        let param = format!(
            "offset=0&limit={}&orderBy=createdAt&order=desc&tz=-180",
            limit
        );
        self.get_checks(&format!("{}{}", url, param))
    }

    pub fn create_report(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = generate_report(self.get_unparsed_data(300)?);
        let mut export = String::from("legajo,nombre,sucursal,fecha,entrada,salida\n");
        for (_, map) in result.iter() {
            for (_, k) in map.iter() {
                export.push_str(&k.to_string());
            }
            File::write_to(&*export, "export.csv", false)?;
        }
        Ok(())
    }
}