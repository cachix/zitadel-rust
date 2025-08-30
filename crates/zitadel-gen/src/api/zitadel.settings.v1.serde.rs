// @generated
impl serde::Serialize for DebugNotificationProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if self.compact {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.DebugNotificationProvider", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.compact {
            struct_ser.serialize_field("compact", &self.compact)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DebugNotificationProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "compact",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Compact,
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
                            "details" => Ok(GeneratedField::Details),
                            "compact" => Ok(GeneratedField::Compact),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DebugNotificationProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.DebugNotificationProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DebugNotificationProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut compact__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Compact => {
                            if compact__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compact"));
                            }
                            compact__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DebugNotificationProvider {
                    details: details__,
                    compact: compact__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.DebugNotificationProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmailProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.EmailProvider", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.state != 0 {
            let v = EmailProviderState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                email_provider::Config::Smtp(v) => {
                    struct_ser.serialize_field("smtp", v)?;
                }
                email_provider::Config::Http(v) => {
                    struct_ser.serialize_field("http", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmailProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "id",
            "state",
            "description",
            "smtp",
            "http",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Id,
            State,
            Description,
            Smtp,
            Http,
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
                            "details" => Ok(GeneratedField::Details),
                            "id" => Ok(GeneratedField::Id),
                            "state" => Ok(GeneratedField::State),
                            "description" => Ok(GeneratedField::Description),
                            "smtp" => Ok(GeneratedField::Smtp),
                            "http" => Ok(GeneratedField::Http),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.EmailProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut id__ = None;
                let mut state__ = None;
                let mut description__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<EmailProviderState>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Smtp => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("smtp"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(email_provider::Config::Smtp)
;
                        }
                        GeneratedField::Http => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(email_provider::Config::Http)
;
                        }
                    }
                }
                Ok(EmailProvider {
                    details: details__,
                    id: id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.EmailProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmailProviderHttp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoint.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.EmailProviderHTTP", len)?;
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmailProviderHttp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailProviderHttp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.EmailProviderHTTP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailProviderHttp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EmailProviderHttp {
                    endpoint: endpoint__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.EmailProviderHTTP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmailProviderSmtp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender_address.is_empty() {
            len += 1;
        }
        if !self.sender_name.is_empty() {
            len += 1;
        }
        if self.tls {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.reply_to_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.EmailProviderSMTP", len)?;
        if !self.sender_address.is_empty() {
            struct_ser.serialize_field("senderAddress", &self.sender_address)?;
        }
        if !self.sender_name.is_empty() {
            struct_ser.serialize_field("senderName", &self.sender_name)?;
        }
        if self.tls {
            struct_ser.serialize_field("tls", &self.tls)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.reply_to_address.is_empty() {
            struct_ser.serialize_field("replyToAddress", &self.reply_to_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmailProviderSmtp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender_address",
            "senderAddress",
            "sender_name",
            "senderName",
            "tls",
            "host",
            "user",
            "reply_to_address",
            "replyToAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SenderAddress,
            SenderName,
            Tls,
            Host,
            User,
            ReplyToAddress,
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
                            "senderAddress" | "sender_address" => Ok(GeneratedField::SenderAddress),
                            "senderName" | "sender_name" => Ok(GeneratedField::SenderName),
                            "tls" => Ok(GeneratedField::Tls),
                            "host" => Ok(GeneratedField::Host),
                            "user" => Ok(GeneratedField::User),
                            "replyToAddress" | "reply_to_address" => Ok(GeneratedField::ReplyToAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailProviderSmtp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.EmailProviderSMTP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailProviderSmtp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender_address__ = None;
                let mut sender_name__ = None;
                let mut tls__ = None;
                let mut host__ = None;
                let mut user__ = None;
                let mut reply_to_address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SenderAddress => {
                            if sender_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderAddress"));
                            }
                            sender_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SenderName => {
                            if sender_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderName"));
                            }
                            sender_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tls => {
                            if tls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tls"));
                            }
                            tls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReplyToAddress => {
                            if reply_to_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replyToAddress"));
                            }
                            reply_to_address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EmailProviderSmtp {
                    sender_address: sender_address__.unwrap_or_default(),
                    sender_name: sender_name__.unwrap_or_default(),
                    tls: tls__.unwrap_or_default(),
                    host: host__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    reply_to_address: reply_to_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.EmailProviderSMTP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmailProviderState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EMAIL_PROVIDER_STATE_UNSPECIFIED",
            Self::EmailProviderActive => "EMAIL_PROVIDER_ACTIVE",
            Self::EmailProviderInactive => "EMAIL_PROVIDER_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EmailProviderState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EMAIL_PROVIDER_STATE_UNSPECIFIED",
            "EMAIL_PROVIDER_ACTIVE",
            "EMAIL_PROVIDER_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailProviderState;

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
                    "EMAIL_PROVIDER_STATE_UNSPECIFIED" => Ok(EmailProviderState::Unspecified),
                    "EMAIL_PROVIDER_ACTIVE" => Ok(EmailProviderState::EmailProviderActive),
                    "EMAIL_PROVIDER_INACTIVE" => Ok(EmailProviderState::EmailProviderInactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HttpConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoint.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.HTTPConfig", len)?;
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.HTTPConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HttpConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HttpConfig {
                    endpoint: endpoint__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.HTTPConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OidcSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if self.access_token_lifetime.is_some() {
            len += 1;
        }
        if self.id_token_lifetime.is_some() {
            len += 1;
        }
        if self.refresh_token_idle_expiration.is_some() {
            len += 1;
        }
        if self.refresh_token_expiration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.OIDCSettings", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.access_token_lifetime.as_ref() {
            struct_ser.serialize_field("accessTokenLifetime", v)?;
        }
        if let Some(v) = self.id_token_lifetime.as_ref() {
            struct_ser.serialize_field("idTokenLifetime", v)?;
        }
        if let Some(v) = self.refresh_token_idle_expiration.as_ref() {
            struct_ser.serialize_field("refreshTokenIdleExpiration", v)?;
        }
        if let Some(v) = self.refresh_token_expiration.as_ref() {
            struct_ser.serialize_field("refreshTokenExpiration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OidcSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "access_token_lifetime",
            "accessTokenLifetime",
            "id_token_lifetime",
            "idTokenLifetime",
            "refresh_token_idle_expiration",
            "refreshTokenIdleExpiration",
            "refresh_token_expiration",
            "refreshTokenExpiration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            AccessTokenLifetime,
            IdTokenLifetime,
            RefreshTokenIdleExpiration,
            RefreshTokenExpiration,
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
                            "details" => Ok(GeneratedField::Details),
                            "accessTokenLifetime" | "access_token_lifetime" => Ok(GeneratedField::AccessTokenLifetime),
                            "idTokenLifetime" | "id_token_lifetime" => Ok(GeneratedField::IdTokenLifetime),
                            "refreshTokenIdleExpiration" | "refresh_token_idle_expiration" => Ok(GeneratedField::RefreshTokenIdleExpiration),
                            "refreshTokenExpiration" | "refresh_token_expiration" => Ok(GeneratedField::RefreshTokenExpiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.OIDCSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OidcSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut access_token_lifetime__ = None;
                let mut id_token_lifetime__ = None;
                let mut refresh_token_idle_expiration__ = None;
                let mut refresh_token_expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::AccessTokenLifetime => {
                            if access_token_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenLifetime"));
                            }
                            access_token_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::IdTokenLifetime => {
                            if id_token_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idTokenLifetime"));
                            }
                            id_token_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::RefreshTokenIdleExpiration => {
                            if refresh_token_idle_expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshTokenIdleExpiration"));
                            }
                            refresh_token_idle_expiration__ = map_.next_value()?;
                        }
                        GeneratedField::RefreshTokenExpiration => {
                            if refresh_token_expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshTokenExpiration"));
                            }
                            refresh_token_expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OidcSettings {
                    details: details__,
                    access_token_lifetime: access_token_lifetime__,
                    id_token_lifetime: id_token_lifetime__,
                    refresh_token_idle_expiration: refresh_token_idle_expiration__,
                    refresh_token_expiration: refresh_token_expiration__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.OIDCSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SmsProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.SMSProvider", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.state != 0 {
            let v = SmsProviderConfigState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                sms_provider::Config::Twilio(v) => {
                    struct_ser.serialize_field("twilio", v)?;
                }
                sms_provider::Config::Http(v) => {
                    struct_ser.serialize_field("http", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SmsProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "id",
            "state",
            "description",
            "twilio",
            "http",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Id,
            State,
            Description,
            Twilio,
            Http,
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
                            "details" => Ok(GeneratedField::Details),
                            "id" => Ok(GeneratedField::Id),
                            "state" => Ok(GeneratedField::State),
                            "description" => Ok(GeneratedField::Description),
                            "twilio" => Ok(GeneratedField::Twilio),
                            "http" => Ok(GeneratedField::Http),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SmsProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.SMSProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SmsProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut id__ = None;
                let mut state__ = None;
                let mut description__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<SmsProviderConfigState>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Twilio => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("twilio"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(sms_provider::Config::Twilio)
;
                        }
                        GeneratedField::Http => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(sms_provider::Config::Http)
;
                        }
                    }
                }
                Ok(SmsProvider {
                    details: details__,
                    id: id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.SMSProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SmsProviderConfigState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SMS_PROVIDER_CONFIG_STATE_UNSPECIFIED",
            Self::SmsProviderConfigActive => "SMS_PROVIDER_CONFIG_ACTIVE",
            Self::SmsProviderConfigInactive => "SMS_PROVIDER_CONFIG_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SmsProviderConfigState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SMS_PROVIDER_CONFIG_STATE_UNSPECIFIED",
            "SMS_PROVIDER_CONFIG_ACTIVE",
            "SMS_PROVIDER_CONFIG_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SmsProviderConfigState;

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
                    "SMS_PROVIDER_CONFIG_STATE_UNSPECIFIED" => Ok(SmsProviderConfigState::Unspecified),
                    "SMS_PROVIDER_CONFIG_ACTIVE" => Ok(SmsProviderConfigState::SmsProviderConfigActive),
                    "SMS_PROVIDER_CONFIG_INACTIVE" => Ok(SmsProviderConfigState::SmsProviderConfigInactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SmtpConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if !self.sender_address.is_empty() {
            len += 1;
        }
        if !self.sender_name.is_empty() {
            len += 1;
        }
        if self.tls {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.reply_to_address.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.SMTPConfig", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.sender_address.is_empty() {
            struct_ser.serialize_field("senderAddress", &self.sender_address)?;
        }
        if !self.sender_name.is_empty() {
            struct_ser.serialize_field("senderName", &self.sender_name)?;
        }
        if self.tls {
            struct_ser.serialize_field("tls", &self.tls)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.reply_to_address.is_empty() {
            struct_ser.serialize_field("replyToAddress", &self.reply_to_address)?;
        }
        if self.state != 0 {
            let v = SmtpConfigState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SmtpConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "sender_address",
            "senderAddress",
            "sender_name",
            "senderName",
            "tls",
            "host",
            "user",
            "reply_to_address",
            "replyToAddress",
            "state",
            "description",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SenderAddress,
            SenderName,
            Tls,
            Host,
            User,
            ReplyToAddress,
            State,
            Description,
            Id,
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
                            "details" => Ok(GeneratedField::Details),
                            "senderAddress" | "sender_address" => Ok(GeneratedField::SenderAddress),
                            "senderName" | "sender_name" => Ok(GeneratedField::SenderName),
                            "tls" => Ok(GeneratedField::Tls),
                            "host" => Ok(GeneratedField::Host),
                            "user" => Ok(GeneratedField::User),
                            "replyToAddress" | "reply_to_address" => Ok(GeneratedField::ReplyToAddress),
                            "state" => Ok(GeneratedField::State),
                            "description" => Ok(GeneratedField::Description),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SmtpConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.SMTPConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SmtpConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut sender_address__ = None;
                let mut sender_name__ = None;
                let mut tls__ = None;
                let mut host__ = None;
                let mut user__ = None;
                let mut reply_to_address__ = None;
                let mut state__ = None;
                let mut description__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SenderAddress => {
                            if sender_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderAddress"));
                            }
                            sender_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SenderName => {
                            if sender_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderName"));
                            }
                            sender_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tls => {
                            if tls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tls"));
                            }
                            tls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReplyToAddress => {
                            if reply_to_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replyToAddress"));
                            }
                            reply_to_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<SmtpConfigState>()? as i32);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SmtpConfig {
                    details: details__,
                    sender_address: sender_address__.unwrap_or_default(),
                    sender_name: sender_name__.unwrap_or_default(),
                    tls: tls__.unwrap_or_default(),
                    host: host__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    reply_to_address: reply_to_address__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.SMTPConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SmtpConfigState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SMTP_CONFIG_STATE_UNSPECIFIED",
            Self::SmtpConfigActive => "SMTP_CONFIG_ACTIVE",
            Self::SmtpConfigInactive => "SMTP_CONFIG_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SmtpConfigState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SMTP_CONFIG_STATE_UNSPECIFIED",
            "SMTP_CONFIG_ACTIVE",
            "SMTP_CONFIG_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SmtpConfigState;

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
                    "SMTP_CONFIG_STATE_UNSPECIFIED" => Ok(SmtpConfigState::Unspecified),
                    "SMTP_CONFIG_ACTIVE" => Ok(SmtpConfigState::SmtpConfigActive),
                    "SMTP_CONFIG_INACTIVE" => Ok(SmtpConfigState::SmtpConfigInactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SecretGenerator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.generator_type != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.length != 0 {
            len += 1;
        }
        if self.expiry.is_some() {
            len += 1;
        }
        if self.include_lower_letters {
            len += 1;
        }
        if self.include_upper_letters {
            len += 1;
        }
        if self.include_digits {
            len += 1;
        }
        if self.include_symbols {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.SecretGenerator", len)?;
        if self.generator_type != 0 {
            let v = SecretGeneratorType::try_from(self.generator_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.generator_type)))?;
            struct_ser.serialize_field("generatorType", &v)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.length != 0 {
            struct_ser.serialize_field("length", &self.length)?;
        }
        if let Some(v) = self.expiry.as_ref() {
            struct_ser.serialize_field("expiry", v)?;
        }
        if self.include_lower_letters {
            struct_ser.serialize_field("includeLowerLetters", &self.include_lower_letters)?;
        }
        if self.include_upper_letters {
            struct_ser.serialize_field("includeUpperLetters", &self.include_upper_letters)?;
        }
        if self.include_digits {
            struct_ser.serialize_field("includeDigits", &self.include_digits)?;
        }
        if self.include_symbols {
            struct_ser.serialize_field("includeSymbols", &self.include_symbols)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecretGenerator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "generator_type",
            "generatorType",
            "details",
            "length",
            "expiry",
            "include_lower_letters",
            "includeLowerLetters",
            "include_upper_letters",
            "includeUpperLetters",
            "include_digits",
            "includeDigits",
            "include_symbols",
            "includeSymbols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GeneratorType,
            Details,
            Length,
            Expiry,
            IncludeLowerLetters,
            IncludeUpperLetters,
            IncludeDigits,
            IncludeSymbols,
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
                            "generatorType" | "generator_type" => Ok(GeneratedField::GeneratorType),
                            "details" => Ok(GeneratedField::Details),
                            "length" => Ok(GeneratedField::Length),
                            "expiry" => Ok(GeneratedField::Expiry),
                            "includeLowerLetters" | "include_lower_letters" => Ok(GeneratedField::IncludeLowerLetters),
                            "includeUpperLetters" | "include_upper_letters" => Ok(GeneratedField::IncludeUpperLetters),
                            "includeDigits" | "include_digits" => Ok(GeneratedField::IncludeDigits),
                            "includeSymbols" | "include_symbols" => Ok(GeneratedField::IncludeSymbols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecretGenerator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.SecretGenerator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecretGenerator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut generator_type__ = None;
                let mut details__ = None;
                let mut length__ = None;
                let mut expiry__ = None;
                let mut include_lower_letters__ = None;
                let mut include_upper_letters__ = None;
                let mut include_digits__ = None;
                let mut include_symbols__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GeneratorType => {
                            if generator_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generatorType"));
                            }
                            generator_type__ = Some(map_.next_value::<SecretGeneratorType>()? as i32);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Length => {
                            if length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Expiry => {
                            if expiry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiry"));
                            }
                            expiry__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeLowerLetters => {
                            if include_lower_letters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeLowerLetters"));
                            }
                            include_lower_letters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeUpperLetters => {
                            if include_upper_letters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeUpperLetters"));
                            }
                            include_upper_letters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeDigits => {
                            if include_digits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeDigits"));
                            }
                            include_digits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeSymbols => {
                            if include_symbols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeSymbols"));
                            }
                            include_symbols__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SecretGenerator {
                    generator_type: generator_type__.unwrap_or_default(),
                    details: details__,
                    length: length__.unwrap_or_default(),
                    expiry: expiry__,
                    include_lower_letters: include_lower_letters__.unwrap_or_default(),
                    include_upper_letters: include_upper_letters__.unwrap_or_default(),
                    include_digits: include_digits__.unwrap_or_default(),
                    include_symbols: include_symbols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.SecretGenerator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecretGeneratorQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.SecretGeneratorQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                secret_generator_query::Query::TypeQuery(v) => {
                    struct_ser.serialize_field("typeQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecretGeneratorQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_query",
            "typeQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeQuery,
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
                            "typeQuery" | "type_query" => Ok(GeneratedField::TypeQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecretGeneratorQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.SecretGeneratorQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecretGeneratorQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypeQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(secret_generator_query::Query::TypeQuery)
;
                        }
                    }
                }
                Ok(SecretGeneratorQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.SecretGeneratorQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecretGeneratorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SECRET_GENERATOR_TYPE_UNSPECIFIED",
            Self::InitCode => "SECRET_GENERATOR_TYPE_INIT_CODE",
            Self::VerifyEmailCode => "SECRET_GENERATOR_TYPE_VERIFY_EMAIL_CODE",
            Self::VerifyPhoneCode => "SECRET_GENERATOR_TYPE_VERIFY_PHONE_CODE",
            Self::PasswordResetCode => "SECRET_GENERATOR_TYPE_PASSWORD_RESET_CODE",
            Self::PasswordlessInitCode => "SECRET_GENERATOR_TYPE_PASSWORDLESS_INIT_CODE",
            Self::AppSecret => "SECRET_GENERATOR_TYPE_APP_SECRET",
            Self::OtpSms => "SECRET_GENERATOR_TYPE_OTP_SMS",
            Self::OtpEmail => "SECRET_GENERATOR_TYPE_OTP_EMAIL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SecretGeneratorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SECRET_GENERATOR_TYPE_UNSPECIFIED",
            "SECRET_GENERATOR_TYPE_INIT_CODE",
            "SECRET_GENERATOR_TYPE_VERIFY_EMAIL_CODE",
            "SECRET_GENERATOR_TYPE_VERIFY_PHONE_CODE",
            "SECRET_GENERATOR_TYPE_PASSWORD_RESET_CODE",
            "SECRET_GENERATOR_TYPE_PASSWORDLESS_INIT_CODE",
            "SECRET_GENERATOR_TYPE_APP_SECRET",
            "SECRET_GENERATOR_TYPE_OTP_SMS",
            "SECRET_GENERATOR_TYPE_OTP_EMAIL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecretGeneratorType;

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
                    "SECRET_GENERATOR_TYPE_UNSPECIFIED" => Ok(SecretGeneratorType::Unspecified),
                    "SECRET_GENERATOR_TYPE_INIT_CODE" => Ok(SecretGeneratorType::InitCode),
                    "SECRET_GENERATOR_TYPE_VERIFY_EMAIL_CODE" => Ok(SecretGeneratorType::VerifyEmailCode),
                    "SECRET_GENERATOR_TYPE_VERIFY_PHONE_CODE" => Ok(SecretGeneratorType::VerifyPhoneCode),
                    "SECRET_GENERATOR_TYPE_PASSWORD_RESET_CODE" => Ok(SecretGeneratorType::PasswordResetCode),
                    "SECRET_GENERATOR_TYPE_PASSWORDLESS_INIT_CODE" => Ok(SecretGeneratorType::PasswordlessInitCode),
                    "SECRET_GENERATOR_TYPE_APP_SECRET" => Ok(SecretGeneratorType::AppSecret),
                    "SECRET_GENERATOR_TYPE_OTP_SMS" => Ok(SecretGeneratorType::OtpSms),
                    "SECRET_GENERATOR_TYPE_OTP_EMAIL" => Ok(SecretGeneratorType::OtpEmail),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SecretGeneratorTypeQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.generator_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.SecretGeneratorTypeQuery", len)?;
        if self.generator_type != 0 {
            let v = SecretGeneratorType::try_from(self.generator_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.generator_type)))?;
            struct_ser.serialize_field("generatorType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecretGeneratorTypeQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "generator_type",
            "generatorType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GeneratorType,
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
                            "generatorType" | "generator_type" => Ok(GeneratedField::GeneratorType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecretGeneratorTypeQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.SecretGeneratorTypeQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecretGeneratorTypeQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut generator_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GeneratorType => {
                            if generator_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generatorType"));
                            }
                            generator_type__ = Some(map_.next_value::<SecretGeneratorType>()? as i32);
                        }
                    }
                }
                Ok(SecretGeneratorTypeQuery {
                    generator_type: generator_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.SecretGeneratorTypeQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecurityPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if self.enable_iframe_embedding {
            len += 1;
        }
        if !self.allowed_origins.is_empty() {
            len += 1;
        }
        if self.enable_impersonation {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.SecurityPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.enable_iframe_embedding {
            struct_ser.serialize_field("enableIframeEmbedding", &self.enable_iframe_embedding)?;
        }
        if !self.allowed_origins.is_empty() {
            struct_ser.serialize_field("allowedOrigins", &self.allowed_origins)?;
        }
        if self.enable_impersonation {
            struct_ser.serialize_field("enableImpersonation", &self.enable_impersonation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecurityPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "enable_iframe_embedding",
            "enableIframeEmbedding",
            "allowed_origins",
            "allowedOrigins",
            "enable_impersonation",
            "enableImpersonation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            EnableIframeEmbedding,
            AllowedOrigins,
            EnableImpersonation,
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
                            "details" => Ok(GeneratedField::Details),
                            "enableIframeEmbedding" | "enable_iframe_embedding" => Ok(GeneratedField::EnableIframeEmbedding),
                            "allowedOrigins" | "allowed_origins" => Ok(GeneratedField::AllowedOrigins),
                            "enableImpersonation" | "enable_impersonation" => Ok(GeneratedField::EnableImpersonation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecurityPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.SecurityPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecurityPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut enable_iframe_embedding__ = None;
                let mut allowed_origins__ = None;
                let mut enable_impersonation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::EnableIframeEmbedding => {
                            if enable_iframe_embedding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableIframeEmbedding"));
                            }
                            enable_iframe_embedding__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedOrigins => {
                            if allowed_origins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedOrigins"));
                            }
                            allowed_origins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EnableImpersonation => {
                            if enable_impersonation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableImpersonation"));
                            }
                            enable_impersonation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SecurityPolicy {
                    details: details__,
                    enable_iframe_embedding: enable_iframe_embedding__.unwrap_or_default(),
                    allowed_origins: allowed_origins__.unwrap_or_default(),
                    enable_impersonation: enable_impersonation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.SecurityPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TwilioConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sid.is_empty() {
            len += 1;
        }
        if !self.sender_number.is_empty() {
            len += 1;
        }
        if !self.verify_service_sid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v1.TwilioConfig", len)?;
        if !self.sid.is_empty() {
            struct_ser.serialize_field("sid", &self.sid)?;
        }
        if !self.sender_number.is_empty() {
            struct_ser.serialize_field("senderNumber", &self.sender_number)?;
        }
        if !self.verify_service_sid.is_empty() {
            struct_ser.serialize_field("verifyServiceSid", &self.verify_service_sid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TwilioConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sid",
            "sender_number",
            "senderNumber",
            "verify_service_sid",
            "verifyServiceSid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sid,
            SenderNumber,
            VerifyServiceSid,
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
                            "sid" => Ok(GeneratedField::Sid),
                            "senderNumber" | "sender_number" => Ok(GeneratedField::SenderNumber),
                            "verifyServiceSid" | "verify_service_sid" => Ok(GeneratedField::VerifyServiceSid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TwilioConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v1.TwilioConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TwilioConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sid__ = None;
                let mut sender_number__ = None;
                let mut verify_service_sid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sid => {
                            if sid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sid"));
                            }
                            sid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SenderNumber => {
                            if sender_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("senderNumber"));
                            }
                            sender_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VerifyServiceSid => {
                            if verify_service_sid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyServiceSid"));
                            }
                            verify_service_sid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TwilioConfig {
                    sid: sid__.unwrap_or_default(),
                    sender_number: sender_number__.unwrap_or_default(),
                    verify_service_sid: verify_service_sid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v1.TwilioConfig", FIELDS, GeneratedVisitor)
    }
}
