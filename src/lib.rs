use std::collections::HashMap;

mod checks;
mod login;
mod parser;

use crate::login::LoginResponse;
use crate::parser::parse;
use crate::parser::ParsedStruct;
use ::parser::files::File;

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

pub struct ApiConsumer {
    token: String,
}
impl ApiConsumer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ApiConsumer {
            token: get_token()?.token,
        })
    }
    pub fn set_token(&mut self, token: String) {
        self.token = token
    }

    pub fn get_token(&mut self) -> String {
        self.token.clone()
    }
    pub fn get_parsed_struct(&mut self) -> Option<Vec<ParsedStruct>> {
        match parse(self) {
            Ok(v) => Some(v),
            Err(err) => {
                File::write_to(&*err.to_string(), "./log.txt", true).unwrap();
                None
            }
        }
    }

    pub fn write_parse_file(&mut self, dir: &str) -> bool {
        match parse(self) {
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
}

#[cfg(test)]
mod test {

    use crate::ApiConsumer;

    #[test]
    fn hacer_llamado() {
        let mut consumer = ApiConsumer::new().unwrap();
        consumer.write_parse_file("./");
    }
}
