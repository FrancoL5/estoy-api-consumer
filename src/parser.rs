use std::fmt;

use crate::{
    checks::{Check, CheckType},
    util::{format_datetime_offset, parse_time},
};
#[derive(Debug)]
pub struct ParsedStruct {
    pub numero_interno: u16,
    pub fecha: String,
    pub horario: String,
    pub tipo: String,
    pub sucursal: String,
}

impl ParsedStruct {
    pub fn parse_checks(check: &Check) -> Result<Self, Box<dyn std::error::Error>> {
        let fecha = parse_time(&check.date).unwrap();
        let algo = get_location(check.locationId.unwrap_or(0)).into();
        Ok(ParsedStruct {
            numero_interno: check.employeeId,
            fecha: format_datetime_offset(fecha),
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
            self.numero_interno, self.fecha, self.horario, self.sucursal, self.tipo
        )
    }
}

pub fn get_location(location: u16) -> impl Into<String> {
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
