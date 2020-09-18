package ca.gearzero.rgc.client;

public class GetUserCredentialTask implements Runnable {

    private int repeat;

    private final UserServiceClient client;

    private final String key;

    public GetUserCredentialTask(int repeat, UserServiceClient client, String key) {
        this.repeat = repeat;
        this.client = client;
        this.key = key;
    }

    @Override
    public void run() {
        while (--repeat > 0) {
            String uc = client.getUserCredential(key);
            assert("bbc@gearzero.ca".equals(uc));
        }
    }
}
