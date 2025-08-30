// @generated
impl serde::Serialize for BrandingSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.light_theme.is_some() {
            len += 1;
        }
        if self.dark_theme.is_some() {
            len += 1;
        }
        if !self.font_url.is_empty() {
            len += 1;
        }
        if self.hide_login_name_suffix {
            len += 1;
        }
        if self.disable_watermark {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        if self.theme_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.BrandingSettings", len)?;
        if let Some(v) = self.light_theme.as_ref() {
            struct_ser.serialize_field("lightTheme", v)?;
        }
        if let Some(v) = self.dark_theme.as_ref() {
            struct_ser.serialize_field("darkTheme", v)?;
        }
        if !self.font_url.is_empty() {
            struct_ser.serialize_field("fontUrl", &self.font_url)?;
        }
        if self.hide_login_name_suffix {
            struct_ser.serialize_field("hideLoginNameSuffix", &self.hide_login_name_suffix)?;
        }
        if self.disable_watermark {
            struct_ser.serialize_field("disableWatermark", &self.disable_watermark)?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        if self.theme_mode != 0 {
            let v = ThemeMode::try_from(self.theme_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.theme_mode)))?;
            struct_ser.serialize_field("themeMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BrandingSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "light_theme",
            "lightTheme",
            "dark_theme",
            "darkTheme",
            "font_url",
            "fontUrl",
            "hide_login_name_suffix",
            "hideLoginNameSuffix",
            "disable_watermark",
            "disableWatermark",
            "resource_owner_type",
            "resourceOwnerType",
            "theme_mode",
            "themeMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LightTheme,
            DarkTheme,
            FontUrl,
            HideLoginNameSuffix,
            DisableWatermark,
            ResourceOwnerType,
            ThemeMode,
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
                            "lightTheme" | "light_theme" => Ok(GeneratedField::LightTheme),
                            "darkTheme" | "dark_theme" => Ok(GeneratedField::DarkTheme),
                            "fontUrl" | "font_url" => Ok(GeneratedField::FontUrl),
                            "hideLoginNameSuffix" | "hide_login_name_suffix" => Ok(GeneratedField::HideLoginNameSuffix),
                            "disableWatermark" | "disable_watermark" => Ok(GeneratedField::DisableWatermark),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            "themeMode" | "theme_mode" => Ok(GeneratedField::ThemeMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BrandingSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.BrandingSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BrandingSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut light_theme__ = None;
                let mut dark_theme__ = None;
                let mut font_url__ = None;
                let mut hide_login_name_suffix__ = None;
                let mut disable_watermark__ = None;
                let mut resource_owner_type__ = None;
                let mut theme_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LightTheme => {
                            if light_theme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lightTheme"));
                            }
                            light_theme__ = map_.next_value()?;
                        }
                        GeneratedField::DarkTheme => {
                            if dark_theme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("darkTheme"));
                            }
                            dark_theme__ = map_.next_value()?;
                        }
                        GeneratedField::FontUrl => {
                            if font_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fontUrl"));
                            }
                            font_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HideLoginNameSuffix => {
                            if hide_login_name_suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hideLoginNameSuffix"));
                            }
                            hide_login_name_suffix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableWatermark => {
                            if disable_watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableWatermark"));
                            }
                            disable_watermark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                        GeneratedField::ThemeMode => {
                            if theme_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("themeMode"));
                            }
                            theme_mode__ = Some(map_.next_value::<ThemeMode>()? as i32);
                        }
                    }
                }
                Ok(BrandingSettings {
                    light_theme: light_theme__,
                    dark_theme: dark_theme__,
                    font_url: font_url__.unwrap_or_default(),
                    hide_login_name_suffix: hide_login_name_suffix__.unwrap_or_default(),
                    disable_watermark: disable_watermark__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                    theme_mode: theme_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.BrandingSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DomainSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.login_name_includes_domain {
            len += 1;
        }
        if self.require_org_domain_verification {
            len += 1;
        }
        if self.smtp_sender_address_matches_instance_domain {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.DomainSettings", len)?;
        if self.login_name_includes_domain {
            struct_ser.serialize_field("loginNameIncludesDomain", &self.login_name_includes_domain)?;
        }
        if self.require_org_domain_verification {
            struct_ser.serialize_field("requireOrgDomainVerification", &self.require_org_domain_verification)?;
        }
        if self.smtp_sender_address_matches_instance_domain {
            struct_ser.serialize_field("smtpSenderAddressMatchesInstanceDomain", &self.smtp_sender_address_matches_instance_domain)?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DomainSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login_name_includes_domain",
            "loginNameIncludesDomain",
            "require_org_domain_verification",
            "requireOrgDomainVerification",
            "smtp_sender_address_matches_instance_domain",
            "smtpSenderAddressMatchesInstanceDomain",
            "resource_owner_type",
            "resourceOwnerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LoginNameIncludesDomain,
            RequireOrgDomainVerification,
            SmtpSenderAddressMatchesInstanceDomain,
            ResourceOwnerType,
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
                            "loginNameIncludesDomain" | "login_name_includes_domain" => Ok(GeneratedField::LoginNameIncludesDomain),
                            "requireOrgDomainVerification" | "require_org_domain_verification" => Ok(GeneratedField::RequireOrgDomainVerification),
                            "smtpSenderAddressMatchesInstanceDomain" | "smtp_sender_address_matches_instance_domain" => Ok(GeneratedField::SmtpSenderAddressMatchesInstanceDomain),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DomainSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.DomainSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DomainSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut login_name_includes_domain__ = None;
                let mut require_org_domain_verification__ = None;
                let mut smtp_sender_address_matches_instance_domain__ = None;
                let mut resource_owner_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LoginNameIncludesDomain => {
                            if login_name_includes_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginNameIncludesDomain"));
                            }
                            login_name_includes_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequireOrgDomainVerification => {
                            if require_org_domain_verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireOrgDomainVerification"));
                            }
                            require_org_domain_verification__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SmtpSenderAddressMatchesInstanceDomain => {
                            if smtp_sender_address_matches_instance_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("smtpSenderAddressMatchesInstanceDomain"));
                            }
                            smtp_sender_address_matches_instance_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                    }
                }
                Ok(DomainSettings {
                    login_name_includes_domain: login_name_includes_domain__.unwrap_or_default(),
                    require_org_domain_verification: require_org_domain_verification__.unwrap_or_default(),
                    smtp_sender_address_matches_instance_domain: smtp_sender_address_matches_instance_domain__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.DomainSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmbeddedIframeSettings {
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
        if !self.allowed_origins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.EmbeddedIframeSettings", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if !self.allowed_origins.is_empty() {
            struct_ser.serialize_field("allowedOrigins", &self.allowed_origins)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmbeddedIframeSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "allowed_origins",
            "allowedOrigins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            AllowedOrigins,
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
                            "allowedOrigins" | "allowed_origins" => Ok(GeneratedField::AllowedOrigins),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmbeddedIframeSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.EmbeddedIframeSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmbeddedIframeSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut allowed_origins__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedOrigins => {
                            if allowed_origins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedOrigins"));
                            }
                            allowed_origins__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EmbeddedIframeSettings {
                    enabled: enabled__.unwrap_or_default(),
                    allowed_origins: allowed_origins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.EmbeddedIframeSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetActiveIdentityProvidersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetActiveIdentityProvidersRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetActiveIdentityProvidersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetActiveIdentityProvidersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetActiveIdentityProvidersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetActiveIdentityProvidersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetActiveIdentityProvidersRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetActiveIdentityProvidersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetActiveIdentityProvidersResponse {
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
        if !self.identity_providers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetActiveIdentityProvidersResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.identity_providers.is_empty() {
            struct_ser.serialize_field("identityProviders", &self.identity_providers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetActiveIdentityProvidersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "identity_providers",
            "identityProviders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            IdentityProviders,
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
                            "identityProviders" | "identity_providers" => Ok(GeneratedField::IdentityProviders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetActiveIdentityProvidersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetActiveIdentityProvidersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetActiveIdentityProvidersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut identity_providers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::IdentityProviders => {
                            if identity_providers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identityProviders"));
                            }
                            identity_providers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetActiveIdentityProvidersResponse {
                    details: details__,
                    identity_providers: identity_providers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetActiveIdentityProvidersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBrandingSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetBrandingSettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBrandingSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBrandingSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetBrandingSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBrandingSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetBrandingSettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetBrandingSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBrandingSettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetBrandingSettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBrandingSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBrandingSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetBrandingSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBrandingSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetBrandingSettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetBrandingSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDomainSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetDomainSettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDomainSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDomainSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetDomainSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDomainSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDomainSettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetDomainSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDomainSettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetDomainSettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDomainSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDomainSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetDomainSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDomainSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDomainSettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetDomainSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGeneralSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetGeneralSettingsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetGeneralSettingsRequest {
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
            type Value = GetGeneralSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetGeneralSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetGeneralSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetGeneralSettingsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetGeneralSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGeneralSettingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.default_org_id.is_empty() {
            len += 1;
        }
        if !self.default_language.is_empty() {
            len += 1;
        }
        if !self.supported_languages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetGeneralSettingsResponse", len)?;
        if !self.default_org_id.is_empty() {
            struct_ser.serialize_field("defaultOrgId", &self.default_org_id)?;
        }
        if !self.default_language.is_empty() {
            struct_ser.serialize_field("defaultLanguage", &self.default_language)?;
        }
        if !self.supported_languages.is_empty() {
            struct_ser.serialize_field("supportedLanguages", &self.supported_languages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetGeneralSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_org_id",
            "defaultOrgId",
            "default_language",
            "defaultLanguage",
            "supported_languages",
            "supportedLanguages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultOrgId,
            DefaultLanguage,
            SupportedLanguages,
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
                            "defaultOrgId" | "default_org_id" => Ok(GeneratedField::DefaultOrgId),
                            "defaultLanguage" | "default_language" => Ok(GeneratedField::DefaultLanguage),
                            "supportedLanguages" | "supported_languages" => Ok(GeneratedField::SupportedLanguages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetGeneralSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetGeneralSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetGeneralSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_org_id__ = None;
                let mut default_language__ = None;
                let mut supported_languages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DefaultOrgId => {
                            if default_org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultOrgId"));
                            }
                            default_org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultLanguage => {
                            if default_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLanguage"));
                            }
                            default_language__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SupportedLanguages => {
                            if supported_languages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportedLanguages"));
                            }
                            supported_languages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetGeneralSettingsResponse {
                    default_org_id: default_org_id__.unwrap_or_default(),
                    default_language: default_language__.unwrap_or_default(),
                    supported_languages: supported_languages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetGeneralSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLegalAndSupportSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetLegalAndSupportSettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLegalAndSupportSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLegalAndSupportSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetLegalAndSupportSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLegalAndSupportSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLegalAndSupportSettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetLegalAndSupportSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLegalAndSupportSettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetLegalAndSupportSettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLegalAndSupportSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLegalAndSupportSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetLegalAndSupportSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLegalAndSupportSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLegalAndSupportSettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetLegalAndSupportSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLockoutSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetLockoutSettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLockoutSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLockoutSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetLockoutSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLockoutSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLockoutSettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetLockoutSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLockoutSettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetLockoutSettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLockoutSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLockoutSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetLockoutSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLockoutSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLockoutSettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetLockoutSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLoginSettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetLoginSettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLoginSettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLoginSettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetLoginSettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLoginSettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLoginSettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetLoginSettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLoginSettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetLoginSettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLoginSettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLoginSettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetLoginSettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLoginSettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLoginSettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetLoginSettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPasswordComplexitySettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetPasswordComplexitySettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPasswordComplexitySettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPasswordComplexitySettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetPasswordComplexitySettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPasswordComplexitySettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPasswordComplexitySettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetPasswordComplexitySettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPasswordComplexitySettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetPasswordComplexitySettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPasswordComplexitySettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPasswordComplexitySettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetPasswordComplexitySettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPasswordComplexitySettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPasswordComplexitySettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetPasswordComplexitySettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPasswordExpirySettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ctx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetPasswordExpirySettingsRequest", len)?;
        if let Some(v) = self.ctx.as_ref() {
            struct_ser.serialize_field("ctx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPasswordExpirySettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ctx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ctx,
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
                            "ctx" => Ok(GeneratedField::Ctx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPasswordExpirySettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetPasswordExpirySettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPasswordExpirySettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ctx__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ctx => {
                            if ctx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ctx"));
                            }
                            ctx__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPasswordExpirySettingsRequest {
                    ctx: ctx__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetPasswordExpirySettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPasswordExpirySettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetPasswordExpirySettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPasswordExpirySettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPasswordExpirySettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetPasswordExpirySettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPasswordExpirySettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPasswordExpirySettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetPasswordExpirySettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSecuritySettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetSecuritySettingsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSecuritySettingsRequest {
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
            type Value = GetSecuritySettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetSecuritySettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSecuritySettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetSecuritySettingsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetSecuritySettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSecuritySettingsResponse {
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
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.GetSecuritySettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSecuritySettingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Settings,
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
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSecuritySettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.GetSecuritySettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSecuritySettingsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSecuritySettingsResponse {
                    details: details__,
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.GetSecuritySettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentityProvider {
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
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.IdentityProvider", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.r#type != 0 {
            let v = IdentityProviderType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdentityProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
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
            type Value = IdentityProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.IdentityProvider")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdentityProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut r#type__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<IdentityProviderType>()? as i32);
                        }
                    }
                }
                Ok(IdentityProvider {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.IdentityProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentityProviderType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IDENTITY_PROVIDER_TYPE_UNSPECIFIED",
            Self::Oidc => "IDENTITY_PROVIDER_TYPE_OIDC",
            Self::Jwt => "IDENTITY_PROVIDER_TYPE_JWT",
            Self::Ldap => "IDENTITY_PROVIDER_TYPE_LDAP",
            Self::Oauth => "IDENTITY_PROVIDER_TYPE_OAUTH",
            Self::AzureAd => "IDENTITY_PROVIDER_TYPE_AZURE_AD",
            Self::Github => "IDENTITY_PROVIDER_TYPE_GITHUB",
            Self::GithubEs => "IDENTITY_PROVIDER_TYPE_GITHUB_ES",
            Self::Gitlab => "IDENTITY_PROVIDER_TYPE_GITLAB",
            Self::GitlabSelfHosted => "IDENTITY_PROVIDER_TYPE_GITLAB_SELF_HOSTED",
            Self::Google => "IDENTITY_PROVIDER_TYPE_GOOGLE",
            Self::Saml => "IDENTITY_PROVIDER_TYPE_SAML",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdentityProviderType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDENTITY_PROVIDER_TYPE_UNSPECIFIED",
            "IDENTITY_PROVIDER_TYPE_OIDC",
            "IDENTITY_PROVIDER_TYPE_JWT",
            "IDENTITY_PROVIDER_TYPE_LDAP",
            "IDENTITY_PROVIDER_TYPE_OAUTH",
            "IDENTITY_PROVIDER_TYPE_AZURE_AD",
            "IDENTITY_PROVIDER_TYPE_GITHUB",
            "IDENTITY_PROVIDER_TYPE_GITHUB_ES",
            "IDENTITY_PROVIDER_TYPE_GITLAB",
            "IDENTITY_PROVIDER_TYPE_GITLAB_SELF_HOSTED",
            "IDENTITY_PROVIDER_TYPE_GOOGLE",
            "IDENTITY_PROVIDER_TYPE_SAML",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentityProviderType;

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
                    "IDENTITY_PROVIDER_TYPE_UNSPECIFIED" => Ok(IdentityProviderType::Unspecified),
                    "IDENTITY_PROVIDER_TYPE_OIDC" => Ok(IdentityProviderType::Oidc),
                    "IDENTITY_PROVIDER_TYPE_JWT" => Ok(IdentityProviderType::Jwt),
                    "IDENTITY_PROVIDER_TYPE_LDAP" => Ok(IdentityProviderType::Ldap),
                    "IDENTITY_PROVIDER_TYPE_OAUTH" => Ok(IdentityProviderType::Oauth),
                    "IDENTITY_PROVIDER_TYPE_AZURE_AD" => Ok(IdentityProviderType::AzureAd),
                    "IDENTITY_PROVIDER_TYPE_GITHUB" => Ok(IdentityProviderType::Github),
                    "IDENTITY_PROVIDER_TYPE_GITHUB_ES" => Ok(IdentityProviderType::GithubEs),
                    "IDENTITY_PROVIDER_TYPE_GITLAB" => Ok(IdentityProviderType::Gitlab),
                    "IDENTITY_PROVIDER_TYPE_GITLAB_SELF_HOSTED" => Ok(IdentityProviderType::GitlabSelfHosted),
                    "IDENTITY_PROVIDER_TYPE_GOOGLE" => Ok(IdentityProviderType::Google),
                    "IDENTITY_PROVIDER_TYPE_SAML" => Ok(IdentityProviderType::Saml),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LegalAndSupportSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tos_link.is_empty() {
            len += 1;
        }
        if !self.privacy_policy_link.is_empty() {
            len += 1;
        }
        if !self.help_link.is_empty() {
            len += 1;
        }
        if !self.support_email.is_empty() {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        if !self.docs_link.is_empty() {
            len += 1;
        }
        if !self.custom_link.is_empty() {
            len += 1;
        }
        if !self.custom_link_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.LegalAndSupportSettings", len)?;
        if !self.tos_link.is_empty() {
            struct_ser.serialize_field("tosLink", &self.tos_link)?;
        }
        if !self.privacy_policy_link.is_empty() {
            struct_ser.serialize_field("privacyPolicyLink", &self.privacy_policy_link)?;
        }
        if !self.help_link.is_empty() {
            struct_ser.serialize_field("helpLink", &self.help_link)?;
        }
        if !self.support_email.is_empty() {
            struct_ser.serialize_field("supportEmail", &self.support_email)?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        if !self.docs_link.is_empty() {
            struct_ser.serialize_field("docsLink", &self.docs_link)?;
        }
        if !self.custom_link.is_empty() {
            struct_ser.serialize_field("customLink", &self.custom_link)?;
        }
        if !self.custom_link_text.is_empty() {
            struct_ser.serialize_field("customLinkText", &self.custom_link_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LegalAndSupportSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tos_link",
            "tosLink",
            "privacy_policy_link",
            "privacyPolicyLink",
            "help_link",
            "helpLink",
            "support_email",
            "supportEmail",
            "resource_owner_type",
            "resourceOwnerType",
            "docs_link",
            "docsLink",
            "custom_link",
            "customLink",
            "custom_link_text",
            "customLinkText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TosLink,
            PrivacyPolicyLink,
            HelpLink,
            SupportEmail,
            ResourceOwnerType,
            DocsLink,
            CustomLink,
            CustomLinkText,
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
                            "tosLink" | "tos_link" => Ok(GeneratedField::TosLink),
                            "privacyPolicyLink" | "privacy_policy_link" => Ok(GeneratedField::PrivacyPolicyLink),
                            "helpLink" | "help_link" => Ok(GeneratedField::HelpLink),
                            "supportEmail" | "support_email" => Ok(GeneratedField::SupportEmail),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            "docsLink" | "docs_link" => Ok(GeneratedField::DocsLink),
                            "customLink" | "custom_link" => Ok(GeneratedField::CustomLink),
                            "customLinkText" | "custom_link_text" => Ok(GeneratedField::CustomLinkText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LegalAndSupportSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.LegalAndSupportSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LegalAndSupportSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tos_link__ = None;
                let mut privacy_policy_link__ = None;
                let mut help_link__ = None;
                let mut support_email__ = None;
                let mut resource_owner_type__ = None;
                let mut docs_link__ = None;
                let mut custom_link__ = None;
                let mut custom_link_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TosLink => {
                            if tos_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosLink"));
                            }
                            tos_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyPolicyLink => {
                            if privacy_policy_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyPolicyLink"));
                            }
                            privacy_policy_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HelpLink => {
                            if help_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("helpLink"));
                            }
                            help_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SupportEmail => {
                            if support_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportEmail"));
                            }
                            support_email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                        GeneratedField::DocsLink => {
                            if docs_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("docsLink"));
                            }
                            docs_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomLink => {
                            if custom_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customLink"));
                            }
                            custom_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomLinkText => {
                            if custom_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customLinkText"));
                            }
                            custom_link_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LegalAndSupportSettings {
                    tos_link: tos_link__.unwrap_or_default(),
                    privacy_policy_link: privacy_policy_link__.unwrap_or_default(),
                    help_link: help_link__.unwrap_or_default(),
                    support_email: support_email__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                    docs_link: docs_link__.unwrap_or_default(),
                    custom_link: custom_link__.unwrap_or_default(),
                    custom_link_text: custom_link_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.LegalAndSupportSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LockoutSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_password_attempts != 0 {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        if self.max_otp_attempts != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.LockoutSettings", len)?;
        if self.max_password_attempts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxPasswordAttempts", ToString::to_string(&self.max_password_attempts).as_str())?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        if self.max_otp_attempts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxOtpAttempts", ToString::to_string(&self.max_otp_attempts).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LockoutSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_password_attempts",
            "maxPasswordAttempts",
            "resource_owner_type",
            "resourceOwnerType",
            "max_otp_attempts",
            "maxOtpAttempts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxPasswordAttempts,
            ResourceOwnerType,
            MaxOtpAttempts,
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
                            "maxPasswordAttempts" | "max_password_attempts" => Ok(GeneratedField::MaxPasswordAttempts),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            "maxOtpAttempts" | "max_otp_attempts" => Ok(GeneratedField::MaxOtpAttempts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LockoutSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.LockoutSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LockoutSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_password_attempts__ = None;
                let mut resource_owner_type__ = None;
                let mut max_otp_attempts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxPasswordAttempts => {
                            if max_password_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPasswordAttempts"));
                            }
                            max_password_attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                        GeneratedField::MaxOtpAttempts => {
                            if max_otp_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxOtpAttempts"));
                            }
                            max_otp_attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LockoutSettings {
                    max_password_attempts: max_password_attempts__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                    max_otp_attempts: max_otp_attempts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.LockoutSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allow_username_password {
            len += 1;
        }
        if self.allow_register {
            len += 1;
        }
        if self.allow_external_idp {
            len += 1;
        }
        if self.force_mfa {
            len += 1;
        }
        if self.passkeys_type != 0 {
            len += 1;
        }
        if self.hide_password_reset {
            len += 1;
        }
        if self.ignore_unknown_usernames {
            len += 1;
        }
        if !self.default_redirect_uri.is_empty() {
            len += 1;
        }
        if self.password_check_lifetime.is_some() {
            len += 1;
        }
        if self.external_login_check_lifetime.is_some() {
            len += 1;
        }
        if self.mfa_init_skip_lifetime.is_some() {
            len += 1;
        }
        if self.second_factor_check_lifetime.is_some() {
            len += 1;
        }
        if self.multi_factor_check_lifetime.is_some() {
            len += 1;
        }
        if !self.second_factors.is_empty() {
            len += 1;
        }
        if !self.multi_factors.is_empty() {
            len += 1;
        }
        if self.allow_domain_discovery {
            len += 1;
        }
        if self.disable_login_with_email {
            len += 1;
        }
        if self.disable_login_with_phone {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        if self.force_mfa_local_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.LoginSettings", len)?;
        if self.allow_username_password {
            struct_ser.serialize_field("allowUsernamePassword", &self.allow_username_password)?;
        }
        if self.allow_register {
            struct_ser.serialize_field("allowRegister", &self.allow_register)?;
        }
        if self.allow_external_idp {
            struct_ser.serialize_field("allowExternalIdp", &self.allow_external_idp)?;
        }
        if self.force_mfa {
            struct_ser.serialize_field("forceMfa", &self.force_mfa)?;
        }
        if self.passkeys_type != 0 {
            let v = PasskeysType::try_from(self.passkeys_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.passkeys_type)))?;
            struct_ser.serialize_field("passkeysType", &v)?;
        }
        if self.hide_password_reset {
            struct_ser.serialize_field("hidePasswordReset", &self.hide_password_reset)?;
        }
        if self.ignore_unknown_usernames {
            struct_ser.serialize_field("ignoreUnknownUsernames", &self.ignore_unknown_usernames)?;
        }
        if !self.default_redirect_uri.is_empty() {
            struct_ser.serialize_field("defaultRedirectUri", &self.default_redirect_uri)?;
        }
        if let Some(v) = self.password_check_lifetime.as_ref() {
            struct_ser.serialize_field("passwordCheckLifetime", v)?;
        }
        if let Some(v) = self.external_login_check_lifetime.as_ref() {
            struct_ser.serialize_field("externalLoginCheckLifetime", v)?;
        }
        if let Some(v) = self.mfa_init_skip_lifetime.as_ref() {
            struct_ser.serialize_field("mfaInitSkipLifetime", v)?;
        }
        if let Some(v) = self.second_factor_check_lifetime.as_ref() {
            struct_ser.serialize_field("secondFactorCheckLifetime", v)?;
        }
        if let Some(v) = self.multi_factor_check_lifetime.as_ref() {
            struct_ser.serialize_field("multiFactorCheckLifetime", v)?;
        }
        if !self.second_factors.is_empty() {
            let v = self.second_factors.iter().cloned().map(|v| {
                SecondFactorType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("secondFactors", &v)?;
        }
        if !self.multi_factors.is_empty() {
            let v = self.multi_factors.iter().cloned().map(|v| {
                MultiFactorType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("multiFactors", &v)?;
        }
        if self.allow_domain_discovery {
            struct_ser.serialize_field("allowDomainDiscovery", &self.allow_domain_discovery)?;
        }
        if self.disable_login_with_email {
            struct_ser.serialize_field("disableLoginWithEmail", &self.disable_login_with_email)?;
        }
        if self.disable_login_with_phone {
            struct_ser.serialize_field("disableLoginWithPhone", &self.disable_login_with_phone)?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        if self.force_mfa_local_only {
            struct_ser.serialize_field("forceMfaLocalOnly", &self.force_mfa_local_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_username_password",
            "allowUsernamePassword",
            "allow_register",
            "allowRegister",
            "allow_external_idp",
            "allowExternalIdp",
            "force_mfa",
            "forceMfa",
            "passkeys_type",
            "passkeysType",
            "hide_password_reset",
            "hidePasswordReset",
            "ignore_unknown_usernames",
            "ignoreUnknownUsernames",
            "default_redirect_uri",
            "defaultRedirectUri",
            "password_check_lifetime",
            "passwordCheckLifetime",
            "external_login_check_lifetime",
            "externalLoginCheckLifetime",
            "mfa_init_skip_lifetime",
            "mfaInitSkipLifetime",
            "second_factor_check_lifetime",
            "secondFactorCheckLifetime",
            "multi_factor_check_lifetime",
            "multiFactorCheckLifetime",
            "second_factors",
            "secondFactors",
            "multi_factors",
            "multiFactors",
            "allow_domain_discovery",
            "allowDomainDiscovery",
            "disable_login_with_email",
            "disableLoginWithEmail",
            "disable_login_with_phone",
            "disableLoginWithPhone",
            "resource_owner_type",
            "resourceOwnerType",
            "force_mfa_local_only",
            "forceMfaLocalOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowUsernamePassword,
            AllowRegister,
            AllowExternalIdp,
            ForceMfa,
            PasskeysType,
            HidePasswordReset,
            IgnoreUnknownUsernames,
            DefaultRedirectUri,
            PasswordCheckLifetime,
            ExternalLoginCheckLifetime,
            MfaInitSkipLifetime,
            SecondFactorCheckLifetime,
            MultiFactorCheckLifetime,
            SecondFactors,
            MultiFactors,
            AllowDomainDiscovery,
            DisableLoginWithEmail,
            DisableLoginWithPhone,
            ResourceOwnerType,
            ForceMfaLocalOnly,
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
                            "allowUsernamePassword" | "allow_username_password" => Ok(GeneratedField::AllowUsernamePassword),
                            "allowRegister" | "allow_register" => Ok(GeneratedField::AllowRegister),
                            "allowExternalIdp" | "allow_external_idp" => Ok(GeneratedField::AllowExternalIdp),
                            "forceMfa" | "force_mfa" => Ok(GeneratedField::ForceMfa),
                            "passkeysType" | "passkeys_type" => Ok(GeneratedField::PasskeysType),
                            "hidePasswordReset" | "hide_password_reset" => Ok(GeneratedField::HidePasswordReset),
                            "ignoreUnknownUsernames" | "ignore_unknown_usernames" => Ok(GeneratedField::IgnoreUnknownUsernames),
                            "defaultRedirectUri" | "default_redirect_uri" => Ok(GeneratedField::DefaultRedirectUri),
                            "passwordCheckLifetime" | "password_check_lifetime" => Ok(GeneratedField::PasswordCheckLifetime),
                            "externalLoginCheckLifetime" | "external_login_check_lifetime" => Ok(GeneratedField::ExternalLoginCheckLifetime),
                            "mfaInitSkipLifetime" | "mfa_init_skip_lifetime" => Ok(GeneratedField::MfaInitSkipLifetime),
                            "secondFactorCheckLifetime" | "second_factor_check_lifetime" => Ok(GeneratedField::SecondFactorCheckLifetime),
                            "multiFactorCheckLifetime" | "multi_factor_check_lifetime" => Ok(GeneratedField::MultiFactorCheckLifetime),
                            "secondFactors" | "second_factors" => Ok(GeneratedField::SecondFactors),
                            "multiFactors" | "multi_factors" => Ok(GeneratedField::MultiFactors),
                            "allowDomainDiscovery" | "allow_domain_discovery" => Ok(GeneratedField::AllowDomainDiscovery),
                            "disableLoginWithEmail" | "disable_login_with_email" => Ok(GeneratedField::DisableLoginWithEmail),
                            "disableLoginWithPhone" | "disable_login_with_phone" => Ok(GeneratedField::DisableLoginWithPhone),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            "forceMfaLocalOnly" | "force_mfa_local_only" => Ok(GeneratedField::ForceMfaLocalOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.LoginSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_username_password__ = None;
                let mut allow_register__ = None;
                let mut allow_external_idp__ = None;
                let mut force_mfa__ = None;
                let mut passkeys_type__ = None;
                let mut hide_password_reset__ = None;
                let mut ignore_unknown_usernames__ = None;
                let mut default_redirect_uri__ = None;
                let mut password_check_lifetime__ = None;
                let mut external_login_check_lifetime__ = None;
                let mut mfa_init_skip_lifetime__ = None;
                let mut second_factor_check_lifetime__ = None;
                let mut multi_factor_check_lifetime__ = None;
                let mut second_factors__ = None;
                let mut multi_factors__ = None;
                let mut allow_domain_discovery__ = None;
                let mut disable_login_with_email__ = None;
                let mut disable_login_with_phone__ = None;
                let mut resource_owner_type__ = None;
                let mut force_mfa_local_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowUsernamePassword => {
                            if allow_username_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUsernamePassword"));
                            }
                            allow_username_password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowRegister => {
                            if allow_register__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowRegister"));
                            }
                            allow_register__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowExternalIdp => {
                            if allow_external_idp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowExternalIdp"));
                            }
                            allow_external_idp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ForceMfa => {
                            if force_mfa__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceMfa"));
                            }
                            force_mfa__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasskeysType => {
                            if passkeys_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passkeysType"));
                            }
                            passkeys_type__ = Some(map_.next_value::<PasskeysType>()? as i32);
                        }
                        GeneratedField::HidePasswordReset => {
                            if hide_password_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hidePasswordReset"));
                            }
                            hide_password_reset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IgnoreUnknownUsernames => {
                            if ignore_unknown_usernames__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreUnknownUsernames"));
                            }
                            ignore_unknown_usernames__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultRedirectUri => {
                            if default_redirect_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultRedirectUri"));
                            }
                            default_redirect_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordCheckLifetime => {
                            if password_check_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordCheckLifetime"));
                            }
                            password_check_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::ExternalLoginCheckLifetime => {
                            if external_login_check_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalLoginCheckLifetime"));
                            }
                            external_login_check_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::MfaInitSkipLifetime => {
                            if mfa_init_skip_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mfaInitSkipLifetime"));
                            }
                            mfa_init_skip_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::SecondFactorCheckLifetime => {
                            if second_factor_check_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondFactorCheckLifetime"));
                            }
                            second_factor_check_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::MultiFactorCheckLifetime => {
                            if multi_factor_check_lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiFactorCheckLifetime"));
                            }
                            multi_factor_check_lifetime__ = map_.next_value()?;
                        }
                        GeneratedField::SecondFactors => {
                            if second_factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondFactors"));
                            }
                            second_factors__ = Some(map_.next_value::<Vec<SecondFactorType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::MultiFactors => {
                            if multi_factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiFactors"));
                            }
                            multi_factors__ = Some(map_.next_value::<Vec<MultiFactorType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::AllowDomainDiscovery => {
                            if allow_domain_discovery__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowDomainDiscovery"));
                            }
                            allow_domain_discovery__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableLoginWithEmail => {
                            if disable_login_with_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableLoginWithEmail"));
                            }
                            disable_login_with_email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableLoginWithPhone => {
                            if disable_login_with_phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableLoginWithPhone"));
                            }
                            disable_login_with_phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                        GeneratedField::ForceMfaLocalOnly => {
                            if force_mfa_local_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceMfaLocalOnly"));
                            }
                            force_mfa_local_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginSettings {
                    allow_username_password: allow_username_password__.unwrap_or_default(),
                    allow_register: allow_register__.unwrap_or_default(),
                    allow_external_idp: allow_external_idp__.unwrap_or_default(),
                    force_mfa: force_mfa__.unwrap_or_default(),
                    passkeys_type: passkeys_type__.unwrap_or_default(),
                    hide_password_reset: hide_password_reset__.unwrap_or_default(),
                    ignore_unknown_usernames: ignore_unknown_usernames__.unwrap_or_default(),
                    default_redirect_uri: default_redirect_uri__.unwrap_or_default(),
                    password_check_lifetime: password_check_lifetime__,
                    external_login_check_lifetime: external_login_check_lifetime__,
                    mfa_init_skip_lifetime: mfa_init_skip_lifetime__,
                    second_factor_check_lifetime: second_factor_check_lifetime__,
                    multi_factor_check_lifetime: multi_factor_check_lifetime__,
                    second_factors: second_factors__.unwrap_or_default(),
                    multi_factors: multi_factors__.unwrap_or_default(),
                    allow_domain_discovery: allow_domain_discovery__.unwrap_or_default(),
                    disable_login_with_email: disable_login_with_email__.unwrap_or_default(),
                    disable_login_with_phone: disable_login_with_phone__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                    force_mfa_local_only: force_mfa_local_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.LoginSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MultiFactorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MULTI_FACTOR_TYPE_UNSPECIFIED",
            Self::U2fWithVerification => "MULTI_FACTOR_TYPE_U2F_WITH_VERIFICATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MultiFactorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MULTI_FACTOR_TYPE_UNSPECIFIED",
            "MULTI_FACTOR_TYPE_U2F_WITH_VERIFICATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MultiFactorType;

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
                    "MULTI_FACTOR_TYPE_UNSPECIFIED" => Ok(MultiFactorType::Unspecified),
                    "MULTI_FACTOR_TYPE_U2F_WITH_VERIFICATION" => Ok(MultiFactorType::U2fWithVerification),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PasskeysType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NotAllowed => "PASSKEYS_TYPE_NOT_ALLOWED",
            Self::Allowed => "PASSKEYS_TYPE_ALLOWED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PasskeysType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PASSKEYS_TYPE_NOT_ALLOWED",
            "PASSKEYS_TYPE_ALLOWED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasskeysType;

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
                    "PASSKEYS_TYPE_NOT_ALLOWED" => Ok(PasskeysType::NotAllowed),
                    "PASSKEYS_TYPE_ALLOWED" => Ok(PasskeysType::Allowed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordComplexitySettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_length != 0 {
            len += 1;
        }
        if self.requires_uppercase {
            len += 1;
        }
        if self.requires_lowercase {
            len += 1;
        }
        if self.requires_number {
            len += 1;
        }
        if self.requires_symbol {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.PasswordComplexitySettings", len)?;
        if self.min_length != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minLength", ToString::to_string(&self.min_length).as_str())?;
        }
        if self.requires_uppercase {
            struct_ser.serialize_field("requiresUppercase", &self.requires_uppercase)?;
        }
        if self.requires_lowercase {
            struct_ser.serialize_field("requiresLowercase", &self.requires_lowercase)?;
        }
        if self.requires_number {
            struct_ser.serialize_field("requiresNumber", &self.requires_number)?;
        }
        if self.requires_symbol {
            struct_ser.serialize_field("requiresSymbol", &self.requires_symbol)?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordComplexitySettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_length",
            "minLength",
            "requires_uppercase",
            "requiresUppercase",
            "requires_lowercase",
            "requiresLowercase",
            "requires_number",
            "requiresNumber",
            "requires_symbol",
            "requiresSymbol",
            "resource_owner_type",
            "resourceOwnerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinLength,
            RequiresUppercase,
            RequiresLowercase,
            RequiresNumber,
            RequiresSymbol,
            ResourceOwnerType,
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
                            "minLength" | "min_length" => Ok(GeneratedField::MinLength),
                            "requiresUppercase" | "requires_uppercase" => Ok(GeneratedField::RequiresUppercase),
                            "requiresLowercase" | "requires_lowercase" => Ok(GeneratedField::RequiresLowercase),
                            "requiresNumber" | "requires_number" => Ok(GeneratedField::RequiresNumber),
                            "requiresSymbol" | "requires_symbol" => Ok(GeneratedField::RequiresSymbol),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordComplexitySettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.PasswordComplexitySettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordComplexitySettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_length__ = None;
                let mut requires_uppercase__ = None;
                let mut requires_lowercase__ = None;
                let mut requires_number__ = None;
                let mut requires_symbol__ = None;
                let mut resource_owner_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinLength => {
                            if min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLength"));
                            }
                            min_length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequiresUppercase => {
                            if requires_uppercase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiresUppercase"));
                            }
                            requires_uppercase__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequiresLowercase => {
                            if requires_lowercase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiresLowercase"));
                            }
                            requires_lowercase__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequiresNumber => {
                            if requires_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiresNumber"));
                            }
                            requires_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequiresSymbol => {
                            if requires_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiresSymbol"));
                            }
                            requires_symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                    }
                }
                Ok(PasswordComplexitySettings {
                    min_length: min_length__.unwrap_or_default(),
                    requires_uppercase: requires_uppercase__.unwrap_or_default(),
                    requires_lowercase: requires_lowercase__.unwrap_or_default(),
                    requires_number: requires_number__.unwrap_or_default(),
                    requires_symbol: requires_symbol__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.PasswordComplexitySettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordExpirySettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_age_days != 0 {
            len += 1;
        }
        if self.expire_warn_days != 0 {
            len += 1;
        }
        if self.resource_owner_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.PasswordExpirySettings", len)?;
        if self.max_age_days != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxAgeDays", ToString::to_string(&self.max_age_days).as_str())?;
        }
        if self.expire_warn_days != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("expireWarnDays", ToString::to_string(&self.expire_warn_days).as_str())?;
        }
        if self.resource_owner_type != 0 {
            let v = ResourceOwnerType::try_from(self.resource_owner_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_owner_type)))?;
            struct_ser.serialize_field("resourceOwnerType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordExpirySettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_age_days",
            "maxAgeDays",
            "expire_warn_days",
            "expireWarnDays",
            "resource_owner_type",
            "resourceOwnerType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxAgeDays,
            ExpireWarnDays,
            ResourceOwnerType,
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
                            "maxAgeDays" | "max_age_days" => Ok(GeneratedField::MaxAgeDays),
                            "expireWarnDays" | "expire_warn_days" => Ok(GeneratedField::ExpireWarnDays),
                            "resourceOwnerType" | "resource_owner_type" => Ok(GeneratedField::ResourceOwnerType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordExpirySettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.PasswordExpirySettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordExpirySettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_age_days__ = None;
                let mut expire_warn_days__ = None;
                let mut resource_owner_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxAgeDays => {
                            if max_age_days__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAgeDays"));
                            }
                            max_age_days__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExpireWarnDays => {
                            if expire_warn_days__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expireWarnDays"));
                            }
                            expire_warn_days__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceOwnerType => {
                            if resource_owner_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerType"));
                            }
                            resource_owner_type__ = Some(map_.next_value::<ResourceOwnerType>()? as i32);
                        }
                    }
                }
                Ok(PasswordExpirySettings {
                    max_age_days: max_age_days__.unwrap_or_default(),
                    expire_warn_days: expire_warn_days__.unwrap_or_default(),
                    resource_owner_type: resource_owner_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.PasswordExpirySettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceOwnerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RESOURCE_OWNER_TYPE_UNSPECIFIED",
            Self::Instance => "RESOURCE_OWNER_TYPE_INSTANCE",
            Self::Org => "RESOURCE_OWNER_TYPE_ORG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ResourceOwnerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RESOURCE_OWNER_TYPE_UNSPECIFIED",
            "RESOURCE_OWNER_TYPE_INSTANCE",
            "RESOURCE_OWNER_TYPE_ORG",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceOwnerType;

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
                    "RESOURCE_OWNER_TYPE_UNSPECIFIED" => Ok(ResourceOwnerType::Unspecified),
                    "RESOURCE_OWNER_TYPE_INSTANCE" => Ok(ResourceOwnerType::Instance),
                    "RESOURCE_OWNER_TYPE_ORG" => Ok(ResourceOwnerType::Org),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SecondFactorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SECOND_FACTOR_TYPE_UNSPECIFIED",
            Self::Otp => "SECOND_FACTOR_TYPE_OTP",
            Self::U2f => "SECOND_FACTOR_TYPE_U2F",
            Self::OtpEmail => "SECOND_FACTOR_TYPE_OTP_EMAIL",
            Self::OtpSms => "SECOND_FACTOR_TYPE_OTP_SMS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SecondFactorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SECOND_FACTOR_TYPE_UNSPECIFIED",
            "SECOND_FACTOR_TYPE_OTP",
            "SECOND_FACTOR_TYPE_U2F",
            "SECOND_FACTOR_TYPE_OTP_EMAIL",
            "SECOND_FACTOR_TYPE_OTP_SMS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecondFactorType;

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
                    "SECOND_FACTOR_TYPE_UNSPECIFIED" => Ok(SecondFactorType::Unspecified),
                    "SECOND_FACTOR_TYPE_OTP" => Ok(SecondFactorType::Otp),
                    "SECOND_FACTOR_TYPE_U2F" => Ok(SecondFactorType::U2f),
                    "SECOND_FACTOR_TYPE_OTP_EMAIL" => Ok(SecondFactorType::OtpEmail),
                    "SECOND_FACTOR_TYPE_OTP_SMS" => Ok(SecondFactorType::OtpSms),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SecuritySettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.embedded_iframe.is_some() {
            len += 1;
        }
        if self.enable_impersonation {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.SecuritySettings", len)?;
        if let Some(v) = self.embedded_iframe.as_ref() {
            struct_ser.serialize_field("embeddedIframe", v)?;
        }
        if self.enable_impersonation {
            struct_ser.serialize_field("enableImpersonation", &self.enable_impersonation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecuritySettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "embedded_iframe",
            "embeddedIframe",
            "enable_impersonation",
            "enableImpersonation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmbeddedIframe,
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
                            "embeddedIframe" | "embedded_iframe" => Ok(GeneratedField::EmbeddedIframe),
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
            type Value = SecuritySettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.SecuritySettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SecuritySettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut embedded_iframe__ = None;
                let mut enable_impersonation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmbeddedIframe => {
                            if embedded_iframe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("embeddedIframe"));
                            }
                            embedded_iframe__ = map_.next_value()?;
                        }
                        GeneratedField::EnableImpersonation => {
                            if enable_impersonation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableImpersonation"));
                            }
                            enable_impersonation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SecuritySettings {
                    embedded_iframe: embedded_iframe__,
                    enable_impersonation: enable_impersonation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.SecuritySettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSecuritySettingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.embedded_iframe.is_some() {
            len += 1;
        }
        if self.enable_impersonation {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.SetSecuritySettingsRequest", len)?;
        if let Some(v) = self.embedded_iframe.as_ref() {
            struct_ser.serialize_field("embeddedIframe", v)?;
        }
        if self.enable_impersonation {
            struct_ser.serialize_field("enableImpersonation", &self.enable_impersonation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSecuritySettingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "embedded_iframe",
            "embeddedIframe",
            "enable_impersonation",
            "enableImpersonation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmbeddedIframe,
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
                            "embeddedIframe" | "embedded_iframe" => Ok(GeneratedField::EmbeddedIframe),
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
            type Value = SetSecuritySettingsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.SetSecuritySettingsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSecuritySettingsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut embedded_iframe__ = None;
                let mut enable_impersonation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EmbeddedIframe => {
                            if embedded_iframe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("embeddedIframe"));
                            }
                            embedded_iframe__ = map_.next_value()?;
                        }
                        GeneratedField::EnableImpersonation => {
                            if enable_impersonation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableImpersonation"));
                            }
                            enable_impersonation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetSecuritySettingsRequest {
                    embedded_iframe: embedded_iframe__,
                    enable_impersonation: enable_impersonation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.SetSecuritySettingsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSecuritySettingsResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.SetSecuritySettingsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSecuritySettingsResponse {
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
            type Value = SetSecuritySettingsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.SetSecuritySettingsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSecuritySettingsResponse, V::Error>
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
                Ok(SetSecuritySettingsResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.SetSecuritySettingsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Theme {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.primary_color.is_empty() {
            len += 1;
        }
        if !self.background_color.is_empty() {
            len += 1;
        }
        if !self.warn_color.is_empty() {
            len += 1;
        }
        if !self.font_color.is_empty() {
            len += 1;
        }
        if !self.logo_url.is_empty() {
            len += 1;
        }
        if !self.icon_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.settings.v2beta.Theme", len)?;
        if !self.primary_color.is_empty() {
            struct_ser.serialize_field("primaryColor", &self.primary_color)?;
        }
        if !self.background_color.is_empty() {
            struct_ser.serialize_field("backgroundColor", &self.background_color)?;
        }
        if !self.warn_color.is_empty() {
            struct_ser.serialize_field("warnColor", &self.warn_color)?;
        }
        if !self.font_color.is_empty() {
            struct_ser.serialize_field("fontColor", &self.font_color)?;
        }
        if !self.logo_url.is_empty() {
            struct_ser.serialize_field("logoUrl", &self.logo_url)?;
        }
        if !self.icon_url.is_empty() {
            struct_ser.serialize_field("iconUrl", &self.icon_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Theme {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "primary_color",
            "primaryColor",
            "background_color",
            "backgroundColor",
            "warn_color",
            "warnColor",
            "font_color",
            "fontColor",
            "logo_url",
            "logoUrl",
            "icon_url",
            "iconUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrimaryColor,
            BackgroundColor,
            WarnColor,
            FontColor,
            LogoUrl,
            IconUrl,
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
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "backgroundColor" | "background_color" => Ok(GeneratedField::BackgroundColor),
                            "warnColor" | "warn_color" => Ok(GeneratedField::WarnColor),
                            "fontColor" | "font_color" => Ok(GeneratedField::FontColor),
                            "logoUrl" | "logo_url" => Ok(GeneratedField::LogoUrl),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Theme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.settings.v2beta.Theme")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Theme, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut primary_color__ = None;
                let mut background_color__ = None;
                let mut warn_color__ = None;
                let mut font_color__ = None;
                let mut logo_url__ = None;
                let mut icon_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackgroundColor => {
                            if background_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backgroundColor"));
                            }
                            background_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarnColor => {
                            if warn_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warnColor"));
                            }
                            warn_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FontColor => {
                            if font_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fontColor"));
                            }
                            font_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LogoUrl => {
                            if logo_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoUrl"));
                            }
                            logo_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconUrl => {
                            if icon_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrl"));
                            }
                            icon_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Theme {
                    primary_color: primary_color__.unwrap_or_default(),
                    background_color: background_color__.unwrap_or_default(),
                    warn_color: warn_color__.unwrap_or_default(),
                    font_color: font_color__.unwrap_or_default(),
                    logo_url: logo_url__.unwrap_or_default(),
                    icon_url: icon_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.settings.v2beta.Theme", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThemeMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "THEME_MODE_UNSPECIFIED",
            Self::Auto => "THEME_MODE_AUTO",
            Self::Light => "THEME_MODE_LIGHT",
            Self::Dark => "THEME_MODE_DARK",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ThemeMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "THEME_MODE_UNSPECIFIED",
            "THEME_MODE_AUTO",
            "THEME_MODE_LIGHT",
            "THEME_MODE_DARK",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ThemeMode;

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
                    "THEME_MODE_UNSPECIFIED" => Ok(ThemeMode::Unspecified),
                    "THEME_MODE_AUTO" => Ok(ThemeMode::Auto),
                    "THEME_MODE_LIGHT" => Ok(ThemeMode::Light),
                    "THEME_MODE_DARK" => Ok(ThemeMode::Dark),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
