use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use uuid;

use super::game::Game;
use crate::schema::rounds;

#[derive(Identifiable, Queryable, Insertable, Associations, Clone, PartialEq, Hash, Eq)]
#[belongs_to(Game)]
pub struct Round {
    id: uuid::Uuid,
    expression: String,
    game_id: uuid::Uuid,
    time_end: Option<NaiveDateTime>,
}

impl Round {
    pub fn insert(game: &Game, expression: String, conn: &PgConnection) -> QueryResult<Round> {
        let round = Round {
            id: uuid::Uuid::new_v4(),
            expression,
            time_end: None,
            game_id: game.get_id(),
        };

        diesel::insert_into(rounds::table)
            .values(&round)
            .get_result(conn)
    }

    pub fn get(id: uuid::Uuid, conn: &PgConnection) -> QueryResult<Round> {
        rounds::table.find(id).get_result::<Round>(conn)
    }

    pub fn delete(&self, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(self).execute(conn)
    }

    pub fn set_time_end(&self, time_end: NaiveDateTime, conn: &PgConnection) {
        if self.time_end.is_none() {
            diesel::update(self)
                .set(rounds::time_end.eq(time_end))
                .get_result::<Round>(conn)
                .ok();
        }
    }

    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn get_expression(&self) -> &str {
        &self.expression
    }

    pub fn get_time_end(&self) -> Option<NaiveDateTime> {
        self.time_end
    }
}
