use robusta_jni::bridge;


#[bridge]
mod jni {

    use std::str::FromStr;
    use maxminddb::{MaxMindDBError};
    use maxminddb::geoip2::City;
    use robusta_jni::convert::{
        IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue
    };
    use robusta_jni::jni::errors::Error as JniError;
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::JNIEnv;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(io.doraig)]
    pub struct CheckIpImpl<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> CheckIpImpl<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}

        #[call_type(safe(
            exception_class = "java.lang.Exception",
            message = "Evaluation failed unexpectedly"
        ))]
        pub extern "jni" fn checkCountry(self, _env: &'borrow JNIEnv<'env>, ip: String) -> JniResult<String> {
            let bytes = include_bytes!("GeoLite2-City.mmdb");
            let reader = maxminddb::Reader::from_source(bytes.to_vec()).unwrap();
            match FromStr::from_str(ip.as_str()) {
                Ok(ip) => {
                    let city: Result<City, MaxMindDBError> = reader.lookup(ip);
                    match city {
                        Ok(value) =>  {
                            let country = value.country.unwrap();
                            let iso_code = country.iso_code.unwrap();
                            Ok(iso_code.to_string())
                        },
                        Err(_e) => Err(JniError::MethodNotFound{name: _e.to_string(), sig: ip.to_string()}),
                    }
                },
                Err(_e) => Err(JniError::JavaException),
            }
        }
    }



}
