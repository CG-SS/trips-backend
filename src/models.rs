use diesel::prelude::*;
/*
        id -> Integer,
        age -> Integer,
        name -> Text,
        traveler_at -> Nullable<Integer>,
        updated_at -> Integer,
        created_at -> Integer,
 */
#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub age: i32,
    pub name: String,
    pub traveler_at: Option<i32>,
    pub updated_at: i32,
    pub created_at: i32,
}