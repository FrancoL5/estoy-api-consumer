use std::fmt;

use crate::{
    checks::{get_checks, Check, CheckType}, ApiConsumer,
};
use chrono::FixedOffset;

// with_timezone(&FixedOffset::west_opt(TRES_HORAS.into()).unwrap()
const TRES_HORAS: u16 = 3 * 3600;

pub fn parse(consumer: &mut ApiConsumer) -> Result<Vec<ParsedStruct>, Box<dyn std::error::Error>> {
    let url = "https://api.estoy.com.ar/admin/company/404745/check?";
    let param = "offset=0&limit=100&orderBy=createdAt&order=desc&tz=-180";
    let checks = get_checks(consumer, &format!("{}{}", url, param))?;

    let parsed: Vec<ParsedStruct> = checks
        .iter()
        .map(|v: &Check| ParsedStruct::parse_checks(v).unwrap())
        .collect();

    Ok(parsed)
}

pub struct ParsedStruct {
    pub numero_interno: u16,
    pub fecha: String,
    pub horario: String,
    pub tipo: String,
    pub sucursal: String,
}

impl ParsedStruct {
    pub fn parse_checks(check: &Check) -> Result<Self, Box<dyn std::error::Error>> {
        let fecha = chrono::DateTime::parse_from_str(&check.date, "%+")?
            .with_timezone(&FixedOffset::west_opt(TRES_HORAS.into()).unwrap());
        let algo = get_location(check.locationId).into();
        Ok(ParsedStruct {
            numero_interno: check.employeeId,
            fecha: fecha.format("%d/%m/%Y").to_string(),
            horario: fecha.time().format("%H:%M").to_string(),
            tipo: match check.r#type {
                CheckType::In => "E".into(),
                CheckType::Out => "S".into(),
            },
            sucursal: algo,
        })
    }
}

impl fmt::Display for ParsedStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}\n",
            self.numero_interno, self.fecha, self.horario, self.tipo, self.sucursal
        )
    }
}

fn get_location(location: u16) -> impl Into<String>{
    match location {
        1727 => "010",
        1730 => "009",
        1731 => "011",
        1729 => "012",
        1728 => "008",
        1726 => "002",
        1725 => "004",
        1723 => "001",
        _ => "013",
    }
}
