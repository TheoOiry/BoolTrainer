#![feature(decl_macro, proc_macro_hygiene, trait_alias)]

extern crate serde_derive;
use bt_api::router;
use dotenv::dotenv;
use serde_json::json;
use regex::Regex;
use rocket::local::{Client, LocalResponse, LocalRequest};
use serde_json::Value;
use rocket::http::{Header, ContentType};

fn get_client() -> Client {
    dotenv().ok();

    let rocket = router::get_rocket();
    Client::new(rocket).expect("valid rocket instance")
}

fn get_json_from(url: &str) -> Value {
    let client = get_client();
    let mut response = client.post(url).dispatch();
    serde_json::from_str(&response.body_string().unwrap()).unwrap()
}

#[test]
fn create_session() {
    let json = get_json_from("/api/create_session");
    let re = Regex::new(r"^[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*$").unwrap();
    assert!(re.is_match(json["jwt_token"].as_str().unwrap()));
}

struct ClientSession {
    client: Client,
    session_jwt: String
}

impl ClientSession {
    pub fn new() -> Self {
        let session_jwt =  get_json_from("/api/create_session")["jwt_token"]
            .as_str()
            .unwrap()
            .to_owned();

        ClientSession {
            client: get_client(),
            session_jwt
        }
    }

    pub fn send_with_session(&self, url: &str) -> Value {
        self.get_json_from_response(self.get_request(url).dispatch())
    }

    fn get_request<'a>(&'a self, url: &'a str) -> LocalRequest {
        let mut req = self.client.post(url);
        req.add_header(Header::new("Authorization", self.session_jwt.clone()));
        req
    }

    fn get_json_from_response(&self, mut response: LocalResponse) -> Value {
        serde_json::from_str(&response.body_string().unwrap()).unwrap()
    }

    pub fn send_json_with_session(&self, url: &str, json: Value) -> Value {
        let mut req = self.get_request(url);
        req.set_body(json.to_string());
        req.add_header(ContentType::JSON);
        self.get_json_from_response(req.dispatch())
    }

}

#[test]
fn full_game_scenario() {
    let client = ClientSession::new();
    let create_game_json = client.send_with_session("/api/create_game");
    let game_id = create_game_json["game_id"].as_str().unwrap();
    let mut item_id = create_game_json["first_round"]["items"][0]["item_id"].as_str().unwrap().to_owned();

    for _ in 0..4 {
        let json_response = client.send_json_with_session(
            "/api/answer_round",
            get_json_to_send_response(&item_id, game_id)
        );

        item_id = json_response["next_round"]["items"][0]["item_id"].as_str().unwrap().to_owned();
    }

    let json_response_results = client.send_json_with_session(
        "/api/answer_round",
        get_json_to_send_response(&item_id, game_id)
    );

    assert_eq!(json_response_results["score"]["nb_items"].as_i64().unwrap(), 25);
}

fn get_json_to_send_response(item_id: &str, game_id: &str) -> Value {
    json!({
        "game_id": game_id,
        "answers": [
            { "item_id": item_id, "answer": true }
        ]
    })
}