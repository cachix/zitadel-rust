// @generated
impl serde::Serialize for ApiAuthMethodType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Basic => "API_AUTH_METHOD_TYPE_BASIC",
            Self::PrivateKeyJwt => "API_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ApiAuthMethodType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "API_AUTH_METHOD_TYPE_BASIC",
            "API_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiAuthMethodType;

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
                    "API_AUTH_METHOD_TYPE_BASIC" => Ok(ApiAuthMethodType::Basic),
                    "API_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT" => Ok(ApiAuthMethodType::PrivateKeyJwt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ApiConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.auth_method_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.APIConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if self.auth_method_type != 0 {
            let v = ApiAuthMethodType::try_from(self.auth_method_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.auth_method_type)))?;
            struct_ser.serialize_field("authMethodType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "auth_method_type",
            "authMethodType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            AuthMethodType,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "authMethodType" | "auth_method_type" => Ok(GeneratedField::AuthMethodType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.APIConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApiConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut auth_method_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthMethodType => {
                            if auth_method_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authMethodType"));
                            }
                            auth_method_type__ = Some(map_.next_value::<ApiAuthMethodType>()? as i32);
                        }
                    }
                }
                Ok(ApiConfig {
                    client_id: client_id__.unwrap_or_default(),
                    auth_method_type: auth_method_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.APIConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for App {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.App", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.state != 0 {
            let v = AppState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                app::Config::OidcConfig(v) => {
                    struct_ser.serialize_field("oidcConfig", v)?;
                }
                app::Config::ApiConfig(v) => {
                    struct_ser.serialize_field("apiConfig", v)?;
                }
                app::Config::SamlConfig(v) => {
                    struct_ser.serialize_field("samlConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for App {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "details",
            "state",
            "name",
            "oidc_config",
            "oidcConfig",
            "api_config",
            "apiConfig",
            "saml_config",
            "samlConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            State,
            Name,
            OidcConfig,
            ApiConfig,
            SamlConfig,
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
                            "name" => Ok(GeneratedField::Name),
                            "oidcConfig" | "oidc_config" => Ok(GeneratedField::OidcConfig),
                            "apiConfig" | "api_config" => Ok(GeneratedField::ApiConfig),
                            "samlConfig" | "saml_config" => Ok(GeneratedField::SamlConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = App;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.App")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<App, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut state__ = None;
                let mut name__ = None;
                let mut config__ = None;
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
                            state__ = Some(map_.next_value::<AppState>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OidcConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(app::Config::OidcConfig)
;
                        }
                        GeneratedField::ApiConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(app::Config::ApiConfig)
;
                        }
                        GeneratedField::SamlConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(app::Config::SamlConfig)
;
                        }
                    }
                }
                Ok(App {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    state: state__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.App", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AppNameQuery {
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
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.AppNameQuery", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppNameQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = AppNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.AppNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AppNameQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(AppNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.AppNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AppQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.AppQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                app_query::Query::NameQuery(v) => {
                    struct_ser.serialize_field("nameQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name_query",
            "nameQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NameQuery,
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
                            "nameQuery" | "name_query" => Ok(GeneratedField::NameQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.AppQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AppQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(app_query::Query::NameQuery)
;
                        }
                    }
                }
                Ok(AppQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.AppQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AppState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "APP_STATE_UNSPECIFIED",
            Self::Active => "APP_STATE_ACTIVE",
            Self::Inactive => "APP_STATE_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AppState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "APP_STATE_UNSPECIFIED",
            "APP_STATE_ACTIVE",
            "APP_STATE_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppState;

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
                    "APP_STATE_UNSPECIFIED" => Ok(AppState::Unspecified),
                    "APP_STATE_ACTIVE" => Ok(AppState::Active),
                    "APP_STATE_INACTIVE" => Ok(AppState::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LoginV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.app.v1.LoginV1", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginV1 {
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
            type Value = LoginV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.LoginV1")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LoginV1 {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.LoginV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginV2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_uri.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.LoginV2", len)?;
        if let Some(v) = self.base_uri.as_ref() {
            struct_ser.serialize_field("baseUri", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginV2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_uri",
            "baseUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseUri,
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
                            "baseUri" | "base_uri" => Ok(GeneratedField::BaseUri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginV2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.LoginV2")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginV2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_uri__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BaseUri => {
                            if base_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseUri"));
                            }
                            base_uri__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginV2 {
                    base_uri: base_uri__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.LoginV2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.LoginVersion", len)?;
        if let Some(v) = self.version.as_ref() {
            match v {
                login_version::Version::LoginV1(v) => {
                    struct_ser.serialize_field("loginV1", v)?;
                }
                login_version::Version::LoginV2(v) => {
                    struct_ser.serialize_field("loginV2", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login_v1",
            "loginV1",
            "login_v2",
            "loginV2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoginV1,
            LoginV2,
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
                            "loginV1" | "login_v1" => Ok(GeneratedField::LoginV1),
                            "loginV2" | "login_v2" => Ok(GeneratedField::LoginV2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.LoginVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoginV1 => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginV1"));
                            }
                            version__ = map_.next_value::<::std::option::Option<_>>()?.map(login_version::Version::LoginV1)
;
                        }
                        GeneratedField::LoginV2 => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginV2"));
                            }
                            version__ = map_.next_value::<::std::option::Option<_>>()?.map(login_version::Version::LoginV2)
;
                        }
                    }
                }
                Ok(LoginVersion {
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.LoginVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OidcAppType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Web => "OIDC_APP_TYPE_WEB",
            Self::UserAgent => "OIDC_APP_TYPE_USER_AGENT",
            Self::Native => "OIDC_APP_TYPE_NATIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcAppType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_APP_TYPE_WEB",
            "OIDC_APP_TYPE_USER_AGENT",
            "OIDC_APP_TYPE_NATIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcAppType;

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
                    "OIDC_APP_TYPE_WEB" => Ok(OidcAppType::Web),
                    "OIDC_APP_TYPE_USER_AGENT" => Ok(OidcAppType::UserAgent),
                    "OIDC_APP_TYPE_NATIVE" => Ok(OidcAppType::Native),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OidcAuthMethodType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Basic => "OIDC_AUTH_METHOD_TYPE_BASIC",
            Self::Post => "OIDC_AUTH_METHOD_TYPE_POST",
            Self::None => "OIDC_AUTH_METHOD_TYPE_NONE",
            Self::PrivateKeyJwt => "OIDC_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcAuthMethodType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_AUTH_METHOD_TYPE_BASIC",
            "OIDC_AUTH_METHOD_TYPE_POST",
            "OIDC_AUTH_METHOD_TYPE_NONE",
            "OIDC_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcAuthMethodType;

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
                    "OIDC_AUTH_METHOD_TYPE_BASIC" => Ok(OidcAuthMethodType::Basic),
                    "OIDC_AUTH_METHOD_TYPE_POST" => Ok(OidcAuthMethodType::Post),
                    "OIDC_AUTH_METHOD_TYPE_NONE" => Ok(OidcAuthMethodType::None),
                    "OIDC_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT" => Ok(OidcAuthMethodType::PrivateKeyJwt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OidcConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.redirect_uris.is_empty() {
            len += 1;
        }
        if !self.response_types.is_empty() {
            len += 1;
        }
        if !self.grant_types.is_empty() {
            len += 1;
        }
        if self.app_type != 0 {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.auth_method_type != 0 {
            len += 1;
        }
        if !self.post_logout_redirect_uris.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if self.none_compliant {
            len += 1;
        }
        if !self.compliance_problems.is_empty() {
            len += 1;
        }
        if self.dev_mode {
            len += 1;
        }
        if self.access_token_type != 0 {
            len += 1;
        }
        if self.access_token_role_assertion {
            len += 1;
        }
        if self.id_token_role_assertion {
            len += 1;
        }
        if self.id_token_userinfo_assertion {
            len += 1;
        }
        if self.clock_skew.is_some() {
            len += 1;
        }
        if !self.additional_origins.is_empty() {
            len += 1;
        }
        if !self.allowed_origins.is_empty() {
            len += 1;
        }
        if self.skip_native_app_success_page {
            len += 1;
        }
        if !self.back_channel_logout_uri.is_empty() {
            len += 1;
        }
        if self.login_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.OIDCConfig", len)?;
        if !self.redirect_uris.is_empty() {
            struct_ser.serialize_field("redirectUris", &self.redirect_uris)?;
        }
        if !self.response_types.is_empty() {
            let v = self.response_types.iter().cloned().map(|v| {
                OidcResponseType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("responseTypes", &v)?;
        }
        if !self.grant_types.is_empty() {
            let v = self.grant_types.iter().cloned().map(|v| {
                OidcGrantType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("grantTypes", &v)?;
        }
        if self.app_type != 0 {
            let v = OidcAppType::try_from(self.app_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.app_type)))?;
            struct_ser.serialize_field("appType", &v)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if self.auth_method_type != 0 {
            let v = OidcAuthMethodType::try_from(self.auth_method_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.auth_method_type)))?;
            struct_ser.serialize_field("authMethodType", &v)?;
        }
        if !self.post_logout_redirect_uris.is_empty() {
            struct_ser.serialize_field("postLogoutRedirectUris", &self.post_logout_redirect_uris)?;
        }
        if self.version != 0 {
            let v = OidcVersion::try_from(self.version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.version)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        if self.none_compliant {
            struct_ser.serialize_field("noneCompliant", &self.none_compliant)?;
        }
        if !self.compliance_problems.is_empty() {
            struct_ser.serialize_field("complianceProblems", &self.compliance_problems)?;
        }
        if self.dev_mode {
            struct_ser.serialize_field("devMode", &self.dev_mode)?;
        }
        if self.access_token_type != 0 {
            let v = OidcTokenType::try_from(self.access_token_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.access_token_type)))?;
            struct_ser.serialize_field("accessTokenType", &v)?;
        }
        if self.access_token_role_assertion {
            struct_ser.serialize_field("accessTokenRoleAssertion", &self.access_token_role_assertion)?;
        }
        if self.id_token_role_assertion {
            struct_ser.serialize_field("idTokenRoleAssertion", &self.id_token_role_assertion)?;
        }
        if self.id_token_userinfo_assertion {
            struct_ser.serialize_field("idTokenUserinfoAssertion", &self.id_token_userinfo_assertion)?;
        }
        if let Some(v) = self.clock_skew.as_ref() {
            struct_ser.serialize_field("clockSkew", v)?;
        }
        if !self.additional_origins.is_empty() {
            struct_ser.serialize_field("additionalOrigins", &self.additional_origins)?;
        }
        if !self.allowed_origins.is_empty() {
            struct_ser.serialize_field("allowedOrigins", &self.allowed_origins)?;
        }
        if self.skip_native_app_success_page {
            struct_ser.serialize_field("skipNativeAppSuccessPage", &self.skip_native_app_success_page)?;
        }
        if !self.back_channel_logout_uri.is_empty() {
            struct_ser.serialize_field("backChannelLogoutUri", &self.back_channel_logout_uri)?;
        }
        if let Some(v) = self.login_version.as_ref() {
            struct_ser.serialize_field("loginVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OidcConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "redirect_uris",
            "redirectUris",
            "response_types",
            "responseTypes",
            "grant_types",
            "grantTypes",
            "app_type",
            "appType",
            "client_id",
            "clientId",
            "auth_method_type",
            "authMethodType",
            "post_logout_redirect_uris",
            "postLogoutRedirectUris",
            "version",
            "none_compliant",
            "noneCompliant",
            "compliance_problems",
            "complianceProblems",
            "dev_mode",
            "devMode",
            "access_token_type",
            "accessTokenType",
            "access_token_role_assertion",
            "accessTokenRoleAssertion",
            "id_token_role_assertion",
            "idTokenRoleAssertion",
            "id_token_userinfo_assertion",
            "idTokenUserinfoAssertion",
            "clock_skew",
            "clockSkew",
            "additional_origins",
            "additionalOrigins",
            "allowed_origins",
            "allowedOrigins",
            "skip_native_app_success_page",
            "skipNativeAppSuccessPage",
            "back_channel_logout_uri",
            "backChannelLogoutUri",
            "login_version",
            "loginVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RedirectUris,
            ResponseTypes,
            GrantTypes,
            AppType,
            ClientId,
            AuthMethodType,
            PostLogoutRedirectUris,
            Version,
            NoneCompliant,
            ComplianceProblems,
            DevMode,
            AccessTokenType,
            AccessTokenRoleAssertion,
            IdTokenRoleAssertion,
            IdTokenUserinfoAssertion,
            ClockSkew,
            AdditionalOrigins,
            AllowedOrigins,
            SkipNativeAppSuccessPage,
            BackChannelLogoutUri,
            LoginVersion,
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
                            "redirectUris" | "redirect_uris" => Ok(GeneratedField::RedirectUris),
                            "responseTypes" | "response_types" => Ok(GeneratedField::ResponseTypes),
                            "grantTypes" | "grant_types" => Ok(GeneratedField::GrantTypes),
                            "appType" | "app_type" => Ok(GeneratedField::AppType),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "authMethodType" | "auth_method_type" => Ok(GeneratedField::AuthMethodType),
                            "postLogoutRedirectUris" | "post_logout_redirect_uris" => Ok(GeneratedField::PostLogoutRedirectUris),
                            "version" => Ok(GeneratedField::Version),
                            "noneCompliant" | "none_compliant" => Ok(GeneratedField::NoneCompliant),
                            "complianceProblems" | "compliance_problems" => Ok(GeneratedField::ComplianceProblems),
                            "devMode" | "dev_mode" => Ok(GeneratedField::DevMode),
                            "accessTokenType" | "access_token_type" => Ok(GeneratedField::AccessTokenType),
                            "accessTokenRoleAssertion" | "access_token_role_assertion" => Ok(GeneratedField::AccessTokenRoleAssertion),
                            "idTokenRoleAssertion" | "id_token_role_assertion" => Ok(GeneratedField::IdTokenRoleAssertion),
                            "idTokenUserinfoAssertion" | "id_token_userinfo_assertion" => Ok(GeneratedField::IdTokenUserinfoAssertion),
                            "clockSkew" | "clock_skew" => Ok(GeneratedField::ClockSkew),
                            "additionalOrigins" | "additional_origins" => Ok(GeneratedField::AdditionalOrigins),
                            "allowedOrigins" | "allowed_origins" => Ok(GeneratedField::AllowedOrigins),
                            "skipNativeAppSuccessPage" | "skip_native_app_success_page" => Ok(GeneratedField::SkipNativeAppSuccessPage),
                            "backChannelLogoutUri" | "back_channel_logout_uri" => Ok(GeneratedField::BackChannelLogoutUri),
                            "loginVersion" | "login_version" => Ok(GeneratedField::LoginVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.OIDCConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OidcConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut redirect_uris__ = None;
                let mut response_types__ = None;
                let mut grant_types__ = None;
                let mut app_type__ = None;
                let mut client_id__ = None;
                let mut auth_method_type__ = None;
                let mut post_logout_redirect_uris__ = None;
                let mut version__ = None;
                let mut none_compliant__ = None;
                let mut compliance_problems__ = None;
                let mut dev_mode__ = None;
                let mut access_token_type__ = None;
                let mut access_token_role_assertion__ = None;
                let mut id_token_role_assertion__ = None;
                let mut id_token_userinfo_assertion__ = None;
                let mut clock_skew__ = None;
                let mut additional_origins__ = None;
                let mut allowed_origins__ = None;
                let mut skip_native_app_success_page__ = None;
                let mut back_channel_logout_uri__ = None;
                let mut login_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RedirectUris => {
                            if redirect_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectUris"));
                            }
                            redirect_uris__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResponseTypes => {
                            if response_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTypes"));
                            }
                            response_types__ = Some(map_.next_value::<Vec<OidcResponseType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::GrantTypes => {
                            if grant_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantTypes"));
                            }
                            grant_types__ = Some(map_.next_value::<Vec<OidcGrantType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::AppType => {
                            if app_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appType"));
                            }
                            app_type__ = Some(map_.next_value::<OidcAppType>()? as i32);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthMethodType => {
                            if auth_method_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authMethodType"));
                            }
                            auth_method_type__ = Some(map_.next_value::<OidcAuthMethodType>()? as i32);
                        }
                        GeneratedField::PostLogoutRedirectUris => {
                            if post_logout_redirect_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postLogoutRedirectUris"));
                            }
                            post_logout_redirect_uris__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value::<OidcVersion>()? as i32);
                        }
                        GeneratedField::NoneCompliant => {
                            if none_compliant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noneCompliant"));
                            }
                            none_compliant__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ComplianceProblems => {
                            if compliance_problems__.is_some() {
                                return Err(serde::de::Error::duplicate_field("complianceProblems"));
                            }
                            compliance_problems__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DevMode => {
                            if dev_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devMode"));
                            }
                            dev_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccessTokenType => {
                            if access_token_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenType"));
                            }
                            access_token_type__ = Some(map_.next_value::<OidcTokenType>()? as i32);
                        }
                        GeneratedField::AccessTokenRoleAssertion => {
                            if access_token_role_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenRoleAssertion"));
                            }
                            access_token_role_assertion__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdTokenRoleAssertion => {
                            if id_token_role_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idTokenRoleAssertion"));
                            }
                            id_token_role_assertion__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdTokenUserinfoAssertion => {
                            if id_token_userinfo_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idTokenUserinfoAssertion"));
                            }
                            id_token_userinfo_assertion__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClockSkew => {
                            if clock_skew__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clockSkew"));
                            }
                            clock_skew__ = map_.next_value()?;
                        }
                        GeneratedField::AdditionalOrigins => {
                            if additional_origins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalOrigins"));
                            }
                            additional_origins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedOrigins => {
                            if allowed_origins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedOrigins"));
                            }
                            allowed_origins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipNativeAppSuccessPage => {
                            if skip_native_app_success_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipNativeAppSuccessPage"));
                            }
                            skip_native_app_success_page__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackChannelLogoutUri => {
                            if back_channel_logout_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backChannelLogoutUri"));
                            }
                            back_channel_logout_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginVersion => {
                            if login_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginVersion"));
                            }
                            login_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OidcConfig {
                    redirect_uris: redirect_uris__.unwrap_or_default(),
                    response_types: response_types__.unwrap_or_default(),
                    grant_types: grant_types__.unwrap_or_default(),
                    app_type: app_type__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    auth_method_type: auth_method_type__.unwrap_or_default(),
                    post_logout_redirect_uris: post_logout_redirect_uris__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    none_compliant: none_compliant__.unwrap_or_default(),
                    compliance_problems: compliance_problems__.unwrap_or_default(),
                    dev_mode: dev_mode__.unwrap_or_default(),
                    access_token_type: access_token_type__.unwrap_or_default(),
                    access_token_role_assertion: access_token_role_assertion__.unwrap_or_default(),
                    id_token_role_assertion: id_token_role_assertion__.unwrap_or_default(),
                    id_token_userinfo_assertion: id_token_userinfo_assertion__.unwrap_or_default(),
                    clock_skew: clock_skew__,
                    additional_origins: additional_origins__.unwrap_or_default(),
                    allowed_origins: allowed_origins__.unwrap_or_default(),
                    skip_native_app_success_page: skip_native_app_success_page__.unwrap_or_default(),
                    back_channel_logout_uri: back_channel_logout_uri__.unwrap_or_default(),
                    login_version: login_version__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.OIDCConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OidcGrantType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AuthorizationCode => "OIDC_GRANT_TYPE_AUTHORIZATION_CODE",
            Self::Implicit => "OIDC_GRANT_TYPE_IMPLICIT",
            Self::RefreshToken => "OIDC_GRANT_TYPE_REFRESH_TOKEN",
            Self::DeviceCode => "OIDC_GRANT_TYPE_DEVICE_CODE",
            Self::TokenExchange => "OIDC_GRANT_TYPE_TOKEN_EXCHANGE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcGrantType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_GRANT_TYPE_AUTHORIZATION_CODE",
            "OIDC_GRANT_TYPE_IMPLICIT",
            "OIDC_GRANT_TYPE_REFRESH_TOKEN",
            "OIDC_GRANT_TYPE_DEVICE_CODE",
            "OIDC_GRANT_TYPE_TOKEN_EXCHANGE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcGrantType;

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
                    "OIDC_GRANT_TYPE_AUTHORIZATION_CODE" => Ok(OidcGrantType::AuthorizationCode),
                    "OIDC_GRANT_TYPE_IMPLICIT" => Ok(OidcGrantType::Implicit),
                    "OIDC_GRANT_TYPE_REFRESH_TOKEN" => Ok(OidcGrantType::RefreshToken),
                    "OIDC_GRANT_TYPE_DEVICE_CODE" => Ok(OidcGrantType::DeviceCode),
                    "OIDC_GRANT_TYPE_TOKEN_EXCHANGE" => Ok(OidcGrantType::TokenExchange),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OidcResponseType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Code => "OIDC_RESPONSE_TYPE_CODE",
            Self::IdToken => "OIDC_RESPONSE_TYPE_ID_TOKEN",
            Self::IdTokenToken => "OIDC_RESPONSE_TYPE_ID_TOKEN_TOKEN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcResponseType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_RESPONSE_TYPE_CODE",
            "OIDC_RESPONSE_TYPE_ID_TOKEN",
            "OIDC_RESPONSE_TYPE_ID_TOKEN_TOKEN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcResponseType;

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
                    "OIDC_RESPONSE_TYPE_CODE" => Ok(OidcResponseType::Code),
                    "OIDC_RESPONSE_TYPE_ID_TOKEN" => Ok(OidcResponseType::IdToken),
                    "OIDC_RESPONSE_TYPE_ID_TOKEN_TOKEN" => Ok(OidcResponseType::IdTokenToken),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OidcTokenType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Bearer => "OIDC_TOKEN_TYPE_BEARER",
            Self::Jwt => "OIDC_TOKEN_TYPE_JWT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcTokenType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_TOKEN_TYPE_BEARER",
            "OIDC_TOKEN_TYPE_JWT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcTokenType;

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
                    "OIDC_TOKEN_TYPE_BEARER" => Ok(OidcTokenType::Bearer),
                    "OIDC_TOKEN_TYPE_JWT" => Ok(OidcTokenType::Jwt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OidcVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::OidcVersion10 => "OIDC_VERSION_1_0",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_VERSION_1_0",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcVersion;

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
                    "OIDC_VERSION_1_0" => Ok(OidcVersion::OidcVersion10),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SamlConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.login_version.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v1.SAMLConfig", len)?;
        if let Some(v) = self.login_version.as_ref() {
            struct_ser.serialize_field("loginVersion", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            match v {
                saml_config::Metadata::MetadataXml(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("metadataXml", pbjson::private::base64::encode(&v).as_str())?;
                }
                saml_config::Metadata::MetadataUrl(v) => {
                    struct_ser.serialize_field("metadataUrl", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SamlConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login_version",
            "loginVersion",
            "metadata_xml",
            "metadataXml",
            "metadata_url",
            "metadataUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoginVersion,
            MetadataXml,
            MetadataUrl,
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
                            "loginVersion" | "login_version" => Ok(GeneratedField::LoginVersion),
                            "metadataXml" | "metadata_xml" => Ok(GeneratedField::MetadataXml),
                            "metadataUrl" | "metadata_url" => Ok(GeneratedField::MetadataUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SamlConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v1.SAMLConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SamlConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut login_version__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoginVersion => {
                            if login_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginVersion"));
                            }
                            login_version__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataXml => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataXml"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| saml_config::Metadata::MetadataXml(x.0));
                        }
                        GeneratedField::MetadataUrl => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUrl"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(saml_config::Metadata::MetadataUrl);
                        }
                    }
                }
                Ok(SamlConfig {
                    login_version: login_version__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v1.SAMLConfig", FIELDS, GeneratedVisitor)
    }
}
