package ca.gearzero.rgc.client;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;

import java.util.Arrays;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

public class Client {
    private static final int CONN_SIZE = Runtime.getRuntime().availableProcessors() * 2;

    private static final int REPEAT = 1000;

    private static final int BATCH = 1000;

    public static void main(String[] args) throws InterruptedException {
        ManagedChannel[] channels = new ManagedChannel[CONN_SIZE];
        UserServiceClient[] clients = new UserServiceClient[CONN_SIZE];
        for (int i = 0; i < CONN_SIZE; ++i) {
            channels[i] = ManagedChannelBuilder.forTarget("localhost:50051")
                    .usePlaintext()
                    .build();
            clients[i] = new UserServiceClient(channels[i]);
        }

        try {
            int loop = 0;
            long userFlagsTotal = 0;
            long userCredentialTotal = 0;
            do {
                {
                    ExecutorService pool = Executors.newFixedThreadPool(Runtime.getRuntime().availableProcessors());
                    long start = System.nanoTime();

                    for (int repeat = 0; repeat < REPEAT; ++repeat) {
                        String key = "UID_" + repeat;
                        pool.submit(new GetUserFlagsTask(BATCH, clients[repeat % CONN_SIZE], key));
                    }
                    pool.shutdown();
                    boolean shutdown = pool.awaitTermination(1, TimeUnit.MINUTES);
                    if (!shutdown) {
                        pool.shutdownNow();
                    }
                    long end = System.nanoTime();
                    long time = (end - start) / 1_000_000L;
                    if (loop != 0) {
                        userFlagsTotal += time;
                    }
                    System.out.println(String.format("GetUserFlagsTask %d tasks, each task %d ops, number of channels %d, time %dms", REPEAT, BATCH, CONN_SIZE, time));
                }
                {
                    ExecutorService pool = Executors.newFixedThreadPool(Runtime.getRuntime().availableProcessors());
                    long start = System.nanoTime();
                    for (int repeat = 0; repeat < REPEAT; ++repeat) {
                        String key = "UID_" + repeat;
                        pool.submit(new GetUserCredentialTask(BATCH, clients[repeat % CONN_SIZE], key));
                    }
                    pool.shutdown();
                    boolean shutdown = pool.awaitTermination(1, TimeUnit.MINUTES);
                    if (!shutdown) {
                        pool.shutdownNow();
                    }
                    long end = System.nanoTime();
                    long time = (end - start) / 1_000_000L;
                    if (loop != 0) {
                        userCredentialTotal += time;
                    }
                    System.out.println(String.format("GetUserCredentialTask %d tasks, each task %d ops, number of channels %d, time %dms", REPEAT, BATCH, CONN_SIZE, time));
                }
            } while (loop++ < 5);
            System.out.println(String.format("GetUserFlagsTask average: %dms", userFlagsTotal / (loop - 1)));
            System.out.println(String.format("GetUserCredentialTask average: %dms", userCredentialTotal / (loop - 1)));
        } finally {
            Arrays.stream(channels).forEach(c -> {
                try {
                    c.shutdownNow().awaitTermination(5, TimeUnit.SECONDS);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            });
        }

    }
}
