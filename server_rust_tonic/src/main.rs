use rgc::{GetUserCredentialReply, GetUserCredentialRequest, GetUserFlagsReply, GetUserFlagsRequest,UserCredential,UserFlags};
use rgc::user_service_server::{UserService, UserServiceServer};

pub mod rgc {
    tonic::include_proto!("rgc");
}

#[derive(Debug, Clone)]
struct User<'a> {
    uuid: &'a str,
    email: &'a str,
    password_salt: &'a str,
    password_hash: &'a str,
    status_flag: i32,
    permission_flag: i32,
    date_created: i64,
}

#[derive(Debug, Default)]
pub struct UserServiceImpl<'s> {
    cache: Vec<User<'s>>,
}

#[tonic::async_trait]
impl UserService for UserServiceImpl<'static> {
    async fn get_user_flags(
        &self,
        req: tonic::Request<GetUserFlagsRequest>,
    ) -> Result<tonic::Response<GetUserFlagsReply>, tonic::Status> {
        let user = &self.cache[0];
        let reply = GetUserFlagsReply {
            user_flags: Some(UserFlags {
                uuid: req.into_inner().uuid,
                permission_flag: user.permission_flag,
                status_flag: user.status_flag,
            })
        };
        Ok(tonic::Response::new(reply))
    }
    async fn get_user_credential(
        &self,
        req: tonic::Request<GetUserCredentialRequest>,
    ) -> Result<tonic::Response<GetUserCredentialReply>, tonic::Status> {
        let user = &self.cache[0];
        let reply = GetUserCredentialReply {
            user_credential: Some(UserCredential {
                uuid: req.into_inner().uuid,
                email: user.email.to_string(),
                password_salt: user.password_salt.to_string(),
                password_hash: user.password_hash.to_string(),
                status_flag: user.status_flag,
                permission_flag: user.permission_flag,
                date_created: user.date_created,
            })
        };

        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let svc = UserServiceImpl {
        cache: vec!(User {
            uuid: "6daf4c42-7aa2-4a50-9d97-6b1c8956ac3a",
            email: "bbc@gearzero.ca",
            password_salt: "RPyotOip",
            password_hash: "wSxINtwWzJiwsBeleBtmJVBuARLihvLbelAlhhnIULqxgSmq",
            status_flag: 256 + 128 + 8 + 1,
            permission_flag: 4 + 1024,
            date_created: 1577678540_000,
        }),
    };

    tonic::transport::Server::builder()
        .add_service(UserServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}