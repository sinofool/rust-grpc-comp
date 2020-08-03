package ca.gearzero.rgc.client;

import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.time.format.DateTimeFormatter;
import java.util.Date;
import java.util.Random;
import java.util.UUID;
import java.util.logging.SimpleFormatter;

public class RandomUtil {
    private static final Random RAND = new Random(System.currentTimeMillis());

    private static final String RAND_CHARS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    private RandomUtil() {
    }

    public static String RandomString(int length) {
        char[] chars = new char[length];
        for (int i = 0; i < length; ++i) {
            chars[i] = RAND_CHARS.charAt(RAND.nextInt(RAND_CHARS.length()));
        }
        return new String(chars);
    }

    public static void main(String[] args) throws ParseException {
        System.out.println(UUID.randomUUID());
        System.out.println(RandomString(8));
        System.out.println(RandomString(48));
        System.out.println(new SimpleDateFormat("YYYY-mm-dd HH:MM:ss").parse("2020-02-20 20:02:20").getTime());
    }
}
