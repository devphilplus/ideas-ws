pub mod email;
pub mod slug;

// use log::{
//     debug
// };

// use postgres_types::{
//     ToSql,
//     to_sql_checked
// };


// use chrono::prelude::*;


#[derive(Debug)]
pub enum DataError {
    ToBeImplemented(String),
    ConfigurationError,
    DatabaseError
}




// impl ToSql for DateTime<Utc> {
//     fn accepts(ty: &postgres_types::Type) -> bool
//         where
//             Self: Sized {
//         debug!("ToSql: {:?}", ty);
//         return ty.name() == "datetime";
//     }

//     to_sql_checked!();

//     fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
//         where
//             Self: Sized {
//             debug!("{:?}", ty);
//         return self.to_sql(ty, out);
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
