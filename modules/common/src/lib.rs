pub mod countries;


#[derive(Debug)]
pub enum DataError {
    ToBeImplemented(String),
    ConfigurationError,
    DatabaseError
}




#[cfg(test)]
mod tests {
    use super::*;
}
