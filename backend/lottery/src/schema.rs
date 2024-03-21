// @generated automatically by Diesel CLI.

diesel::table! {
    problem (id) {
        id -> Bigint,
        #[max_length = 1024]
        stem -> Varchar,
        #[max_length = 30]
        problem_type -> Varchar,
        #[max_length = 1024]
        option_a -> Nullable<Varchar>,
        #[max_length = 1024]
        option_b -> Nullable<Varchar>,
        #[max_length = 1024]
        option_c -> Nullable<Varchar>,
        #[max_length = 1024]
        option_d -> Nullable<Varchar>,
        #[max_length = 1024]
        option_e -> Nullable<Varchar>,
        #[max_length = 1024]
        option_f -> Nullable<Varchar>,
        #[max_length = 32]
        answer -> Varchar,
        status -> Integer,
        #[max_length = 1]
        is_delete -> Char,
        #[max_length = 64]
        create_by -> Varchar,
        create_time -> Datetime,
        #[max_length = 64]
        update_by -> Varchar,
        update_time -> Datetime,
    }
}
