use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failes to read configuration");
    let addess = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(addess).await.unwrap();
    run(listener).await;
}
