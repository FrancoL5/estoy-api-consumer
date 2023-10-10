use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, FixedOffset};

use crate::{
    checks::{self, Check, CheckType},
    parser::get_location,
    util,
};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct EmployeeChecks {
    employeeId: u16,
    fecha: DateTime<FixedOffset>,
    entrada: Vec<String>,
    salida: Vec<String>,
    sucursal: String,
    nombre: String,
}

impl Display for EmployeeChecks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut salida_iter = self.salida.iter();
        let mut result = String::new();
        let default = &"-".to_owned();
        for hora in self.entrada.iter() {
            let entrada = hora;
            let salida = salida_iter.next().unwrap_or(default);
            result.push_str(&format!(
                "{},{},{},{},{},{}\n",
                self.employeeId,
                self.nombre,
                self.sucursal,
                util::format_datetime_offset(self.fecha),
                entrada,
                salida
            ))
        }
        write!(f, "{}", result)
    }
}

pub fn generate_report(
    checks: Vec<checks::Check>,
) -> HashMap<String, HashMap<u16, EmployeeChecks>> {
    let mut map: HashMap<String, HashMap<u16, EmployeeChecks>> = HashMap::with_capacity(4);

    for check in &checks[..] {
        // println!("{check:?}");
        let date = util::format_datetime_offset(util::parse_time(&check.date).unwrap());
        match map.get_mut(&date) {
            Some(employees) => match employees.get_mut(&check.employeeId) {
                Some(employee) => {
                    let hora = util::parse_to_time(util::parse_time(&check.date).unwrap());
                    if check.r#type == CheckType::In {
                        employee.entrada.push(hora)
                    } else {
                        employee.salida.push(hora)
                    }
                }
                None => {
                    employees.insert(check.employeeId, calculate_check(check));
                }
            },
            None => {
                let fecha = util::format_datetime_offset(util::parse_time(&check.date).unwrap());
                map.insert(
                    fecha,
                    HashMap::from([(check.employeeId, calculate_check(check))]),
                );
            }
        };
    }
    map
}
fn calculate_check(check: &Check) -> EmployeeChecks {
    let fecha = util::parse_time(&check.date).unwrap();
    let nombre = format!("{} {}", check.employee.firstName, check.employee.lastName);
    let mut entrada: Vec<String> = Vec::with_capacity(4);
    let mut salida: Vec<String> = Vec::with_capacity(4);
    let hora = util::parse_to_time(util::parse_time(&check.date).unwrap());
    if check.r#type == CheckType::In {
        entrada.push(hora);
    } else {
        salida.push(hora)
    };

    EmployeeChecks {
        employeeId: check.employeeId,
        fecha,
        entrada,
        salida,
        sucursal: get_location(check.locationId.unwrap_or(0)).into(),
        nombre,
    }
}
// Deberia ser  un HashMap de este orden HashMap<fecha, HashMap<employeeId, EmployeeChecks>>
// aunque creo que seria relativamente lento pero es la solución mas limpia
