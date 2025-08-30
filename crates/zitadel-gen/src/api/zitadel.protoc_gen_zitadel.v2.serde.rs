// @generated
impl serde::Serialize for AuthOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permission.is_empty() {
            len += 1;
        }
        if !self.org_field.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.protoc_gen_zitadel.v2.AuthOption", len)?;
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if !self.org_field.is_empty() {
            struct_ser.serialize_field("orgField", &self.org_field)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permission",
            "org_field",
            "orgField",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permission,
            OrgField,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "permission" => Ok(GeneratedField::Permission),
                            "orgField" | "org_field" => Ok(GeneratedField::OrgField),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.protoc_gen_zitadel.v2.AuthOption")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthOption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permission__ = None;
                let mut org_field__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgField => {
                            if org_field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgField"));
                            }
                            org_field__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthOption {
                    permission: permission__.unwrap_or_default(),
                    org_field: org_field__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.protoc_gen_zitadel.v2.AuthOption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomHttpResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success_code != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.protoc_gen_zitadel.v2.CustomHTTPResponse", len)?;
        if self.success_code != 0 {
            struct_ser.serialize_field("successCode", &self.success_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomHttpResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success_code",
            "successCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SuccessCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "successCode" | "success_code" => Ok(GeneratedField::SuccessCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomHttpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.protoc_gen_zitadel.v2.CustomHTTPResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CustomHttpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SuccessCode => {
                            if success_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successCode"));
                            }
                            success_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CustomHttpResponse {
                    success_code: success_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.protoc_gen_zitadel.v2.CustomHTTPResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Options {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.auth_option.is_some() {
            len += 1;
        }
        if self.http_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.protoc_gen_zitadel.v2.Options", len)?;
        if let Some(v) = self.auth_option.as_ref() {
            struct_ser.serialize_field("authOption", v)?;
        }
        if let Some(v) = self.http_response.as_ref() {
            struct_ser.serialize_field("httpResponse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Options {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_option",
            "authOption",
            "http_response",
            "httpResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthOption,
            HttpResponse,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authOption" | "auth_option" => Ok(GeneratedField::AuthOption),
                            "httpResponse" | "http_response" => Ok(GeneratedField::HttpResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Options;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.protoc_gen_zitadel.v2.Options")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Options, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_option__ = None;
                let mut http_response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthOption => {
                            if auth_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authOption"));
                            }
                            auth_option__ = map_.next_value()?;
                        }
                        GeneratedField::HttpResponse => {
                            if http_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpResponse"));
                            }
                            http_response__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Options {
                    auth_option: auth_option__,
                    http_response: http_response__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.protoc_gen_zitadel.v2.Options", FIELDS, GeneratedVisitor)
    }
}
