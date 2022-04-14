table! {
    sensor_info (id) {
        id -> Int8,
        sensor_name -> Varchar,
        sensor_type -> Int4,
        register_date -> Date,
        is_enabled -> Bool,
        sensor_description -> Varchar,
        measurement_unit -> Nullable<Int4>,
        random_serial -> Nullable<Float8>,
        last_check_status -> Nullable<Int4>,
        last_check_status_date -> Nullable<Timestamp>,
    }
}
