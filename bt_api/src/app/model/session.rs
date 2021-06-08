use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Outcome, Request};
use uuid;

use crate::app::action::session::SessionClaims;
use crate::app::helper::jwt::jwt_decode;
use crate::connection::DbConn;
use crate::schema::sessions;

#[derive(Identifiable, Queryable, Insertable)]
pub struct Session {
    id: uuid::Uuid,
}

impl Session {
    pub fn insert(conn: &PgConnection) -> QueryResult<Session> {
        let session = Session {
            id: uuid::Uuid::new_v4(),
        };
        diesel::insert_into(sessions::table)
            .values(&session)
            .get_result(conn)
    }

    pub fn get(uuid: uuid::Uuid, conn: &PgConnection) -> QueryResult<Session> {
        sessions::table.find(uuid).get_result::<Session>(conn)
    }

    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Session, Self::Error> {
        let conn = request.guard::<DbConn>()?.0;
        let session = request
            .headers()
            .get_one("Authorization")
            .and_then(|token| {
                jwt_decode::<SessionClaims>(token)
                    .ok()
                    .and_then(|session_claims| Session::get(session_claims.get_id(), &conn).ok())
            });

        match session {
            Some(sess) => Outcome::Success(sess),
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
