package ca.gearzero.rgc.server;

import io.grpc.ServerBuilder;

import java.io.IOException;

public class Server {
    public static void main(String[] args) throws IOException, InterruptedException {
        io.grpc.Server server = ServerBuilder.forPort(50051)
                .addService(new UserServiceImpl())
                .build()
                .start();
        System.out.println("Server started");
        server.awaitTermination();
    }
}
