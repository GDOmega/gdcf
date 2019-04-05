use crate::wrap::Wrapped;
use diesel::{
    associations::Identifiable,
    backend::Backend,
    deserialize::FromSqlRow,
    sql_types::{Int8, Nullable, Text},
    ExpressionMethods, Queryable,
};
use gdcf_model::user::Creator;

impl<'a> Identifiable for &'a Wrapped<Creator> {
    type Id = &'a u64;

    fn id(self) -> Self::Id {
        &self.0.user_id
    }
}

diesel_stuff! {
    creator (user_id, Creator) {
        (user_id, Int8, i64, i64),
        (name, Text, String, &'a str),
        (account_id, Nullable<Int8>, Option<i64>, Option<i64>)
    }
}
meta_table!(creator_meta, user_id);

store_simply!(Creator, creator, creator_meta, user_id);
lookup_simply!(Creator, creator, creator_meta, user_id);

fn values(creator: &Creator) -> Values {
    use creator::columns::*;

    (
        user_id.eq(creator.user_id as i64),
        name.eq(&creator.name[..]),
        account_id.eq(creator.account_id.map(|i| i as i64)),
    )
}

impl<DB: Backend> Queryable<SqlType, DB> for Wrapped<Creator>
where
    Row: FromSqlRow<SqlType, DB>,
{
    type Row = Row;

    fn build(row: Self::Row) -> Self {
        Wrapped(Creator {
            user_id: row.0 as u64,
            name: row.1,
            account_id: row.2.map(|i| i as u64),
        })
    }
}
