package io.doraig;

import io.questdb.jar.jni.JarJniLoader;

public class CheckIpImpl implements ICheckip {

    private native String checkCountry(String ip);

    static {

        try {
            JarJniLoader.loadLib(
                    ICheckip.class,
                    "libs",
                    "geo_ip"
            );
        } catch (Exception e) {
            System.loadLibrary("geo_ip");
        }

    }

    @Override
    public String lookUpCountry(String ip) {
        return checkCountry(ip);
    }
}
