pub mod data;
pub mod employees;

use log::{
    info,
    debug,
    error
};

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[derive(Debug)]
pub enum HrError {
    ToBeImplemented(String),
    ConfigurationError,
    ValidationError
}

#[derive(Clone)]
pub struct Hr {
    employees: employees::Employees
}

impl Hr {

    pub fn new(
        cfg: configuration::ApplicationConfiguration,
        people: people::people::People
    ) -> Result<Self, HrError> {
        let mut employees: Option<crate::employees::Employees> = None;
        if let Ok(emp) = crate::employees::Employees::new(cfg, people) {
            employees = Some(emp);
        }

        if employees.is_some() {
            return Ok(Self {
                employees: employees.unwrap()
            });
        } else {
            return Err(HrError::ConfigurationError);
        }
    }

    pub fn employees(&self) -> crate::employees::Employees {
        return self.employees.clone();
    }
}