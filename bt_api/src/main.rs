use bt_api::router;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    router::get_rocket().launch();
}
