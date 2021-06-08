table! {
    games (id) {
        id -> Uuid,
        time_start -> Timestamp,
        session_id -> Uuid,
    }
}

table! {
    items (id) {
        id -> Uuid,
        expected -> Bool,
        found -> Nullable<Bool>,
        round_id -> Uuid,
    }
}

table! {
    rounds (id) {
        id -> Uuid,
        expression -> Varchar,
        game_id -> Uuid,
        time_end -> Nullable<Timestamp>,
    }
}

table! {
    sessions (id) {
        id -> Uuid,
    }
}

table! {
    variables (id) {
        id -> Uuid,
        name -> Varchar,
        value -> Varchar,
        item_id -> Uuid,
    }
}

joinable!(games -> sessions (session_id));
joinable!(items -> rounds (round_id));
joinable!(rounds -> games (game_id));
joinable!(variables -> items (item_id));

allow_tables_to_appear_in_same_query!(games, items, rounds, sessions, variables,);
