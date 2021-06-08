use super::jwt;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    data: String,
}

#[test]
fn jwt_encode_decode() {
    dotenv().ok();
    let base_claims = Claims {
        exp: 10000000000,
        data: String::from("some data"),
    };

    let decoded_claims: Claims = jwt::jwt_decode(&jwt::jwt_encode(&base_claims)).unwrap();

    assert_eq!(base_claims.data, decoded_claims.data)
}
