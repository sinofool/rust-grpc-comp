package ca.gearzero.rgc.client;

public class GetUserFlagsTask implements Runnable {

    private int repeat;

    private final UserServiceClient client;

    private final String key;

    public GetUserFlagsTask(int repeat, UserServiceClient client, String key) {
        this.repeat = repeat;
        this.client = client;
        this.key = key;
    }

    @Override
    public void run() {
        while (--repeat > 0) {
            client.getUserFlags(key);
        }
    }
}
