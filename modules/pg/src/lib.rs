use postgres_types::{
    ToSql,
    to_sql_checked
};


#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Self {
        return Self(String::from(email));
    }
}

impl ToSql for Email {
    fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
        where
            Self: Sized {
        return self.0.to_sql(ty, out);
    }

    to_sql_checked!();

    fn accepts(ty: &postgres_types::Type) -> bool
        where
            Self: Sized {
        return ty.name() == "email_address";
    }
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
