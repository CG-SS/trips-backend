// @generated automatically by Diesel CLI.

diesel::table! {
    peoples (id) {
        id -> Integer,
        age -> Integer,
        name -> Text,
        traveler_at -> Nullable<Integer>,
        updated_at -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    trip_locations (id) {
        id -> Integer,
        location_of -> Integer,
        name -> Text,
        weather -> Nullable<Integer>,
        updated_at -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    trips (id) {
        id -> Integer,
        travelers -> Nullable<Text>,
        schedule -> Nullable<Text>,
        start_date -> Integer,
        finish_date -> Integer,
        updated_at -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    weather (id) {
        id -> Integer,
        weather_for -> Integer,
        avg_temp -> Float,
        rain_chance -> Float,
        created_at -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    peoples,
    trip_locations,
    trips,
    weather,
);
