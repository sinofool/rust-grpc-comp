use std::thread;

use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use crate::generated::user_service::{GetUserCredentialReply, GetUserCredentialRequest, GetUserFlagsReply, GetUserFlagsRequest};
use crate::generated::user_service_grpc::{UserService, UserServiceServer};

mod generated;

#[derive(Clone)]
struct User<'a> {
    uuid: &'a str,
    email: &'a str,
    password_salt: &'a str,
    password_hash: &'a str,
    status_flag: i32,
    permission_flag: i32,
    date_created: i64,
}

#[derive(Clone)]
struct UserServiceImpl<'s> {
    cache: Vec<User<'s>>,
}

impl UserService for UserServiceImpl<'_> {
    fn get_user_flags(&self, _o: ServerHandlerContext, req: ServerRequestSingle<GetUserFlagsRequest>, resp: ServerResponseUnarySink<GetUserFlagsReply>) -> grpc::Result<()> {
        let user = &self.cache[0];
        let mut reply = GetUserFlagsReply::default();
        reply.mut_user_flags().set_uuid(req.message.uuid);
        reply.mut_user_flags().set_permission_flag(user.permission_flag);
        reply.mut_user_flags().set_status_flag(user.status_flag);
        resp.finish(reply)
    }

    fn get_user_credential(&self, _o: ServerHandlerContext, req: ServerRequestSingle<GetUserCredentialRequest>, resp: ServerResponseUnarySink<GetUserCredentialReply>) -> grpc::Result<()> {
        let user = &self.cache[0];
        let mut reply = GetUserCredentialReply::default();
        reply.mut_user_credential().set_uuid(req.message.uuid);
        reply.mut_user_credential().set_email(user.email.to_string());
        reply.mut_user_credential().set_password_salt(user.password_salt.to_string());
        reply.mut_user_credential().set_password_hash(user.password_hash.to_string());
        reply.mut_user_credential().set_status_flag(user.status_flag);
        reply.mut_user_credential().set_permission_flag(user.permission_flag);
        reply.mut_user_credential().set_date_created(user.date_created);
        resp.finish(reply)
    }
}

fn main() {
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

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(50051);
    server.add_service(UserServiceServer::new_service_def(svc));
    
    let _server = server.build().expect("server");

    println!("Server started");

    loop {
        thread::park();
    }
}