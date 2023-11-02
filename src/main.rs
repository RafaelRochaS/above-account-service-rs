use above_account_service_rs::server::{user::user_server::UserServer, UserSvc};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_svc = UserSvc::default();

    println!("Starting server on [::1]:50051");

    Server::builder()
        .add_service(UserServer::new(user_svc))
        .serve(addr)
        .await?;

    Ok(())
}
