// @generated automatically by Diesel CLI.

diesel::table! {
    hive_infos (hive_id) {
        hive_id -> Int4,
        info -> Text,
        longitude -> Float4,
        altitude -> Float4,
    }
}

diesel::table! {
    measurements (id) {
        id -> Uuid,
        hive_id -> Int4,
        layer -> Int4,
        time -> Timestamptz,
        weight -> Nullable<Float4>,
        temp -> Array<Nullable<Float4>>,
    }
}

diesel::joinable!(measurements -> hive_infos (hive_id));

diesel::allow_tables_to_appear_in_same_query!(
    hive_infos,
    measurements,
);
