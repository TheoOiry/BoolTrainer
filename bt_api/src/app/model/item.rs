use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use rocket_contrib::json::JsonValue;
use uuid;

use super::round::Round;
use super::variable::Variable;
use crate::schema::items;

#[derive(Identifiable, Queryable, Insertable, Associations, Eq, PartialEq, Hash, Clone, Copy)]
#[belongs_to(Round)]
pub struct Item {
    id: uuid::Uuid,
    expected: bool,
    found: Option<bool>,
    round_id: uuid::Uuid,
}

impl Item {
    pub fn insert(round: &Round, expected: bool, conn: &PgConnection) -> QueryResult<Item> {
        let item = Item {
            id: uuid::Uuid::new_v4(),
            expected,
            found: None,
            round_id: round.get_id(),
        };

        diesel::insert_into(items::table)
            .values(&item)
            .get_result(conn)
    }

    pub fn get(id: uuid::Uuid, conn: &PgConnection) -> QueryResult<Item> {
        items::table.find(id).get_result::<Item>(conn)
    }

    pub fn set_answer(&self, answer: bool, conn: &PgConnection) {
        if self.found.is_none() {
            diesel::update(self)
                .set(items::found.eq(answer))
                .get_result::<Item>(conn)
                .ok();
        }
    }

    pub fn json_hidden_expected(&self, variables: &[Variable]) -> JsonValue {
        json!({
            "item_id": self.id,
            "variables": variables.iter().map(|variable| variable.to_json()).collect::<Vec<JsonValue>>()
        })
    }

    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn get_expected(&self) -> bool {
        self.expected
    }

    pub fn get_found(&self) -> Option<bool> {
        self.found
    }

    pub fn get_round_id(&self) -> uuid::Uuid {
        self.round_id
    }
}
