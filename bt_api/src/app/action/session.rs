use crate::app::helper::jwt::jwt_encode;
use crate::app::model::session::Session;

#[derive(Serialize, Deserialize)]
pub struct SessionClaims {
    session_id: uuid::Uuid,
    exp: usize,
}

impl SessionClaims {
    pub fn new(session: &Session) -> Self {
        SessionClaims {
            session_id: session.get_id(),
            exp: 10000000000,
        }
    }

    pub fn get_id(&self) -> uuid::Uuid {
        self.session_id
    }

    pub fn get_jwt_token(&self) -> String {
        jwt_encode(self)
    }
}
