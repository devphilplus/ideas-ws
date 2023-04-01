use postgres_types::{
    ToSql,
    to_sql_checked
};

#[derive(Debug)]
pub struct Slug(String);

impl Slug {
    pub fn new(slug: &str) -> Self {
        return Self(String::from(slug));
    }
}

impl ToSql for Slug {
    fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
        where
            Self: Sized {
        return self.0.to_sql(ty, out);
    }

    to_sql_checked!();

    fn accepts(ty: &postgres_types::Type) -> bool
        where
            Self: Sized {
        return ty.name() == "slug_text";
    }
}