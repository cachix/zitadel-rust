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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.APIConfig", len)?;
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
                formatter.write_str("struct zitadel.app.v2beta.APIConfig")
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
        deserializer.deserialize_struct("zitadel.app.v2beta.APIConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AppSorting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AppSortById => "APP_SORT_BY_ID",
            Self::AppSortByName => "APP_SORT_BY_NAME",
            Self::AppSortByState => "APP_SORT_BY_STATE",
            Self::AppSortByCreationDate => "APP_SORT_BY_CREATION_DATE",
            Self::AppSortByChangeDate => "APP_SORT_BY_CHANGE_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AppSorting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "APP_SORT_BY_ID",
            "APP_SORT_BY_NAME",
            "APP_SORT_BY_STATE",
            "APP_SORT_BY_CREATION_DATE",
            "APP_SORT_BY_CHANGE_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppSorting;

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
                    "APP_SORT_BY_ID" => Ok(AppSorting::AppSortById),
                    "APP_SORT_BY_NAME" => Ok(AppSorting::AppSortByName),
                    "APP_SORT_BY_STATE" => Ok(AppSorting::AppSortByState),
                    "APP_SORT_BY_CREATION_DATE" => Ok(AppSorting::AppSortByCreationDate),
                    "APP_SORT_BY_CHANGE_DATE" => Ok(AppSorting::AppSortByChangeDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
            Self::Removed => "APP_STATE_REMOVED",
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
            "APP_STATE_REMOVED",
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
                    "APP_STATE_REMOVED" => Ok(AppState::Removed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Application {
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
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.change_date.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.Application", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
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
                application::Config::OidcConfig(v) => {
                    struct_ser.serialize_field("oidcConfig", v)?;
                }
                application::Config::ApiConfig(v) => {
                    struct_ser.serialize_field("apiConfig", v)?;
                }
                application::Config::SamlConfig(v) => {
                    struct_ser.serialize_field("samlConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Application {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
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
            CreationDate,
            ChangeDate,
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
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
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
            type Value = Application;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.Application")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Application, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
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
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::ChangeDate => {
                            if change_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            change_date__ = map_.next_value()?;
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
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(application::Config::OidcConfig)
;
                        }
                        GeneratedField::ApiConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(application::Config::ApiConfig)
;
                        }
                        GeneratedField::SamlConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(application::Config::SamlConfig)
;
                        }
                    }
                }
                Ok(Application {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    change_date: change_date__,
                    state: state__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.Application", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationKey {
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
        if !self.application_id.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ApplicationKey", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.application_id.is_empty() {
            struct_ser.serialize_field("applicationId", &self.application_id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "application_id",
            "applicationId",
            "project_id",
            "projectId",
            "creation_date",
            "creationDate",
            "organization_id",
            "organizationId",
            "expiration_date",
            "expirationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ApplicationId,
            ProjectId,
            CreationDate,
            OrganizationId,
            ExpirationDate,
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
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ApplicationKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApplicationKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut application_id__ = None;
                let mut project_id__ = None;
                let mut creation_date__ = None;
                let mut organization_id__ = None;
                let mut expiration_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationId => {
                            if application_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            application_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ApplicationKey {
                    id: id__.unwrap_or_default(),
                    application_id: application_id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    organization_id: organization_id__.unwrap_or_default(),
                    expiration_date: expiration_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ApplicationKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationKeysSorting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ApplicationKeysSortById => "APPLICATION_KEYS_SORT_BY_ID",
            Self::ApplicationKeysSortByProjectId => "APPLICATION_KEYS_SORT_BY_PROJECT_ID",
            Self::ApplicationKeysSortByApplicationId => "APPLICATION_KEYS_SORT_BY_APPLICATION_ID",
            Self::ApplicationKeysSortByCreationDate => "APPLICATION_KEYS_SORT_BY_CREATION_DATE",
            Self::ApplicationKeysSortByOrganizationId => "APPLICATION_KEYS_SORT_BY_ORGANIZATION_ID",
            Self::ApplicationKeysSortByExpiration => "APPLICATION_KEYS_SORT_BY_EXPIRATION",
            Self::ApplicationKeysSortByType => "APPLICATION_KEYS_SORT_BY_TYPE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationKeysSorting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "APPLICATION_KEYS_SORT_BY_ID",
            "APPLICATION_KEYS_SORT_BY_PROJECT_ID",
            "APPLICATION_KEYS_SORT_BY_APPLICATION_ID",
            "APPLICATION_KEYS_SORT_BY_CREATION_DATE",
            "APPLICATION_KEYS_SORT_BY_ORGANIZATION_ID",
            "APPLICATION_KEYS_SORT_BY_EXPIRATION",
            "APPLICATION_KEYS_SORT_BY_TYPE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationKeysSorting;

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
                    "APPLICATION_KEYS_SORT_BY_ID" => Ok(ApplicationKeysSorting::ApplicationKeysSortById),
                    "APPLICATION_KEYS_SORT_BY_PROJECT_ID" => Ok(ApplicationKeysSorting::ApplicationKeysSortByProjectId),
                    "APPLICATION_KEYS_SORT_BY_APPLICATION_ID" => Ok(ApplicationKeysSorting::ApplicationKeysSortByApplicationId),
                    "APPLICATION_KEYS_SORT_BY_CREATION_DATE" => Ok(ApplicationKeysSorting::ApplicationKeysSortByCreationDate),
                    "APPLICATION_KEYS_SORT_BY_ORGANIZATION_ID" => Ok(ApplicationKeysSorting::ApplicationKeysSortByOrganizationId),
                    "APPLICATION_KEYS_SORT_BY_EXPIRATION" => Ok(ApplicationKeysSorting::ApplicationKeysSortByExpiration),
                    "APPLICATION_KEYS_SORT_BY_TYPE" => Ok(ApplicationKeysSorting::ApplicationKeysSortByType),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ApplicationNameQuery", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationNameQuery {
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
            type Value = ApplicationNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ApplicationNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApplicationNameQuery, V::Error>
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
                            method__ = Some(map_.next_value::<super::super::filter::v2::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(ApplicationNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ApplicationNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApplicationSearchFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ApplicationSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                application_search_filter::Filter::NameFilter(v) => {
                    struct_ser.serialize_field("nameFilter", v)?;
                }
                application_search_filter::Filter::StateFilter(v) => {
                    let v = AppState::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("stateFilter", &v)?;
                }
                application_search_filter::Filter::ApiAppOnly(v) => {
                    struct_ser.serialize_field("apiAppOnly", v)?;
                }
                application_search_filter::Filter::OidcAppOnly(v) => {
                    struct_ser.serialize_field("oidcAppOnly", v)?;
                }
                application_search_filter::Filter::SamlAppOnly(v) => {
                    struct_ser.serialize_field("samlAppOnly", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApplicationSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name_filter",
            "nameFilter",
            "state_filter",
            "stateFilter",
            "api_app_only",
            "apiAppOnly",
            "oidc_app_only",
            "oidcAppOnly",
            "saml_app_only",
            "samlAppOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NameFilter,
            StateFilter,
            ApiAppOnly,
            OidcAppOnly,
            SamlAppOnly,
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
                            "nameFilter" | "name_filter" => Ok(GeneratedField::NameFilter),
                            "stateFilter" | "state_filter" => Ok(GeneratedField::StateFilter),
                            "apiAppOnly" | "api_app_only" => Ok(GeneratedField::ApiAppOnly),
                            "oidcAppOnly" | "oidc_app_only" => Ok(GeneratedField::OidcAppOnly),
                            "samlAppOnly" | "saml_app_only" => Ok(GeneratedField::SamlAppOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApplicationSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ApplicationSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ApplicationSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NameFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(application_search_filter::Filter::NameFilter)
;
                        }
                        GeneratedField::StateFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<AppState>>()?.map(|x| application_search_filter::Filter::StateFilter(x as i32));
                        }
                        GeneratedField::ApiAppOnly => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiAppOnly"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(application_search_filter::Filter::ApiAppOnly);
                        }
                        GeneratedField::OidcAppOnly => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcAppOnly"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(application_search_filter::Filter::OidcAppOnly);
                        }
                        GeneratedField::SamlAppOnly => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlAppOnly"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(application_search_filter::Filter::SamlAppOnly);
                        }
                    }
                }
                Ok(ApplicationSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ApplicationSearchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApiApplicationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.auth_method_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateAPIApplicationRequest", len)?;
        if self.auth_method_type != 0 {
            let v = ApiAuthMethodType::try_from(self.auth_method_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.auth_method_type)))?;
            struct_ser.serialize_field("authMethodType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApiApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_method_type",
            "authMethodType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateApiApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateAPIApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApiApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_method_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthMethodType => {
                            if auth_method_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authMethodType"));
                            }
                            auth_method_type__ = Some(map_.next_value::<ApiAuthMethodType>()? as i32);
                        }
                    }
                }
                Ok(CreateApiApplicationRequest {
                    auth_method_type: auth_method_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateAPIApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApiApplicationResponse {
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
        if !self.client_secret.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateAPIApplicationResponse", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.client_secret.is_empty() {
            struct_ser.serialize_field("clientSecret", &self.client_secret)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApiApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "client_secret",
            "clientSecret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            ClientSecret,
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
                            "clientSecret" | "client_secret" => Ok(GeneratedField::ClientSecret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApiApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateAPIApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApiApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut client_secret__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientSecret => {
                            if client_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSecret"));
                            }
                            client_secret__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateApiApplicationResponse {
                    client_id: client_id__.unwrap_or_default(),
                    client_secret: client_secret__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateAPIApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApplicationKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_id.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateApplicationKeyRequest", len)?;
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("appId", &self.app_id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApplicationKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app_id",
            "appId",
            "project_id",
            "projectId",
            "expiration_date",
            "expirationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppId,
            ProjectId,
            ExpirationDate,
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
                            "appId" | "app_id" => Ok(GeneratedField::AppId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApplicationKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateApplicationKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApplicationKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_id__ = None;
                let mut project_id__ = None;
                let mut expiration_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appId"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateApplicationKeyRequest {
                    app_id: app_id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    expiration_date: expiration_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateApplicationKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApplicationKeyResponse {
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
        if self.creation_date.is_some() {
            len += 1;
        }
        if !self.key_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateApplicationKeyResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if !self.key_details.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("keyDetails", pbjson::private::base64::encode(&self.key_details).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApplicationKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "key_details",
            "keyDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            KeyDetails,
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
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "keyDetails" | "key_details" => Ok(GeneratedField::KeyDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApplicationKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateApplicationKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApplicationKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut key_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::KeyDetails => {
                            if key_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyDetails"));
                            }
                            key_details__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateApplicationKeyResponse {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    key_details: key_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateApplicationKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApplicationRequest {
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
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.creation_request_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateApplicationRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.creation_request_type.as_ref() {
            match v {
                create_application_request::CreationRequestType::OidcRequest(v) => {
                    struct_ser.serialize_field("oidcRequest", v)?;
                }
                create_application_request::CreationRequestType::SamlRequest(v) => {
                    struct_ser.serialize_field("samlRequest", v)?;
                }
                create_application_request::CreationRequestType::ApiRequest(v) => {
                    struct_ser.serialize_field("apiRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "id",
            "name",
            "oidc_request",
            "oidcRequest",
            "saml_request",
            "samlRequest",
            "api_request",
            "apiRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            Id,
            Name,
            OidcRequest,
            SamlRequest,
            ApiRequest,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "oidcRequest" | "oidc_request" => Ok(GeneratedField::OidcRequest),
                            "samlRequest" | "saml_request" => Ok(GeneratedField::SamlRequest),
                            "apiRequest" | "api_request" => Ok(GeneratedField::ApiRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut id__ = None;
                let mut name__ = None;
                let mut creation_request_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::OidcRequest => {
                            if creation_request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcRequest"));
                            }
                            creation_request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_application_request::CreationRequestType::OidcRequest)
;
                        }
                        GeneratedField::SamlRequest => {
                            if creation_request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlRequest"));
                            }
                            creation_request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_application_request::CreationRequestType::SamlRequest)
;
                        }
                        GeneratedField::ApiRequest => {
                            if creation_request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiRequest"));
                            }
                            creation_request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_application_request::CreationRequestType::ApiRequest)
;
                        }
                    }
                }
                Ok(CreateApplicationRequest {
                    project_id: project_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    creation_request_type: creation_request_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_id.is_empty() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.creation_response_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateApplicationResponse", len)?;
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("appId", &self.app_id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.creation_response_type.as_ref() {
            match v {
                create_application_response::CreationResponseType::OidcResponse(v) => {
                    struct_ser.serialize_field("oidcResponse", v)?;
                }
                create_application_response::CreationResponseType::SamlResponse(v) => {
                    struct_ser.serialize_field("samlResponse", v)?;
                }
                create_application_response::CreationResponseType::ApiResponse(v) => {
                    struct_ser.serialize_field("apiResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app_id",
            "appId",
            "creation_date",
            "creationDate",
            "oidc_response",
            "oidcResponse",
            "saml_response",
            "samlResponse",
            "api_response",
            "apiResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppId,
            CreationDate,
            OidcResponse,
            SamlResponse,
            ApiResponse,
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
                            "appId" | "app_id" => Ok(GeneratedField::AppId),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "oidcResponse" | "oidc_response" => Ok(GeneratedField::OidcResponse),
                            "samlResponse" | "saml_response" => Ok(GeneratedField::SamlResponse),
                            "apiResponse" | "api_response" => Ok(GeneratedField::ApiResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_id__ = None;
                let mut creation_date__ = None;
                let mut creation_response_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appId"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::OidcResponse => {
                            if creation_response_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcResponse"));
                            }
                            creation_response_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_application_response::CreationResponseType::OidcResponse)
;
                        }
                        GeneratedField::SamlResponse => {
                            if creation_response_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlResponse"));
                            }
                            creation_response_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_application_response::CreationResponseType::SamlResponse)
;
                        }
                        GeneratedField::ApiResponse => {
                            if creation_response_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiResponse"));
                            }
                            creation_response_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_application_response::CreationResponseType::ApiResponse)
;
                        }
                    }
                }
                Ok(CreateApplicationResponse {
                    app_id: app_id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    creation_response_type: creation_response_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOidcApplicationRequest {
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
        if self.auth_method_type != 0 {
            len += 1;
        }
        if !self.post_logout_redirect_uris.is_empty() {
            len += 1;
        }
        if self.version != 0 {
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
        if self.skip_native_app_success_page {
            len += 1;
        }
        if !self.back_channel_logout_uri.is_empty() {
            len += 1;
        }
        if self.login_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateOIDCApplicationRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for CreateOidcApplicationRequest {
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
            "auth_method_type",
            "authMethodType",
            "post_logout_redirect_uris",
            "postLogoutRedirectUris",
            "version",
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
            AuthMethodType,
            PostLogoutRedirectUris,
            Version,
            DevMode,
            AccessTokenType,
            AccessTokenRoleAssertion,
            IdTokenRoleAssertion,
            IdTokenUserinfoAssertion,
            ClockSkew,
            AdditionalOrigins,
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
                            "authMethodType" | "auth_method_type" => Ok(GeneratedField::AuthMethodType),
                            "postLogoutRedirectUris" | "post_logout_redirect_uris" => Ok(GeneratedField::PostLogoutRedirectUris),
                            "version" => Ok(GeneratedField::Version),
                            "devMode" | "dev_mode" => Ok(GeneratedField::DevMode),
                            "accessTokenType" | "access_token_type" => Ok(GeneratedField::AccessTokenType),
                            "accessTokenRoleAssertion" | "access_token_role_assertion" => Ok(GeneratedField::AccessTokenRoleAssertion),
                            "idTokenRoleAssertion" | "id_token_role_assertion" => Ok(GeneratedField::IdTokenRoleAssertion),
                            "idTokenUserinfoAssertion" | "id_token_userinfo_assertion" => Ok(GeneratedField::IdTokenUserinfoAssertion),
                            "clockSkew" | "clock_skew" => Ok(GeneratedField::ClockSkew),
                            "additionalOrigins" | "additional_origins" => Ok(GeneratedField::AdditionalOrigins),
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
            type Value = CreateOidcApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateOIDCApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOidcApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut redirect_uris__ = None;
                let mut response_types__ = None;
                let mut grant_types__ = None;
                let mut app_type__ = None;
                let mut auth_method_type__ = None;
                let mut post_logout_redirect_uris__ = None;
                let mut version__ = None;
                let mut dev_mode__ = None;
                let mut access_token_type__ = None;
                let mut access_token_role_assertion__ = None;
                let mut id_token_role_assertion__ = None;
                let mut id_token_userinfo_assertion__ = None;
                let mut clock_skew__ = None;
                let mut additional_origins__ = None;
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
                Ok(CreateOidcApplicationRequest {
                    redirect_uris: redirect_uris__.unwrap_or_default(),
                    response_types: response_types__.unwrap_or_default(),
                    grant_types: grant_types__.unwrap_or_default(),
                    app_type: app_type__.unwrap_or_default(),
                    auth_method_type: auth_method_type__.unwrap_or_default(),
                    post_logout_redirect_uris: post_logout_redirect_uris__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    dev_mode: dev_mode__.unwrap_or_default(),
                    access_token_type: access_token_type__.unwrap_or_default(),
                    access_token_role_assertion: access_token_role_assertion__.unwrap_or_default(),
                    id_token_role_assertion: id_token_role_assertion__.unwrap_or_default(),
                    id_token_userinfo_assertion: id_token_userinfo_assertion__.unwrap_or_default(),
                    clock_skew: clock_skew__,
                    additional_origins: additional_origins__.unwrap_or_default(),
                    skip_native_app_success_page: skip_native_app_success_page__.unwrap_or_default(),
                    back_channel_logout_uri: back_channel_logout_uri__.unwrap_or_default(),
                    login_version: login_version__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateOIDCApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateOidcApplicationResponse {
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
        if !self.client_secret.is_empty() {
            len += 1;
        }
        if self.none_compliant {
            len += 1;
        }
        if !self.compliance_problems.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateOIDCApplicationResponse", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.client_secret.is_empty() {
            struct_ser.serialize_field("clientSecret", &self.client_secret)?;
        }
        if self.none_compliant {
            struct_ser.serialize_field("noneCompliant", &self.none_compliant)?;
        }
        if !self.compliance_problems.is_empty() {
            struct_ser.serialize_field("complianceProblems", &self.compliance_problems)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateOidcApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "client_secret",
            "clientSecret",
            "none_compliant",
            "noneCompliant",
            "compliance_problems",
            "complianceProblems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            ClientSecret,
            NoneCompliant,
            ComplianceProblems,
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
                            "clientSecret" | "client_secret" => Ok(GeneratedField::ClientSecret),
                            "noneCompliant" | "none_compliant" => Ok(GeneratedField::NoneCompliant),
                            "complianceProblems" | "compliance_problems" => Ok(GeneratedField::ComplianceProblems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateOidcApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateOIDCApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateOidcApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut client_secret__ = None;
                let mut none_compliant__ = None;
                let mut compliance_problems__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientSecret => {
                            if client_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSecret"));
                            }
                            client_secret__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(CreateOidcApplicationResponse {
                    client_id: client_id__.unwrap_or_default(),
                    client_secret: client_secret__.unwrap_or_default(),
                    none_compliant: none_compliant__.unwrap_or_default(),
                    compliance_problems: compliance_problems__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateOIDCApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSamlApplicationRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateSAMLApplicationRequest", len)?;
        if let Some(v) = self.login_version.as_ref() {
            struct_ser.serialize_field("loginVersion", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            match v {
                create_saml_application_request::Metadata::MetadataXml(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("metadataXml", pbjson::private::base64::encode(&v).as_str())?;
                }
                create_saml_application_request::Metadata::MetadataUrl(v) => {
                    struct_ser.serialize_field("metadataUrl", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSamlApplicationRequest {
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
            type Value = CreateSamlApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateSAMLApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSamlApplicationRequest, V::Error>
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
                            metadata__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| create_saml_application_request::Metadata::MetadataXml(x.0));
                        }
                        GeneratedField::MetadataUrl => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUrl"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(create_saml_application_request::Metadata::MetadataUrl);
                        }
                    }
                }
                Ok(CreateSamlApplicationRequest {
                    login_version: login_version__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateSAMLApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSamlApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.app.v2beta.CreateSAMLApplicationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSamlApplicationResponse {
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
            type Value = CreateSamlApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.CreateSAMLApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSamlApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreateSamlApplicationResponse {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.CreateSAMLApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateApplicationRequest {
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
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.DeactivateApplicationRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
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
            type Value = DeactivateApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.DeactivateApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeactivateApplicationRequest {
                    project_id: project_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.DeactivateApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deactivation_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.DeactivateApplicationResponse", len)?;
        if let Some(v) = self.deactivation_date.as_ref() {
            struct_ser.serialize_field("deactivationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deactivation_date",
            "deactivationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeactivationDate,
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
                            "deactivationDate" | "deactivation_date" => Ok(GeneratedField::DeactivationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeactivateApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.DeactivateApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deactivation_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeactivationDate => {
                            if deactivation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deactivationDate"));
                            }
                            deactivation_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeactivateApplicationResponse {
                    deactivation_date: deactivation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.DeactivateApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteApplicationKeyRequest {
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
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.application_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.DeleteApplicationKeyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.application_id.is_empty() {
            struct_ser.serialize_field("applicationId", &self.application_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteApplicationKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "project_id",
            "projectId",
            "application_id",
            "applicationId",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProjectId,
            ApplicationId,
            OrganizationId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteApplicationKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.DeleteApplicationKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteApplicationKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut project_id__ = None;
                let mut application_id__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationId => {
                            if application_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            application_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteApplicationKeyRequest {
                    id: id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    application_id: application_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.DeleteApplicationKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteApplicationKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deletion_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.DeleteApplicationKeyResponse", len)?;
        if let Some(v) = self.deletion_date.as_ref() {
            struct_ser.serialize_field("deletionDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteApplicationKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deletion_date",
            "deletionDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeletionDate,
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
                            "deletionDate" | "deletion_date" => Ok(GeneratedField::DeletionDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteApplicationKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.DeleteApplicationKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteApplicationKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deletion_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeletionDate => {
                            if deletion_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletionDate"));
                            }
                            deletion_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteApplicationKeyResponse {
                    deletion_date: deletion_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.DeleteApplicationKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteApplicationRequest {
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
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.DeleteApplicationRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
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
            type Value = DeleteApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.DeleteApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteApplicationRequest {
                    project_id: project_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.DeleteApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.deletion_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.DeleteApplicationResponse", len)?;
        if let Some(v) = self.deletion_date.as_ref() {
            struct_ser.serialize_field("deletionDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deletion_date",
            "deletionDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeletionDate,
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
                            "deletionDate" | "deletion_date" => Ok(GeneratedField::DeletionDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.DeleteApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deletion_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeletionDate => {
                            if deletion_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletionDate"));
                            }
                            deletion_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteApplicationResponse {
                    deletion_date: deletion_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.DeleteApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetApplicationKeyRequest {
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
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.application_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.GetApplicationKeyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.application_id.is_empty() {
            struct_ser.serialize_field("applicationId", &self.application_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetApplicationKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "project_id",
            "projectId",
            "application_id",
            "applicationId",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProjectId,
            ApplicationId,
            OrganizationId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetApplicationKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.GetApplicationKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetApplicationKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut project_id__ = None;
                let mut application_id__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationId => {
                            if application_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            application_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetApplicationKeyRequest {
                    id: id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    application_id: application_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.GetApplicationKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetApplicationKeyResponse {
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
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.GetApplicationKeyResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetApplicationKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "expiration_date",
            "expirationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            ExpirationDate,
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
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetApplicationKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.GetApplicationKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetApplicationKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut expiration_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetApplicationKeyResponse {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    expiration_date: expiration_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.GetApplicationKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetApplicationRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.GetApplicationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.GetApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetApplicationRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.GetApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.app.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.GetApplicationResponse", len)?;
        if let Some(v) = self.app.as_ref() {
            struct_ser.serialize_field("app", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            App,
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
                            "app" => Ok(GeneratedField::App),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.GetApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::App => {
                            if app__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app"));
                            }
                            app__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetApplicationResponse {
                    app: app__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.GetApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApplicationKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if self.sorting_column != 0 {
            len += 1;
        }
        if self.resource_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ListApplicationKeysRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if self.sorting_column != 0 {
            let v = ApplicationKeysSorting::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if let Some(v) = self.resource_id.as_ref() {
            match v {
                list_application_keys_request::ResourceId::ApplicationId(v) => {
                    struct_ser.serialize_field("applicationId", v)?;
                }
                list_application_keys_request::ResourceId::ProjectId(v) => {
                    struct_ser.serialize_field("projectId", v)?;
                }
                list_application_keys_request::ResourceId::OrganizationId(v) => {
                    struct_ser.serialize_field("organizationId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApplicationKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "sorting_column",
            "sortingColumn",
            "application_id",
            "applicationId",
            "project_id",
            "projectId",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            SortingColumn,
            ApplicationId,
            ProjectId,
            OrganizationId,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApplicationKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ListApplicationKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApplicationKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut sorting_column__ = None;
                let mut resource_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<ApplicationKeysSorting>()? as i32);
                        }
                        GeneratedField::ApplicationId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            resource_id__ = map_.next_value::<::std::option::Option<_>>()?.map(list_application_keys_request::ResourceId::ApplicationId);
                        }
                        GeneratedField::ProjectId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            resource_id__ = map_.next_value::<::std::option::Option<_>>()?.map(list_application_keys_request::ResourceId::ProjectId);
                        }
                        GeneratedField::OrganizationId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            resource_id__ = map_.next_value::<::std::option::Option<_>>()?.map(list_application_keys_request::ResourceId::OrganizationId);
                        }
                    }
                }
                Ok(ListApplicationKeysRequest {
                    pagination: pagination__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    resource_id: resource_id__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ListApplicationKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApplicationKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ListApplicationKeysResponse", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApplicationKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
            Pagination,
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
                            "keys" => Ok(GeneratedField::Keys),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApplicationKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ListApplicationKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApplicationKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListApplicationKeysResponse {
                    keys: keys__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ListApplicationKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApplicationsRequest {
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
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.sorting_column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ListApplicationsRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if self.sorting_column != 0 {
            let v = AppSorting::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApplicationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "pagination",
            "filters",
            "sorting_column",
            "sortingColumn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            Pagination,
            Filters,
            SortingColumn,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            "filters" => Ok(GeneratedField::Filters),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApplicationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ListApplicationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApplicationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut pagination__ = None;
                let mut filters__ = None;
                let mut sorting_column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<AppSorting>()? as i32);
                        }
                    }
                }
                Ok(ListApplicationsRequest {
                    project_id: project_id__.unwrap_or_default(),
                    pagination: pagination__,
                    filters: filters__.unwrap_or_default(),
                    sorting_column: sorting_column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ListApplicationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApplicationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.applications.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ListApplicationsResponse", len)?;
        if !self.applications.is_empty() {
            struct_ser.serialize_field("applications", &self.applications)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApplicationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "applications",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Applications,
            Pagination,
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
                            "applications" => Ok(GeneratedField::Applications),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApplicationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ListApplicationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApplicationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut applications__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Applications => {
                            if applications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applications"));
                            }
                            applications__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListApplicationsResponse {
                    applications: applications__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ListApplicationsResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("zitadel.app.v2beta.LoginV1", len)?;
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
                formatter.write_str("struct zitadel.app.v2beta.LoginV1")
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
        deserializer.deserialize_struct("zitadel.app.v2beta.LoginV1", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.LoginV2", len)?;
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
                formatter.write_str("struct zitadel.app.v2beta.LoginV2")
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
        deserializer.deserialize_struct("zitadel.app.v2beta.LoginV2", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.LoginVersion", len)?;
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
                formatter.write_str("struct zitadel.app.v2beta.LoginVersion")
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
        deserializer.deserialize_struct("zitadel.app.v2beta.LoginVersion", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.OIDCConfig", len)?;
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
                formatter.write_str("struct zitadel.app.v2beta.OIDCConfig")
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
        deserializer.deserialize_struct("zitadel.app.v2beta.OIDCConfig", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for OidcLocalizedMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.localized_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.OIDCLocalizedMessage", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.localized_message.is_empty() {
            struct_ser.serialize_field("localizedMessage", &self.localized_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OidcLocalizedMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "localized_message",
            "localizedMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            LocalizedMessage,
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
                            "key" => Ok(GeneratedField::Key),
                            "localizedMessage" | "localized_message" => Ok(GeneratedField::LocalizedMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcLocalizedMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.OIDCLocalizedMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OidcLocalizedMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut localized_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalizedMessage => {
                            if localized_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localizedMessage"));
                            }
                            localized_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OidcLocalizedMessage {
                    key: key__.unwrap_or_default(),
                    localized_message: localized_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.OIDCLocalizedMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OidcResponseType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OIDC_RESPONSE_TYPE_UNSPECIFIED",
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
            "OIDC_RESPONSE_TYPE_UNSPECIFIED",
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
                    "OIDC_RESPONSE_TYPE_UNSPECIFIED" => Ok(OidcResponseType::Unspecified),
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
impl serde::Serialize for ReactivateApplicationRequest {
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
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ReactivateApplicationRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactivateApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
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
            type Value = ReactivateApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ReactivateApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactivateApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReactivateApplicationRequest {
                    project_id: project_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ReactivateApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactivateApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reactivation_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.ReactivateApplicationResponse", len)?;
        if let Some(v) = self.reactivation_date.as_ref() {
            struct_ser.serialize_field("reactivationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactivateApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reactivation_date",
            "reactivationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReactivationDate,
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
                            "reactivationDate" | "reactivation_date" => Ok(GeneratedField::ReactivationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactivateApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.ReactivateApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactivateApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reactivation_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReactivationDate => {
                            if reactivation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reactivationDate"));
                            }
                            reactivation_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReactivateApplicationResponse {
                    reactivation_date: reactivation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.ReactivateApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegenerateClientSecretRequest {
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
        if !self.application_id.is_empty() {
            len += 1;
        }
        if self.app_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.RegenerateClientSecretRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.application_id.is_empty() {
            struct_ser.serialize_field("applicationId", &self.application_id)?;
        }
        if let Some(v) = self.app_type.as_ref() {
            match v {
                regenerate_client_secret_request::AppType::IsOidc(v) => {
                    struct_ser.serialize_field("isOidc", v)?;
                }
                regenerate_client_secret_request::AppType::IsApi(v) => {
                    struct_ser.serialize_field("isApi", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegenerateClientSecretRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "application_id",
            "applicationId",
            "is_oidc",
            "isOidc",
            "is_api",
            "isApi",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            ApplicationId,
            IsOidc,
            IsApi,
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
                            "applicationId" | "application_id" => Ok(GeneratedField::ApplicationId),
                            "isOidc" | "is_oidc" => Ok(GeneratedField::IsOidc),
                            "isApi" | "is_api" => Ok(GeneratedField::IsApi),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegenerateClientSecretRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.RegenerateClientSecretRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegenerateClientSecretRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut application_id__ = None;
                let mut app_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationId => {
                            if application_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationId"));
                            }
                            application_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsOidc => {
                            if app_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isOidc"));
                            }
                            app_type__ = map_.next_value::<::std::option::Option<_>>()?.map(regenerate_client_secret_request::AppType::IsOidc);
                        }
                        GeneratedField::IsApi => {
                            if app_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isApi"));
                            }
                            app_type__ = map_.next_value::<::std::option::Option<_>>()?.map(regenerate_client_secret_request::AppType::IsApi);
                        }
                    }
                }
                Ok(RegenerateClientSecretRequest {
                    project_id: project_id__.unwrap_or_default(),
                    application_id: application_id__.unwrap_or_default(),
                    app_type: app_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.RegenerateClientSecretRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegenerateClientSecretResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_secret.is_empty() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.RegenerateClientSecretResponse", len)?;
        if !self.client_secret.is_empty() {
            struct_ser.serialize_field("clientSecret", &self.client_secret)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegenerateClientSecretResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_secret",
            "clientSecret",
            "creation_date",
            "creationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientSecret,
            CreationDate,
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
                            "clientSecret" | "client_secret" => Ok(GeneratedField::ClientSecret),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegenerateClientSecretResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.RegenerateClientSecretResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegenerateClientSecretResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_secret__ = None;
                let mut creation_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientSecret => {
                            if client_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSecret"));
                            }
                            client_secret__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegenerateClientSecretResponse {
                    client_secret: client_secret__.unwrap_or_default(),
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.RegenerateClientSecretResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.SAMLConfig", len)?;
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
                formatter.write_str("struct zitadel.app.v2beta.SAMLConfig")
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
        deserializer.deserialize_struct("zitadel.app.v2beta.SAMLConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateApiApplicationConfigurationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.auth_method_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.UpdateAPIApplicationConfigurationRequest", len)?;
        if self.auth_method_type != 0 {
            let v = ApiAuthMethodType::try_from(self.auth_method_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.auth_method_type)))?;
            struct_ser.serialize_field("authMethodType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateApiApplicationConfigurationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_method_type",
            "authMethodType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = UpdateApiApplicationConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.UpdateAPIApplicationConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateApiApplicationConfigurationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_method_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthMethodType => {
                            if auth_method_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authMethodType"));
                            }
                            auth_method_type__ = Some(map_.next_value::<ApiAuthMethodType>()? as i32);
                        }
                    }
                }
                Ok(UpdateApiApplicationConfigurationRequest {
                    auth_method_type: auth_method_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.UpdateAPIApplicationConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateApplicationRequest {
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
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.update_request_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.UpdateApplicationRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.update_request_type.as_ref() {
            match v {
                update_application_request::UpdateRequestType::SamlConfigurationRequest(v) => {
                    struct_ser.serialize_field("samlConfigurationRequest", v)?;
                }
                update_application_request::UpdateRequestType::OidcConfigurationRequest(v) => {
                    struct_ser.serialize_field("oidcConfigurationRequest", v)?;
                }
                update_application_request::UpdateRequestType::ApiConfigurationRequest(v) => {
                    struct_ser.serialize_field("apiConfigurationRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateApplicationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "id",
            "name",
            "saml_configuration_request",
            "samlConfigurationRequest",
            "oidc_configuration_request",
            "oidcConfigurationRequest",
            "api_configuration_request",
            "apiConfigurationRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            Id,
            Name,
            SamlConfigurationRequest,
            OidcConfigurationRequest,
            ApiConfigurationRequest,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "samlConfigurationRequest" | "saml_configuration_request" => Ok(GeneratedField::SamlConfigurationRequest),
                            "oidcConfigurationRequest" | "oidc_configuration_request" => Ok(GeneratedField::OidcConfigurationRequest),
                            "apiConfigurationRequest" | "api_configuration_request" => Ok(GeneratedField::ApiConfigurationRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateApplicationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.UpdateApplicationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateApplicationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut id__ = None;
                let mut name__ = None;
                let mut update_request_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::SamlConfigurationRequest => {
                            if update_request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlConfigurationRequest"));
                            }
                            update_request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(update_application_request::UpdateRequestType::SamlConfigurationRequest)
;
                        }
                        GeneratedField::OidcConfigurationRequest => {
                            if update_request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcConfigurationRequest"));
                            }
                            update_request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(update_application_request::UpdateRequestType::OidcConfigurationRequest)
;
                        }
                        GeneratedField::ApiConfigurationRequest => {
                            if update_request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiConfigurationRequest"));
                            }
                            update_request_type__ = map_.next_value::<::std::option::Option<_>>()?.map(update_application_request::UpdateRequestType::ApiConfigurationRequest)
;
                        }
                    }
                }
                Ok(UpdateApplicationRequest {
                    project_id: project_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    update_request_type: update_request_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.UpdateApplicationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.change_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.UpdateApplicationResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateApplicationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change_date",
            "changeDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChangeDate,
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
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.UpdateApplicationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut change_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChangeDate => {
                            if change_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            change_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateApplicationResponse {
                    change_date: change_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.UpdateApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOidcApplicationConfigurationRequest {
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
        if self.app_type.is_some() {
            len += 1;
        }
        if self.auth_method_type.is_some() {
            len += 1;
        }
        if !self.post_logout_redirect_uris.is_empty() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.dev_mode.is_some() {
            len += 1;
        }
        if self.access_token_type.is_some() {
            len += 1;
        }
        if self.access_token_role_assertion.is_some() {
            len += 1;
        }
        if self.id_token_role_assertion.is_some() {
            len += 1;
        }
        if self.id_token_userinfo_assertion.is_some() {
            len += 1;
        }
        if self.clock_skew.is_some() {
            len += 1;
        }
        if !self.additional_origins.is_empty() {
            len += 1;
        }
        if self.skip_native_app_success_page.is_some() {
            len += 1;
        }
        if self.back_channel_logout_uri.is_some() {
            len += 1;
        }
        if self.login_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.UpdateOIDCApplicationConfigurationRequest", len)?;
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
        if let Some(v) = self.app_type.as_ref() {
            let v = OidcAppType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("appType", &v)?;
        }
        if let Some(v) = self.auth_method_type.as_ref() {
            let v = OidcAuthMethodType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("authMethodType", &v)?;
        }
        if !self.post_logout_redirect_uris.is_empty() {
            struct_ser.serialize_field("postLogoutRedirectUris", &self.post_logout_redirect_uris)?;
        }
        if let Some(v) = self.version.as_ref() {
            let v = OidcVersion::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        if let Some(v) = self.dev_mode.as_ref() {
            struct_ser.serialize_field("devMode", v)?;
        }
        if let Some(v) = self.access_token_type.as_ref() {
            let v = OidcTokenType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("accessTokenType", &v)?;
        }
        if let Some(v) = self.access_token_role_assertion.as_ref() {
            struct_ser.serialize_field("accessTokenRoleAssertion", v)?;
        }
        if let Some(v) = self.id_token_role_assertion.as_ref() {
            struct_ser.serialize_field("idTokenRoleAssertion", v)?;
        }
        if let Some(v) = self.id_token_userinfo_assertion.as_ref() {
            struct_ser.serialize_field("idTokenUserinfoAssertion", v)?;
        }
        if let Some(v) = self.clock_skew.as_ref() {
            struct_ser.serialize_field("clockSkew", v)?;
        }
        if !self.additional_origins.is_empty() {
            struct_ser.serialize_field("additionalOrigins", &self.additional_origins)?;
        }
        if let Some(v) = self.skip_native_app_success_page.as_ref() {
            struct_ser.serialize_field("skipNativeAppSuccessPage", v)?;
        }
        if let Some(v) = self.back_channel_logout_uri.as_ref() {
            struct_ser.serialize_field("backChannelLogoutUri", v)?;
        }
        if let Some(v) = self.login_version.as_ref() {
            struct_ser.serialize_field("loginVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOidcApplicationConfigurationRequest {
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
            "auth_method_type",
            "authMethodType",
            "post_logout_redirect_uris",
            "postLogoutRedirectUris",
            "version",
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
            AuthMethodType,
            PostLogoutRedirectUris,
            Version,
            DevMode,
            AccessTokenType,
            AccessTokenRoleAssertion,
            IdTokenRoleAssertion,
            IdTokenUserinfoAssertion,
            ClockSkew,
            AdditionalOrigins,
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
                            "authMethodType" | "auth_method_type" => Ok(GeneratedField::AuthMethodType),
                            "postLogoutRedirectUris" | "post_logout_redirect_uris" => Ok(GeneratedField::PostLogoutRedirectUris),
                            "version" => Ok(GeneratedField::Version),
                            "devMode" | "dev_mode" => Ok(GeneratedField::DevMode),
                            "accessTokenType" | "access_token_type" => Ok(GeneratedField::AccessTokenType),
                            "accessTokenRoleAssertion" | "access_token_role_assertion" => Ok(GeneratedField::AccessTokenRoleAssertion),
                            "idTokenRoleAssertion" | "id_token_role_assertion" => Ok(GeneratedField::IdTokenRoleAssertion),
                            "idTokenUserinfoAssertion" | "id_token_userinfo_assertion" => Ok(GeneratedField::IdTokenUserinfoAssertion),
                            "clockSkew" | "clock_skew" => Ok(GeneratedField::ClockSkew),
                            "additionalOrigins" | "additional_origins" => Ok(GeneratedField::AdditionalOrigins),
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
            type Value = UpdateOidcApplicationConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.UpdateOIDCApplicationConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOidcApplicationConfigurationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut redirect_uris__ = None;
                let mut response_types__ = None;
                let mut grant_types__ = None;
                let mut app_type__ = None;
                let mut auth_method_type__ = None;
                let mut post_logout_redirect_uris__ = None;
                let mut version__ = None;
                let mut dev_mode__ = None;
                let mut access_token_type__ = None;
                let mut access_token_role_assertion__ = None;
                let mut id_token_role_assertion__ = None;
                let mut id_token_userinfo_assertion__ = None;
                let mut clock_skew__ = None;
                let mut additional_origins__ = None;
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
                            app_type__ = map_.next_value::<::std::option::Option<OidcAppType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::AuthMethodType => {
                            if auth_method_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authMethodType"));
                            }
                            auth_method_type__ = map_.next_value::<::std::option::Option<OidcAuthMethodType>>()?.map(|x| x as i32);
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
                            version__ = map_.next_value::<::std::option::Option<OidcVersion>>()?.map(|x| x as i32);
                        }
                        GeneratedField::DevMode => {
                            if dev_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devMode"));
                            }
                            dev_mode__ = map_.next_value()?;
                        }
                        GeneratedField::AccessTokenType => {
                            if access_token_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenType"));
                            }
                            access_token_type__ = map_.next_value::<::std::option::Option<OidcTokenType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::AccessTokenRoleAssertion => {
                            if access_token_role_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenRoleAssertion"));
                            }
                            access_token_role_assertion__ = map_.next_value()?;
                        }
                        GeneratedField::IdTokenRoleAssertion => {
                            if id_token_role_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idTokenRoleAssertion"));
                            }
                            id_token_role_assertion__ = map_.next_value()?;
                        }
                        GeneratedField::IdTokenUserinfoAssertion => {
                            if id_token_userinfo_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idTokenUserinfoAssertion"));
                            }
                            id_token_userinfo_assertion__ = map_.next_value()?;
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
                        GeneratedField::SkipNativeAppSuccessPage => {
                            if skip_native_app_success_page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipNativeAppSuccessPage"));
                            }
                            skip_native_app_success_page__ = map_.next_value()?;
                        }
                        GeneratedField::BackChannelLogoutUri => {
                            if back_channel_logout_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backChannelLogoutUri"));
                            }
                            back_channel_logout_uri__ = map_.next_value()?;
                        }
                        GeneratedField::LoginVersion => {
                            if login_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginVersion"));
                            }
                            login_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateOidcApplicationConfigurationRequest {
                    redirect_uris: redirect_uris__.unwrap_or_default(),
                    response_types: response_types__.unwrap_or_default(),
                    grant_types: grant_types__.unwrap_or_default(),
                    app_type: app_type__,
                    auth_method_type: auth_method_type__,
                    post_logout_redirect_uris: post_logout_redirect_uris__.unwrap_or_default(),
                    version: version__,
                    dev_mode: dev_mode__,
                    access_token_type: access_token_type__,
                    access_token_role_assertion: access_token_role_assertion__,
                    id_token_role_assertion: id_token_role_assertion__,
                    id_token_userinfo_assertion: id_token_userinfo_assertion__,
                    clock_skew: clock_skew__,
                    additional_origins: additional_origins__.unwrap_or_default(),
                    skip_native_app_success_page: skip_native_app_success_page__,
                    back_channel_logout_uri: back_channel_logout_uri__,
                    login_version: login_version__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.UpdateOIDCApplicationConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSamlApplicationConfigurationRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.app.v2beta.UpdateSAMLApplicationConfigurationRequest", len)?;
        if let Some(v) = self.login_version.as_ref() {
            struct_ser.serialize_field("loginVersion", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            match v {
                update_saml_application_configuration_request::Metadata::MetadataXml(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("metadataXml", pbjson::private::base64::encode(&v).as_str())?;
                }
                update_saml_application_configuration_request::Metadata::MetadataUrl(v) => {
                    struct_ser.serialize_field("metadataUrl", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSamlApplicationConfigurationRequest {
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
            type Value = UpdateSamlApplicationConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.app.v2beta.UpdateSAMLApplicationConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSamlApplicationConfigurationRequest, V::Error>
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
                            metadata__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| update_saml_application_configuration_request::Metadata::MetadataXml(x.0));
                        }
                        GeneratedField::MetadataUrl => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataUrl"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(update_saml_application_configuration_request::Metadata::MetadataUrl);
                        }
                    }
                }
                Ok(UpdateSamlApplicationConfigurationRequest {
                    login_version: login_version__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.app.v2beta.UpdateSAMLApplicationConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
