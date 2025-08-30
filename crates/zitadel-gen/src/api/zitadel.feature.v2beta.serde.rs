// @generated
impl serde::Serialize for FeatureFlag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.FeatureFlag", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if self.source != 0 {
            let v = Source::try_from(self.source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeatureFlag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "source",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            Source,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "source" => Ok(GeneratedField::Source),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeatureFlag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.FeatureFlag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeatureFlag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value::<Source>()? as i32);
                        }
                    }
                }
                Ok(FeatureFlag {
                    enabled: enabled__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.FeatureFlag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInstanceFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.inheritance {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetInstanceFeaturesRequest", len)?;
        if self.inheritance {
            struct_ser.serialize_field("inheritance", &self.inheritance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInstanceFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inheritance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Inheritance,
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
                            "inheritance" => Ok(GeneratedField::Inheritance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetInstanceFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetInstanceFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInstanceFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inheritance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Inheritance => {
                            if inheritance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inheritance"));
                            }
                            inheritance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetInstanceFeaturesRequest {
                    inheritance: inheritance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetInstanceFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInstanceFeaturesResponse {
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
        if self.login_default_org.is_some() {
            len += 1;
        }
        if self.user_schema.is_some() {
            len += 1;
        }
        if self.oidc_token_exchange.is_some() {
            len += 1;
        }
        if self.improved_performance.is_some() {
            len += 1;
        }
        if self.debug_oidc_parent_error.is_some() {
            len += 1;
        }
        if self.oidc_single_v1_session_termination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetInstanceFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.login_default_org.as_ref() {
            struct_ser.serialize_field("loginDefaultOrg", v)?;
        }
        if let Some(v) = self.user_schema.as_ref() {
            struct_ser.serialize_field("userSchema", v)?;
        }
        if let Some(v) = self.oidc_token_exchange.as_ref() {
            struct_ser.serialize_field("oidcTokenExchange", v)?;
        }
        if let Some(v) = self.improved_performance.as_ref() {
            struct_ser.serialize_field("improvedPerformance", v)?;
        }
        if let Some(v) = self.debug_oidc_parent_error.as_ref() {
            struct_ser.serialize_field("debugOidcParentError", v)?;
        }
        if let Some(v) = self.oidc_single_v1_session_termination.as_ref() {
            struct_ser.serialize_field("oidcSingleV1SessionTermination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInstanceFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "login_default_org",
            "loginDefaultOrg",
            "user_schema",
            "userSchema",
            "oidc_token_exchange",
            "oidcTokenExchange",
            "improved_performance",
            "improvedPerformance",
            "debug_oidc_parent_error",
            "debugOidcParentError",
            "oidc_single_v1_session_termination",
            "oidcSingleV1SessionTermination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            LoginDefaultOrg,
            UserSchema,
            OidcTokenExchange,
            ImprovedPerformance,
            DebugOidcParentError,
            OidcSingleV1SessionTermination,
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
                            "loginDefaultOrg" | "login_default_org" => Ok(GeneratedField::LoginDefaultOrg),
                            "userSchema" | "user_schema" => Ok(GeneratedField::UserSchema),
                            "oidcTokenExchange" | "oidc_token_exchange" => Ok(GeneratedField::OidcTokenExchange),
                            "improvedPerformance" | "improved_performance" => Ok(GeneratedField::ImprovedPerformance),
                            "debugOidcParentError" | "debug_oidc_parent_error" => Ok(GeneratedField::DebugOidcParentError),
                            "oidcSingleV1SessionTermination" | "oidc_single_v1_session_termination" => Ok(GeneratedField::OidcSingleV1SessionTermination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetInstanceFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetInstanceFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInstanceFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut login_default_org__ = None;
                let mut user_schema__ = None;
                let mut oidc_token_exchange__ = None;
                let mut improved_performance__ = None;
                let mut debug_oidc_parent_error__ = None;
                let mut oidc_single_v1_session_termination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::LoginDefaultOrg => {
                            if login_default_org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginDefaultOrg"));
                            }
                            login_default_org__ = map_.next_value()?;
                        }
                        GeneratedField::UserSchema => {
                            if user_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userSchema"));
                            }
                            user_schema__ = map_.next_value()?;
                        }
                        GeneratedField::OidcTokenExchange => {
                            if oidc_token_exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcTokenExchange"));
                            }
                            oidc_token_exchange__ = map_.next_value()?;
                        }
                        GeneratedField::ImprovedPerformance => {
                            if improved_performance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("improvedPerformance"));
                            }
                            improved_performance__ = map_.next_value()?;
                        }
                        GeneratedField::DebugOidcParentError => {
                            if debug_oidc_parent_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debugOidcParentError"));
                            }
                            debug_oidc_parent_error__ = map_.next_value()?;
                        }
                        GeneratedField::OidcSingleV1SessionTermination => {
                            if oidc_single_v1_session_termination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcSingleV1SessionTermination"));
                            }
                            oidc_single_v1_session_termination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetInstanceFeaturesResponse {
                    details: details__,
                    login_default_org: login_default_org__,
                    user_schema: user_schema__,
                    oidc_token_exchange: oidc_token_exchange__,
                    improved_performance: improved_performance__,
                    debug_oidc_parent_error: debug_oidc_parent_error__,
                    oidc_single_v1_session_termination: oidc_single_v1_session_termination__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetInstanceFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrganizationFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.inheritance {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetOrganizationFeaturesRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if self.inheritance {
            struct_ser.serialize_field("inheritance", &self.inheritance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrganizationFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "inheritance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            Inheritance,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "inheritance" => Ok(GeneratedField::Inheritance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrganizationFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetOrganizationFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrganizationFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut inheritance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Inheritance => {
                            if inheritance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inheritance"));
                            }
                            inheritance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetOrganizationFeaturesRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                    inheritance: inheritance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetOrganizationFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOrganizationFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetOrganizationFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOrganizationFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOrganizationFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetOrganizationFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetOrganizationFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetOrganizationFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetOrganizationFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSystemFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetSystemFeaturesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSystemFeaturesRequest {
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
            type Value = GetSystemFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetSystemFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSystemFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetSystemFeaturesRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetSystemFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSystemFeaturesResponse {
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
        if self.login_default_org.is_some() {
            len += 1;
        }
        if self.user_schema.is_some() {
            len += 1;
        }
        if self.oidc_token_exchange.is_some() {
            len += 1;
        }
        if self.improved_performance.is_some() {
            len += 1;
        }
        if self.oidc_single_v1_session_termination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetSystemFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.login_default_org.as_ref() {
            struct_ser.serialize_field("loginDefaultOrg", v)?;
        }
        if let Some(v) = self.user_schema.as_ref() {
            struct_ser.serialize_field("userSchema", v)?;
        }
        if let Some(v) = self.oidc_token_exchange.as_ref() {
            struct_ser.serialize_field("oidcTokenExchange", v)?;
        }
        if let Some(v) = self.improved_performance.as_ref() {
            struct_ser.serialize_field("improvedPerformance", v)?;
        }
        if let Some(v) = self.oidc_single_v1_session_termination.as_ref() {
            struct_ser.serialize_field("oidcSingleV1SessionTermination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSystemFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "login_default_org",
            "loginDefaultOrg",
            "user_schema",
            "userSchema",
            "oidc_token_exchange",
            "oidcTokenExchange",
            "improved_performance",
            "improvedPerformance",
            "oidc_single_v1_session_termination",
            "oidcSingleV1SessionTermination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            LoginDefaultOrg,
            UserSchema,
            OidcTokenExchange,
            ImprovedPerformance,
            OidcSingleV1SessionTermination,
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
                            "loginDefaultOrg" | "login_default_org" => Ok(GeneratedField::LoginDefaultOrg),
                            "userSchema" | "user_schema" => Ok(GeneratedField::UserSchema),
                            "oidcTokenExchange" | "oidc_token_exchange" => Ok(GeneratedField::OidcTokenExchange),
                            "improvedPerformance" | "improved_performance" => Ok(GeneratedField::ImprovedPerformance),
                            "oidcSingleV1SessionTermination" | "oidc_single_v1_session_termination" => Ok(GeneratedField::OidcSingleV1SessionTermination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSystemFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetSystemFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSystemFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut login_default_org__ = None;
                let mut user_schema__ = None;
                let mut oidc_token_exchange__ = None;
                let mut improved_performance__ = None;
                let mut oidc_single_v1_session_termination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::LoginDefaultOrg => {
                            if login_default_org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginDefaultOrg"));
                            }
                            login_default_org__ = map_.next_value()?;
                        }
                        GeneratedField::UserSchema => {
                            if user_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userSchema"));
                            }
                            user_schema__ = map_.next_value()?;
                        }
                        GeneratedField::OidcTokenExchange => {
                            if oidc_token_exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcTokenExchange"));
                            }
                            oidc_token_exchange__ = map_.next_value()?;
                        }
                        GeneratedField::ImprovedPerformance => {
                            if improved_performance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("improvedPerformance"));
                            }
                            improved_performance__ = map_.next_value()?;
                        }
                        GeneratedField::OidcSingleV1SessionTermination => {
                            if oidc_single_v1_session_termination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcSingleV1SessionTermination"));
                            }
                            oidc_single_v1_session_termination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSystemFeaturesResponse {
                    details: details__,
                    login_default_org: login_default_org__,
                    user_schema: user_schema__,
                    oidc_token_exchange: oidc_token_exchange__,
                    improved_performance: improved_performance__,
                    oidc_single_v1_session_termination: oidc_single_v1_session_termination__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetSystemFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserFeaturesRequest {
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
        if self.inheritance {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetUserFeaturesRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.inheritance {
            struct_ser.serialize_field("inheritance", &self.inheritance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "inheritance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Inheritance,
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
                            "inheritance" => Ok(GeneratedField::Inheritance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetUserFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut inheritance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Inheritance => {
                            if inheritance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inheritance"));
                            }
                            inheritance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserFeaturesRequest {
                    user_id: user_id__.unwrap_or_default(),
                    inheritance: inheritance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetUserFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.GetUserFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.GetUserFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.GetUserFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImprovedPerformance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IMPROVED_PERFORMANCE_UNSPECIFIED",
            Self::ProjectGrant => "IMPROVED_PERFORMANCE_PROJECT_GRANT",
            Self::Project => "IMPROVED_PERFORMANCE_PROJECT",
            Self::UserGrant => "IMPROVED_PERFORMANCE_USER_GRANT",
            Self::OrgDomainVerified => "IMPROVED_PERFORMANCE_ORG_DOMAIN_VERIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ImprovedPerformance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IMPROVED_PERFORMANCE_UNSPECIFIED",
            "IMPROVED_PERFORMANCE_PROJECT_GRANT",
            "IMPROVED_PERFORMANCE_PROJECT",
            "IMPROVED_PERFORMANCE_USER_GRANT",
            "IMPROVED_PERFORMANCE_ORG_DOMAIN_VERIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImprovedPerformance;

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
                    "IMPROVED_PERFORMANCE_UNSPECIFIED" => Ok(ImprovedPerformance::Unspecified),
                    "IMPROVED_PERFORMANCE_PROJECT_GRANT" => Ok(ImprovedPerformance::ProjectGrant),
                    "IMPROVED_PERFORMANCE_PROJECT" => Ok(ImprovedPerformance::Project),
                    "IMPROVED_PERFORMANCE_USER_GRANT" => Ok(ImprovedPerformance::UserGrant),
                    "IMPROVED_PERFORMANCE_ORG_DOMAIN_VERIFIED" => Ok(ImprovedPerformance::OrgDomainVerified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ImprovedPerformanceFeatureFlag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.execution_paths.is_empty() {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ImprovedPerformanceFeatureFlag", len)?;
        if !self.execution_paths.is_empty() {
            let v = self.execution_paths.iter().cloned().map(|v| {
                ImprovedPerformance::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("executionPaths", &v)?;
        }
        if self.source != 0 {
            let v = Source::try_from(self.source)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImprovedPerformanceFeatureFlag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "execution_paths",
            "executionPaths",
            "source",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExecutionPaths,
            Source,
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
                            "executionPaths" | "execution_paths" => Ok(GeneratedField::ExecutionPaths),
                            "source" => Ok(GeneratedField::Source),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImprovedPerformanceFeatureFlag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ImprovedPerformanceFeatureFlag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImprovedPerformanceFeatureFlag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut execution_paths__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExecutionPaths => {
                            if execution_paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionPaths"));
                            }
                            execution_paths__ = Some(map_.next_value::<Vec<ImprovedPerformance>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value::<Source>()? as i32);
                        }
                    }
                }
                Ok(ImprovedPerformanceFeatureFlag {
                    execution_paths: execution_paths__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ImprovedPerformanceFeatureFlag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetInstanceFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetInstanceFeaturesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetInstanceFeaturesRequest {
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
            type Value = ResetInstanceFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetInstanceFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetInstanceFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResetInstanceFeaturesRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetInstanceFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetInstanceFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetInstanceFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetInstanceFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetInstanceFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetInstanceFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetInstanceFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResetInstanceFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetInstanceFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetOrganizationFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetOrganizationFeaturesRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetOrganizationFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ResetOrganizationFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetOrganizationFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetOrganizationFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResetOrganizationFeaturesRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetOrganizationFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetOrganizationFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetOrganizationFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetOrganizationFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetOrganizationFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetOrganizationFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetOrganizationFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResetOrganizationFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetOrganizationFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetSystemFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetSystemFeaturesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetSystemFeaturesRequest {
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
            type Value = ResetSystemFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetSystemFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetSystemFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResetSystemFeaturesRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetSystemFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetSystemFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetSystemFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetSystemFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetSystemFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetSystemFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetSystemFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResetSystemFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetSystemFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetUserFeaturesRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetUserFeaturesRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetUserFeaturesRequest {
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
            type Value = ResetUserFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetUserFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetUserFeaturesRequest, V::Error>
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
                Ok(ResetUserFeaturesRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetUserFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetUserFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.ResetUserFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetUserFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetUserFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.ResetUserFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetUserFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResetUserFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.ResetUserFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetInstanceFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.login_default_org.is_some() {
            len += 1;
        }
        if self.user_schema.is_some() {
            len += 1;
        }
        if self.oidc_token_exchange.is_some() {
            len += 1;
        }
        if !self.improved_performance.is_empty() {
            len += 1;
        }
        if self.debug_oidc_parent_error.is_some() {
            len += 1;
        }
        if self.oidc_single_v1_session_termination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetInstanceFeaturesRequest", len)?;
        if let Some(v) = self.login_default_org.as_ref() {
            struct_ser.serialize_field("loginDefaultOrg", v)?;
        }
        if let Some(v) = self.user_schema.as_ref() {
            struct_ser.serialize_field("userSchema", v)?;
        }
        if let Some(v) = self.oidc_token_exchange.as_ref() {
            struct_ser.serialize_field("oidcTokenExchange", v)?;
        }
        if !self.improved_performance.is_empty() {
            let v = self.improved_performance.iter().cloned().map(|v| {
                ImprovedPerformance::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("improvedPerformance", &v)?;
        }
        if let Some(v) = self.debug_oidc_parent_error.as_ref() {
            struct_ser.serialize_field("debugOidcParentError", v)?;
        }
        if let Some(v) = self.oidc_single_v1_session_termination.as_ref() {
            struct_ser.serialize_field("oidcSingleV1SessionTermination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetInstanceFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login_default_org",
            "loginDefaultOrg",
            "user_schema",
            "userSchema",
            "oidc_token_exchange",
            "oidcTokenExchange",
            "improved_performance",
            "improvedPerformance",
            "debug_oidc_parent_error",
            "debugOidcParentError",
            "oidc_single_v1_session_termination",
            "oidcSingleV1SessionTermination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoginDefaultOrg,
            UserSchema,
            OidcTokenExchange,
            ImprovedPerformance,
            DebugOidcParentError,
            OidcSingleV1SessionTermination,
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
                            "loginDefaultOrg" | "login_default_org" => Ok(GeneratedField::LoginDefaultOrg),
                            "userSchema" | "user_schema" => Ok(GeneratedField::UserSchema),
                            "oidcTokenExchange" | "oidc_token_exchange" => Ok(GeneratedField::OidcTokenExchange),
                            "improvedPerformance" | "improved_performance" => Ok(GeneratedField::ImprovedPerformance),
                            "debugOidcParentError" | "debug_oidc_parent_error" => Ok(GeneratedField::DebugOidcParentError),
                            "oidcSingleV1SessionTermination" | "oidc_single_v1_session_termination" => Ok(GeneratedField::OidcSingleV1SessionTermination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetInstanceFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetInstanceFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetInstanceFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut login_default_org__ = None;
                let mut user_schema__ = None;
                let mut oidc_token_exchange__ = None;
                let mut improved_performance__ = None;
                let mut debug_oidc_parent_error__ = None;
                let mut oidc_single_v1_session_termination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoginDefaultOrg => {
                            if login_default_org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginDefaultOrg"));
                            }
                            login_default_org__ = map_.next_value()?;
                        }
                        GeneratedField::UserSchema => {
                            if user_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userSchema"));
                            }
                            user_schema__ = map_.next_value()?;
                        }
                        GeneratedField::OidcTokenExchange => {
                            if oidc_token_exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcTokenExchange"));
                            }
                            oidc_token_exchange__ = map_.next_value()?;
                        }
                        GeneratedField::ImprovedPerformance => {
                            if improved_performance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("improvedPerformance"));
                            }
                            improved_performance__ = Some(map_.next_value::<Vec<ImprovedPerformance>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::DebugOidcParentError => {
                            if debug_oidc_parent_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debugOidcParentError"));
                            }
                            debug_oidc_parent_error__ = map_.next_value()?;
                        }
                        GeneratedField::OidcSingleV1SessionTermination => {
                            if oidc_single_v1_session_termination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcSingleV1SessionTermination"));
                            }
                            oidc_single_v1_session_termination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetInstanceFeaturesRequest {
                    login_default_org: login_default_org__,
                    user_schema: user_schema__,
                    oidc_token_exchange: oidc_token_exchange__,
                    improved_performance: improved_performance__.unwrap_or_default(),
                    debug_oidc_parent_error: debug_oidc_parent_error__,
                    oidc_single_v1_session_termination: oidc_single_v1_session_termination__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetInstanceFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetInstanceFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetInstanceFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetInstanceFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetInstanceFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetInstanceFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetInstanceFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetInstanceFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetInstanceFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetOrganizationFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetOrganizationFeaturesRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetOrganizationFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SetOrganizationFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetOrganizationFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetOrganizationFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetOrganizationFeaturesRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetOrganizationFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetOrganizationFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetOrganizationFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetOrganizationFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetOrganizationFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetOrganizationFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetOrganizationFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetOrganizationFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetOrganizationFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSystemFeaturesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.login_default_org.is_some() {
            len += 1;
        }
        if self.user_schema.is_some() {
            len += 1;
        }
        if self.oidc_token_exchange.is_some() {
            len += 1;
        }
        if !self.improved_performance.is_empty() {
            len += 1;
        }
        if self.oidc_single_v1_session_termination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetSystemFeaturesRequest", len)?;
        if let Some(v) = self.login_default_org.as_ref() {
            struct_ser.serialize_field("loginDefaultOrg", v)?;
        }
        if let Some(v) = self.user_schema.as_ref() {
            struct_ser.serialize_field("userSchema", v)?;
        }
        if let Some(v) = self.oidc_token_exchange.as_ref() {
            struct_ser.serialize_field("oidcTokenExchange", v)?;
        }
        if !self.improved_performance.is_empty() {
            let v = self.improved_performance.iter().cloned().map(|v| {
                ImprovedPerformance::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("improvedPerformance", &v)?;
        }
        if let Some(v) = self.oidc_single_v1_session_termination.as_ref() {
            struct_ser.serialize_field("oidcSingleV1SessionTermination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSystemFeaturesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login_default_org",
            "loginDefaultOrg",
            "user_schema",
            "userSchema",
            "oidc_token_exchange",
            "oidcTokenExchange",
            "improved_performance",
            "improvedPerformance",
            "oidc_single_v1_session_termination",
            "oidcSingleV1SessionTermination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoginDefaultOrg,
            UserSchema,
            OidcTokenExchange,
            ImprovedPerformance,
            OidcSingleV1SessionTermination,
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
                            "loginDefaultOrg" | "login_default_org" => Ok(GeneratedField::LoginDefaultOrg),
                            "userSchema" | "user_schema" => Ok(GeneratedField::UserSchema),
                            "oidcTokenExchange" | "oidc_token_exchange" => Ok(GeneratedField::OidcTokenExchange),
                            "improvedPerformance" | "improved_performance" => Ok(GeneratedField::ImprovedPerformance),
                            "oidcSingleV1SessionTermination" | "oidc_single_v1_session_termination" => Ok(GeneratedField::OidcSingleV1SessionTermination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetSystemFeaturesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetSystemFeaturesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSystemFeaturesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut login_default_org__ = None;
                let mut user_schema__ = None;
                let mut oidc_token_exchange__ = None;
                let mut improved_performance__ = None;
                let mut oidc_single_v1_session_termination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoginDefaultOrg => {
                            if login_default_org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginDefaultOrg"));
                            }
                            login_default_org__ = map_.next_value()?;
                        }
                        GeneratedField::UserSchema => {
                            if user_schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userSchema"));
                            }
                            user_schema__ = map_.next_value()?;
                        }
                        GeneratedField::OidcTokenExchange => {
                            if oidc_token_exchange__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcTokenExchange"));
                            }
                            oidc_token_exchange__ = map_.next_value()?;
                        }
                        GeneratedField::ImprovedPerformance => {
                            if improved_performance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("improvedPerformance"));
                            }
                            improved_performance__ = Some(map_.next_value::<Vec<ImprovedPerformance>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::OidcSingleV1SessionTermination => {
                            if oidc_single_v1_session_termination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcSingleV1SessionTermination"));
                            }
                            oidc_single_v1_session_termination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetSystemFeaturesRequest {
                    login_default_org: login_default_org__,
                    user_schema: user_schema__,
                    oidc_token_exchange: oidc_token_exchange__,
                    improved_performance: improved_performance__.unwrap_or_default(),
                    oidc_single_v1_session_termination: oidc_single_v1_session_termination__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetSystemFeaturesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSystemFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetSystemFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSystemFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetSystemFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetSystemFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSystemFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetSystemFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetSystemFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetUserFeatureRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetUserFeatureRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetUserFeatureRequest {
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
            type Value = SetUserFeatureRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetUserFeatureRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetUserFeatureRequest, V::Error>
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
                Ok(SetUserFeatureRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetUserFeatureRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetUserFeaturesResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.feature.v2beta.SetUserFeaturesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetUserFeaturesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetUserFeaturesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.feature.v2beta.SetUserFeaturesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetUserFeaturesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetUserFeaturesResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.feature.v2beta.SetUserFeaturesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SOURCE_UNSPECIFIED",
            Self::System => "SOURCE_SYSTEM",
            Self::Instance => "SOURCE_INSTANCE",
            Self::Organization => "SOURCE_ORGANIZATION",
            Self::Project => "SOURCE_PROJECT",
            Self::App => "SOURCE_APP",
            Self::User => "SOURCE_USER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SOURCE_UNSPECIFIED",
            "SOURCE_SYSTEM",
            "SOURCE_INSTANCE",
            "SOURCE_ORGANIZATION",
            "SOURCE_PROJECT",
            "SOURCE_APP",
            "SOURCE_USER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Source;

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
                    "SOURCE_UNSPECIFIED" => Ok(Source::Unspecified),
                    "SOURCE_SYSTEM" => Ok(Source::System),
                    "SOURCE_INSTANCE" => Ok(Source::Instance),
                    "SOURCE_ORGANIZATION" => Ok(Source::Organization),
                    "SOURCE_PROJECT" => Ok(Source::Project),
                    "SOURCE_APP" => Ok(Source::App),
                    "SOURCE_USER" => Ok(Source::User),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
