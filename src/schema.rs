// @generated automatically by Diesel CLI.

diesel::table! {
    PeriodStatus (id) {
        id -> Integer,
        #[max_length = 255]
        status_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    PeriodType (id) {
        id -> Integer,
        #[max_length = 255]
        type_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    TreeType (id) {
        id -> Integer,
        #[max_length = 255]
        type_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    trae_type (id) {
        id -> Integer,
        #[max_length = 255]
        type_name -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    PeriodStatus,
    PeriodType,
    TreeType,
    trae_type,
);
