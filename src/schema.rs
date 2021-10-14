table! {
    info_of_location (site_idx) {
        site_idx -> Integer,
        site_id -> Varchar,
        site_pw -> Varchar,
        site_name -> Varchar,
        site_type -> Nullable<Varchar>,
        open_date -> Nullable<Timestamp>,
        regi_date -> Nullable<Timestamp>,
        exe_date -> Nullable<Timestamp>,
        site_system -> Nullable<Varchar>,
        site_name2 -> Nullable<Varchar>,
        site_ip -> Varchar,
        site_ping -> Nullable<Varchar>,
        site_memo -> Nullable<Varchar>,
        site_router -> Nullable<Varchar>,
    }
}

table! {
    info_of_action (idx) {
        idx -> Integer,
        userid -> Varchar,
        flapid -> Varchar,
        action -> Varchar,
        memo -> Nullable<Varchar>,
        insdaytime -> Varchar,
        f_file_ip -> Varchar,
        site_idx -> Integer,
        result -> Nullable<Varchar>,
        reason -> Nullable<Varchar>,
    }
}

table! {
    users (username) {
        useridx -> Integer,
        username -> Varchar,
        userid -> Varchar,
        password -> Varchar,
    }
}

table! {
    admin (admidx) {
        admidx -> Integer,
        admid -> Varchar,
        admpw -> Varchar,
        admname -> Nullable<Varchar>,
        admmemo -> Nullable<Varchar>,
        admregdate -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(info_of_action, users, info_of_location, admin);
