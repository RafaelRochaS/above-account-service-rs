use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use above_account_service_rs::server::{user::user_server::UserServer, UserSvc};
use tonic::transport::Server;

struct ServerConfig {
    host: IpAddr,
    port: u16,
}

mod consts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_config = load_configs();
    let addr = SocketAddr::from((server_config.host, server_config.port));

    let user_svc = UserSvc::default();

    println!(
        "Starting server on {}:{}",
        server_config.host, server_config.port
    );

    Server::builder()
        .add_service(UserServer::new(user_svc))
        .serve(addr)
        .await?;

    Ok(())
}

fn load_configs() -> ServerConfig {
    dotenv::dotenv().ok();

    return ServerConfig {
        host: match env::var("HOST") {
            Ok(value) => value
                .parse::<IpAddr>()
                .unwrap_or_else(|_| consts::DEFAULT_HOST),
            Err(_) => consts::DEFAULT_HOST,
        },

        port: match env::var("PORT") {
            Ok(value) => value
                .parse::<u16>()
                .unwrap_or_else(|_| consts::DEFAULT_PORT),
            Err(_) => consts::DEFAULT_PORT,
        },
    };
}
