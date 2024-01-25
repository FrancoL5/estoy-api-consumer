

// use api_consumer::ApiConsumer;

pub mod api_consumer;
mod checks;
pub mod consumer;
mod create_report;
mod login;
pub mod parser;
mod util;
// #[derive(Clone)]
// pub type ThreadApiConsumer = Arc<Mutex<ApiConsumer>>;
#[cfg(test)]
mod test {

    // use std::thread;

    // use crate::consumer::Consumer;

    use std::str::FromStr;

    use chrono::NaiveDate;

    use crate::{api_consumer::ApiConsumer, parser::ParsedStruct};

    // use crate::api_consumer::ApiConsumer;

    #[test]
    fn hacer_llamado() {
        // let mut api = ApiConsumer::new().unwrap();
        // let data = api
        //     // .get_parsed_struct(Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()))
        //     .checks_with_date_filter(NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(),NaiveDate::from_ymd_opt(2024, 1, 9).unwrap());
        // println!("{:?}", data);

        let api = ApiConsumer::new();
        let result: Vec<ParsedStruct> = api
            .write_parse_file(
                NaiveDate::from_str("2024-01-23").unwrap(),
                NaiveDate::from_str("2024-01-22").unwrap(),
                "./",
            )
            .unwrap();

        result.iter().for_each(|check| println!("{}", check))
    }
    // let mut consumer = ApiConsumer::new().unwrap();
    // consumer.write_parse_file("./");
    // let parsed_struct = consumer.get_parsed_struct().unwrap();
    // parsed_struct.iter().for_each(|v| println!("{v}"));
    // let checks = consumer.get_unparsed_data().unwrap();

    // checks.iter().for_each(|v| println!("{:?}", v));

    // consumer.create_report().unwrap()
    // }

    // #[test]
    // fn concurrencia() {
    //     let consumer = Consumer::new();
    //     let api1 = consumer.new_agent();
    //     let api2 = consumer.new_agent();

    //     thread::scope(|s| {
    //         s.spawn(move || {
    //             api1.lock().unwrap().set_token("sadasdasd".to_owned());
    //             println!("api1: {api1:?}");
    //         });
    //         s.spawn(move || {
    //             api2.lock().unwrap().get_parsed_struct().unwrap();
    //             println!("api2: {api2:?}");
    //         });
    //     });
    //     println!("consumer: {:?}", consumer.agent);
}
