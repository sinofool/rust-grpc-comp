package ca.gearzero.rgc.server;

import lombok.Data;

@Data
public class UserFlagsModel {
    private String uuid;
    private String email;
    private String password_salt;
    private String password_hash;
    private int status_flag;
    private int permission_flag;
    private long date_created;
}
