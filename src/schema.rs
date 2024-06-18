// @generated automatically by Diesel CLI.

diesel::table! {
    captures (id) {
        id -> Int4,
        uuid -> Uuid,
        url -> Text,
        time -> Timestamp,
        owner -> Int4,
        public -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        passhash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    captures,
    users,
);
