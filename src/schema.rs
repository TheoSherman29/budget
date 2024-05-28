// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Integer,
        date -> Text,
        tpe -> Text,
        amount -> Float,
        source -> Nullable<Text>,
        destination -> Nullable<Text>,
        category -> Text,
        description -> Nullable<Text>,
        earmark -> Nullable<Text>,
    }
}
