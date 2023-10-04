use chrono::{DateTime, FixedOffset};

const TRES_HORAS: u16 = 3 * 3600;

pub fn parse_time(date: &str) -> Result<DateTime<FixedOffset>, Box<dyn std::error::Error>> {
    Ok(chrono::DateTime::parse_from_str(date, "%+")?
        .with_timezone(&FixedOffset::west_opt(TRES_HORAS.into()).unwrap()))
}
pub fn format_datetime_offset(date: DateTime<FixedOffset>) -> String {
    date.format("%d/%m/%Y").to_string()
}
pub fn parse_to_time(date: DateTime<FixedOffset>) -> String {
    date.time().format("%H:%M").to_string()
}
