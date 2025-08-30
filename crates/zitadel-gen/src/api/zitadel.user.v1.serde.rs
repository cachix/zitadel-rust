// @generated
impl serde::Serialize for AccessTokenType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Bearer => "ACCESS_TOKEN_TYPE_BEARER",
            Self::Jwt => "ACCESS_TOKEN_TYPE_JWT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AccessTokenType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACCESS_TOKEN_TYPE_BEARER",
            "ACCESS_TOKEN_TYPE_JWT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessTokenType;

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
                    "ACCESS_TOKEN_TYPE_BEARER" => Ok(AccessTokenType::Bearer),
                    "ACCESS_TOKEN_TYPE_JWT" => Ok(AccessTokenType::Jwt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AndQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.queries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.AndQuery", len)?;
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AndQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Queries,
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
                            "queries" => Ok(GeneratedField::Queries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AndQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.AndQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AndQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AndQuery {
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.AndQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.AuthFactor", len)?;
        if self.state != 0 {
            let v = AuthFactorState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                auth_factor::Type::Otp(v) => {
                    struct_ser.serialize_field("otp", v)?;
                }
                auth_factor::Type::U2f(v) => {
                    struct_ser.serialize_field("u2f", v)?;
                }
                auth_factor::Type::OtpSms(v) => {
                    struct_ser.serialize_field("otpSms", v)?;
                }
                auth_factor::Type::OtpEmail(v) => {
                    struct_ser.serialize_field("otpEmail", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "otp",
            "u2f",
            "otp_sms",
            "otpSms",
            "otp_email",
            "otpEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Otp,
            U2f,
            OtpSms,
            OtpEmail,
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
                            "state" => Ok(GeneratedField::State),
                            "otp" => Ok(GeneratedField::Otp),
                            "u2f" => Ok(GeneratedField::U2f),
                            "otpSms" | "otp_sms" => Ok(GeneratedField::OtpSms),
                            "otpEmail" | "otp_email" => Ok(GeneratedField::OtpEmail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.AuthFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<AuthFactorState>()? as i32);
                        }
                        GeneratedField::Otp => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otp"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(auth_factor::Type::Otp)
;
                        }
                        GeneratedField::U2f => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u2f"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(auth_factor::Type::U2f)
;
                        }
                        GeneratedField::OtpSms => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpSms"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(auth_factor::Type::OtpSms)
;
                        }
                        GeneratedField::OtpEmail => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpEmail"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(auth_factor::Type::OtpEmail)
;
                        }
                    }
                }
                Ok(AuthFactor {
                    state: state__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.AuthFactor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthFactorOtp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v1.AuthFactorOTP", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthFactorOtp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthFactorOtp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.AuthFactorOTP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthFactorOtp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AuthFactorOtp {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.AuthFactorOTP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthFactorOtpEmail {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v1.AuthFactorOTPEmail", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthFactorOtpEmail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthFactorOtpEmail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.AuthFactorOTPEmail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthFactorOtpEmail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AuthFactorOtpEmail {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.AuthFactorOTPEmail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthFactorOtpsms {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v1.AuthFactorOTPSMS", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthFactorOtpsms {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthFactorOtpsms;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.AuthFactorOTPSMS")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthFactorOtpsms, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AuthFactorOtpsms {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.AuthFactorOTPSMS", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthFactorState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTH_FACTOR_STATE_UNSPECIFIED",
            Self::NotReady => "AUTH_FACTOR_STATE_NOT_READY",
            Self::Ready => "AUTH_FACTOR_STATE_READY",
            Self::Removed => "AUTH_FACTOR_STATE_REMOVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AuthFactorState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTH_FACTOR_STATE_UNSPECIFIED",
            "AUTH_FACTOR_STATE_NOT_READY",
            "AUTH_FACTOR_STATE_READY",
            "AUTH_FACTOR_STATE_REMOVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthFactorState;

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
                    "AUTH_FACTOR_STATE_UNSPECIFIED" => Ok(AuthFactorState::Unspecified),
                    "AUTH_FACTOR_STATE_NOT_READY" => Ok(AuthFactorState::NotReady),
                    "AUTH_FACTOR_STATE_READY" => Ok(AuthFactorState::Ready),
                    "AUTH_FACTOR_STATE_REMOVED" => Ok(AuthFactorState::Removed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AuthFactorU2f {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.AuthFactorU2F", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthFactorU2f {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthFactorU2f;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.AuthFactorU2F")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthFactorU2f, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthFactorU2f {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.AuthFactorU2F", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DisplayNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.DisplayNameQuery", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DisplayNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            Method,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DisplayNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.DisplayNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DisplayNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(DisplayNameQuery {
                    display_name: display_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.DisplayNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Email {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if self.is_email_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Email", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.is_email_verified {
            struct_ser.serialize_field("isEmailVerified", &self.is_email_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Email {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "is_email_verified",
            "isEmailVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            IsEmailVerified,
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
                            "email" => Ok(GeneratedField::Email),
                            "isEmailVerified" | "is_email_verified" => Ok(GeneratedField::IsEmailVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Email")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Email, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut is_email_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsEmailVerified => {
                            if is_email_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEmailVerified"));
                            }
                            is_email_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Email {
                    email: email__.unwrap_or_default(),
                    is_email_verified: is_email_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Email", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmailQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email_address.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.EmailQuery", len)?;
        if !self.email_address.is_empty() {
            struct_ser.serialize_field("emailAddress", &self.email_address)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmailQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email_address",
            "emailAddress",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmailAddress,
            Method,
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
                            "emailAddress" | "email_address" => Ok(GeneratedField::EmailAddress),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.EmailQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email_address__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmailAddress => {
                            if email_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailAddress"));
                            }
                            email_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(EmailQuery {
                    email_address: email_address__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.EmailQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FirstNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.first_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.FirstNameQuery", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FirstNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "first_name",
            "firstName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstName,
            Method,
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
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FirstNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.FirstNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FirstNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(FirstNameQuery {
                    first_name: first_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.FirstNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Gender {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "GENDER_UNSPECIFIED",
            Self::Female => "GENDER_FEMALE",
            Self::Male => "GENDER_MALE",
            Self::Diverse => "GENDER_DIVERSE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Gender {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "GENDER_UNSPECIFIED",
            "GENDER_FEMALE",
            "GENDER_MALE",
            "GENDER_DIVERSE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Gender;

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
                    "GENDER_UNSPECIFIED" => Ok(Gender::Unspecified),
                    "GENDER_FEMALE" => Ok(Gender::Female),
                    "GENDER_MALE" => Ok(Gender::Male),
                    "GENDER_DIVERSE" => Ok(Gender::Diverse),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Human {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.profile.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.password_changed.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Human", len)?;
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.password_changed.as_ref() {
            struct_ser.serialize_field("passwordChanged", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Human {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "profile",
            "email",
            "phone",
            "password_changed",
            "passwordChanged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Profile,
            Email,
            Phone,
            PasswordChanged,
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
                            "profile" => Ok(GeneratedField::Profile),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "passwordChanged" | "password_changed" => Ok(GeneratedField::PasswordChanged),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Human;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Human")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Human, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut profile__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut password_changed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Profile => {
                            if profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            profile__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordChanged => {
                            if password_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChanged"));
                            }
                            password_changed__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Human {
                    profile: profile__,
                    email: email__,
                    phone: phone__,
                    password_changed: password_changed__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Human", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InUserEmailsQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_emails.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.InUserEmailsQuery", len)?;
        if !self.user_emails.is_empty() {
            struct_ser.serialize_field("userEmails", &self.user_emails)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InUserEmailsQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_emails",
            "userEmails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserEmails,
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
                            "userEmails" | "user_emails" => Ok(GeneratedField::UserEmails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InUserEmailsQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.InUserEmailsQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InUserEmailsQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_emails__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserEmails => {
                            if user_emails__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userEmails"));
                            }
                            user_emails__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InUserEmailsQuery {
                    user_emails: user_emails__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.InUserEmailsQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InUserIdQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.InUserIDQuery", len)?;
        if !self.user_ids.is_empty() {
            struct_ser.serialize_field("userIds", &self.user_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InUserIdQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_ids",
            "userIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserIds,
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
                            "userIds" | "user_ids" => Ok(GeneratedField::UserIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InUserIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.InUserIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InUserIdQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserIds => {
                            if user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIds"));
                            }
                            user_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InUserIdQuery {
                    user_ids: user_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.InUserIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LastNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.last_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.LastNameQuery", len)?;
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LastNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "last_name",
            "lastName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastName,
            Method,
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
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LastNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.LastNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LastNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut last_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(LastNameQuery {
                    last_name: last_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.LastNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.login_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.LoginNameQuery", len)?;
        if !self.login_name.is_empty() {
            struct_ser.serialize_field("loginName", &self.login_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login_name",
            "loginName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoginName,
            Method,
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
                            "loginName" | "login_name" => Ok(GeneratedField::LoginName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.LoginNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut login_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoginName => {
                            if login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginName"));
                            }
                            login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(LoginNameQuery {
                    login_name: login_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.LoginNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Machine {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.has_secret {
            len += 1;
        }
        if self.access_token_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Machine", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.has_secret {
            struct_ser.serialize_field("hasSecret", &self.has_secret)?;
        }
        if self.access_token_type != 0 {
            let v = AccessTokenType::try_from(self.access_token_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.access_token_type)))?;
            struct_ser.serialize_field("accessTokenType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Machine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "has_secret",
            "hasSecret",
            "access_token_type",
            "accessTokenType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            HasSecret,
            AccessTokenType,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "hasSecret" | "has_secret" => Ok(GeneratedField::HasSecret),
                            "accessTokenType" | "access_token_type" => Ok(GeneratedField::AccessTokenType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Machine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Machine")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Machine, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut has_secret__ = None;
                let mut access_token_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasSecret => {
                            if has_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasSecret"));
                            }
                            has_secret__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessTokenType => {
                            if access_token_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenType"));
                            }
                            access_token_type__ = Some(map_.next_value::<AccessTokenType>()? as i32);
                        }
                    }
                }
                Ok(Machine {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    has_secret: has_secret__.unwrap_or_default(),
                    access_token_type: access_token_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Machine", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Membership {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if !self.roles.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Membership", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                membership::Type::Iam(v) => {
                    struct_ser.serialize_field("iam", v)?;
                }
                membership::Type::OrgId(v) => {
                    struct_ser.serialize_field("orgId", v)?;
                }
                membership::Type::ProjectId(v) => {
                    struct_ser.serialize_field("projectId", v)?;
                }
                membership::Type::ProjectGrantId(v) => {
                    struct_ser.serialize_field("projectGrantId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Membership {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "details",
            "roles",
            "display_name",
            "displayName",
            "iam",
            "org_id",
            "orgId",
            "project_id",
            "projectId",
            "project_grant_id",
            "projectGrantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Details,
            Roles,
            DisplayName,
            Iam,
            OrgId,
            ProjectId,
            ProjectGrantId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "details" => Ok(GeneratedField::Details),
                            "roles" => Ok(GeneratedField::Roles),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "iam" => Ok(GeneratedField::Iam),
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Membership;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Membership")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Membership, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut details__ = None;
                let mut roles__ = None;
                let mut display_name__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Iam => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iam"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(membership::Type::Iam);
                        }
                        GeneratedField::OrgId => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(membership::Type::OrgId);
                        }
                        GeneratedField::ProjectId => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(membership::Type::ProjectId);
                        }
                        GeneratedField::ProjectGrantId => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(membership::Type::ProjectGrantId);
                        }
                    }
                }
                Ok(Membership {
                    user_id: user_id__.unwrap_or_default(),
                    details: details__,
                    roles: roles__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Membership", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MembershipIamQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.iam {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.MembershipIAMQuery", len)?;
        if self.iam {
            struct_ser.serialize_field("iam", &self.iam)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MembershipIamQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "iam",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Iam,
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
                            "iam" => Ok(GeneratedField::Iam),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MembershipIamQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.MembershipIAMQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MembershipIamQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut iam__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Iam => {
                            if iam__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iam"));
                            }
                            iam__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MembershipIamQuery {
                    iam: iam__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.MembershipIAMQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MembershipOrgQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.org_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.MembershipOrgQuery", len)?;
        if !self.org_id.is_empty() {
            struct_ser.serialize_field("orgId", &self.org_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MembershipOrgQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_id",
            "orgId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgId,
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
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MembershipOrgQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.MembershipOrgQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MembershipOrgQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MembershipOrgQuery {
                    org_id: org_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.MembershipOrgQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MembershipProjectGrantQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_grant_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.MembershipProjectGrantQuery", len)?;
        if !self.project_grant_id.is_empty() {
            struct_ser.serialize_field("projectGrantId", &self.project_grant_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MembershipProjectGrantQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_grant_id",
            "projectGrantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectGrantId,
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
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MembershipProjectGrantQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.MembershipProjectGrantQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MembershipProjectGrantQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_grant_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectGrantId => {
                            if project_grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            project_grant_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MembershipProjectGrantQuery {
                    project_grant_id: project_grant_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.MembershipProjectGrantQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MembershipProjectQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.MembershipProjectQuery", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MembershipProjectQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MembershipProjectQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.MembershipProjectQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MembershipProjectQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MembershipProjectQuery {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.MembershipProjectQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MembershipQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.MembershipQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                membership_query::Query::OrgQuery(v) => {
                    struct_ser.serialize_field("orgQuery", v)?;
                }
                membership_query::Query::ProjectQuery(v) => {
                    struct_ser.serialize_field("projectQuery", v)?;
                }
                membership_query::Query::ProjectGrantQuery(v) => {
                    struct_ser.serialize_field("projectGrantQuery", v)?;
                }
                membership_query::Query::IamQuery(v) => {
                    struct_ser.serialize_field("iamQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MembershipQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_query",
            "orgQuery",
            "project_query",
            "projectQuery",
            "project_grant_query",
            "projectGrantQuery",
            "iam_query",
            "iamQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgQuery,
            ProjectQuery,
            ProjectGrantQuery,
            IamQuery,
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
                            "orgQuery" | "org_query" => Ok(GeneratedField::OrgQuery),
                            "projectQuery" | "project_query" => Ok(GeneratedField::ProjectQuery),
                            "projectGrantQuery" | "project_grant_query" => Ok(GeneratedField::ProjectGrantQuery),
                            "iamQuery" | "iam_query" => Ok(GeneratedField::IamQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MembershipQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.MembershipQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MembershipQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(membership_query::Query::OrgQuery)
;
                        }
                        GeneratedField::ProjectQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(membership_query::Query::ProjectQuery)
;
                        }
                        GeneratedField::ProjectGrantQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(membership_query::Query::ProjectGrantQuery)
;
                        }
                        GeneratedField::IamQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iamQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(membership_query::Query::IamQuery)
;
                        }
                    }
                }
                Ok(MembershipQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.MembershipQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NickNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nick_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.NickNameQuery", len)?;
        if !self.nick_name.is_empty() {
            struct_ser.serialize_field("nickName", &self.nick_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NickNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nick_name",
            "nickName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NickName,
            Method,
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
                            "nickName" | "nick_name" => Ok(GeneratedField::NickName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NickNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.NickNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NickNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nick_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NickName => {
                            if nick_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickName"));
                            }
                            nick_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(NickNameQuery {
                    nick_name: nick_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.NickNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.NotQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
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
                            "query" => Ok(GeneratedField::Query),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.NotQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NotQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.NotQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.queries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.OrQuery", len)?;
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "queries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Queries,
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
                            "queries" => Ok(GeneratedField::Queries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.OrQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut queries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrQuery {
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.OrQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PersonalAccessToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.PersonalAccessToken", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PersonalAccessToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "details",
            "expiration_date",
            "expirationDate",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            ExpirationDate,
            Scopes,
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
                            "id" => Ok(GeneratedField::Id),
                            "details" => Ok(GeneratedField::Details),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            "scopes" => Ok(GeneratedField::Scopes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PersonalAccessToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.PersonalAccessToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PersonalAccessToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut expiration_date__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PersonalAccessToken {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    expiration_date: expiration_date__,
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.PersonalAccessToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Phone {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone.is_empty() {
            len += 1;
        }
        if self.is_phone_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Phone", len)?;
        if !self.phone.is_empty() {
            struct_ser.serialize_field("phone", &self.phone)?;
        }
        if self.is_phone_verified {
            struct_ser.serialize_field("isPhoneVerified", &self.is_phone_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Phone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phone",
            "is_phone_verified",
            "isPhoneVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phone,
            IsPhoneVerified,
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
                            "phone" => Ok(GeneratedField::Phone),
                            "isPhoneVerified" | "is_phone_verified" => Ok(GeneratedField::IsPhoneVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Phone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Phone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Phone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone__ = None;
                let mut is_phone_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPhoneVerified => {
                            if is_phone_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPhoneVerified"));
                            }
                            is_phone_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Phone {
                    phone: phone__.unwrap_or_default(),
                    is_phone_verified: is_phone_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Phone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Profile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.first_name.is_empty() {
            len += 1;
        }
        if !self.last_name.is_empty() {
            len += 1;
        }
        if !self.nick_name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.preferred_language.is_empty() {
            len += 1;
        }
        if self.gender != 0 {
            len += 1;
        }
        if !self.avatar_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Profile", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if !self.nick_name.is_empty() {
            struct_ser.serialize_field("nickName", &self.nick_name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.preferred_language.is_empty() {
            struct_ser.serialize_field("preferredLanguage", &self.preferred_language)?;
        }
        if self.gender != 0 {
            let v = Gender::try_from(self.gender)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.gender)))?;
            struct_ser.serialize_field("gender", &v)?;
        }
        if !self.avatar_url.is_empty() {
            struct_ser.serialize_field("avatarUrl", &self.avatar_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Profile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "nick_name",
            "nickName",
            "display_name",
            "displayName",
            "preferred_language",
            "preferredLanguage",
            "gender",
            "avatar_url",
            "avatarUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstName,
            LastName,
            NickName,
            DisplayName,
            PreferredLanguage,
            Gender,
            AvatarUrl,
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
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "nickName" | "nick_name" => Ok(GeneratedField::NickName),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "preferredLanguage" | "preferred_language" => Ok(GeneratedField::PreferredLanguage),
                            "gender" => Ok(GeneratedField::Gender),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Profile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Profile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut nick_name__ = None;
                let mut display_name__ = None;
                let mut preferred_language__ = None;
                let mut gender__ = None;
                let mut avatar_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NickName => {
                            if nick_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickName"));
                            }
                            nick_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredLanguage => {
                            if preferred_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguage"));
                            }
                            preferred_language__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gender => {
                            if gender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            gender__ = Some(map_.next_value::<Gender>()? as i32);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Profile {
                    first_name: first_name__.unwrap_or_default(),
                    last_name: last_name__.unwrap_or_default(),
                    nick_name: nick_name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    preferred_language: preferred_language__.unwrap_or_default(),
                    gender: gender__.unwrap_or_default(),
                    avatar_url: avatar_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Profile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.auth_time.is_some() {
            len += 1;
        }
        if self.idle_expiration.is_some() {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        if !self.audience.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.RefreshToken", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if let Some(v) = self.auth_time.as_ref() {
            struct_ser.serialize_field("authTime", v)?;
        }
        if let Some(v) = self.idle_expiration.as_ref() {
            struct_ser.serialize_field("idleExpiration", v)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        if !self.audience.is_empty() {
            struct_ser.serialize_field("audience", &self.audience)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "details",
            "client_id",
            "clientId",
            "auth_time",
            "authTime",
            "idle_expiration",
            "idleExpiration",
            "expiration",
            "scopes",
            "audience",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            ClientId,
            AuthTime,
            IdleExpiration,
            Expiration,
            Scopes,
            Audience,
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
                            "id" => Ok(GeneratedField::Id),
                            "details" => Ok(GeneratedField::Details),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "authTime" | "auth_time" => Ok(GeneratedField::AuthTime),
                            "idleExpiration" | "idle_expiration" => Ok(GeneratedField::IdleExpiration),
                            "expiration" => Ok(GeneratedField::Expiration),
                            "scopes" => Ok(GeneratedField::Scopes),
                            "audience" => Ok(GeneratedField::Audience),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.RefreshToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut client_id__ = None;
                let mut auth_time__ = None;
                let mut idle_expiration__ = None;
                let mut expiration__ = None;
                let mut scopes__ = None;
                let mut audience__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthTime => {
                            if auth_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authTime"));
                            }
                            auth_time__ = map_.next_value()?;
                        }
                        GeneratedField::IdleExpiration => {
                            if idle_expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleExpiration"));
                            }
                            idle_expiration__ = map_.next_value()?;
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Audience => {
                            if audience__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audience"));
                            }
                            audience__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshToken {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    client_id: client_id__.unwrap_or_default(),
                    auth_time: auth_time__,
                    idle_expiration: idle_expiration__,
                    expiration: expiration__,
                    scopes: scopes__.unwrap_or_default(),
                    audience: audience__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.RefreshToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.SearchQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                search_query::Query::UserNameQuery(v) => {
                    struct_ser.serialize_field("userNameQuery", v)?;
                }
                search_query::Query::FirstNameQuery(v) => {
                    struct_ser.serialize_field("firstNameQuery", v)?;
                }
                search_query::Query::LastNameQuery(v) => {
                    struct_ser.serialize_field("lastNameQuery", v)?;
                }
                search_query::Query::NickNameQuery(v) => {
                    struct_ser.serialize_field("nickNameQuery", v)?;
                }
                search_query::Query::DisplayNameQuery(v) => {
                    struct_ser.serialize_field("displayNameQuery", v)?;
                }
                search_query::Query::EmailQuery(v) => {
                    struct_ser.serialize_field("emailQuery", v)?;
                }
                search_query::Query::StateQuery(v) => {
                    struct_ser.serialize_field("stateQuery", v)?;
                }
                search_query::Query::TypeQuery(v) => {
                    struct_ser.serialize_field("typeQuery", v)?;
                }
                search_query::Query::LoginNameQuery(v) => {
                    struct_ser.serialize_field("loginNameQuery", v)?;
                }
                search_query::Query::InUserIdsQuery(v) => {
                    struct_ser.serialize_field("inUserIdsQuery", v)?;
                }
                search_query::Query::OrQuery(v) => {
                    struct_ser.serialize_field("orQuery", v)?;
                }
                search_query::Query::AndQuery(v) => {
                    struct_ser.serialize_field("andQuery", v)?;
                }
                search_query::Query::NotQuery(v) => {
                    struct_ser.serialize_field("notQuery", v)?;
                }
                search_query::Query::InUserEmailsQuery(v) => {
                    struct_ser.serialize_field("inUserEmailsQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name_query",
            "userNameQuery",
            "first_name_query",
            "firstNameQuery",
            "last_name_query",
            "lastNameQuery",
            "nick_name_query",
            "nickNameQuery",
            "display_name_query",
            "displayNameQuery",
            "email_query",
            "emailQuery",
            "state_query",
            "stateQuery",
            "type_query",
            "typeQuery",
            "login_name_query",
            "loginNameQuery",
            "in_user_ids_query",
            "inUserIdsQuery",
            "or_query",
            "orQuery",
            "and_query",
            "andQuery",
            "not_query",
            "notQuery",
            "in_user_emails_query",
            "inUserEmailsQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserNameQuery,
            FirstNameQuery,
            LastNameQuery,
            NickNameQuery,
            DisplayNameQuery,
            EmailQuery,
            StateQuery,
            TypeQuery,
            LoginNameQuery,
            InUserIdsQuery,
            OrQuery,
            AndQuery,
            NotQuery,
            InUserEmailsQuery,
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
                            "userNameQuery" | "user_name_query" => Ok(GeneratedField::UserNameQuery),
                            "firstNameQuery" | "first_name_query" => Ok(GeneratedField::FirstNameQuery),
                            "lastNameQuery" | "last_name_query" => Ok(GeneratedField::LastNameQuery),
                            "nickNameQuery" | "nick_name_query" => Ok(GeneratedField::NickNameQuery),
                            "displayNameQuery" | "display_name_query" => Ok(GeneratedField::DisplayNameQuery),
                            "emailQuery" | "email_query" => Ok(GeneratedField::EmailQuery),
                            "stateQuery" | "state_query" => Ok(GeneratedField::StateQuery),
                            "typeQuery" | "type_query" => Ok(GeneratedField::TypeQuery),
                            "loginNameQuery" | "login_name_query" => Ok(GeneratedField::LoginNameQuery),
                            "inUserIdsQuery" | "in_user_ids_query" => Ok(GeneratedField::InUserIdsQuery),
                            "orQuery" | "or_query" => Ok(GeneratedField::OrQuery),
                            "andQuery" | "and_query" => Ok(GeneratedField::AndQuery),
                            "notQuery" | "not_query" => Ok(GeneratedField::NotQuery),
                            "inUserEmailsQuery" | "in_user_emails_query" => Ok(GeneratedField::InUserEmailsQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.SearchQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::UserNameQuery)
;
                        }
                        GeneratedField::FirstNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::FirstNameQuery)
;
                        }
                        GeneratedField::LastNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::LastNameQuery)
;
                        }
                        GeneratedField::NickNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::NickNameQuery)
;
                        }
                        GeneratedField::DisplayNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::DisplayNameQuery)
;
                        }
                        GeneratedField::EmailQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::EmailQuery)
;
                        }
                        GeneratedField::StateQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::StateQuery)
;
                        }
                        GeneratedField::TypeQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::TypeQuery)
;
                        }
                        GeneratedField::LoginNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::LoginNameQuery)
;
                        }
                        GeneratedField::InUserIdsQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inUserIdsQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::InUserIdsQuery)
;
                        }
                        GeneratedField::OrQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::OrQuery)
;
                        }
                        GeneratedField::AndQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::AndQuery)
;
                        }
                        GeneratedField::NotQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::NotQuery)
;
                        }
                        GeneratedField::InUserEmailsQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inUserEmailsQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::InUserEmailsQuery)
;
                        }
                    }
                }
                Ok(SearchQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.SearchQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Session {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_id.is_empty() {
            len += 1;
        }
        if !self.agent_id.is_empty() {
            len += 1;
        }
        if self.auth_state != 0 {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        if !self.login_name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if !self.avatar_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.Session", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if !self.agent_id.is_empty() {
            struct_ser.serialize_field("agentId", &self.agent_id)?;
        }
        if self.auth_state != 0 {
            let v = SessionState::try_from(self.auth_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.auth_state)))?;
            struct_ser.serialize_field("authState", &v)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if !self.login_name.is_empty() {
            struct_ser.serialize_field("loginName", &self.login_name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.avatar_url.is_empty() {
            struct_ser.serialize_field("avatarUrl", &self.avatar_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Session {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "agent_id",
            "agentId",
            "auth_state",
            "authState",
            "user_id",
            "userId",
            "user_name",
            "userName",
            "login_name",
            "loginName",
            "display_name",
            "displayName",
            "details",
            "avatar_url",
            "avatarUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            AgentId,
            AuthState,
            UserId,
            UserName,
            LoginName,
            DisplayName,
            Details,
            AvatarUrl,
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
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            "agentId" | "agent_id" => Ok(GeneratedField::AgentId),
                            "authState" | "auth_state" => Ok(GeneratedField::AuthState),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "loginName" | "login_name" => Ok(GeneratedField::LoginName),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "details" => Ok(GeneratedField::Details),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Session;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.Session")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Session, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut agent_id__ = None;
                let mut auth_state__ = None;
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut login_name__ = None;
                let mut display_name__ = None;
                let mut details__ = None;
                let mut avatar_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AgentId => {
                            if agent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentId"));
                            }
                            agent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthState => {
                            if auth_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authState"));
                            }
                            auth_state__ = Some(map_.next_value::<SessionState>()? as i32);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginName => {
                            if login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginName"));
                            }
                            login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Session {
                    session_id: session_id__.unwrap_or_default(),
                    agent_id: agent_id__.unwrap_or_default(),
                    auth_state: auth_state__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    login_name: login_name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    details: details__,
                    avatar_url: avatar_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.Session", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SessionState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SESSION_STATE_UNSPECIFIED",
            Self::Active => "SESSION_STATE_ACTIVE",
            Self::Terminated => "SESSION_STATE_TERMINATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SessionState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SESSION_STATE_UNSPECIFIED",
            "SESSION_STATE_ACTIVE",
            "SESSION_STATE_TERMINATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionState;

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
                    "SESSION_STATE_UNSPECIFIED" => Ok(SessionState::Unspecified),
                    "SESSION_STATE_ACTIVE" => Ok(SessionState::Active),
                    "SESSION_STATE_TERMINATED" => Ok(SessionState::Terminated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StateQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.StateQuery", len)?;
        if self.state != 0 {
            let v = UserState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StateQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
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
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StateQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.StateQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StateQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<UserState>()? as i32);
                        }
                    }
                }
                Ok(StateQuery {
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.StateQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TYPE_UNSPECIFIED",
            Self::Human => "TYPE_HUMAN",
            Self::Machine => "TYPE_MACHINE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_UNSPECIFIED",
            "TYPE_HUMAN",
            "TYPE_MACHINE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Type;

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
                    "TYPE_UNSPECIFIED" => Ok(Type::Unspecified),
                    "TYPE_HUMAN" => Ok(Type::Human),
                    "TYPE_MACHINE" => Ok(Type::Machine),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TypeQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.TypeQuery", len)?;
        if self.r#type != 0 {
            let v = Type::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypeQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
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
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypeQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.TypeQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TypeQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<Type>()? as i32);
                        }
                    }
                }
                Ok(TypeQuery {
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.TypeQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        if !self.login_names.is_empty() {
            len += 1;
        }
        if !self.preferred_login_name.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.User", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.state != 0 {
            let v = UserState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if !self.login_names.is_empty() {
            struct_ser.serialize_field("loginNames", &self.login_names)?;
        }
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                user::Type::Human(v) => {
                    struct_ser.serialize_field("human", v)?;
                }
                user::Type::Machine(v) => {
                    struct_ser.serialize_field("machine", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "details",
            "state",
            "user_name",
            "userName",
            "login_names",
            "loginNames",
            "preferred_login_name",
            "preferredLoginName",
            "human",
            "machine",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            State,
            UserName,
            LoginNames,
            PreferredLoginName,
            Human,
            Machine,
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
                            "id" => Ok(GeneratedField::Id),
                            "details" => Ok(GeneratedField::Details),
                            "state" => Ok(GeneratedField::State),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "loginNames" | "login_names" => Ok(GeneratedField::LoginNames),
                            "preferredLoginName" | "preferred_login_name" => Ok(GeneratedField::PreferredLoginName),
                            "human" => Ok(GeneratedField::Human),
                            "machine" => Ok(GeneratedField::Machine),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut state__ = None;
                let mut user_name__ = None;
                let mut login_names__ = None;
                let mut preferred_login_name__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<UserState>()? as i32);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginNames => {
                            if login_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginNames"));
                            }
                            login_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredLoginName => {
                            if preferred_login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLoginName"));
                            }
                            preferred_login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Human => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("human"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(user::Type::Human)
;
                        }
                        GeneratedField::Machine => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machine"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(user::Type::Machine)
;
                        }
                    }
                }
                Ok(User {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    state: state__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    login_names: login_names__.unwrap_or_default(),
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "USER_FIELD_NAME_UNSPECIFIED",
            Self::UserName => "USER_FIELD_NAME_USER_NAME",
            Self::FirstName => "USER_FIELD_NAME_FIRST_NAME",
            Self::LastName => "USER_FIELD_NAME_LAST_NAME",
            Self::NickName => "USER_FIELD_NAME_NICK_NAME",
            Self::DisplayName => "USER_FIELD_NAME_DISPLAY_NAME",
            Self::Email => "USER_FIELD_NAME_EMAIL",
            Self::State => "USER_FIELD_NAME_STATE",
            Self::Type => "USER_FIELD_NAME_TYPE",
            Self::CreationDate => "USER_FIELD_NAME_CREATION_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UserFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USER_FIELD_NAME_UNSPECIFIED",
            "USER_FIELD_NAME_USER_NAME",
            "USER_FIELD_NAME_FIRST_NAME",
            "USER_FIELD_NAME_LAST_NAME",
            "USER_FIELD_NAME_NICK_NAME",
            "USER_FIELD_NAME_DISPLAY_NAME",
            "USER_FIELD_NAME_EMAIL",
            "USER_FIELD_NAME_STATE",
            "USER_FIELD_NAME_TYPE",
            "USER_FIELD_NAME_CREATION_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserFieldName;

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
                    "USER_FIELD_NAME_UNSPECIFIED" => Ok(UserFieldName::Unspecified),
                    "USER_FIELD_NAME_USER_NAME" => Ok(UserFieldName::UserName),
                    "USER_FIELD_NAME_FIRST_NAME" => Ok(UserFieldName::FirstName),
                    "USER_FIELD_NAME_LAST_NAME" => Ok(UserFieldName::LastName),
                    "USER_FIELD_NAME_NICK_NAME" => Ok(UserFieldName::NickName),
                    "USER_FIELD_NAME_DISPLAY_NAME" => Ok(UserFieldName::DisplayName),
                    "USER_FIELD_NAME_EMAIL" => Ok(UserFieldName::Email),
                    "USER_FIELD_NAME_STATE" => Ok(UserFieldName::State),
                    "USER_FIELD_NAME_TYPE" => Ok(UserFieldName::Type),
                    "USER_FIELD_NAME_CREATION_DATE" => Ok(UserFieldName::CreationDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if !self.role_keys.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        if !self.first_name.is_empty() {
            len += 1;
        }
        if !self.last_name.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.org_id.is_empty() {
            len += 1;
        }
        if !self.org_name.is_empty() {
            len += 1;
        }
        if !self.org_domain.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.project_name.is_empty() {
            len += 1;
        }
        if !self.project_grant_id.is_empty() {
            len += 1;
        }
        if !self.avatar_url.is_empty() {
            len += 1;
        }
        if !self.preferred_login_name.is_empty() {
            len += 1;
        }
        if self.user_type != 0 {
            len += 1;
        }
        if !self.granted_org_id.is_empty() {
            len += 1;
        }
        if !self.granted_org_name.is_empty() {
            len += 1;
        }
        if !self.granted_org_domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrant", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.role_keys.is_empty() {
            struct_ser.serialize_field("roleKeys", &self.role_keys)?;
        }
        if self.state != 0 {
            let v = UserGrantState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.org_id.is_empty() {
            struct_ser.serialize_field("orgId", &self.org_id)?;
        }
        if !self.org_name.is_empty() {
            struct_ser.serialize_field("orgName", &self.org_name)?;
        }
        if !self.org_domain.is_empty() {
            struct_ser.serialize_field("orgDomain", &self.org_domain)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.project_name.is_empty() {
            struct_ser.serialize_field("projectName", &self.project_name)?;
        }
        if !self.project_grant_id.is_empty() {
            struct_ser.serialize_field("projectGrantId", &self.project_grant_id)?;
        }
        if !self.avatar_url.is_empty() {
            struct_ser.serialize_field("avatarUrl", &self.avatar_url)?;
        }
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
        }
        if self.user_type != 0 {
            let v = Type::try_from(self.user_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.user_type)))?;
            struct_ser.serialize_field("userType", &v)?;
        }
        if !self.granted_org_id.is_empty() {
            struct_ser.serialize_field("grantedOrgId", &self.granted_org_id)?;
        }
        if !self.granted_org_name.is_empty() {
            struct_ser.serialize_field("grantedOrgName", &self.granted_org_name)?;
        }
        if !self.granted_org_domain.is_empty() {
            struct_ser.serialize_field("grantedOrgDomain", &self.granted_org_domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "details",
            "role_keys",
            "roleKeys",
            "state",
            "user_id",
            "userId",
            "user_name",
            "userName",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "email",
            "display_name",
            "displayName",
            "org_id",
            "orgId",
            "org_name",
            "orgName",
            "org_domain",
            "orgDomain",
            "project_id",
            "projectId",
            "project_name",
            "projectName",
            "project_grant_id",
            "projectGrantId",
            "avatar_url",
            "avatarUrl",
            "preferred_login_name",
            "preferredLoginName",
            "user_type",
            "userType",
            "granted_org_id",
            "grantedOrgId",
            "granted_org_name",
            "grantedOrgName",
            "granted_org_domain",
            "grantedOrgDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            RoleKeys,
            State,
            UserId,
            UserName,
            FirstName,
            LastName,
            Email,
            DisplayName,
            OrgId,
            OrgName,
            OrgDomain,
            ProjectId,
            ProjectName,
            ProjectGrantId,
            AvatarUrl,
            PreferredLoginName,
            UserType,
            GrantedOrgId,
            GrantedOrgName,
            GrantedOrgDomain,
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
                            "id" => Ok(GeneratedField::Id),
                            "details" => Ok(GeneratedField::Details),
                            "roleKeys" | "role_keys" => Ok(GeneratedField::RoleKeys),
                            "state" => Ok(GeneratedField::State),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "email" => Ok(GeneratedField::Email),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            "orgName" | "org_name" => Ok(GeneratedField::OrgName),
                            "orgDomain" | "org_domain" => Ok(GeneratedField::OrgDomain),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "preferredLoginName" | "preferred_login_name" => Ok(GeneratedField::PreferredLoginName),
                            "userType" | "user_type" => Ok(GeneratedField::UserType),
                            "grantedOrgId" | "granted_org_id" => Ok(GeneratedField::GrantedOrgId),
                            "grantedOrgName" | "granted_org_name" => Ok(GeneratedField::GrantedOrgName),
                            "grantedOrgDomain" | "granted_org_domain" => Ok(GeneratedField::GrantedOrgDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut role_keys__ = None;
                let mut state__ = None;
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut email__ = None;
                let mut display_name__ = None;
                let mut org_id__ = None;
                let mut org_name__ = None;
                let mut org_domain__ = None;
                let mut project_id__ = None;
                let mut project_name__ = None;
                let mut project_grant_id__ = None;
                let mut avatar_url__ = None;
                let mut preferred_login_name__ = None;
                let mut user_type__ = None;
                let mut granted_org_id__ = None;
                let mut granted_org_name__ = None;
                let mut granted_org_domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::RoleKeys => {
                            if role_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKeys"));
                            }
                            role_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<UserGrantState>()? as i32);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgName => {
                            if org_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgName"));
                            }
                            org_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgDomain => {
                            if org_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgDomain"));
                            }
                            org_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectName => {
                            if project_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectName"));
                            }
                            project_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectGrantId => {
                            if project_grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            project_grant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredLoginName => {
                            if preferred_login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLoginName"));
                            }
                            preferred_login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserType => {
                            if user_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userType"));
                            }
                            user_type__ = Some(map_.next_value::<Type>()? as i32);
                        }
                        GeneratedField::GrantedOrgId => {
                            if granted_org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgId"));
                            }
                            granted_org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedOrgName => {
                            if granted_org_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgName"));
                            }
                            granted_org_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedOrgDomain => {
                            if granted_org_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgDomain"));
                            }
                            granted_org_domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserGrant {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    role_keys: role_keys__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    first_name: first_name__.unwrap_or_default(),
                    last_name: last_name__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    org_id: org_id__.unwrap_or_default(),
                    org_name: org_name__.unwrap_or_default(),
                    org_domain: org_domain__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    project_name: project_name__.unwrap_or_default(),
                    project_grant_id: project_grant_id__.unwrap_or_default(),
                    avatar_url: avatar_url__.unwrap_or_default(),
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    user_type: user_type__.unwrap_or_default(),
                    granted_org_id: granted_org_id__.unwrap_or_default(),
                    granted_org_name: granted_org_name__.unwrap_or_default(),
                    granted_org_domain: granted_org_domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantDisplayNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantDisplayNameQuery", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantDisplayNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            Method,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantDisplayNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantDisplayNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantDisplayNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantDisplayNameQuery {
                    display_name: display_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantDisplayNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantEmailQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantEmailQuery", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantEmailQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            Method,
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
                            "email" => Ok(GeneratedField::Email),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantEmailQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantEmailQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantEmailQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantEmailQuery {
                    email: email__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantEmailQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantFirstNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.first_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantFirstNameQuery", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantFirstNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "first_name",
            "firstName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstName,
            Method,
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
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantFirstNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantFirstNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantFirstNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantFirstNameQuery {
                    first_name: first_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantFirstNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantLastNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.last_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantLastNameQuery", len)?;
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantLastNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "last_name",
            "lastName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LastName,
            Method,
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
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantLastNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantLastNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantLastNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut last_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantLastNameQuery {
                    last_name: last_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantLastNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantOrgDomainQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.org_domain.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantOrgDomainQuery", len)?;
        if !self.org_domain.is_empty() {
            struct_ser.serialize_field("orgDomain", &self.org_domain)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantOrgDomainQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_domain",
            "orgDomain",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgDomain,
            Method,
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
                            "orgDomain" | "org_domain" => Ok(GeneratedField::OrgDomain),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantOrgDomainQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantOrgDomainQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantOrgDomainQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org_domain__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgDomain => {
                            if org_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgDomain"));
                            }
                            org_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantOrgDomainQuery {
                    org_domain: org_domain__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantOrgDomainQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantOrgNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.org_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantOrgNameQuery", len)?;
        if !self.org_name.is_empty() {
            struct_ser.serialize_field("orgName", &self.org_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantOrgNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_name",
            "orgName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgName,
            Method,
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
                            "orgName" | "org_name" => Ok(GeneratedField::OrgName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantOrgNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantOrgNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantOrgNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgName => {
                            if org_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgName"));
                            }
                            org_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantOrgNameQuery {
                    org_name: org_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantOrgNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantProjectGrantIdQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_grant_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantProjectGrantIDQuery", len)?;
        if !self.project_grant_id.is_empty() {
            struct_ser.serialize_field("projectGrantId", &self.project_grant_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantProjectGrantIdQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_grant_id",
            "projectGrantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectGrantId,
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
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantProjectGrantIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantProjectGrantIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantProjectGrantIdQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_grant_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectGrantId => {
                            if project_grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            project_grant_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserGrantProjectGrantIdQuery {
                    project_grant_id: project_grant_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantProjectGrantIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantProjectIdQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantProjectIDQuery", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantProjectIdQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantProjectIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantProjectIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantProjectIdQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserGrantProjectIdQuery {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantProjectIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantProjectNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantProjectNameQuery", len)?;
        if !self.project_name.is_empty() {
            struct_ser.serialize_field("projectName", &self.project_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantProjectNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_name",
            "projectName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectName,
            Method,
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
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantProjectNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantProjectNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantProjectNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectName => {
                            if project_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectName"));
                            }
                            project_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantProjectNameQuery {
                    project_name: project_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantProjectNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                user_grant_query::Query::ProjectIdQuery(v) => {
                    struct_ser.serialize_field("projectIdQuery", v)?;
                }
                user_grant_query::Query::UserIdQuery(v) => {
                    struct_ser.serialize_field("userIdQuery", v)?;
                }
                user_grant_query::Query::WithGrantedQuery(v) => {
                    struct_ser.serialize_field("withGrantedQuery", v)?;
                }
                user_grant_query::Query::RoleKeyQuery(v) => {
                    struct_ser.serialize_field("roleKeyQuery", v)?;
                }
                user_grant_query::Query::ProjectGrantIdQuery(v) => {
                    struct_ser.serialize_field("projectGrantIdQuery", v)?;
                }
                user_grant_query::Query::UserNameQuery(v) => {
                    struct_ser.serialize_field("userNameQuery", v)?;
                }
                user_grant_query::Query::FirstNameQuery(v) => {
                    struct_ser.serialize_field("firstNameQuery", v)?;
                }
                user_grant_query::Query::LastNameQuery(v) => {
                    struct_ser.serialize_field("lastNameQuery", v)?;
                }
                user_grant_query::Query::EmailQuery(v) => {
                    struct_ser.serialize_field("emailQuery", v)?;
                }
                user_grant_query::Query::OrgNameQuery(v) => {
                    struct_ser.serialize_field("orgNameQuery", v)?;
                }
                user_grant_query::Query::OrgDomainQuery(v) => {
                    struct_ser.serialize_field("orgDomainQuery", v)?;
                }
                user_grant_query::Query::ProjectNameQuery(v) => {
                    struct_ser.serialize_field("projectNameQuery", v)?;
                }
                user_grant_query::Query::DisplayNameQuery(v) => {
                    struct_ser.serialize_field("displayNameQuery", v)?;
                }
                user_grant_query::Query::UserTypeQuery(v) => {
                    struct_ser.serialize_field("userTypeQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id_query",
            "projectIdQuery",
            "user_id_query",
            "userIdQuery",
            "with_granted_query",
            "withGrantedQuery",
            "role_key_query",
            "roleKeyQuery",
            "project_grant_id_query",
            "projectGrantIdQuery",
            "user_name_query",
            "userNameQuery",
            "first_name_query",
            "firstNameQuery",
            "last_name_query",
            "lastNameQuery",
            "email_query",
            "emailQuery",
            "org_name_query",
            "orgNameQuery",
            "org_domain_query",
            "orgDomainQuery",
            "project_name_query",
            "projectNameQuery",
            "display_name_query",
            "displayNameQuery",
            "user_type_query",
            "userTypeQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectIdQuery,
            UserIdQuery,
            WithGrantedQuery,
            RoleKeyQuery,
            ProjectGrantIdQuery,
            UserNameQuery,
            FirstNameQuery,
            LastNameQuery,
            EmailQuery,
            OrgNameQuery,
            OrgDomainQuery,
            ProjectNameQuery,
            DisplayNameQuery,
            UserTypeQuery,
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
                            "projectIdQuery" | "project_id_query" => Ok(GeneratedField::ProjectIdQuery),
                            "userIdQuery" | "user_id_query" => Ok(GeneratedField::UserIdQuery),
                            "withGrantedQuery" | "with_granted_query" => Ok(GeneratedField::WithGrantedQuery),
                            "roleKeyQuery" | "role_key_query" => Ok(GeneratedField::RoleKeyQuery),
                            "projectGrantIdQuery" | "project_grant_id_query" => Ok(GeneratedField::ProjectGrantIdQuery),
                            "userNameQuery" | "user_name_query" => Ok(GeneratedField::UserNameQuery),
                            "firstNameQuery" | "first_name_query" => Ok(GeneratedField::FirstNameQuery),
                            "lastNameQuery" | "last_name_query" => Ok(GeneratedField::LastNameQuery),
                            "emailQuery" | "email_query" => Ok(GeneratedField::EmailQuery),
                            "orgNameQuery" | "org_name_query" => Ok(GeneratedField::OrgNameQuery),
                            "orgDomainQuery" | "org_domain_query" => Ok(GeneratedField::OrgDomainQuery),
                            "projectNameQuery" | "project_name_query" => Ok(GeneratedField::ProjectNameQuery),
                            "displayNameQuery" | "display_name_query" => Ok(GeneratedField::DisplayNameQuery),
                            "userTypeQuery" | "user_type_query" => Ok(GeneratedField::UserTypeQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::ProjectIdQuery)
;
                        }
                        GeneratedField::UserIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::UserIdQuery)
;
                        }
                        GeneratedField::WithGrantedQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withGrantedQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::WithGrantedQuery)
;
                        }
                        GeneratedField::RoleKeyQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKeyQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::RoleKeyQuery)
;
                        }
                        GeneratedField::ProjectGrantIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::ProjectGrantIdQuery)
;
                        }
                        GeneratedField::UserNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::UserNameQuery)
;
                        }
                        GeneratedField::FirstNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::FirstNameQuery)
;
                        }
                        GeneratedField::LastNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::LastNameQuery)
;
                        }
                        GeneratedField::EmailQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::EmailQuery)
;
                        }
                        GeneratedField::OrgNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::OrgNameQuery)
;
                        }
                        GeneratedField::OrgDomainQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgDomainQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::OrgDomainQuery)
;
                        }
                        GeneratedField::ProjectNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::ProjectNameQuery)
;
                        }
                        GeneratedField::DisplayNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::DisplayNameQuery)
;
                        }
                        GeneratedField::UserTypeQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userTypeQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(user_grant_query::Query::UserTypeQuery)
;
                        }
                    }
                }
                Ok(UserGrantQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantRoleKeyQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.role_key.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantRoleKeyQuery", len)?;
        if !self.role_key.is_empty() {
            struct_ser.serialize_field("roleKey", &self.role_key)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantRoleKeyQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "role_key",
            "roleKey",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoleKey,
            Method,
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
                            "roleKey" | "role_key" => Ok(GeneratedField::RoleKey),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantRoleKeyQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantRoleKeyQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantRoleKeyQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut role_key__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoleKey => {
                            if role_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKey"));
                            }
                            role_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantRoleKeyQuery {
                    role_key: role_key__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantRoleKeyQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "USER_GRANT_STATE_UNSPECIFIED",
            Self::Active => "USER_GRANT_STATE_ACTIVE",
            Self::Inactive => "USER_GRANT_STATE_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USER_GRANT_STATE_UNSPECIFIED",
            "USER_GRANT_STATE_ACTIVE",
            "USER_GRANT_STATE_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantState;

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
                    "USER_GRANT_STATE_UNSPECIFIED" => Ok(UserGrantState::Unspecified),
                    "USER_GRANT_STATE_ACTIVE" => Ok(UserGrantState::Active),
                    "USER_GRANT_STATE_INACTIVE" => Ok(UserGrantState::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantUserIdQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantUserIDQuery", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantUserIdQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantUserIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantUserIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantUserIdQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserGrantUserIdQuery {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantUserIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantUserNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantUserNameQuery", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantUserNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name",
            "userName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserName,
            Method,
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantUserNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantUserNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantUserNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserGrantUserNameQuery {
                    user_name: user_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantUserNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantUserTypeQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantUserTypeQuery", len)?;
        if self.r#type != 0 {
            let v = Type::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantUserTypeQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
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
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantUserTypeQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantUserTypeQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantUserTypeQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<Type>()? as i32);
                        }
                    }
                }
                Ok(UserGrantUserTypeQuery {
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantUserTypeQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGrantWithGrantedQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.with_granted {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserGrantWithGrantedQuery", len)?;
        if self.with_granted {
            struct_ser.serialize_field("withGranted", &self.with_granted)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGrantWithGrantedQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "with_granted",
            "withGranted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WithGranted,
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
                            "withGranted" | "with_granted" => Ok(GeneratedField::WithGranted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGrantWithGrantedQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserGrantWithGrantedQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGrantWithGrantedQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut with_granted__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WithGranted => {
                            if with_granted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withGranted"));
                            }
                            with_granted__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserGrantWithGrantedQuery {
                    with_granted: with_granted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserGrantWithGrantedQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserNameQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.UserNameQuery", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name",
            "userName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserName,
            Method,
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.UserNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserNameQuery {
                    user_name: user_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.UserNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "USER_STATE_UNSPECIFIED",
            Self::Active => "USER_STATE_ACTIVE",
            Self::Inactive => "USER_STATE_INACTIVE",
            Self::Deleted => "USER_STATE_DELETED",
            Self::Locked => "USER_STATE_LOCKED",
            Self::Suspend => "USER_STATE_SUSPEND",
            Self::Initial => "USER_STATE_INITIAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UserState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USER_STATE_UNSPECIFIED",
            "USER_STATE_ACTIVE",
            "USER_STATE_INACTIVE",
            "USER_STATE_DELETED",
            "USER_STATE_LOCKED",
            "USER_STATE_SUSPEND",
            "USER_STATE_INITIAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserState;

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
                    "USER_STATE_UNSPECIFIED" => Ok(UserState::Unspecified),
                    "USER_STATE_ACTIVE" => Ok(UserState::Active),
                    "USER_STATE_INACTIVE" => Ok(UserState::Inactive),
                    "USER_STATE_DELETED" => Ok(UserState::Deleted),
                    "USER_STATE_LOCKED" => Ok(UserState::Locked),
                    "USER_STATE_SUSPEND" => Ok(UserState::Suspend),
                    "USER_STATE_INITIAL" => Ok(UserState::Initial),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebAuthNKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.WebAuthNKey", len)?;
        if !self.public_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebAuthNKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKey,
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
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebAuthNKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.WebAuthNKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebAuthNKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WebAuthNKey {
                    public_key: public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.WebAuthNKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebAuthNToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.WebAuthNToken", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.state != 0 {
            let v = AuthFactorState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebAuthNToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "state",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            State,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "state" => Ok(GeneratedField::State),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebAuthNToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.WebAuthNToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebAuthNToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut state__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                            state__ = Some(map_.next_value::<AuthFactorState>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WebAuthNToken {
                    id: id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.WebAuthNToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebAuthNVerification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_key_credential.is_empty() {
            len += 1;
        }
        if !self.token_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v1.WebAuthNVerification", len)?;
        if !self.public_key_credential.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("publicKeyCredential", pbjson::private::base64::encode(&self.public_key_credential).as_str())?;
        }
        if !self.token_name.is_empty() {
            struct_ser.serialize_field("tokenName", &self.token_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebAuthNVerification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_key_credential",
            "publicKeyCredential",
            "token_name",
            "tokenName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKeyCredential,
            TokenName,
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
                            "publicKeyCredential" | "public_key_credential" => Ok(GeneratedField::PublicKeyCredential),
                            "tokenName" | "token_name" => Ok(GeneratedField::TokenName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebAuthNVerification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v1.WebAuthNVerification")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebAuthNVerification, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key_credential__ = None;
                let mut token_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicKeyCredential => {
                            if public_key_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCredential"));
                            }
                            public_key_credential__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TokenName => {
                            if token_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenName"));
                            }
                            token_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WebAuthNVerification {
                    public_key_credential: public_key_credential__.unwrap_or_default(),
                    token_name: token_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v1.WebAuthNVerification", FIELDS, GeneratedVisitor)
    }
}
