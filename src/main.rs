mod lib;

#[tokio::main]
async fn main() {
    lib::run_opt_listener().await
}
