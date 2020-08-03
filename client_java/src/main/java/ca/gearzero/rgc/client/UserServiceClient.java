package ca.gearzero.rgc.client;

import ca.gearzero.rgc.*;
import io.grpc.Channel;
import io.grpc.StatusRuntimeException;

import java.util.logging.Level;
import java.util.logging.Logger;

public class UserServiceClient {
    private static final Logger logger = Logger.getLogger(UserServiceClient.class.getName());

    private final UserServiceGrpc.UserServiceBlockingStub blockingStub;

    public UserServiceClient(Channel channel) {
        blockingStub = UserServiceGrpc.newBlockingStub(channel);
    }

    public String getUserCredential(String uuid) {
        GetUserCredentialRequest request = GetUserCredentialRequest.newBuilder().setUuid(uuid).build();
        GetUserCredentialReply response;
        try {
            response = blockingStub.getUserCredential(request);
            return response.getUserCredential().getEmail();
        } catch (StatusRuntimeException e) {
            logger.log(Level.WARNING, "RPC failed: {0}", e.getStatus());
        }
        return null;
    }

    public int getUserFlags(String uuid) {
        GetUserFlagsRequest request = GetUserFlagsRequest.newBuilder()
                .setUuid(uuid)
                .build();
        GetUserFlagsReply response;
        try {
            response = blockingStub.getUserFlags(request);
            return response.getUserFlags().getStatusFlag();
        } catch (StatusRuntimeException e) {
            logger.log(Level.WARNING, "RPC failed: {0}", e.getStatus());
        }
        return -1;
    }
}
