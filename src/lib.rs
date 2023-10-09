mod api_consumer;
mod checks;
pub mod consumer;
mod create_report;
mod login;
pub mod parser;
mod util;

// #[derive(Clone)]

#[cfg(test)]
mod test {

    use std::thread;

    use crate::consumer::Consumer;

    // #[test]
    // fn hacer_llamado() {
    // let mut consumer = ApiConsumer::new().unwrap();
    // consumer.write_parse_file("./");
    // let parsed_struct = consumer.get_parsed_struct().unwrap();
    // parsed_struct.iter().for_each(|v| println!("{v}"));
    // let checks = consumer.get_unparsed_data().unwrap();

    // checks.iter().for_each(|v| println!("{:?}", v));

    // consumer.create_report().unwrap()
    // }

    #[test]
    fn concurrencia() {
        let consumer = Consumer::new();
        let api1 = consumer.new_agent();
        let api2 = consumer.new_agent();

        thread::scope(|s| {
            s.spawn(move || {
                api1.lock().unwrap().set_token("sadasdasd".to_owned());
                println!("api1: {api1:?}");
            });
            s.spawn(move || {
                api2.lock().unwrap().get_parsed_struct().unwrap();
                println!("api2: {api2:?}");
            });
        });
        println!("consumer: {:?}", consumer.agent);
    }
}
