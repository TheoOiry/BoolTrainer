use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use uuid;

use super::session::Session;
use crate::app::model::round::Round;
use crate::schema::games;

#[derive(Identifiable, Queryable, Insertable, Associations, PartialEq)]
#[belongs_to(Session)]
pub struct Game {
    id: uuid::Uuid,
    time_start: NaiveDateTime,
    session_id: uuid::Uuid,
}

impl Game {
    pub fn insert(session: &Session, conn: &PgConnection) -> QueryResult<Game> {
        let game = Game {
            id: uuid::Uuid::new_v4(),
            time_start: Utc::now().naive_utc(),
            session_id: session.get_id(),
        };

        diesel::insert_into(games::table)
            .values(&game)
            .get_result(conn)
    }

    pub fn get(id: uuid::Uuid, conn: &PgConnection) -> QueryResult<Game> {
        games::table.find(id).get_result::<Game>(conn)
    }

    pub fn is_end(&self, conn: &PgConnection) -> bool {
        let rounds = self.get_rounds(conn).unwrap();
        return rounds.len() == 5 && rounds.iter().all(|round| round.get_time_end().is_some());
    }

    pub fn get_rounds(&self, conn: &PgConnection) -> Result<Vec<Round>, Error> {
        Round::belonging_to(self).load::<Round>(conn)
    }

    pub fn set_last_round_time_now(&self, conn: &PgConnection) {
        if let Some(round) = self.get_last_round(conn) {
            round.set_time_end(Utc::now().naive_utc(), conn);
        }
    }

    pub fn get_last_round(&self, conn: &PgConnection) -> Option<Round> {
        let mut rounds = self.get_rounds(conn).unwrap();

        return if let Some(round) = rounds.iter().find(|round| round.get_time_end().is_none()) {
            Some(round.clone())
        } else {
            rounds.sort_by(|r1, r2| {
                r1.get_time_end()
                    .unwrap()
                    .partial_cmp(&r2.get_time_end().unwrap())
                    .unwrap()
            });
            rounds.into_iter().last()
        };
    }

    pub fn get_json_result() {}

    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn get_time_start(&self) -> NaiveDateTime {
        self.time_start
    }
}
