// @generated
impl serde::Serialize for DomainPolicy {
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
        if self.user_login_must_be_domain {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        if self.validate_org_domains {
            len += 1;
        }
        if self.smtp_sender_address_matches_instance_domain {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.DomainPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.user_login_must_be_domain {
            struct_ser.serialize_field("userLoginMustBeDomain", &self.user_login_must_be_domain)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if self.validate_org_domains {
            struct_ser.serialize_field("validateOrgDomains", &self.validate_org_domains)?;
        }
        if self.smtp_sender_address_matches_instance_domain {
            struct_ser.serialize_field("smtpSenderAddressMatchesInstanceDomain", &self.smtp_sender_address_matches_instance_domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DomainPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "user_login_must_be_domain",
            "userLoginMustBeDomain",
            "is_default",
            "isDefault",
            "validate_org_domains",
            "validateOrgDomains",
            "smtp_sender_address_matches_instance_domain",
            "smtpSenderAddressMatchesInstanceDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            UserLoginMustBeDomain,
            IsDefault,
            ValidateOrgDomains,
            SmtpSenderAddressMatchesInstanceDomain,
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
                            "userLoginMustBeDomain" | "user_login_must_be_domain" => Ok(GeneratedField::UserLoginMustBeDomain),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "validateOrgDomains" | "validate_org_domains" => Ok(GeneratedField::ValidateOrgDomains),
                            "smtpSenderAddressMatchesInstanceDomain" | "smtp_sender_address_matches_instance_domain" => Ok(GeneratedField::SmtpSenderAddressMatchesInstanceDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DomainPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.DomainPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DomainPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut user_login_must_be_domain__ = None;
                let mut is_default__ = None;
                let mut validate_org_domains__ = None;
                let mut smtp_sender_address_matches_instance_domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::UserLoginMustBeDomain => {
                            if user_login_must_be_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userLoginMustBeDomain"));
                            }
                            user_login_must_be_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidateOrgDomains => {
                            if validate_org_domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateOrgDomains"));
                            }
                            validate_org_domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SmtpSenderAddressMatchesInstanceDomain => {
                            if smtp_sender_address_matches_instance_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("smtpSenderAddressMatchesInstanceDomain"));
                            }
                            smtp_sender_address_matches_instance_domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DomainPolicy {
                    details: details__,
                    user_login_must_be_domain: user_login_must_be_domain__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                    validate_org_domains: validate_org_domains__.unwrap_or_default(),
                    smtp_sender_address_matches_instance_domain: smtp_sender_address_matches_instance_domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.DomainPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LabelPolicy {
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
        if !self.primary_color.is_empty() {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        if self.hide_login_name_suffix {
            len += 1;
        }
        if !self.warn_color.is_empty() {
            len += 1;
        }
        if !self.background_color.is_empty() {
            len += 1;
        }
        if !self.font_color.is_empty() {
            len += 1;
        }
        if !self.primary_color_dark.is_empty() {
            len += 1;
        }
        if !self.background_color_dark.is_empty() {
            len += 1;
        }
        if !self.warn_color_dark.is_empty() {
            len += 1;
        }
        if !self.font_color_dark.is_empty() {
            len += 1;
        }
        if self.disable_watermark {
            len += 1;
        }
        if !self.logo_url.is_empty() {
            len += 1;
        }
        if !self.icon_url.is_empty() {
            len += 1;
        }
        if !self.logo_url_dark.is_empty() {
            len += 1;
        }
        if !self.icon_url_dark.is_empty() {
            len += 1;
        }
        if !self.font_url.is_empty() {
            len += 1;
        }
        if self.theme_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.LabelPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.primary_color.is_empty() {
            struct_ser.serialize_field("primaryColor", &self.primary_color)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if self.hide_login_name_suffix {
            struct_ser.serialize_field("hideLoginNameSuffix", &self.hide_login_name_suffix)?;
        }
        if !self.warn_color.is_empty() {
            struct_ser.serialize_field("warnColor", &self.warn_color)?;
        }
        if !self.background_color.is_empty() {
            struct_ser.serialize_field("backgroundColor", &self.background_color)?;
        }
        if !self.font_color.is_empty() {
            struct_ser.serialize_field("fontColor", &self.font_color)?;
        }
        if !self.primary_color_dark.is_empty() {
            struct_ser.serialize_field("primaryColorDark", &self.primary_color_dark)?;
        }
        if !self.background_color_dark.is_empty() {
            struct_ser.serialize_field("backgroundColorDark", &self.background_color_dark)?;
        }
        if !self.warn_color_dark.is_empty() {
            struct_ser.serialize_field("warnColorDark", &self.warn_color_dark)?;
        }
        if !self.font_color_dark.is_empty() {
            struct_ser.serialize_field("fontColorDark", &self.font_color_dark)?;
        }
        if self.disable_watermark {
            struct_ser.serialize_field("disableWatermark", &self.disable_watermark)?;
        }
        if !self.logo_url.is_empty() {
            struct_ser.serialize_field("logoUrl", &self.logo_url)?;
        }
        if !self.icon_url.is_empty() {
            struct_ser.serialize_field("iconUrl", &self.icon_url)?;
        }
        if !self.logo_url_dark.is_empty() {
            struct_ser.serialize_field("logoUrlDark", &self.logo_url_dark)?;
        }
        if !self.icon_url_dark.is_empty() {
            struct_ser.serialize_field("iconUrlDark", &self.icon_url_dark)?;
        }
        if !self.font_url.is_empty() {
            struct_ser.serialize_field("fontUrl", &self.font_url)?;
        }
        if self.theme_mode != 0 {
            let v = ThemeMode::try_from(self.theme_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.theme_mode)))?;
            struct_ser.serialize_field("themeMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LabelPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "primary_color",
            "primaryColor",
            "is_default",
            "isDefault",
            "hide_login_name_suffix",
            "hideLoginNameSuffix",
            "warn_color",
            "warnColor",
            "background_color",
            "backgroundColor",
            "font_color",
            "fontColor",
            "primary_color_dark",
            "primaryColorDark",
            "background_color_dark",
            "backgroundColorDark",
            "warn_color_dark",
            "warnColorDark",
            "font_color_dark",
            "fontColorDark",
            "disable_watermark",
            "disableWatermark",
            "logo_url",
            "logoUrl",
            "icon_url",
            "iconUrl",
            "logo_url_dark",
            "logoUrlDark",
            "icon_url_dark",
            "iconUrlDark",
            "font_url",
            "fontUrl",
            "theme_mode",
            "themeMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            PrimaryColor,
            IsDefault,
            HideLoginNameSuffix,
            WarnColor,
            BackgroundColor,
            FontColor,
            PrimaryColorDark,
            BackgroundColorDark,
            WarnColorDark,
            FontColorDark,
            DisableWatermark,
            LogoUrl,
            IconUrl,
            LogoUrlDark,
            IconUrlDark,
            FontUrl,
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
                            "details" => Ok(GeneratedField::Details),
                            "primaryColor" | "primary_color" => Ok(GeneratedField::PrimaryColor),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "hideLoginNameSuffix" | "hide_login_name_suffix" => Ok(GeneratedField::HideLoginNameSuffix),
                            "warnColor" | "warn_color" => Ok(GeneratedField::WarnColor),
                            "backgroundColor" | "background_color" => Ok(GeneratedField::BackgroundColor),
                            "fontColor" | "font_color" => Ok(GeneratedField::FontColor),
                            "primaryColorDark" | "primary_color_dark" => Ok(GeneratedField::PrimaryColorDark),
                            "backgroundColorDark" | "background_color_dark" => Ok(GeneratedField::BackgroundColorDark),
                            "warnColorDark" | "warn_color_dark" => Ok(GeneratedField::WarnColorDark),
                            "fontColorDark" | "font_color_dark" => Ok(GeneratedField::FontColorDark),
                            "disableWatermark" | "disable_watermark" => Ok(GeneratedField::DisableWatermark),
                            "logoUrl" | "logo_url" => Ok(GeneratedField::LogoUrl),
                            "iconUrl" | "icon_url" => Ok(GeneratedField::IconUrl),
                            "logoUrlDark" | "logo_url_dark" => Ok(GeneratedField::LogoUrlDark),
                            "iconUrlDark" | "icon_url_dark" => Ok(GeneratedField::IconUrlDark),
                            "fontUrl" | "font_url" => Ok(GeneratedField::FontUrl),
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
            type Value = LabelPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.LabelPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LabelPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut primary_color__ = None;
                let mut is_default__ = None;
                let mut hide_login_name_suffix__ = None;
                let mut warn_color__ = None;
                let mut background_color__ = None;
                let mut font_color__ = None;
                let mut primary_color_dark__ = None;
                let mut background_color_dark__ = None;
                let mut warn_color_dark__ = None;
                let mut font_color_dark__ = None;
                let mut disable_watermark__ = None;
                let mut logo_url__ = None;
                let mut icon_url__ = None;
                let mut logo_url_dark__ = None;
                let mut icon_url_dark__ = None;
                let mut font_url__ = None;
                let mut theme_mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::PrimaryColor => {
                            if primary_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColor"));
                            }
                            primary_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HideLoginNameSuffix => {
                            if hide_login_name_suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hideLoginNameSuffix"));
                            }
                            hide_login_name_suffix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarnColor => {
                            if warn_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warnColor"));
                            }
                            warn_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackgroundColor => {
                            if background_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backgroundColor"));
                            }
                            background_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FontColor => {
                            if font_color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fontColor"));
                            }
                            font_color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrimaryColorDark => {
                            if primary_color_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("primaryColorDark"));
                            }
                            primary_color_dark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackgroundColorDark => {
                            if background_color_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backgroundColorDark"));
                            }
                            background_color_dark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WarnColorDark => {
                            if warn_color_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warnColorDark"));
                            }
                            warn_color_dark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FontColorDark => {
                            if font_color_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fontColorDark"));
                            }
                            font_color_dark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableWatermark => {
                            if disable_watermark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableWatermark"));
                            }
                            disable_watermark__ = Some(map_.next_value()?);
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
                        GeneratedField::LogoUrlDark => {
                            if logo_url_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoUrlDark"));
                            }
                            logo_url_dark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IconUrlDark => {
                            if icon_url_dark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iconUrlDark"));
                            }
                            icon_url_dark__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FontUrl => {
                            if font_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fontUrl"));
                            }
                            font_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ThemeMode => {
                            if theme_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("themeMode"));
                            }
                            theme_mode__ = Some(map_.next_value::<ThemeMode>()? as i32);
                        }
                    }
                }
                Ok(LabelPolicy {
                    details: details__,
                    primary_color: primary_color__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                    hide_login_name_suffix: hide_login_name_suffix__.unwrap_or_default(),
                    warn_color: warn_color__.unwrap_or_default(),
                    background_color: background_color__.unwrap_or_default(),
                    font_color: font_color__.unwrap_or_default(),
                    primary_color_dark: primary_color_dark__.unwrap_or_default(),
                    background_color_dark: background_color_dark__.unwrap_or_default(),
                    warn_color_dark: warn_color_dark__.unwrap_or_default(),
                    font_color_dark: font_color_dark__.unwrap_or_default(),
                    disable_watermark: disable_watermark__.unwrap_or_default(),
                    logo_url: logo_url__.unwrap_or_default(),
                    icon_url: icon_url__.unwrap_or_default(),
                    logo_url_dark: logo_url_dark__.unwrap_or_default(),
                    icon_url_dark: icon_url_dark__.unwrap_or_default(),
                    font_url: font_url__.unwrap_or_default(),
                    theme_mode: theme_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.LabelPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LockoutPolicy {
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
        if self.max_password_attempts != 0 {
            len += 1;
        }
        if self.max_otp_attempts != 0 {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.LockoutPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.max_password_attempts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxPasswordAttempts", ToString::to_string(&self.max_password_attempts).as_str())?;
        }
        if self.max_otp_attempts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxOtpAttempts", ToString::to_string(&self.max_otp_attempts).as_str())?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LockoutPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "max_password_attempts",
            "maxPasswordAttempts",
            "max_otp_attempts",
            "maxOtpAttempts",
            "is_default",
            "isDefault",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            MaxPasswordAttempts,
            MaxOtpAttempts,
            IsDefault,
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
                            "maxPasswordAttempts" | "max_password_attempts" => Ok(GeneratedField::MaxPasswordAttempts),
                            "maxOtpAttempts" | "max_otp_attempts" => Ok(GeneratedField::MaxOtpAttempts),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LockoutPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.LockoutPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LockoutPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut max_password_attempts__ = None;
                let mut max_otp_attempts__ = None;
                let mut is_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::MaxPasswordAttempts => {
                            if max_password_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPasswordAttempts"));
                            }
                            max_password_attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxOtpAttempts => {
                            if max_otp_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxOtpAttempts"));
                            }
                            max_otp_attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LockoutPolicy {
                    details: details__,
                    max_password_attempts: max_password_attempts__.unwrap_or_default(),
                    max_otp_attempts: max_otp_attempts__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.LockoutPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginPolicy {
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
        if self.passwordless_type != 0 {
            len += 1;
        }
        if self.is_default {
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
        if !self.idps.is_empty() {
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
        if self.force_mfa_local_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.LoginPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
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
        if self.passwordless_type != 0 {
            let v = PasswordlessType::try_from(self.passwordless_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.passwordless_type)))?;
            struct_ser.serialize_field("passwordlessType", &v)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
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
        if !self.idps.is_empty() {
            struct_ser.serialize_field("idps", &self.idps)?;
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
        if self.force_mfa_local_only {
            struct_ser.serialize_field("forceMfaLocalOnly", &self.force_mfa_local_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "allow_username_password",
            "allowUsernamePassword",
            "allow_register",
            "allowRegister",
            "allow_external_idp",
            "allowExternalIdp",
            "force_mfa",
            "forceMfa",
            "passwordless_type",
            "passwordlessType",
            "is_default",
            "isDefault",
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
            "idps",
            "allow_domain_discovery",
            "allowDomainDiscovery",
            "disable_login_with_email",
            "disableLoginWithEmail",
            "disable_login_with_phone",
            "disableLoginWithPhone",
            "force_mfa_local_only",
            "forceMfaLocalOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            AllowUsernamePassword,
            AllowRegister,
            AllowExternalIdp,
            ForceMfa,
            PasswordlessType,
            IsDefault,
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
            Idps,
            AllowDomainDiscovery,
            DisableLoginWithEmail,
            DisableLoginWithPhone,
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
                            "details" => Ok(GeneratedField::Details),
                            "allowUsernamePassword" | "allow_username_password" => Ok(GeneratedField::AllowUsernamePassword),
                            "allowRegister" | "allow_register" => Ok(GeneratedField::AllowRegister),
                            "allowExternalIdp" | "allow_external_idp" => Ok(GeneratedField::AllowExternalIdp),
                            "forceMfa" | "force_mfa" => Ok(GeneratedField::ForceMfa),
                            "passwordlessType" | "passwordless_type" => Ok(GeneratedField::PasswordlessType),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
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
                            "idps" => Ok(GeneratedField::Idps),
                            "allowDomainDiscovery" | "allow_domain_discovery" => Ok(GeneratedField::AllowDomainDiscovery),
                            "disableLoginWithEmail" | "disable_login_with_email" => Ok(GeneratedField::DisableLoginWithEmail),
                            "disableLoginWithPhone" | "disable_login_with_phone" => Ok(GeneratedField::DisableLoginWithPhone),
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
            type Value = LoginPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.LoginPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut allow_username_password__ = None;
                let mut allow_register__ = None;
                let mut allow_external_idp__ = None;
                let mut force_mfa__ = None;
                let mut passwordless_type__ = None;
                let mut is_default__ = None;
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
                let mut idps__ = None;
                let mut allow_domain_discovery__ = None;
                let mut disable_login_with_email__ = None;
                let mut disable_login_with_phone__ = None;
                let mut force_mfa_local_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
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
                        GeneratedField::PasswordlessType => {
                            if passwordless_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessType"));
                            }
                            passwordless_type__ = Some(map_.next_value::<PasswordlessType>()? as i32);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
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
                        GeneratedField::Idps => {
                            if idps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idps"));
                            }
                            idps__ = Some(map_.next_value()?);
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
                        GeneratedField::ForceMfaLocalOnly => {
                            if force_mfa_local_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceMfaLocalOnly"));
                            }
                            force_mfa_local_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginPolicy {
                    details: details__,
                    allow_username_password: allow_username_password__.unwrap_or_default(),
                    allow_register: allow_register__.unwrap_or_default(),
                    allow_external_idp: allow_external_idp__.unwrap_or_default(),
                    force_mfa: force_mfa__.unwrap_or_default(),
                    passwordless_type: passwordless_type__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
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
                    idps: idps__.unwrap_or_default(),
                    allow_domain_discovery: allow_domain_discovery__.unwrap_or_default(),
                    disable_login_with_email: disable_login_with_email__.unwrap_or_default(),
                    disable_login_with_phone: disable_login_with_phone__.unwrap_or_default(),
                    force_mfa_local_only: force_mfa_local_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.LoginPolicy", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for NotificationPolicy {
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
        if self.is_default {
            len += 1;
        }
        if self.password_change {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.NotificationPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if self.password_change {
            struct_ser.serialize_field("passwordChange", &self.password_change)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "is_default",
            "isDefault",
            "password_change",
            "passwordChange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            IsDefault,
            PasswordChange,
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
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "passwordChange" | "password_change" => Ok(GeneratedField::PasswordChange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.NotificationPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut is_default__ = None;
                let mut password_change__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordChange => {
                            if password_change__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChange"));
                            }
                            password_change__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NotificationPolicy {
                    details: details__,
                    is_default: is_default__.unwrap_or_default(),
                    password_change: password_change__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.NotificationPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrgIamPolicy {
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
        if self.user_login_must_be_domain {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.OrgIAMPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.user_login_must_be_domain {
            struct_ser.serialize_field("userLoginMustBeDomain", &self.user_login_must_be_domain)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrgIamPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "user_login_must_be_domain",
            "userLoginMustBeDomain",
            "is_default",
            "isDefault",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            UserLoginMustBeDomain,
            IsDefault,
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
                            "userLoginMustBeDomain" | "user_login_must_be_domain" => Ok(GeneratedField::UserLoginMustBeDomain),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrgIamPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.OrgIAMPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrgIamPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut user_login_must_be_domain__ = None;
                let mut is_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::UserLoginMustBeDomain => {
                            if user_login_must_be_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userLoginMustBeDomain"));
                            }
                            user_login_must_be_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OrgIamPolicy {
                    details: details__,
                    user_login_must_be_domain: user_login_must_be_domain__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.OrgIAMPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordAgePolicy {
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
        if self.max_age_days != 0 {
            len += 1;
        }
        if self.expire_warn_days != 0 {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.PasswordAgePolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
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
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordAgePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "max_age_days",
            "maxAgeDays",
            "expire_warn_days",
            "expireWarnDays",
            "is_default",
            "isDefault",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            MaxAgeDays,
            ExpireWarnDays,
            IsDefault,
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
                            "maxAgeDays" | "max_age_days" => Ok(GeneratedField::MaxAgeDays),
                            "expireWarnDays" | "expire_warn_days" => Ok(GeneratedField::ExpireWarnDays),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordAgePolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.PasswordAgePolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordAgePolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut max_age_days__ = None;
                let mut expire_warn_days__ = None;
                let mut is_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
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
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordAgePolicy {
                    details: details__,
                    max_age_days: max_age_days__.unwrap_or_default(),
                    expire_warn_days: expire_warn_days__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.PasswordAgePolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordComplexityPolicy {
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
        if self.min_length != 0 {
            len += 1;
        }
        if self.has_uppercase {
            len += 1;
        }
        if self.has_lowercase {
            len += 1;
        }
        if self.has_number {
            len += 1;
        }
        if self.has_symbol {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.PasswordComplexityPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.min_length != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minLength", ToString::to_string(&self.min_length).as_str())?;
        }
        if self.has_uppercase {
            struct_ser.serialize_field("hasUppercase", &self.has_uppercase)?;
        }
        if self.has_lowercase {
            struct_ser.serialize_field("hasLowercase", &self.has_lowercase)?;
        }
        if self.has_number {
            struct_ser.serialize_field("hasNumber", &self.has_number)?;
        }
        if self.has_symbol {
            struct_ser.serialize_field("hasSymbol", &self.has_symbol)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordComplexityPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "min_length",
            "minLength",
            "has_uppercase",
            "hasUppercase",
            "has_lowercase",
            "hasLowercase",
            "has_number",
            "hasNumber",
            "has_symbol",
            "hasSymbol",
            "is_default",
            "isDefault",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            MinLength,
            HasUppercase,
            HasLowercase,
            HasNumber,
            HasSymbol,
            IsDefault,
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
                            "minLength" | "min_length" => Ok(GeneratedField::MinLength),
                            "hasUppercase" | "has_uppercase" => Ok(GeneratedField::HasUppercase),
                            "hasLowercase" | "has_lowercase" => Ok(GeneratedField::HasLowercase),
                            "hasNumber" | "has_number" => Ok(GeneratedField::HasNumber),
                            "hasSymbol" | "has_symbol" => Ok(GeneratedField::HasSymbol),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordComplexityPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.PasswordComplexityPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordComplexityPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut min_length__ = None;
                let mut has_uppercase__ = None;
                let mut has_lowercase__ = None;
                let mut has_number__ = None;
                let mut has_symbol__ = None;
                let mut is_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::MinLength => {
                            if min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLength"));
                            }
                            min_length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HasUppercase => {
                            if has_uppercase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasUppercase"));
                            }
                            has_uppercase__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasLowercase => {
                            if has_lowercase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasLowercase"));
                            }
                            has_lowercase__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasNumber => {
                            if has_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasNumber"));
                            }
                            has_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasSymbol => {
                            if has_symbol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasSymbol"));
                            }
                            has_symbol__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordComplexityPolicy {
                    details: details__,
                    min_length: min_length__.unwrap_or_default(),
                    has_uppercase: has_uppercase__.unwrap_or_default(),
                    has_lowercase: has_lowercase__.unwrap_or_default(),
                    has_number: has_number__.unwrap_or_default(),
                    has_symbol: has_symbol__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.PasswordComplexityPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordlessType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NotAllowed => "PASSWORDLESS_TYPE_NOT_ALLOWED",
            Self::Allowed => "PASSWORDLESS_TYPE_ALLOWED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PasswordlessType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PASSWORDLESS_TYPE_NOT_ALLOWED",
            "PASSWORDLESS_TYPE_ALLOWED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordlessType;

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
                    "PASSWORDLESS_TYPE_NOT_ALLOWED" => Ok(PasswordlessType::NotAllowed),
                    "PASSWORDLESS_TYPE_ALLOWED" => Ok(PasswordlessType::Allowed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PrivacyPolicy {
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
        if !self.tos_link.is_empty() {
            len += 1;
        }
        if !self.privacy_link.is_empty() {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        if !self.help_link.is_empty() {
            len += 1;
        }
        if !self.support_email.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.policy.v1.PrivacyPolicy", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.tos_link.is_empty() {
            struct_ser.serialize_field("tosLink", &self.tos_link)?;
        }
        if !self.privacy_link.is_empty() {
            struct_ser.serialize_field("privacyLink", &self.privacy_link)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if !self.help_link.is_empty() {
            struct_ser.serialize_field("helpLink", &self.help_link)?;
        }
        if !self.support_email.is_empty() {
            struct_ser.serialize_field("supportEmail", &self.support_email)?;
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
impl<'de> serde::Deserialize<'de> for PrivacyPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "tos_link",
            "tosLink",
            "privacy_link",
            "privacyLink",
            "is_default",
            "isDefault",
            "help_link",
            "helpLink",
            "support_email",
            "supportEmail",
            "docs_link",
            "docsLink",
            "custom_link",
            "customLink",
            "custom_link_text",
            "customLinkText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            TosLink,
            PrivacyLink,
            IsDefault,
            HelpLink,
            SupportEmail,
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
                            "details" => Ok(GeneratedField::Details),
                            "tosLink" | "tos_link" => Ok(GeneratedField::TosLink),
                            "privacyLink" | "privacy_link" => Ok(GeneratedField::PrivacyLink),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "helpLink" | "help_link" => Ok(GeneratedField::HelpLink),
                            "supportEmail" | "support_email" => Ok(GeneratedField::SupportEmail),
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
            type Value = PrivacyPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.policy.v1.PrivacyPolicy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrivacyPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut tos_link__ = None;
                let mut privacy_link__ = None;
                let mut is_default__ = None;
                let mut help_link__ = None;
                let mut support_email__ = None;
                let mut docs_link__ = None;
                let mut custom_link__ = None;
                let mut custom_link_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::TosLink => {
                            if tos_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosLink"));
                            }
                            tos_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyLink => {
                            if privacy_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyLink"));
                            }
                            privacy_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
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
                Ok(PrivacyPolicy {
                    details: details__,
                    tos_link: tos_link__.unwrap_or_default(),
                    privacy_link: privacy_link__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                    help_link: help_link__.unwrap_or_default(),
                    support_email: support_email__.unwrap_or_default(),
                    docs_link: docs_link__.unwrap_or_default(),
                    custom_link: custom_link__.unwrap_or_default(),
                    custom_link_text: custom_link_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.policy.v1.PrivacyPolicy", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ThemeMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "THEME_MODE_UNSPECIFIED",
            Self::Auto => "THEME_MODE_AUTO",
            Self::Dark => "THEME_MODE_DARK",
            Self::Light => "THEME_MODE_LIGHT",
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
            "THEME_MODE_DARK",
            "THEME_MODE_LIGHT",
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
                    "THEME_MODE_DARK" => Ok(ThemeMode::Dark),
                    "THEME_MODE_LIGHT" => Ok(ThemeMode::Light),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
