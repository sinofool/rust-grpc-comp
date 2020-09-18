#[macro_use]
extern crate log;

use std::{io, thread};
use std::io::Read;
use std::sync::Arc;

use futures::Future;
use futures::sync::oneshot;
use grpcio::{ChannelBuilder, Environment, RpcContext, ServerBuilder, UnarySink};
use crate::generated::user_service_grpc::{UserService, create_user_service};
use crate::generated::user_service::{GetUserFlagsRequest, GetUserFlagsReply, GetUserCredentialRequest, GetUserCredentialReply};

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
    fn get_user_flags(&mut self, ctx: RpcContext<'_>, req: GetUserFlagsRequest, sink: UnarySink<GetUserFlagsReply>) {
        let user = User {
            uuid: "6daf4c42-7aa2-4a50-9d97-6b1c8956ac3a",
            email: "bbc@gearzero.ca",
            password_salt: "RPyotOip",
            password_hash: "wSxINtwWzJiwsBeleBtmJVBuARLihvLbelAlhhnIULqxgSmq",
            status_flag: 256 + 128 + 8 + 1,
            permission_flag: 4 + 1024,
            date_created: 1577678540_000,
        };
        let mut reply = GetUserFlagsReply::default();
        reply.mut_user_flags().set_uuid(req.get_uuid().to_string());
        reply.mut_user_flags().set_permission_flag(user.permission_flag);
        reply.mut_user_flags().set_status_flag(user.status_flag);
        let f = sink.success(reply).map_err(move |e| error!("Failed to reply {:?}", e));
        ctx.spawn(f);
    }

    fn get_user_credential(&mut self, ctx: RpcContext<'_>, req: GetUserCredentialRequest, sink: UnarySink<GetUserCredentialReply>) {
        let user = User {
            uuid: "6daf4c42-7aa2-4a50-9d97-6b1c8956ac3a",
            email: "bbc@gearzero.ca",
            password_salt: "RPyotOip",
            password_hash: "wSxINtwWzJiwsBeleBtmJVBuARLihvLbelAlhhnIULqxgSmq",
            status_flag: 256 + 128 + 8 + 1,
            permission_flag: 4 + 1024,
            date_created: 1577678540_000,
        };
        let mut reply = GetUserCredentialReply::default();
        reply.mut_user_credential().set_uuid(req.get_uuid().to_string());
        reply.mut_user_credential().set_email(user.email.to_string());
        reply.mut_user_credential().set_password_salt(user.password_salt.to_string());
        reply.mut_user_credential().set_password_hash(user.password_hash.to_string());
        reply.mut_user_credential().set_status_flag(user.status_flag);
        reply.mut_user_credential().set_permission_flag(user.permission_flag);
        reply.mut_user_credential().set_date_created(user.date_created);
        let f = sink.success(reply).map_err(move |e| error!("Failed to reply {:?}", e));
        ctx.spawn(f);
    }
}

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let env = Arc::new(Environment::new(64));
    let svc = create_user_service(UserServiceImpl {
        cache: vec!(User {
            uuid: "6daf4c42-7aa2-4a50-9d97-6b1c8956ac3a",
            email: "bbc@gearzero.ca",
            password_salt: "RPyotOip",
            password_hash: "wSxINtwWzJiwsBeleBtmJVBuARLihvLbelAlhhnIULqxgSmq",
            status_flag: 256+128+8+1,
            permission_flag: 4+1024,
            date_created: 1577678540_000,
        }),
    });
    let channel = ChannelBuilder::new(env.clone());
    let mut server = ServerBuilder::new(env)
        .register_service(svc)
        .bind("0.0.0.0", 50051)
        .channel_args(channel.build_args())
        .build().unwrap();
    server.start();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press any key to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        let _ = tx.send(());
    });
    let _ = rx.wait();

    let _ = server.shutdown().wait();
}
