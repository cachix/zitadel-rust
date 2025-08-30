// @generated
impl serde::Serialize for AppleConfig {
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
        if !self.team_id.is_empty() {
            len += 1;
        }
        if !self.key_id.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.AppleConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.team_id.is_empty() {
            struct_ser.serialize_field("teamId", &self.team_id)?;
        }
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppleConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "team_id",
            "teamId",
            "key_id",
            "keyId",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            TeamId,
            KeyId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "teamId" | "team_id" => Ok(GeneratedField::TeamId),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
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
            type Value = AppleConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.AppleConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AppleConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut team_id__ = None;
                let mut key_id__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TeamId => {
                            if team_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("teamId"));
                            }
                            team_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AppleConfig {
                    client_id: client_id__.unwrap_or_default(),
                    team_id: team_id__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.AppleConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutoLinkingOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTO_LINKING_OPTION_UNSPECIFIED",
            Self::Username => "AUTO_LINKING_OPTION_USERNAME",
            Self::Email => "AUTO_LINKING_OPTION_EMAIL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AutoLinkingOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO_LINKING_OPTION_UNSPECIFIED",
            "AUTO_LINKING_OPTION_USERNAME",
            "AUTO_LINKING_OPTION_EMAIL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutoLinkingOption;

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
                    "AUTO_LINKING_OPTION_UNSPECIFIED" => Ok(AutoLinkingOption::Unspecified),
                    "AUTO_LINKING_OPTION_USERNAME" => Ok(AutoLinkingOption::Username),
                    "AUTO_LINKING_OPTION_EMAIL" => Ok(AutoLinkingOption::Email),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AzureAdConfig {
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
        if self.tenant.is_some() {
            len += 1;
        }
        if self.email_verified {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.AzureADConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if let Some(v) = self.tenant.as_ref() {
            struct_ser.serialize_field("tenant", v)?;
        }
        if self.email_verified {
            struct_ser.serialize_field("emailVerified", &self.email_verified)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureAdConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "tenant",
            "email_verified",
            "emailVerified",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Tenant,
            EmailVerified,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "tenant" => Ok(GeneratedField::Tenant),
                            "emailVerified" | "email_verified" => Ok(GeneratedField::EmailVerified),
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
            type Value = AzureAdConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.AzureADConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureAdConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut tenant__ = None;
                let mut email_verified__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tenant => {
                            if tenant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenant"));
                            }
                            tenant__ = map_.next_value()?;
                        }
                        GeneratedField::EmailVerified => {
                            if email_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailVerified"));
                            }
                            email_verified__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AzureAdConfig {
                    client_id: client_id__.unwrap_or_default(),
                    tenant: tenant__,
                    email_verified: email_verified__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.AzureADConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzureAdTenant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.AzureADTenant", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                azure_ad_tenant::Type::TenantType(v) => {
                    let v = AzureAdTenantType::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("tenantType", &v)?;
                }
                azure_ad_tenant::Type::TenantId(v) => {
                    struct_ser.serialize_field("tenantId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AzureAdTenant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tenant_type",
            "tenantType",
            "tenant_id",
            "tenantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TenantType,
            TenantId,
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
                            "tenantType" | "tenant_type" => Ok(GeneratedField::TenantType),
                            "tenantId" | "tenant_id" => Ok(GeneratedField::TenantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureAdTenant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.AzureADTenant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AzureAdTenant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TenantType => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenantType"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<AzureAdTenantType>>()?.map(|x| azure_ad_tenant::Type::TenantType(x as i32));
                        }
                        GeneratedField::TenantId => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenantId"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(azure_ad_tenant::Type::TenantId);
                        }
                    }
                }
                Ok(AzureAdTenant {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.AzureADTenant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AzureAdTenantType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Common => "AZURE_AD_TENANT_TYPE_COMMON",
            Self::Organisations => "AZURE_AD_TENANT_TYPE_ORGANISATIONS",
            Self::Consumers => "AZURE_AD_TENANT_TYPE_CONSUMERS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AzureAdTenantType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AZURE_AD_TENANT_TYPE_COMMON",
            "AZURE_AD_TENANT_TYPE_ORGANISATIONS",
            "AZURE_AD_TENANT_TYPE_CONSUMERS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AzureAdTenantType;

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
                    "AZURE_AD_TENANT_TYPE_COMMON" => Ok(AzureAdTenantType::Common),
                    "AZURE_AD_TENANT_TYPE_ORGANISATIONS" => Ok(AzureAdTenantType::Organisations),
                    "AZURE_AD_TENANT_TYPE_CONSUMERS" => Ok(AzureAdTenantType::Consumers),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GenericOidcConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        if self.is_id_token_mapping {
            len += 1;
        }
        if self.use_pkce {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.GenericOIDCConfig", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        if self.is_id_token_mapping {
            struct_ser.serialize_field("isIdTokenMapping", &self.is_id_token_mapping)?;
        }
        if self.use_pkce {
            struct_ser.serialize_field("usePkce", &self.use_pkce)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenericOidcConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "client_id",
            "clientId",
            "scopes",
            "is_id_token_mapping",
            "isIdTokenMapping",
            "use_pkce",
            "usePkce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ClientId,
            Scopes,
            IsIdTokenMapping,
            UsePkce,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "scopes" => Ok(GeneratedField::Scopes),
                            "isIdTokenMapping" | "is_id_token_mapping" => Ok(GeneratedField::IsIdTokenMapping),
                            "usePkce" | "use_pkce" => Ok(GeneratedField::UsePkce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericOidcConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.GenericOIDCConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenericOidcConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut client_id__ = None;
                let mut scopes__ = None;
                let mut is_id_token_mapping__ = None;
                let mut use_pkce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsIdTokenMapping => {
                            if is_id_token_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isIdTokenMapping"));
                            }
                            is_id_token_mapping__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsePkce => {
                            if use_pkce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usePkce"));
                            }
                            use_pkce__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenericOidcConfig {
                    issuer: issuer__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                    is_id_token_mapping: is_id_token_mapping__.unwrap_or_default(),
                    use_pkce: use_pkce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.GenericOIDCConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GitHubConfig {
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
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.GitHubConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GitHubConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
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
            type Value = GitHubConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.GitHubConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GitHubConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GitHubConfig {
                    client_id: client_id__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.GitHubConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GitHubEnterpriseServerConfig {
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
        if !self.authorization_endpoint.is_empty() {
            len += 1;
        }
        if !self.token_endpoint.is_empty() {
            len += 1;
        }
        if !self.user_endpoint.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.GitHubEnterpriseServerConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.authorization_endpoint.is_empty() {
            struct_ser.serialize_field("authorizationEndpoint", &self.authorization_endpoint)?;
        }
        if !self.token_endpoint.is_empty() {
            struct_ser.serialize_field("tokenEndpoint", &self.token_endpoint)?;
        }
        if !self.user_endpoint.is_empty() {
            struct_ser.serialize_field("userEndpoint", &self.user_endpoint)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GitHubEnterpriseServerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "authorization_endpoint",
            "authorizationEndpoint",
            "token_endpoint",
            "tokenEndpoint",
            "user_endpoint",
            "userEndpoint",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            AuthorizationEndpoint,
            TokenEndpoint,
            UserEndpoint,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "authorizationEndpoint" | "authorization_endpoint" => Ok(GeneratedField::AuthorizationEndpoint),
                            "tokenEndpoint" | "token_endpoint" => Ok(GeneratedField::TokenEndpoint),
                            "userEndpoint" | "user_endpoint" => Ok(GeneratedField::UserEndpoint),
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
            type Value = GitHubEnterpriseServerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.GitHubEnterpriseServerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GitHubEnterpriseServerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut authorization_endpoint__ = None;
                let mut token_endpoint__ = None;
                let mut user_endpoint__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationEndpoint => {
                            if authorization_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationEndpoint"));
                            }
                            authorization_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenEndpoint => {
                            if token_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenEndpoint"));
                            }
                            token_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserEndpoint => {
                            if user_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userEndpoint"));
                            }
                            user_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GitHubEnterpriseServerConfig {
                    client_id: client_id__.unwrap_or_default(),
                    authorization_endpoint: authorization_endpoint__.unwrap_or_default(),
                    token_endpoint: token_endpoint__.unwrap_or_default(),
                    user_endpoint: user_endpoint__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.GitHubEnterpriseServerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GitLabConfig {
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
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.GitLabConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GitLabConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
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
            type Value = GitLabConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.GitLabConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GitLabConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GitLabConfig {
                    client_id: client_id__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.GitLabConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GitLabSelfHostedConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.GitLabSelfHostedConfig", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GitLabSelfHostedConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "client_id",
            "clientId",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ClientId,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
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
            type Value = GitLabSelfHostedConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.GitLabSelfHostedConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GitLabSelfHostedConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut client_id__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GitLabSelfHostedConfig {
                    issuer: issuer__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.GitLabSelfHostedConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GoogleConfig {
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
        if !self.scopes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.GoogleConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GoogleConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "scopes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
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
            type Value = GoogleConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.GoogleConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GoogleConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut scopes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GoogleConfig {
                    client_id: client_id__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.GoogleConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Idp {
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
        if self.styling_type != 0 {
            len += 1;
        }
        if self.owner != 0 {
            len += 1;
        }
        if self.auto_register {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.IDP", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.state != 0 {
            let v = IdpState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.styling_type != 0 {
            let v = IdpStylingType::try_from(self.styling_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.styling_type)))?;
            struct_ser.serialize_field("stylingType", &v)?;
        }
        if self.owner != 0 {
            let v = IdpOwnerType::try_from(self.owner)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.owner)))?;
            struct_ser.serialize_field("owner", &v)?;
        }
        if self.auto_register {
            struct_ser.serialize_field("autoRegister", &self.auto_register)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                idp::Config::OidcConfig(v) => {
                    struct_ser.serialize_field("oidcConfig", v)?;
                }
                idp::Config::JwtConfig(v) => {
                    struct_ser.serialize_field("jwtConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Idp {
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
            "styling_type",
            "stylingType",
            "owner",
            "auto_register",
            "autoRegister",
            "oidc_config",
            "oidcConfig",
            "jwt_config",
            "jwtConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            State,
            Name,
            StylingType,
            Owner,
            AutoRegister,
            OidcConfig,
            JwtConfig,
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
                            "stylingType" | "styling_type" => Ok(GeneratedField::StylingType),
                            "owner" => Ok(GeneratedField::Owner),
                            "autoRegister" | "auto_register" => Ok(GeneratedField::AutoRegister),
                            "oidcConfig" | "oidc_config" => Ok(GeneratedField::OidcConfig),
                            "jwtConfig" | "jwt_config" => Ok(GeneratedField::JwtConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Idp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.IDP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Idp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut state__ = None;
                let mut name__ = None;
                let mut styling_type__ = None;
                let mut owner__ = None;
                let mut auto_register__ = None;
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
                            state__ = Some(map_.next_value::<IdpState>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StylingType => {
                            if styling_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stylingType"));
                            }
                            styling_type__ = Some(map_.next_value::<IdpStylingType>()? as i32);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value::<IdpOwnerType>()? as i32);
                        }
                        GeneratedField::AutoRegister => {
                            if auto_register__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoRegister"));
                            }
                            auto_register__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OidcConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(idp::Config::OidcConfig)
;
                        }
                        GeneratedField::JwtConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtConfig"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(idp::Config::JwtConfig)
;
                        }
                    }
                }
                Ok(Idp {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    state: state__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    styling_type: styling_type__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    auto_register: auto_register__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.IDP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IDP_FIELD_NAME_UNSPECIFIED",
            Self::Name => "IDP_FIELD_NAME_NAME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdpFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDP_FIELD_NAME_UNSPECIFIED",
            "IDP_FIELD_NAME_NAME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpFieldName;

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
                    "IDP_FIELD_NAME_UNSPECIFIED" => Ok(IdpFieldName::Unspecified),
                    "IDP_FIELD_NAME_NAME" => Ok(IdpFieldName::Name),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IdpidQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.IDPIDQuery", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpidQuery {
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
            type Value = IdpidQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.IDPIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpidQuery, V::Error>
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
                Ok(IdpidQuery {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.IDPIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpLoginPolicyLink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.idp_id.is_empty() {
            len += 1;
        }
        if !self.idp_name.is_empty() {
            len += 1;
        }
        if self.idp_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.IDPLoginPolicyLink", len)?;
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if !self.idp_name.is_empty() {
            struct_ser.serialize_field("idpName", &self.idp_name)?;
        }
        if self.idp_type != 0 {
            let v = IdpType::try_from(self.idp_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.idp_type)))?;
            struct_ser.serialize_field("idpType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpLoginPolicyLink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_id",
            "idpId",
            "idp_name",
            "idpName",
            "idp_type",
            "idpType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpId,
            IdpName,
            IdpType,
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
                            "idpId" | "idp_id" => Ok(GeneratedField::IdpId),
                            "idpName" | "idp_name" => Ok(GeneratedField::IdpName),
                            "idpType" | "idp_type" => Ok(GeneratedField::IdpType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpLoginPolicyLink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.IDPLoginPolicyLink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpLoginPolicyLink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_id__ = None;
                let mut idp_name__ = None;
                let mut idp_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpName => {
                            if idp_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpName"));
                            }
                            idp_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpType => {
                            if idp_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpType"));
                            }
                            idp_type__ = Some(map_.next_value::<IdpType>()? as i32);
                        }
                    }
                }
                Ok(IdpLoginPolicyLink {
                    idp_id: idp_id__.unwrap_or_default(),
                    idp_name: idp_name__.unwrap_or_default(),
                    idp_type: idp_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.IDPLoginPolicyLink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.IDPNameQuery", len)?;
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
impl<'de> serde::Deserialize<'de> for IdpNameQuery {
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
            type Value = IdpNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.IDPNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpNameQuery, V::Error>
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
                Ok(IdpNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.IDPNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpOwnerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IDP_OWNER_TYPE_UNSPECIFIED",
            Self::System => "IDP_OWNER_TYPE_SYSTEM",
            Self::Org => "IDP_OWNER_TYPE_ORG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdpOwnerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDP_OWNER_TYPE_UNSPECIFIED",
            "IDP_OWNER_TYPE_SYSTEM",
            "IDP_OWNER_TYPE_ORG",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpOwnerType;

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
                    "IDP_OWNER_TYPE_UNSPECIFIED" => Ok(IdpOwnerType::Unspecified),
                    "IDP_OWNER_TYPE_SYSTEM" => Ok(IdpOwnerType::System),
                    "IDP_OWNER_TYPE_ORG" => Ok(IdpOwnerType::Org),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IdpOwnerTypeQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.owner_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.IDPOwnerTypeQuery", len)?;
        if self.owner_type != 0 {
            let v = IdpOwnerType::try_from(self.owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.owner_type)))?;
            struct_ser.serialize_field("ownerType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpOwnerTypeQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner_type",
            "ownerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OwnerType,
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
                            "ownerType" | "owner_type" => Ok(GeneratedField::OwnerType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpOwnerTypeQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.IDPOwnerTypeQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpOwnerTypeQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OwnerType => {
                            if owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerType"));
                            }
                            owner_type__ = Some(map_.next_value::<IdpOwnerType>()? as i32);
                        }
                    }
                }
                Ok(IdpOwnerTypeQuery {
                    owner_type: owner_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.IDPOwnerTypeQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IDP_STATE_UNSPECIFIED",
            Self::Active => "IDP_STATE_ACTIVE",
            Self::Inactive => "IDP_STATE_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdpState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDP_STATE_UNSPECIFIED",
            "IDP_STATE_ACTIVE",
            "IDP_STATE_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpState;

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
                    "IDP_STATE_UNSPECIFIED" => Ok(IdpState::Unspecified),
                    "IDP_STATE_ACTIVE" => Ok(IdpState::Active),
                    "IDP_STATE_INACTIVE" => Ok(IdpState::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IdpStylingType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::StylingTypeUnspecified => "STYLING_TYPE_UNSPECIFIED",
            Self::StylingTypeGoogle => "STYLING_TYPE_GOOGLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdpStylingType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STYLING_TYPE_UNSPECIFIED",
            "STYLING_TYPE_GOOGLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpStylingType;

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
                    "STYLING_TYPE_UNSPECIFIED" => Ok(IdpStylingType::StylingTypeUnspecified),
                    "STYLING_TYPE_GOOGLE" => Ok(IdpStylingType::StylingTypeGoogle),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IdpType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IDP_TYPE_UNSPECIFIED",
            Self::Oidc => "IDP_TYPE_OIDC",
            Self::Jwt => "IDP_TYPE_JWT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdpType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDP_TYPE_UNSPECIFIED",
            "IDP_TYPE_OIDC",
            "IDP_TYPE_JWT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpType;

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
                    "IDP_TYPE_UNSPECIFIED" => Ok(IdpType::Unspecified),
                    "IDP_TYPE_OIDC" => Ok(IdpType::Oidc),
                    "IDP_TYPE_JWT" => Ok(IdpType::Jwt),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IdpUserLink {
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
        if !self.idp_id.is_empty() {
            len += 1;
        }
        if !self.idp_name.is_empty() {
            len += 1;
        }
        if !self.provided_user_id.is_empty() {
            len += 1;
        }
        if !self.provided_user_name.is_empty() {
            len += 1;
        }
        if self.idp_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.IDPUserLink", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if !self.idp_name.is_empty() {
            struct_ser.serialize_field("idpName", &self.idp_name)?;
        }
        if !self.provided_user_id.is_empty() {
            struct_ser.serialize_field("providedUserId", &self.provided_user_id)?;
        }
        if !self.provided_user_name.is_empty() {
            struct_ser.serialize_field("providedUserName", &self.provided_user_name)?;
        }
        if self.idp_type != 0 {
            let v = IdpType::try_from(self.idp_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.idp_type)))?;
            struct_ser.serialize_field("idpType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpUserLink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "idp_id",
            "idpId",
            "idp_name",
            "idpName",
            "provided_user_id",
            "providedUserId",
            "provided_user_name",
            "providedUserName",
            "idp_type",
            "idpType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            IdpId,
            IdpName,
            ProvidedUserId,
            ProvidedUserName,
            IdpType,
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
                            "idpId" | "idp_id" => Ok(GeneratedField::IdpId),
                            "idpName" | "idp_name" => Ok(GeneratedField::IdpName),
                            "providedUserId" | "provided_user_id" => Ok(GeneratedField::ProvidedUserId),
                            "providedUserName" | "provided_user_name" => Ok(GeneratedField::ProvidedUserName),
                            "idpType" | "idp_type" => Ok(GeneratedField::IdpType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpUserLink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.IDPUserLink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpUserLink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut idp_id__ = None;
                let mut idp_name__ = None;
                let mut provided_user_id__ = None;
                let mut provided_user_name__ = None;
                let mut idp_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpName => {
                            if idp_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpName"));
                            }
                            idp_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProvidedUserId => {
                            if provided_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providedUserId"));
                            }
                            provided_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProvidedUserName => {
                            if provided_user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providedUserName"));
                            }
                            provided_user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpType => {
                            if idp_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpType"));
                            }
                            idp_type__ = Some(map_.next_value::<IdpType>()? as i32);
                        }
                    }
                }
                Ok(IdpUserLink {
                    user_id: user_id__.unwrap_or_default(),
                    idp_id: idp_id__.unwrap_or_default(),
                    idp_name: idp_name__.unwrap_or_default(),
                    provided_user_id: provided_user_id__.unwrap_or_default(),
                    provided_user_name: provided_user_name__.unwrap_or_default(),
                    idp_type: idp_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.IDPUserLink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JwtConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.jwt_endpoint.is_empty() {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.keys_endpoint.is_empty() {
            len += 1;
        }
        if !self.header_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.JWTConfig", len)?;
        if !self.jwt_endpoint.is_empty() {
            struct_ser.serialize_field("jwtEndpoint", &self.jwt_endpoint)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.keys_endpoint.is_empty() {
            struct_ser.serialize_field("keysEndpoint", &self.keys_endpoint)?;
        }
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("headerName", &self.header_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JwtConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "jwt_endpoint",
            "jwtEndpoint",
            "issuer",
            "keys_endpoint",
            "keysEndpoint",
            "header_name",
            "headerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JwtEndpoint,
            Issuer,
            KeysEndpoint,
            HeaderName,
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
                            "jwtEndpoint" | "jwt_endpoint" => Ok(GeneratedField::JwtEndpoint),
                            "issuer" => Ok(GeneratedField::Issuer),
                            "keysEndpoint" | "keys_endpoint" => Ok(GeneratedField::KeysEndpoint),
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JwtConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.JWTConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JwtConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut jwt_endpoint__ = None;
                let mut issuer__ = None;
                let mut keys_endpoint__ = None;
                let mut header_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JwtEndpoint => {
                            if jwt_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtEndpoint"));
                            }
                            jwt_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::KeysEndpoint => {
                            if keys_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keysEndpoint"));
                            }
                            keys_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(JwtConfig {
                    jwt_endpoint: jwt_endpoint__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                    keys_endpoint: keys_endpoint__.unwrap_or_default(),
                    header_name: header_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.JWTConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LdapAttributes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id_attribute.is_empty() {
            len += 1;
        }
        if !self.first_name_attribute.is_empty() {
            len += 1;
        }
        if !self.last_name_attribute.is_empty() {
            len += 1;
        }
        if !self.display_name_attribute.is_empty() {
            len += 1;
        }
        if !self.nick_name_attribute.is_empty() {
            len += 1;
        }
        if !self.preferred_username_attribute.is_empty() {
            len += 1;
        }
        if !self.email_attribute.is_empty() {
            len += 1;
        }
        if !self.email_verified_attribute.is_empty() {
            len += 1;
        }
        if !self.phone_attribute.is_empty() {
            len += 1;
        }
        if !self.phone_verified_attribute.is_empty() {
            len += 1;
        }
        if !self.preferred_language_attribute.is_empty() {
            len += 1;
        }
        if !self.avatar_url_attribute.is_empty() {
            len += 1;
        }
        if !self.profile_attribute.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.LDAPAttributes", len)?;
        if !self.id_attribute.is_empty() {
            struct_ser.serialize_field("idAttribute", &self.id_attribute)?;
        }
        if !self.first_name_attribute.is_empty() {
            struct_ser.serialize_field("firstNameAttribute", &self.first_name_attribute)?;
        }
        if !self.last_name_attribute.is_empty() {
            struct_ser.serialize_field("lastNameAttribute", &self.last_name_attribute)?;
        }
        if !self.display_name_attribute.is_empty() {
            struct_ser.serialize_field("displayNameAttribute", &self.display_name_attribute)?;
        }
        if !self.nick_name_attribute.is_empty() {
            struct_ser.serialize_field("nickNameAttribute", &self.nick_name_attribute)?;
        }
        if !self.preferred_username_attribute.is_empty() {
            struct_ser.serialize_field("preferredUsernameAttribute", &self.preferred_username_attribute)?;
        }
        if !self.email_attribute.is_empty() {
            struct_ser.serialize_field("emailAttribute", &self.email_attribute)?;
        }
        if !self.email_verified_attribute.is_empty() {
            struct_ser.serialize_field("emailVerifiedAttribute", &self.email_verified_attribute)?;
        }
        if !self.phone_attribute.is_empty() {
            struct_ser.serialize_field("phoneAttribute", &self.phone_attribute)?;
        }
        if !self.phone_verified_attribute.is_empty() {
            struct_ser.serialize_field("phoneVerifiedAttribute", &self.phone_verified_attribute)?;
        }
        if !self.preferred_language_attribute.is_empty() {
            struct_ser.serialize_field("preferredLanguageAttribute", &self.preferred_language_attribute)?;
        }
        if !self.avatar_url_attribute.is_empty() {
            struct_ser.serialize_field("avatarUrlAttribute", &self.avatar_url_attribute)?;
        }
        if !self.profile_attribute.is_empty() {
            struct_ser.serialize_field("profileAttribute", &self.profile_attribute)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LdapAttributes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id_attribute",
            "idAttribute",
            "first_name_attribute",
            "firstNameAttribute",
            "last_name_attribute",
            "lastNameAttribute",
            "display_name_attribute",
            "displayNameAttribute",
            "nick_name_attribute",
            "nickNameAttribute",
            "preferred_username_attribute",
            "preferredUsernameAttribute",
            "email_attribute",
            "emailAttribute",
            "email_verified_attribute",
            "emailVerifiedAttribute",
            "phone_attribute",
            "phoneAttribute",
            "phone_verified_attribute",
            "phoneVerifiedAttribute",
            "preferred_language_attribute",
            "preferredLanguageAttribute",
            "avatar_url_attribute",
            "avatarUrlAttribute",
            "profile_attribute",
            "profileAttribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdAttribute,
            FirstNameAttribute,
            LastNameAttribute,
            DisplayNameAttribute,
            NickNameAttribute,
            PreferredUsernameAttribute,
            EmailAttribute,
            EmailVerifiedAttribute,
            PhoneAttribute,
            PhoneVerifiedAttribute,
            PreferredLanguageAttribute,
            AvatarUrlAttribute,
            ProfileAttribute,
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
                            "idAttribute" | "id_attribute" => Ok(GeneratedField::IdAttribute),
                            "firstNameAttribute" | "first_name_attribute" => Ok(GeneratedField::FirstNameAttribute),
                            "lastNameAttribute" | "last_name_attribute" => Ok(GeneratedField::LastNameAttribute),
                            "displayNameAttribute" | "display_name_attribute" => Ok(GeneratedField::DisplayNameAttribute),
                            "nickNameAttribute" | "nick_name_attribute" => Ok(GeneratedField::NickNameAttribute),
                            "preferredUsernameAttribute" | "preferred_username_attribute" => Ok(GeneratedField::PreferredUsernameAttribute),
                            "emailAttribute" | "email_attribute" => Ok(GeneratedField::EmailAttribute),
                            "emailVerifiedAttribute" | "email_verified_attribute" => Ok(GeneratedField::EmailVerifiedAttribute),
                            "phoneAttribute" | "phone_attribute" => Ok(GeneratedField::PhoneAttribute),
                            "phoneVerifiedAttribute" | "phone_verified_attribute" => Ok(GeneratedField::PhoneVerifiedAttribute),
                            "preferredLanguageAttribute" | "preferred_language_attribute" => Ok(GeneratedField::PreferredLanguageAttribute),
                            "avatarUrlAttribute" | "avatar_url_attribute" => Ok(GeneratedField::AvatarUrlAttribute),
                            "profileAttribute" | "profile_attribute" => Ok(GeneratedField::ProfileAttribute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LdapAttributes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.LDAPAttributes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LdapAttributes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id_attribute__ = None;
                let mut first_name_attribute__ = None;
                let mut last_name_attribute__ = None;
                let mut display_name_attribute__ = None;
                let mut nick_name_attribute__ = None;
                let mut preferred_username_attribute__ = None;
                let mut email_attribute__ = None;
                let mut email_verified_attribute__ = None;
                let mut phone_attribute__ = None;
                let mut phone_verified_attribute__ = None;
                let mut preferred_language_attribute__ = None;
                let mut avatar_url_attribute__ = None;
                let mut profile_attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdAttribute => {
                            if id_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idAttribute"));
                            }
                            id_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstNameAttribute => {
                            if first_name_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstNameAttribute"));
                            }
                            first_name_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastNameAttribute => {
                            if last_name_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastNameAttribute"));
                            }
                            last_name_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayNameAttribute => {
                            if display_name_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayNameAttribute"));
                            }
                            display_name_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NickNameAttribute => {
                            if nick_name_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickNameAttribute"));
                            }
                            nick_name_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredUsernameAttribute => {
                            if preferred_username_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredUsernameAttribute"));
                            }
                            preferred_username_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailAttribute => {
                            if email_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailAttribute"));
                            }
                            email_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailVerifiedAttribute => {
                            if email_verified_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailVerifiedAttribute"));
                            }
                            email_verified_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PhoneAttribute => {
                            if phone_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneAttribute"));
                            }
                            phone_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PhoneVerifiedAttribute => {
                            if phone_verified_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneVerifiedAttribute"));
                            }
                            phone_verified_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredLanguageAttribute => {
                            if preferred_language_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguageAttribute"));
                            }
                            preferred_language_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrlAttribute => {
                            if avatar_url_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrlAttribute"));
                            }
                            avatar_url_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProfileAttribute => {
                            if profile_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profileAttribute"));
                            }
                            profile_attribute__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LdapAttributes {
                    id_attribute: id_attribute__.unwrap_or_default(),
                    first_name_attribute: first_name_attribute__.unwrap_or_default(),
                    last_name_attribute: last_name_attribute__.unwrap_or_default(),
                    display_name_attribute: display_name_attribute__.unwrap_or_default(),
                    nick_name_attribute: nick_name_attribute__.unwrap_or_default(),
                    preferred_username_attribute: preferred_username_attribute__.unwrap_or_default(),
                    email_attribute: email_attribute__.unwrap_or_default(),
                    email_verified_attribute: email_verified_attribute__.unwrap_or_default(),
                    phone_attribute: phone_attribute__.unwrap_or_default(),
                    phone_verified_attribute: phone_verified_attribute__.unwrap_or_default(),
                    preferred_language_attribute: preferred_language_attribute__.unwrap_or_default(),
                    avatar_url_attribute: avatar_url_attribute__.unwrap_or_default(),
                    profile_attribute: profile_attribute__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.LDAPAttributes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LdapConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.servers.is_empty() {
            len += 1;
        }
        if self.start_tls {
            len += 1;
        }
        if !self.base_dn.is_empty() {
            len += 1;
        }
        if !self.bind_dn.is_empty() {
            len += 1;
        }
        if !self.user_base.is_empty() {
            len += 1;
        }
        if !self.user_object_classes.is_empty() {
            len += 1;
        }
        if !self.user_filters.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        if !self.root_ca.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.LDAPConfig", len)?;
        if !self.servers.is_empty() {
            struct_ser.serialize_field("servers", &self.servers)?;
        }
        if self.start_tls {
            struct_ser.serialize_field("startTls", &self.start_tls)?;
        }
        if !self.base_dn.is_empty() {
            struct_ser.serialize_field("baseDn", &self.base_dn)?;
        }
        if !self.bind_dn.is_empty() {
            struct_ser.serialize_field("bindDn", &self.bind_dn)?;
        }
        if !self.user_base.is_empty() {
            struct_ser.serialize_field("userBase", &self.user_base)?;
        }
        if !self.user_object_classes.is_empty() {
            struct_ser.serialize_field("userObjectClasses", &self.user_object_classes)?;
        }
        if !self.user_filters.is_empty() {
            struct_ser.serialize_field("userFilters", &self.user_filters)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        if !self.root_ca.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rootCa", pbjson::private::base64::encode(&self.root_ca).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LdapConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "servers",
            "start_tls",
            "startTls",
            "base_dn",
            "baseDn",
            "bind_dn",
            "bindDn",
            "user_base",
            "userBase",
            "user_object_classes",
            "userObjectClasses",
            "user_filters",
            "userFilters",
            "timeout",
            "attributes",
            "root_ca",
            "rootCa",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Servers,
            StartTls,
            BaseDn,
            BindDn,
            UserBase,
            UserObjectClasses,
            UserFilters,
            Timeout,
            Attributes,
            RootCa,
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
                            "servers" => Ok(GeneratedField::Servers),
                            "startTls" | "start_tls" => Ok(GeneratedField::StartTls),
                            "baseDn" | "base_dn" => Ok(GeneratedField::BaseDn),
                            "bindDn" | "bind_dn" => Ok(GeneratedField::BindDn),
                            "userBase" | "user_base" => Ok(GeneratedField::UserBase),
                            "userObjectClasses" | "user_object_classes" => Ok(GeneratedField::UserObjectClasses),
                            "userFilters" | "user_filters" => Ok(GeneratedField::UserFilters),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "attributes" => Ok(GeneratedField::Attributes),
                            "rootCa" | "root_ca" => Ok(GeneratedField::RootCa),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LdapConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.LDAPConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LdapConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut servers__ = None;
                let mut start_tls__ = None;
                let mut base_dn__ = None;
                let mut bind_dn__ = None;
                let mut user_base__ = None;
                let mut user_object_classes__ = None;
                let mut user_filters__ = None;
                let mut timeout__ = None;
                let mut attributes__ = None;
                let mut root_ca__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Servers => {
                            if servers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("servers"));
                            }
                            servers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTls => {
                            if start_tls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTls"));
                            }
                            start_tls__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BaseDn => {
                            if base_dn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseDn"));
                            }
                            base_dn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BindDn => {
                            if bind_dn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindDn"));
                            }
                            bind_dn__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserBase => {
                            if user_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userBase"));
                            }
                            user_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserObjectClasses => {
                            if user_object_classes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userObjectClasses"));
                            }
                            user_object_classes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserFilters => {
                            if user_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userFilters"));
                            }
                            user_filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map_.next_value()?;
                        }
                        GeneratedField::RootCa => {
                            if root_ca__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootCa"));
                            }
                            root_ca__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LdapConfig {
                    servers: servers__.unwrap_or_default(),
                    start_tls: start_tls__.unwrap_or_default(),
                    base_dn: base_dn__.unwrap_or_default(),
                    bind_dn: bind_dn__.unwrap_or_default(),
                    user_base: user_base__.unwrap_or_default(),
                    user_object_classes: user_object_classes__.unwrap_or_default(),
                    user_filters: user_filters__.unwrap_or_default(),
                    timeout: timeout__,
                    attributes: attributes__,
                    root_ca: root_ca__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.LDAPConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OAuthConfig {
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
        if !self.authorization_endpoint.is_empty() {
            len += 1;
        }
        if !self.token_endpoint.is_empty() {
            len += 1;
        }
        if !self.user_endpoint.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        if !self.id_attribute.is_empty() {
            len += 1;
        }
        if self.use_pkce {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.OAuthConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.authorization_endpoint.is_empty() {
            struct_ser.serialize_field("authorizationEndpoint", &self.authorization_endpoint)?;
        }
        if !self.token_endpoint.is_empty() {
            struct_ser.serialize_field("tokenEndpoint", &self.token_endpoint)?;
        }
        if !self.user_endpoint.is_empty() {
            struct_ser.serialize_field("userEndpoint", &self.user_endpoint)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        if !self.id_attribute.is_empty() {
            struct_ser.serialize_field("idAttribute", &self.id_attribute)?;
        }
        if self.use_pkce {
            struct_ser.serialize_field("usePkce", &self.use_pkce)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OAuthConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "authorization_endpoint",
            "authorizationEndpoint",
            "token_endpoint",
            "tokenEndpoint",
            "user_endpoint",
            "userEndpoint",
            "scopes",
            "id_attribute",
            "idAttribute",
            "use_pkce",
            "usePkce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            AuthorizationEndpoint,
            TokenEndpoint,
            UserEndpoint,
            Scopes,
            IdAttribute,
            UsePkce,
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
                            "authorizationEndpoint" | "authorization_endpoint" => Ok(GeneratedField::AuthorizationEndpoint),
                            "tokenEndpoint" | "token_endpoint" => Ok(GeneratedField::TokenEndpoint),
                            "userEndpoint" | "user_endpoint" => Ok(GeneratedField::UserEndpoint),
                            "scopes" => Ok(GeneratedField::Scopes),
                            "idAttribute" | "id_attribute" => Ok(GeneratedField::IdAttribute),
                            "usePkce" | "use_pkce" => Ok(GeneratedField::UsePkce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OAuthConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.OAuthConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OAuthConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut authorization_endpoint__ = None;
                let mut token_endpoint__ = None;
                let mut user_endpoint__ = None;
                let mut scopes__ = None;
                let mut id_attribute__ = None;
                let mut use_pkce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationEndpoint => {
                            if authorization_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationEndpoint"));
                            }
                            authorization_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenEndpoint => {
                            if token_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenEndpoint"));
                            }
                            token_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserEndpoint => {
                            if user_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userEndpoint"));
                            }
                            user_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdAttribute => {
                            if id_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idAttribute"));
                            }
                            id_attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsePkce => {
                            if use_pkce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usePkce"));
                            }
                            use_pkce__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OAuthConfig {
                    client_id: client_id__.unwrap_or_default(),
                    authorization_endpoint: authorization_endpoint__.unwrap_or_default(),
                    token_endpoint: token_endpoint__.unwrap_or_default(),
                    user_endpoint: user_endpoint__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                    id_attribute: id_attribute__.unwrap_or_default(),
                    use_pkce: use_pkce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.OAuthConfig", FIELDS, GeneratedVisitor)
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
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.scopes.is_empty() {
            len += 1;
        }
        if self.display_name_mapping != 0 {
            len += 1;
        }
        if self.username_mapping != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.OIDCConfig", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.scopes.is_empty() {
            struct_ser.serialize_field("scopes", &self.scopes)?;
        }
        if self.display_name_mapping != 0 {
            let v = OidcMappingField::try_from(self.display_name_mapping)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.display_name_mapping)))?;
            struct_ser.serialize_field("displayNameMapping", &v)?;
        }
        if self.username_mapping != 0 {
            let v = OidcMappingField::try_from(self.username_mapping)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.username_mapping)))?;
            struct_ser.serialize_field("usernameMapping", &v)?;
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
            "client_id",
            "clientId",
            "issuer",
            "scopes",
            "display_name_mapping",
            "displayNameMapping",
            "username_mapping",
            "usernameMapping",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Issuer,
            Scopes,
            DisplayNameMapping,
            UsernameMapping,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "scopes" => Ok(GeneratedField::Scopes),
                            "displayNameMapping" | "display_name_mapping" => Ok(GeneratedField::DisplayNameMapping),
                            "usernameMapping" | "username_mapping" => Ok(GeneratedField::UsernameMapping),
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
                formatter.write_str("struct zitadel.idp.v1.OIDCConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OidcConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut issuer__ = None;
                let mut scopes__ = None;
                let mut display_name_mapping__ = None;
                let mut username_mapping__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scopes => {
                            if scopes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopes"));
                            }
                            scopes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayNameMapping => {
                            if display_name_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayNameMapping"));
                            }
                            display_name_mapping__ = Some(map_.next_value::<OidcMappingField>()? as i32);
                        }
                        GeneratedField::UsernameMapping => {
                            if username_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameMapping"));
                            }
                            username_mapping__ = Some(map_.next_value::<OidcMappingField>()? as i32);
                        }
                    }
                }
                Ok(OidcConfig {
                    client_id: client_id__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                    scopes: scopes__.unwrap_or_default(),
                    display_name_mapping: display_name_mapping__.unwrap_or_default(),
                    username_mapping: username_mapping__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.OIDCConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OidcMappingField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OIDC_MAPPING_FIELD_UNSPECIFIED",
            Self::PreferredUsername => "OIDC_MAPPING_FIELD_PREFERRED_USERNAME",
            Self::Email => "OIDC_MAPPING_FIELD_EMAIL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OidcMappingField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OIDC_MAPPING_FIELD_UNSPECIFIED",
            "OIDC_MAPPING_FIELD_PREFERRED_USERNAME",
            "OIDC_MAPPING_FIELD_EMAIL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OidcMappingField;

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
                    "OIDC_MAPPING_FIELD_UNSPECIFIED" => Ok(OidcMappingField::Unspecified),
                    "OIDC_MAPPING_FIELD_PREFERRED_USERNAME" => Ok(OidcMappingField::PreferredUsername),
                    "OIDC_MAPPING_FIELD_EMAIL" => Ok(OidcMappingField::Email),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if self.is_linking_allowed {
            len += 1;
        }
        if self.is_creation_allowed {
            len += 1;
        }
        if self.is_auto_creation {
            len += 1;
        }
        if self.is_auto_update {
            len += 1;
        }
        if self.auto_linking != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.Options", len)?;
        if self.is_linking_allowed {
            struct_ser.serialize_field("isLinkingAllowed", &self.is_linking_allowed)?;
        }
        if self.is_creation_allowed {
            struct_ser.serialize_field("isCreationAllowed", &self.is_creation_allowed)?;
        }
        if self.is_auto_creation {
            struct_ser.serialize_field("isAutoCreation", &self.is_auto_creation)?;
        }
        if self.is_auto_update {
            struct_ser.serialize_field("isAutoUpdate", &self.is_auto_update)?;
        }
        if self.auto_linking != 0 {
            let v = AutoLinkingOption::try_from(self.auto_linking)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.auto_linking)))?;
            struct_ser.serialize_field("autoLinking", &v)?;
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
            "is_linking_allowed",
            "isLinkingAllowed",
            "is_creation_allowed",
            "isCreationAllowed",
            "is_auto_creation",
            "isAutoCreation",
            "is_auto_update",
            "isAutoUpdate",
            "auto_linking",
            "autoLinking",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsLinkingAllowed,
            IsCreationAllowed,
            IsAutoCreation,
            IsAutoUpdate,
            AutoLinking,
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
                            "isLinkingAllowed" | "is_linking_allowed" => Ok(GeneratedField::IsLinkingAllowed),
                            "isCreationAllowed" | "is_creation_allowed" => Ok(GeneratedField::IsCreationAllowed),
                            "isAutoCreation" | "is_auto_creation" => Ok(GeneratedField::IsAutoCreation),
                            "isAutoUpdate" | "is_auto_update" => Ok(GeneratedField::IsAutoUpdate),
                            "autoLinking" | "auto_linking" => Ok(GeneratedField::AutoLinking),
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
                formatter.write_str("struct zitadel.idp.v1.Options")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Options, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_linking_allowed__ = None;
                let mut is_creation_allowed__ = None;
                let mut is_auto_creation__ = None;
                let mut is_auto_update__ = None;
                let mut auto_linking__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsLinkingAllowed => {
                            if is_linking_allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isLinkingAllowed"));
                            }
                            is_linking_allowed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsCreationAllowed => {
                            if is_creation_allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isCreationAllowed"));
                            }
                            is_creation_allowed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAutoCreation => {
                            if is_auto_creation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAutoCreation"));
                            }
                            is_auto_creation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAutoUpdate => {
                            if is_auto_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAutoUpdate"));
                            }
                            is_auto_update__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoLinking => {
                            if auto_linking__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoLinking"));
                            }
                            auto_linking__ = Some(map_.next_value::<AutoLinkingOption>()? as i32);
                        }
                    }
                }
                Ok(Options {
                    is_linking_allowed: is_linking_allowed__.unwrap_or_default(),
                    is_creation_allowed: is_creation_allowed__.unwrap_or_default(),
                    is_auto_creation: is_auto_creation__.unwrap_or_default(),
                    is_auto_update: is_auto_update__.unwrap_or_default(),
                    auto_linking: auto_linking__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.Options", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Provider {
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
        if self.owner != 0 {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.Provider", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.state != 0 {
            let v = IdpState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.owner != 0 {
            let v = IdpOwnerType::try_from(self.owner)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.owner)))?;
            struct_ser.serialize_field("owner", &v)?;
        }
        if self.r#type != 0 {
            let v = ProviderType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Provider {
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
            "owner",
            "type",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            State,
            Name,
            Owner,
            Type,
            Config,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "type" => Ok(GeneratedField::Type),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Provider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.Provider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Provider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut state__ = None;
                let mut name__ = None;
                let mut owner__ = None;
                let mut r#type__ = None;
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
                            state__ = Some(map_.next_value::<IdpState>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value::<IdpOwnerType>()? as i32);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<ProviderType>()? as i32);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Provider {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    state: state__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.Provider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.ProviderConfig", len)?;
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                provider_config::Config::Ldap(v) => {
                    struct_ser.serialize_field("ldap", v)?;
                }
                provider_config::Config::Google(v) => {
                    struct_ser.serialize_field("google", v)?;
                }
                provider_config::Config::Oauth(v) => {
                    struct_ser.serialize_field("oauth", v)?;
                }
                provider_config::Config::Oidc(v) => {
                    struct_ser.serialize_field("oidc", v)?;
                }
                provider_config::Config::Jwt(v) => {
                    struct_ser.serialize_field("jwt", v)?;
                }
                provider_config::Config::Github(v) => {
                    struct_ser.serialize_field("github", v)?;
                }
                provider_config::Config::GithubEs(v) => {
                    struct_ser.serialize_field("githubEs", v)?;
                }
                provider_config::Config::Gitlab(v) => {
                    struct_ser.serialize_field("gitlab", v)?;
                }
                provider_config::Config::GitlabSelfHosted(v) => {
                    struct_ser.serialize_field("gitlabSelfHosted", v)?;
                }
                provider_config::Config::AzureAd(v) => {
                    struct_ser.serialize_field("azureAd", v)?;
                }
                provider_config::Config::Apple(v) => {
                    struct_ser.serialize_field("apple", v)?;
                }
                provider_config::Config::Saml(v) => {
                    struct_ser.serialize_field("saml", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProviderConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
            "ldap",
            "google",
            "oauth",
            "oidc",
            "jwt",
            "github",
            "github_es",
            "githubEs",
            "gitlab",
            "gitlab_self_hosted",
            "gitlabSelfHosted",
            "azure_ad",
            "azureAd",
            "apple",
            "saml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
            Ldap,
            Google,
            Oauth,
            Oidc,
            Jwt,
            Github,
            GithubEs,
            Gitlab,
            GitlabSelfHosted,
            AzureAd,
            Apple,
            Saml,
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
                            "options" => Ok(GeneratedField::Options),
                            "ldap" => Ok(GeneratedField::Ldap),
                            "google" => Ok(GeneratedField::Google),
                            "oauth" => Ok(GeneratedField::Oauth),
                            "oidc" => Ok(GeneratedField::Oidc),
                            "jwt" => Ok(GeneratedField::Jwt),
                            "github" => Ok(GeneratedField::Github),
                            "githubEs" | "github_es" => Ok(GeneratedField::GithubEs),
                            "gitlab" => Ok(GeneratedField::Gitlab),
                            "gitlabSelfHosted" | "gitlab_self_hosted" => Ok(GeneratedField::GitlabSelfHosted),
                            "azureAd" | "azure_ad" => Ok(GeneratedField::AzureAd),
                            "apple" => Ok(GeneratedField::Apple),
                            "saml" => Ok(GeneratedField::Saml),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.idp.v1.ProviderConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProviderConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                        GeneratedField::Ldap => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ldap"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Ldap)
;
                        }
                        GeneratedField::Google => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("google"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Google)
;
                        }
                        GeneratedField::Oauth => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauth"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Oauth)
;
                        }
                        GeneratedField::Oidc => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidc"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Oidc)
;
                        }
                        GeneratedField::Jwt => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwt"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Jwt)
;
                        }
                        GeneratedField::Github => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("github"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Github)
;
                        }
                        GeneratedField::GithubEs => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("githubEs"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::GithubEs)
;
                        }
                        GeneratedField::Gitlab => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gitlab"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Gitlab)
;
                        }
                        GeneratedField::GitlabSelfHosted => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gitlabSelfHosted"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::GitlabSelfHosted)
;
                        }
                        GeneratedField::AzureAd => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("azureAd"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::AzureAd)
;
                        }
                        GeneratedField::Apple => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apple"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Apple)
;
                        }
                        GeneratedField::Saml => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saml"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(provider_config::Config::Saml)
;
                        }
                    }
                }
                Ok(ProviderConfig {
                    options: options__,
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.ProviderConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProviderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROVIDER_TYPE_UNSPECIFIED",
            Self::Oidc => "PROVIDER_TYPE_OIDC",
            Self::Jwt => "PROVIDER_TYPE_JWT",
            Self::Ldap => "PROVIDER_TYPE_LDAP",
            Self::Oauth => "PROVIDER_TYPE_OAUTH",
            Self::AzureAd => "PROVIDER_TYPE_AZURE_AD",
            Self::Github => "PROVIDER_TYPE_GITHUB",
            Self::GithubEs => "PROVIDER_TYPE_GITHUB_ES",
            Self::Gitlab => "PROVIDER_TYPE_GITLAB",
            Self::GitlabSelfHosted => "PROVIDER_TYPE_GITLAB_SELF_HOSTED",
            Self::Google => "PROVIDER_TYPE_GOOGLE",
            Self::Apple => "PROVIDER_TYPE_APPLE",
            Self::Saml => "PROVIDER_TYPE_SAML",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProviderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROVIDER_TYPE_UNSPECIFIED",
            "PROVIDER_TYPE_OIDC",
            "PROVIDER_TYPE_JWT",
            "PROVIDER_TYPE_LDAP",
            "PROVIDER_TYPE_OAUTH",
            "PROVIDER_TYPE_AZURE_AD",
            "PROVIDER_TYPE_GITHUB",
            "PROVIDER_TYPE_GITHUB_ES",
            "PROVIDER_TYPE_GITLAB",
            "PROVIDER_TYPE_GITLAB_SELF_HOSTED",
            "PROVIDER_TYPE_GOOGLE",
            "PROVIDER_TYPE_APPLE",
            "PROVIDER_TYPE_SAML",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProviderType;

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
                    "PROVIDER_TYPE_UNSPECIFIED" => Ok(ProviderType::Unspecified),
                    "PROVIDER_TYPE_OIDC" => Ok(ProviderType::Oidc),
                    "PROVIDER_TYPE_JWT" => Ok(ProviderType::Jwt),
                    "PROVIDER_TYPE_LDAP" => Ok(ProviderType::Ldap),
                    "PROVIDER_TYPE_OAUTH" => Ok(ProviderType::Oauth),
                    "PROVIDER_TYPE_AZURE_AD" => Ok(ProviderType::AzureAd),
                    "PROVIDER_TYPE_GITHUB" => Ok(ProviderType::Github),
                    "PROVIDER_TYPE_GITHUB_ES" => Ok(ProviderType::GithubEs),
                    "PROVIDER_TYPE_GITLAB" => Ok(ProviderType::Gitlab),
                    "PROVIDER_TYPE_GITLAB_SELF_HOSTED" => Ok(ProviderType::GitlabSelfHosted),
                    "PROVIDER_TYPE_GOOGLE" => Ok(ProviderType::Google),
                    "PROVIDER_TYPE_APPLE" => Ok(ProviderType::Apple),
                    "PROVIDER_TYPE_SAML" => Ok(ProviderType::Saml),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SamlBinding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SAML_BINDING_UNSPECIFIED",
            Self::Post => "SAML_BINDING_POST",
            Self::Redirect => "SAML_BINDING_REDIRECT",
            Self::Artifact => "SAML_BINDING_ARTIFACT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SamlBinding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SAML_BINDING_UNSPECIFIED",
            "SAML_BINDING_POST",
            "SAML_BINDING_REDIRECT",
            "SAML_BINDING_ARTIFACT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SamlBinding;

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
                    "SAML_BINDING_UNSPECIFIED" => Ok(SamlBinding::Unspecified),
                    "SAML_BINDING_POST" => Ok(SamlBinding::Post),
                    "SAML_BINDING_REDIRECT" => Ok(SamlBinding::Redirect),
                    "SAML_BINDING_ARTIFACT" => Ok(SamlBinding::Artifact),
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
        if !self.metadata_xml.is_empty() {
            len += 1;
        }
        if self.binding != 0 {
            len += 1;
        }
        if self.with_signed_request {
            len += 1;
        }
        if self.name_id_format != 0 {
            len += 1;
        }
        if self.transient_mapping_attribute_name.is_some() {
            len += 1;
        }
        if self.federated_logout_enabled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.idp.v1.SAMLConfig", len)?;
        if !self.metadata_xml.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("metadataXml", pbjson::private::base64::encode(&self.metadata_xml).as_str())?;
        }
        if self.binding != 0 {
            let v = SamlBinding::try_from(self.binding)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.binding)))?;
            struct_ser.serialize_field("binding", &v)?;
        }
        if self.with_signed_request {
            struct_ser.serialize_field("withSignedRequest", &self.with_signed_request)?;
        }
        if self.name_id_format != 0 {
            let v = SamlNameIdFormat::try_from(self.name_id_format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.name_id_format)))?;
            struct_ser.serialize_field("nameIdFormat", &v)?;
        }
        if let Some(v) = self.transient_mapping_attribute_name.as_ref() {
            struct_ser.serialize_field("transientMappingAttributeName", v)?;
        }
        if let Some(v) = self.federated_logout_enabled.as_ref() {
            struct_ser.serialize_field("federatedLogoutEnabled", v)?;
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
            "metadata_xml",
            "metadataXml",
            "binding",
            "with_signed_request",
            "withSignedRequest",
            "name_id_format",
            "nameIdFormat",
            "transient_mapping_attribute_name",
            "transientMappingAttributeName",
            "federated_logout_enabled",
            "federatedLogoutEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataXml,
            Binding,
            WithSignedRequest,
            NameIdFormat,
            TransientMappingAttributeName,
            FederatedLogoutEnabled,
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
                            "metadataXml" | "metadata_xml" => Ok(GeneratedField::MetadataXml),
                            "binding" => Ok(GeneratedField::Binding),
                            "withSignedRequest" | "with_signed_request" => Ok(GeneratedField::WithSignedRequest),
                            "nameIdFormat" | "name_id_format" => Ok(GeneratedField::NameIdFormat),
                            "transientMappingAttributeName" | "transient_mapping_attribute_name" => Ok(GeneratedField::TransientMappingAttributeName),
                            "federatedLogoutEnabled" | "federated_logout_enabled" => Ok(GeneratedField::FederatedLogoutEnabled),
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
                formatter.write_str("struct zitadel.idp.v1.SAMLConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SamlConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_xml__ = None;
                let mut binding__ = None;
                let mut with_signed_request__ = None;
                let mut name_id_format__ = None;
                let mut transient_mapping_attribute_name__ = None;
                let mut federated_logout_enabled__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MetadataXml => {
                            if metadata_xml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataXml"));
                            }
                            metadata_xml__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Binding => {
                            if binding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binding"));
                            }
                            binding__ = Some(map_.next_value::<SamlBinding>()? as i32);
                        }
                        GeneratedField::WithSignedRequest => {
                            if with_signed_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withSignedRequest"));
                            }
                            with_signed_request__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NameIdFormat => {
                            if name_id_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameIdFormat"));
                            }
                            name_id_format__ = Some(map_.next_value::<SamlNameIdFormat>()? as i32);
                        }
                        GeneratedField::TransientMappingAttributeName => {
                            if transient_mapping_attribute_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transientMappingAttributeName"));
                            }
                            transient_mapping_attribute_name__ = map_.next_value()?;
                        }
                        GeneratedField::FederatedLogoutEnabled => {
                            if federated_logout_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("federatedLogoutEnabled"));
                            }
                            federated_logout_enabled__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SamlConfig {
                    metadata_xml: metadata_xml__.unwrap_or_default(),
                    binding: binding__.unwrap_or_default(),
                    with_signed_request: with_signed_request__.unwrap_or_default(),
                    name_id_format: name_id_format__.unwrap_or_default(),
                    transient_mapping_attribute_name: transient_mapping_attribute_name__,
                    federated_logout_enabled: federated_logout_enabled__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.idp.v1.SAMLConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SamlNameIdFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SAML_NAME_ID_FORMAT_UNSPECIFIED",
            Self::EmailAddress => "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS",
            Self::Persistent => "SAML_NAME_ID_FORMAT_PERSISTENT",
            Self::Transient => "SAML_NAME_ID_FORMAT_TRANSIENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SamlNameIdFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SAML_NAME_ID_FORMAT_UNSPECIFIED",
            "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS",
            "SAML_NAME_ID_FORMAT_PERSISTENT",
            "SAML_NAME_ID_FORMAT_TRANSIENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SamlNameIdFormat;

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
                    "SAML_NAME_ID_FORMAT_UNSPECIFIED" => Ok(SamlNameIdFormat::Unspecified),
                    "SAML_NAME_ID_FORMAT_EMAIL_ADDRESS" => Ok(SamlNameIdFormat::EmailAddress),
                    "SAML_NAME_ID_FORMAT_PERSISTENT" => Ok(SamlNameIdFormat::Persistent),
                    "SAML_NAME_ID_FORMAT_TRANSIENT" => Ok(SamlNameIdFormat::Transient),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
