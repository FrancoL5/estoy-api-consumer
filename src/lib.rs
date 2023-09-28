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
pub fn get_parsed_struct() -> Option<Vec<ParsedStruct>> {
    match parse() {
        Ok(v) => Some(v),
        Err(err) => {
            File::write_to(&*err.to_string(), "./log.txt", true).unwrap();
            None
        }
    }
}

pub fn write_parse_file(dir: &str) -> bool {
    match parse() {
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
#[cfg(test)]
mod test {

    use crate::write_parse_file;

    #[test]
    fn hacer_llamado() {
        write_parse_file(".*//");
    }
}
