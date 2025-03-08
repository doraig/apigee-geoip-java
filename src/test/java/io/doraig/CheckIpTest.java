package io.doraig;


import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class CheckIpTest {

    @Test
    void testCheckIp() {
        ICheckip checkIp = CheckIpFactory.getCheckIp();
        String country = checkIp.lookUpCountry("14.143.44.43");
        Assertions.assertEquals("IN", country);
    }
}
