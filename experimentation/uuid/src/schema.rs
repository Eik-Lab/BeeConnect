// @generated automatically by Diesel CLI.

diesel::table! {
    post_uuids (id) {
        id -> Uuid,
        msg -> Nullable<Varchar>,
    }
}
