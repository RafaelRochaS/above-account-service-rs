use tonic::{Request, Response, Status};

use user::user_server::User;
use user::{HealthcheckReply, HealthcheckRequest, UserCreateReply, UserCreateRequest};

pub mod user {
    tonic::include_proto!("above.account_service");
}

#[derive(Debug, Default)]
pub struct UserSvc {}

#[tonic::async_trait]
impl User for UserSvc {
    async fn create_user(
        &self,
        request: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = user::UserCreateReply {
            status: "User created".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn healthcheck(
        &self,
        request: Request<HealthcheckRequest>,
    ) -> Result<Response<HealthcheckReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = user::HealthcheckReply {
            status: "Ok".to_string(),
        };

        Ok(Response::new(reply))
    }
}
