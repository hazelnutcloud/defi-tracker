#[tokio::main]
async fn main() {
    match tracker_bot::run().await {
        Ok(_) => (),
        Err(e) => eprintln!("an error has occured: {}", e),
    }
}
