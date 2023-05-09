pub mod data;
pub mod employees;

use log::{
    info,
    debug,
    error
};

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
        people: people::people::People,
        data: ::data::Data
    ) -> Self {
        // let mut employees: Option<crate::employees::Employees> = None;
        // if let Ok(emp) = crate::employees::Employees::new(cfg, people) {
        //     employees = Some(emp);
        // }

        // if employees.is_some() {
        //     return Ok(Self {
        //         employees: employees.unwrap()
        //     });
        // } else {
        //     return Err(HrError::ConfigurationError);
        // }

        return Self {
            employees: crate::employees::Employees::new(
                cfg,
                people,
                data
            )
        };
    }

    pub fn employees(&self) -> crate::employees::Employees {
        return self.employees.clone();
    }
}