// @generated
impl serde::Serialize for Notification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.percent != 0 {
            len += 1;
        }
        if self.repeat {
            len += 1;
        }
        if !self.call_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.quota.v1.Notification", len)?;
        if self.percent != 0 {
            struct_ser.serialize_field("percent", &self.percent)?;
        }
        if self.repeat {
            struct_ser.serialize_field("repeat", &self.repeat)?;
        }
        if !self.call_url.is_empty() {
            struct_ser.serialize_field("callUrl", &self.call_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Notification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "percent",
            "repeat",
            "call_url",
            "callUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Percent,
            Repeat,
            CallUrl,
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
                            "percent" => Ok(GeneratedField::Percent),
                            "repeat" => Ok(GeneratedField::Repeat),
                            "callUrl" | "call_url" => Ok(GeneratedField::CallUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Notification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.quota.v1.Notification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Notification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut percent__ = None;
                let mut repeat__ = None;
                let mut call_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Percent => {
                            if percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percent"));
                            }
                            percent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Repeat => {
                            if repeat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeat"));
                            }
                            repeat__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CallUrl => {
                            if call_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callUrl"));
                            }
                            call_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Notification {
                    percent: percent__.unwrap_or_default(),
                    repeat: repeat__.unwrap_or_default(),
                    call_url: call_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.quota.v1.Notification", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Unit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unimplemented => "UNIT_UNIMPLEMENTED",
            Self::RequestsAllAuthenticated => "UNIT_REQUESTS_ALL_AUTHENTICATED",
            Self::ActionsAllRunSeconds => "UNIT_ACTIONS_ALL_RUN_SECONDS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Unit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNIT_UNIMPLEMENTED",
            "UNIT_REQUESTS_ALL_AUTHENTICATED",
            "UNIT_ACTIONS_ALL_RUN_SECONDS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Unit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNIT_UNIMPLEMENTED" => Ok(Unit::Unimplemented),
                    "UNIT_REQUESTS_ALL_AUTHENTICATED" => Ok(Unit::RequestsAllAuthenticated),
                    "UNIT_ACTIONS_ALL_RUN_SECONDS" => Ok(Unit::ActionsAllRunSeconds),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
