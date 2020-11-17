package ca.gearzero.rgc.server;

import ca.gearzero.rgc.*;

public class UserServiceImpl extends UserServiceGrpc.UserServiceImplBase {

    @Override
    public void getUserFlags(ca.gearzero.rgc.GetUserFlagsRequest request,
                             io.grpc.stub.StreamObserver<ca.gearzero.rgc.GetUserFlagsReply> responseObserver) {
        UserFlagsModel user = new UserFlagsModel();
        user.setUuid("6daf4c42-7aa2-4a50-9d97-6b1c8956ac3a");
        user.setEmail("bbc@gearzero.ca");
        user.setPassword_salt("RPyotOip");
        user.setPassword_hash("wSxINtwWzJiwsBeleBtmJVBuARLihvLbelAlhhnIULqxgSmq");
        user.setStatus_flag(256 + 128 + 8 + 1);
        user.setPermission_flag(4 + 1024);
        user.setDate_created(1577678540_000L);

        responseObserver.onNext(GetUserFlagsReply.newBuilder().setUserFlags(
                UserFlags.newBuilder().setUuid(request.getUuid())
                        .setStatusFlag(user.getStatus_flag())
                        .setPermissionFlag(user.getPermission_flag())
                        .build()
        ).build());
        responseObserver.onCompleted();
    }

    @Override
    public void getUserCredential(ca.gearzero.rgc.GetUserCredentialRequest request,
                                  io.grpc.stub.StreamObserver<ca.gearzero.rgc.GetUserCredentialReply> responseObserver) {
        UserFlagsModel user = new UserFlagsModel();
        user.setUuid("6daf4c42-7aa2-4a50-9d97-6b1c8956ac3a");
        user.setEmail("bbc@gearzero.ca");
        user.setPassword_salt("RPyotOip");
        user.setPassword_hash("wSxINtwWzJiwsBeleBtmJVBuARLihvLbelAlhhnIULqxgSmq");
        user.setStatus_flag(256 + 128 + 8 + 1);
        user.setPermission_flag(4 + 1024);
        user.setDate_created(1577678540_000L);

        responseObserver.onNext(GetUserCredentialReply.newBuilder().setUserCredential(
                UserCredential.newBuilder().setUuid(request.getUuid())
                        .setEmail(user.getEmail())
                        .setPasswordSalt(user.getPassword_salt())
                        .setPasswordHash(user.getPassword_hash())
                        .setStatusFlag(user.getStatus_flag())
                        .setPermissionFlag(user.getPermission_flag())
                        .setDateCreated(user.getDate_created())
                        .build()
        ).build());
        responseObserver.onCompleted();
    }
}
