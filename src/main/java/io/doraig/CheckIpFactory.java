package io.doraig;

public class CheckIpFactory {

    public static ICheckip getCheckIp() {
        return new CheckIpImpl();
    }
}
