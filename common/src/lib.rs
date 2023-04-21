pub mod currency;
pub mod country;
pub mod user;
// pub mod client;
pub mod tenant;
pub mod hr;

#[derive(Debug)]
pub enum Error {
    ToBeImplemented(String),
    ConfigurationError(String),
    ValidationError(String)
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
