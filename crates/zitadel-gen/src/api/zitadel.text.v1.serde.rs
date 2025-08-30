// @generated
impl serde::Serialize for EmailVerificationDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.login_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.EmailVerificationDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.login_button_text.is_empty() {
            struct_ser.serialize_field("loginButtonText", &self.login_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmailVerificationDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "next_button_text",
            "nextButtonText",
            "cancel_button_text",
            "cancelButtonText",
            "login_button_text",
            "loginButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NextButtonText,
            CancelButtonText,
            LoginButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "loginButtonText" | "login_button_text" => Ok(GeneratedField::LoginButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailVerificationDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.EmailVerificationDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailVerificationDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut next_button_text__ = None;
                let mut cancel_button_text__ = None;
                let mut login_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginButtonText => {
                            if login_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginButtonText"));
                            }
                            login_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EmailVerificationDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    login_button_text: login_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.EmailVerificationDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EmailVerificationScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_label.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.resend_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.EmailVerificationScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_label.is_empty() {
            struct_ser.serialize_field("codeLabel", &self.code_label)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.resend_button_text.is_empty() {
            struct_ser.serialize_field("resendButtonText", &self.resend_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EmailVerificationScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "code_label",
            "codeLabel",
            "next_button_text",
            "nextButtonText",
            "resend_button_text",
            "resendButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeLabel,
            NextButtonText,
            ResendButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeLabel" | "code_label" => Ok(GeneratedField::CodeLabel),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "resendButtonText" | "resend_button_text" => Ok(GeneratedField::ResendButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EmailVerificationScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.EmailVerificationScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailVerificationScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_label__ = None;
                let mut next_button_text__ = None;
                let mut resend_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeLabel => {
                            if code_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeLabel"));
                            }
                            code_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResendButtonText => {
                            if resend_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resendButtonText"));
                            }
                            resend_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EmailVerificationScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_label: code_label__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    resend_button_text: resend_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.EmailVerificationScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExternalRegistrationUserOverviewScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.email_label.is_empty() {
            len += 1;
        }
        if !self.username_label.is_empty() {
            len += 1;
        }
        if !self.firstname_label.is_empty() {
            len += 1;
        }
        if !self.lastname_label.is_empty() {
            len += 1;
        }
        if !self.nickname_label.is_empty() {
            len += 1;
        }
        if !self.language_label.is_empty() {
            len += 1;
        }
        if !self.phone_label.is_empty() {
            len += 1;
        }
        if !self.tos_and_privacy_label.is_empty() {
            len += 1;
        }
        if !self.tos_confirm.is_empty() {
            len += 1;
        }
        if !self.tos_link_text.is_empty() {
            len += 1;
        }
        if !self.privacy_link_text.is_empty() {
            len += 1;
        }
        if !self.back_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.privacy_confirm.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.ExternalRegistrationUserOverviewScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.email_label.is_empty() {
            struct_ser.serialize_field("emailLabel", &self.email_label)?;
        }
        if !self.username_label.is_empty() {
            struct_ser.serialize_field("usernameLabel", &self.username_label)?;
        }
        if !self.firstname_label.is_empty() {
            struct_ser.serialize_field("firstnameLabel", &self.firstname_label)?;
        }
        if !self.lastname_label.is_empty() {
            struct_ser.serialize_field("lastnameLabel", &self.lastname_label)?;
        }
        if !self.nickname_label.is_empty() {
            struct_ser.serialize_field("nicknameLabel", &self.nickname_label)?;
        }
        if !self.language_label.is_empty() {
            struct_ser.serialize_field("languageLabel", &self.language_label)?;
        }
        if !self.phone_label.is_empty() {
            struct_ser.serialize_field("phoneLabel", &self.phone_label)?;
        }
        if !self.tos_and_privacy_label.is_empty() {
            struct_ser.serialize_field("tosAndPrivacyLabel", &self.tos_and_privacy_label)?;
        }
        if !self.tos_confirm.is_empty() {
            struct_ser.serialize_field("tosConfirm", &self.tos_confirm)?;
        }
        if !self.tos_link_text.is_empty() {
            struct_ser.serialize_field("tosLinkText", &self.tos_link_text)?;
        }
        if !self.privacy_link_text.is_empty() {
            struct_ser.serialize_field("privacyLinkText", &self.privacy_link_text)?;
        }
        if !self.back_button_text.is_empty() {
            struct_ser.serialize_field("backButtonText", &self.back_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.privacy_confirm.is_empty() {
            struct_ser.serialize_field("privacyConfirm", &self.privacy_confirm)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternalRegistrationUserOverviewScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "email_label",
            "emailLabel",
            "username_label",
            "usernameLabel",
            "firstname_label",
            "firstnameLabel",
            "lastname_label",
            "lastnameLabel",
            "nickname_label",
            "nicknameLabel",
            "language_label",
            "languageLabel",
            "phone_label",
            "phoneLabel",
            "tos_and_privacy_label",
            "tosAndPrivacyLabel",
            "tos_confirm",
            "tosConfirm",
            "tos_link_text",
            "tosLinkText",
            "privacy_link_text",
            "privacyLinkText",
            "back_button_text",
            "backButtonText",
            "next_button_text",
            "nextButtonText",
            "privacy_confirm",
            "privacyConfirm",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            EmailLabel,
            UsernameLabel,
            FirstnameLabel,
            LastnameLabel,
            NicknameLabel,
            LanguageLabel,
            PhoneLabel,
            TosAndPrivacyLabel,
            TosConfirm,
            TosLinkText,
            PrivacyLinkText,
            BackButtonText,
            NextButtonText,
            PrivacyConfirm,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "emailLabel" | "email_label" => Ok(GeneratedField::EmailLabel),
                            "usernameLabel" | "username_label" => Ok(GeneratedField::UsernameLabel),
                            "firstnameLabel" | "firstname_label" => Ok(GeneratedField::FirstnameLabel),
                            "lastnameLabel" | "lastname_label" => Ok(GeneratedField::LastnameLabel),
                            "nicknameLabel" | "nickname_label" => Ok(GeneratedField::NicknameLabel),
                            "languageLabel" | "language_label" => Ok(GeneratedField::LanguageLabel),
                            "phoneLabel" | "phone_label" => Ok(GeneratedField::PhoneLabel),
                            "tosAndPrivacyLabel" | "tos_and_privacy_label" => Ok(GeneratedField::TosAndPrivacyLabel),
                            "tosConfirm" | "tos_confirm" => Ok(GeneratedField::TosConfirm),
                            "tosLinkText" | "tos_link_text" => Ok(GeneratedField::TosLinkText),
                            "privacyLinkText" | "privacy_link_text" => Ok(GeneratedField::PrivacyLinkText),
                            "backButtonText" | "back_button_text" => Ok(GeneratedField::BackButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "privacyConfirm" | "privacy_confirm" => Ok(GeneratedField::PrivacyConfirm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternalRegistrationUserOverviewScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.ExternalRegistrationUserOverviewScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternalRegistrationUserOverviewScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut email_label__ = None;
                let mut username_label__ = None;
                let mut firstname_label__ = None;
                let mut lastname_label__ = None;
                let mut nickname_label__ = None;
                let mut language_label__ = None;
                let mut phone_label__ = None;
                let mut tos_and_privacy_label__ = None;
                let mut tos_confirm__ = None;
                let mut tos_link_text__ = None;
                let mut privacy_link_text__ = None;
                let mut back_button_text__ = None;
                let mut next_button_text__ = None;
                let mut privacy_confirm__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailLabel => {
                            if email_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailLabel"));
                            }
                            email_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsernameLabel => {
                            if username_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameLabel"));
                            }
                            username_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstnameLabel => {
                            if firstname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstnameLabel"));
                            }
                            firstname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastnameLabel => {
                            if lastname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastnameLabel"));
                            }
                            lastname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NicknameLabel => {
                            if nickname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nicknameLabel"));
                            }
                            nickname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LanguageLabel => {
                            if language_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("languageLabel"));
                            }
                            language_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PhoneLabel => {
                            if phone_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneLabel"));
                            }
                            phone_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosAndPrivacyLabel => {
                            if tos_and_privacy_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosAndPrivacyLabel"));
                            }
                            tos_and_privacy_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosConfirm => {
                            if tos_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosConfirm"));
                            }
                            tos_confirm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosLinkText => {
                            if tos_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosLinkText"));
                            }
                            tos_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyLinkText => {
                            if privacy_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyLinkText"));
                            }
                            privacy_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackButtonText => {
                            if back_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backButtonText"));
                            }
                            back_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyConfirm => {
                            if privacy_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyConfirm"));
                            }
                            privacy_confirm__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExternalRegistrationUserOverviewScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    email_label: email_label__.unwrap_or_default(),
                    username_label: username_label__.unwrap_or_default(),
                    firstname_label: firstname_label__.unwrap_or_default(),
                    lastname_label: lastname_label__.unwrap_or_default(),
                    nickname_label: nickname_label__.unwrap_or_default(),
                    language_label: language_label__.unwrap_or_default(),
                    phone_label: phone_label__.unwrap_or_default(),
                    tos_and_privacy_label: tos_and_privacy_label__.unwrap_or_default(),
                    tos_confirm: tos_confirm__.unwrap_or_default(),
                    tos_link_text: tos_link_text__.unwrap_or_default(),
                    privacy_link_text: privacy_link_text__.unwrap_or_default(),
                    back_button_text: back_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    privacy_confirm: privacy_confirm__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.ExternalRegistrationUserOverviewScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExternalUserNotFoundScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.link_button_text.is_empty() {
            len += 1;
        }
        if !self.auto_register_button_text.is_empty() {
            len += 1;
        }
        if !self.tos_and_privacy_label.is_empty() {
            len += 1;
        }
        if !self.tos_confirm.is_empty() {
            len += 1;
        }
        if !self.tos_link_text.is_empty() {
            len += 1;
        }
        if !self.privacy_link_text.is_empty() {
            len += 1;
        }
        if !self.privacy_confirm.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.ExternalUserNotFoundScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.link_button_text.is_empty() {
            struct_ser.serialize_field("linkButtonText", &self.link_button_text)?;
        }
        if !self.auto_register_button_text.is_empty() {
            struct_ser.serialize_field("autoRegisterButtonText", &self.auto_register_button_text)?;
        }
        if !self.tos_and_privacy_label.is_empty() {
            struct_ser.serialize_field("tosAndPrivacyLabel", &self.tos_and_privacy_label)?;
        }
        if !self.tos_confirm.is_empty() {
            struct_ser.serialize_field("tosConfirm", &self.tos_confirm)?;
        }
        if !self.tos_link_text.is_empty() {
            struct_ser.serialize_field("tosLinkText", &self.tos_link_text)?;
        }
        if !self.privacy_link_text.is_empty() {
            struct_ser.serialize_field("privacyLinkText", &self.privacy_link_text)?;
        }
        if !self.privacy_confirm.is_empty() {
            struct_ser.serialize_field("privacyConfirm", &self.privacy_confirm)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternalUserNotFoundScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "link_button_text",
            "linkButtonText",
            "auto_register_button_text",
            "autoRegisterButtonText",
            "tos_and_privacy_label",
            "tosAndPrivacyLabel",
            "tos_confirm",
            "tosConfirm",
            "tos_link_text",
            "tosLinkText",
            "privacy_link_text",
            "privacyLinkText",
            "privacy_confirm",
            "privacyConfirm",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            LinkButtonText,
            AutoRegisterButtonText,
            TosAndPrivacyLabel,
            TosConfirm,
            TosLinkText,
            PrivacyLinkText,
            PrivacyConfirm,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "linkButtonText" | "link_button_text" => Ok(GeneratedField::LinkButtonText),
                            "autoRegisterButtonText" | "auto_register_button_text" => Ok(GeneratedField::AutoRegisterButtonText),
                            "tosAndPrivacyLabel" | "tos_and_privacy_label" => Ok(GeneratedField::TosAndPrivacyLabel),
                            "tosConfirm" | "tos_confirm" => Ok(GeneratedField::TosConfirm),
                            "tosLinkText" | "tos_link_text" => Ok(GeneratedField::TosLinkText),
                            "privacyLinkText" | "privacy_link_text" => Ok(GeneratedField::PrivacyLinkText),
                            "privacyConfirm" | "privacy_confirm" => Ok(GeneratedField::PrivacyConfirm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternalUserNotFoundScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.ExternalUserNotFoundScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternalUserNotFoundScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut link_button_text__ = None;
                let mut auto_register_button_text__ = None;
                let mut tos_and_privacy_label__ = None;
                let mut tos_confirm__ = None;
                let mut tos_link_text__ = None;
                let mut privacy_link_text__ = None;
                let mut privacy_confirm__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkButtonText => {
                            if link_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkButtonText"));
                            }
                            link_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoRegisterButtonText => {
                            if auto_register_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoRegisterButtonText"));
                            }
                            auto_register_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosAndPrivacyLabel => {
                            if tos_and_privacy_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosAndPrivacyLabel"));
                            }
                            tos_and_privacy_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosConfirm => {
                            if tos_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosConfirm"));
                            }
                            tos_confirm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosLinkText => {
                            if tos_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosLinkText"));
                            }
                            tos_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyLinkText => {
                            if privacy_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyLinkText"));
                            }
                            privacy_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyConfirm => {
                            if privacy_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyConfirm"));
                            }
                            privacy_confirm__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExternalUserNotFoundScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    link_button_text: link_button_text__.unwrap_or_default(),
                    auto_register_button_text: auto_register_button_text__.unwrap_or_default(),
                    tos_and_privacy_label: tos_and_privacy_label__.unwrap_or_default(),
                    tos_confirm: tos_confirm__.unwrap_or_default(),
                    tos_link_text: tos_link_text__.unwrap_or_default(),
                    privacy_link_text: privacy_link_text__.unwrap_or_default(),
                    privacy_confirm: privacy_confirm__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.ExternalUserNotFoundScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FooterText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tos.is_empty() {
            len += 1;
        }
        if !self.privacy_policy.is_empty() {
            len += 1;
        }
        if !self.help.is_empty() {
            len += 1;
        }
        if !self.support_email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.FooterText", len)?;
        if !self.tos.is_empty() {
            struct_ser.serialize_field("tos", &self.tos)?;
        }
        if !self.privacy_policy.is_empty() {
            struct_ser.serialize_field("privacyPolicy", &self.privacy_policy)?;
        }
        if !self.help.is_empty() {
            struct_ser.serialize_field("help", &self.help)?;
        }
        if !self.support_email.is_empty() {
            struct_ser.serialize_field("supportEmail", &self.support_email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FooterText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tos",
            "privacy_policy",
            "privacyPolicy",
            "help",
            "support_email",
            "supportEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tos,
            PrivacyPolicy,
            Help,
            SupportEmail,
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
                            "tos" => Ok(GeneratedField::Tos),
                            "privacyPolicy" | "privacy_policy" => Ok(GeneratedField::PrivacyPolicy),
                            "help" => Ok(GeneratedField::Help),
                            "supportEmail" | "support_email" => Ok(GeneratedField::SupportEmail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FooterText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.FooterText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FooterText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tos__ = None;
                let mut privacy_policy__ = None;
                let mut help__ = None;
                let mut support_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tos => {
                            if tos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tos"));
                            }
                            tos__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyPolicy => {
                            if privacy_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyPolicy"));
                            }
                            privacy_policy__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Help => {
                            if help__.is_some() {
                                return Err(serde::de::Error::duplicate_field("help"));
                            }
                            help__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SupportEmail => {
                            if support_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportEmail"));
                            }
                            support_email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FooterText {
                    tos: tos__.unwrap_or_default(),
                    privacy_policy: privacy_policy__.unwrap_or_default(),
                    help: help__.unwrap_or_default(),
                    support_email: support_email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.FooterText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitMfaDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitMFADoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitMfaDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "cancel_button_text",
            "cancelButtonText",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CancelButtonText,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitMfaDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitMFADoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitMfaDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut cancel_button_text__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitMfaDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitMFADoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitMfaotpScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.description_otp.is_empty() {
            len += 1;
        }
        if !self.secret_label.is_empty() {
            len += 1;
        }
        if !self.code_label.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitMFAOTPScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.description_otp.is_empty() {
            struct_ser.serialize_field("descriptionOtp", &self.description_otp)?;
        }
        if !self.secret_label.is_empty() {
            struct_ser.serialize_field("secretLabel", &self.secret_label)?;
        }
        if !self.code_label.is_empty() {
            struct_ser.serialize_field("codeLabel", &self.code_label)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitMfaotpScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "description_otp",
            "descriptionOtp",
            "secret_label",
            "secretLabel",
            "code_label",
            "codeLabel",
            "next_button_text",
            "nextButtonText",
            "cancel_button_text",
            "cancelButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            DescriptionOtp,
            SecretLabel,
            CodeLabel,
            NextButtonText,
            CancelButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "descriptionOtp" | "description_otp" => Ok(GeneratedField::DescriptionOtp),
                            "secretLabel" | "secret_label" => Ok(GeneratedField::SecretLabel),
                            "codeLabel" | "code_label" => Ok(GeneratedField::CodeLabel),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitMfaotpScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitMFAOTPScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitMfaotpScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut description_otp__ = None;
                let mut secret_label__ = None;
                let mut code_label__ = None;
                let mut next_button_text__ = None;
                let mut cancel_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptionOtp => {
                            if description_otp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionOtp"));
                            }
                            description_otp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SecretLabel => {
                            if secret_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretLabel"));
                            }
                            secret_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeLabel => {
                            if code_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeLabel"));
                            }
                            code_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitMfaotpScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    description_otp: description_otp__.unwrap_or_default(),
                    secret_label: secret_label__.unwrap_or_default(),
                    code_label: code_label__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitMFAOTPScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitMfaPromptScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.otp_option.is_empty() {
            len += 1;
        }
        if !self.u2f_option.is_empty() {
            len += 1;
        }
        if !self.skip_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitMFAPromptScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.otp_option.is_empty() {
            struct_ser.serialize_field("otpOption", &self.otp_option)?;
        }
        if !self.u2f_option.is_empty() {
            struct_ser.serialize_field("u2fOption", &self.u2f_option)?;
        }
        if !self.skip_button_text.is_empty() {
            struct_ser.serialize_field("skipButtonText", &self.skip_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitMfaPromptScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "otp_option",
            "otpOption",
            "u2f_option",
            "u2fOption",
            "skip_button_text",
            "skipButtonText",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            OtpOption,
            U2fOption,
            SkipButtonText,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "otpOption" | "otp_option" => Ok(GeneratedField::OtpOption),
                            "u2fOption" | "u2f_option" => Ok(GeneratedField::U2fOption),
                            "skipButtonText" | "skip_button_text" => Ok(GeneratedField::SkipButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitMfaPromptScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitMFAPromptScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitMfaPromptScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut otp_option__ = None;
                let mut u2f_option__ = None;
                let mut skip_button_text__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtpOption => {
                            if otp_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpOption"));
                            }
                            otp_option__ = Some(map_.next_value()?);
                        }
                        GeneratedField::U2fOption => {
                            if u2f_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u2fOption"));
                            }
                            u2f_option__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipButtonText => {
                            if skip_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipButtonText"));
                            }
                            skip_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitMfaPromptScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    otp_option: otp_option__.unwrap_or_default(),
                    u2f_option: u2f_option__.unwrap_or_default(),
                    skip_button_text: skip_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitMFAPromptScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitMfau2fScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.token_name_label.is_empty() {
            len += 1;
        }
        if !self.not_supported.is_empty() {
            len += 1;
        }
        if !self.register_token_button_text.is_empty() {
            len += 1;
        }
        if !self.error_retry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitMFAU2FScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.token_name_label.is_empty() {
            struct_ser.serialize_field("tokenNameLabel", &self.token_name_label)?;
        }
        if !self.not_supported.is_empty() {
            struct_ser.serialize_field("notSupported", &self.not_supported)?;
        }
        if !self.register_token_button_text.is_empty() {
            struct_ser.serialize_field("registerTokenButtonText", &self.register_token_button_text)?;
        }
        if !self.error_retry.is_empty() {
            struct_ser.serialize_field("errorRetry", &self.error_retry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitMfau2fScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "token_name_label",
            "tokenNameLabel",
            "not_supported",
            "notSupported",
            "register_token_button_text",
            "registerTokenButtonText",
            "error_retry",
            "errorRetry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            TokenNameLabel,
            NotSupported,
            RegisterTokenButtonText,
            ErrorRetry,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "tokenNameLabel" | "token_name_label" => Ok(GeneratedField::TokenNameLabel),
                            "notSupported" | "not_supported" => Ok(GeneratedField::NotSupported),
                            "registerTokenButtonText" | "register_token_button_text" => Ok(GeneratedField::RegisterTokenButtonText),
                            "errorRetry" | "error_retry" => Ok(GeneratedField::ErrorRetry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitMfau2fScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitMFAU2FScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitMfau2fScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut token_name_label__ = None;
                let mut not_supported__ = None;
                let mut register_token_button_text__ = None;
                let mut error_retry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenNameLabel => {
                            if token_name_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenNameLabel"));
                            }
                            token_name_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotSupported => {
                            if not_supported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notSupported"));
                            }
                            not_supported__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegisterTokenButtonText => {
                            if register_token_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerTokenButtonText"));
                            }
                            register_token_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorRetry => {
                            if error_retry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorRetry"));
                            }
                            error_retry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitMfau2fScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    token_name_label: token_name_label__.unwrap_or_default(),
                    not_supported: not_supported__.unwrap_or_default(),
                    register_token_button_text: register_token_button_text__.unwrap_or_default(),
                    error_retry: error_retry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitMFAU2FScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitPasswordDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitPasswordDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitPasswordDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "next_button_text",
            "nextButtonText",
            "cancel_button_text",
            "cancelButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NextButtonText,
            CancelButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitPasswordDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitPasswordDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitPasswordDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut next_button_text__ = None;
                let mut cancel_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitPasswordDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitPasswordDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitPasswordScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_label.is_empty() {
            len += 1;
        }
        if !self.new_password_label.is_empty() {
            len += 1;
        }
        if !self.new_password_confirm_label.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.resend_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitPasswordScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_label.is_empty() {
            struct_ser.serialize_field("codeLabel", &self.code_label)?;
        }
        if !self.new_password_label.is_empty() {
            struct_ser.serialize_field("newPasswordLabel", &self.new_password_label)?;
        }
        if !self.new_password_confirm_label.is_empty() {
            struct_ser.serialize_field("newPasswordConfirmLabel", &self.new_password_confirm_label)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.resend_button_text.is_empty() {
            struct_ser.serialize_field("resendButtonText", &self.resend_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitPasswordScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "code_label",
            "codeLabel",
            "new_password_label",
            "newPasswordLabel",
            "new_password_confirm_label",
            "newPasswordConfirmLabel",
            "next_button_text",
            "nextButtonText",
            "resend_button_text",
            "resendButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeLabel,
            NewPasswordLabel,
            NewPasswordConfirmLabel,
            NextButtonText,
            ResendButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeLabel" | "code_label" => Ok(GeneratedField::CodeLabel),
                            "newPasswordLabel" | "new_password_label" => Ok(GeneratedField::NewPasswordLabel),
                            "newPasswordConfirmLabel" | "new_password_confirm_label" => Ok(GeneratedField::NewPasswordConfirmLabel),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "resendButtonText" | "resend_button_text" => Ok(GeneratedField::ResendButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitPasswordScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitPasswordScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitPasswordScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_label__ = None;
                let mut new_password_label__ = None;
                let mut new_password_confirm_label__ = None;
                let mut next_button_text__ = None;
                let mut resend_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeLabel => {
                            if code_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeLabel"));
                            }
                            code_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPasswordLabel => {
                            if new_password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPasswordLabel"));
                            }
                            new_password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPasswordConfirmLabel => {
                            if new_password_confirm_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPasswordConfirmLabel"));
                            }
                            new_password_confirm_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResendButtonText => {
                            if resend_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resendButtonText"));
                            }
                            resend_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitPasswordScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_label: code_label__.unwrap_or_default(),
                    new_password_label: new_password_label__.unwrap_or_default(),
                    new_password_confirm_label: new_password_confirm_label__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    resend_button_text: resend_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitPasswordScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitializeUserDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitializeUserDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitializeUserDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "cancel_button_text",
            "cancelButtonText",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CancelButtonText,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitializeUserDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitializeUserDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitializeUserDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut cancel_button_text__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitializeUserDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitializeUserDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitializeUserScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_label.is_empty() {
            len += 1;
        }
        if !self.new_password_label.is_empty() {
            len += 1;
        }
        if !self.new_password_confirm_label.is_empty() {
            len += 1;
        }
        if !self.resend_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.InitializeUserScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_label.is_empty() {
            struct_ser.serialize_field("codeLabel", &self.code_label)?;
        }
        if !self.new_password_label.is_empty() {
            struct_ser.serialize_field("newPasswordLabel", &self.new_password_label)?;
        }
        if !self.new_password_confirm_label.is_empty() {
            struct_ser.serialize_field("newPasswordConfirmLabel", &self.new_password_confirm_label)?;
        }
        if !self.resend_button_text.is_empty() {
            struct_ser.serialize_field("resendButtonText", &self.resend_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitializeUserScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "code_label",
            "codeLabel",
            "new_password_label",
            "newPasswordLabel",
            "new_password_confirm_label",
            "newPasswordConfirmLabel",
            "resend_button_text",
            "resendButtonText",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeLabel,
            NewPasswordLabel,
            NewPasswordConfirmLabel,
            ResendButtonText,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeLabel" | "code_label" => Ok(GeneratedField::CodeLabel),
                            "newPasswordLabel" | "new_password_label" => Ok(GeneratedField::NewPasswordLabel),
                            "newPasswordConfirmLabel" | "new_password_confirm_label" => Ok(GeneratedField::NewPasswordConfirmLabel),
                            "resendButtonText" | "resend_button_text" => Ok(GeneratedField::ResendButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InitializeUserScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.InitializeUserScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitializeUserScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_label__ = None;
                let mut new_password_label__ = None;
                let mut new_password_confirm_label__ = None;
                let mut resend_button_text__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeLabel => {
                            if code_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeLabel"));
                            }
                            code_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPasswordLabel => {
                            if new_password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPasswordLabel"));
                            }
                            new_password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPasswordConfirmLabel => {
                            if new_password_confirm_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPasswordConfirmLabel"));
                            }
                            new_password_confirm_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResendButtonText => {
                            if resend_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resendButtonText"));
                            }
                            resend_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InitializeUserScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_label: code_label__.unwrap_or_default(),
                    new_password_label: new_password_label__.unwrap_or_default(),
                    new_password_confirm_label: new_password_confirm_label__.unwrap_or_default(),
                    resend_button_text: resend_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.InitializeUserScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkingUserDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.LinkingUserDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkingUserDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "cancel_button_text",
            "cancelButtonText",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CancelButtonText,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkingUserDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.LinkingUserDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinkingUserDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut cancel_button_text__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LinkingUserDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.LinkingUserDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkingUserPromptScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.link_button_text.is_empty() {
            len += 1;
        }
        if !self.other_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.LinkingUserPromptScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.link_button_text.is_empty() {
            struct_ser.serialize_field("linkButtonText", &self.link_button_text)?;
        }
        if !self.other_button_text.is_empty() {
            struct_ser.serialize_field("otherButtonText", &self.other_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkingUserPromptScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "link_button_text",
            "linkButtonText",
            "other_button_text",
            "otherButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            LinkButtonText,
            OtherButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "linkButtonText" | "link_button_text" => Ok(GeneratedField::LinkButtonText),
                            "otherButtonText" | "other_button_text" => Ok(GeneratedField::OtherButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkingUserPromptScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.LinkingUserPromptScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinkingUserPromptScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut link_button_text__ = None;
                let mut other_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkButtonText => {
                            if link_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkButtonText"));
                            }
                            link_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtherButtonText => {
                            if other_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherButtonText"));
                            }
                            other_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LinkingUserPromptScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    link_button_text: link_button_text__.unwrap_or_default(),
                    other_button_text: other_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.LinkingUserPromptScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginCustomText {
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
        if self.select_account_text.is_some() {
            len += 1;
        }
        if self.login_text.is_some() {
            len += 1;
        }
        if self.password_text.is_some() {
            len += 1;
        }
        if self.username_change_text.is_some() {
            len += 1;
        }
        if self.username_change_done_text.is_some() {
            len += 1;
        }
        if self.init_password_text.is_some() {
            len += 1;
        }
        if self.init_password_done_text.is_some() {
            len += 1;
        }
        if self.email_verification_text.is_some() {
            len += 1;
        }
        if self.email_verification_done_text.is_some() {
            len += 1;
        }
        if self.initialize_user_text.is_some() {
            len += 1;
        }
        if self.initialize_done_text.is_some() {
            len += 1;
        }
        if self.init_mfa_prompt_text.is_some() {
            len += 1;
        }
        if self.init_mfa_otp_text.is_some() {
            len += 1;
        }
        if self.init_mfa_u2f_text.is_some() {
            len += 1;
        }
        if self.init_mfa_done_text.is_some() {
            len += 1;
        }
        if self.mfa_providers_text.is_some() {
            len += 1;
        }
        if self.verify_mfa_otp_text.is_some() {
            len += 1;
        }
        if self.verify_mfa_u2f_text.is_some() {
            len += 1;
        }
        if self.passwordless_text.is_some() {
            len += 1;
        }
        if self.password_change_text.is_some() {
            len += 1;
        }
        if self.password_change_done_text.is_some() {
            len += 1;
        }
        if self.password_reset_done_text.is_some() {
            len += 1;
        }
        if self.registration_option_text.is_some() {
            len += 1;
        }
        if self.registration_user_text.is_some() {
            len += 1;
        }
        if self.registration_org_text.is_some() {
            len += 1;
        }
        if self.linking_user_done_text.is_some() {
            len += 1;
        }
        if self.external_user_not_found_text.is_some() {
            len += 1;
        }
        if self.success_login_text.is_some() {
            len += 1;
        }
        if self.logout_text.is_some() {
            len += 1;
        }
        if self.footer_text.is_some() {
            len += 1;
        }
        if self.passwordless_prompt_text.is_some() {
            len += 1;
        }
        if self.passwordless_registration_text.is_some() {
            len += 1;
        }
        if self.passwordless_registration_done_text.is_some() {
            len += 1;
        }
        if self.external_registration_user_overview_text.is_some() {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        if self.linking_user_prompt_text.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.LoginCustomText", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.select_account_text.as_ref() {
            struct_ser.serialize_field("selectAccountText", v)?;
        }
        if let Some(v) = self.login_text.as_ref() {
            struct_ser.serialize_field("loginText", v)?;
        }
        if let Some(v) = self.password_text.as_ref() {
            struct_ser.serialize_field("passwordText", v)?;
        }
        if let Some(v) = self.username_change_text.as_ref() {
            struct_ser.serialize_field("usernameChangeText", v)?;
        }
        if let Some(v) = self.username_change_done_text.as_ref() {
            struct_ser.serialize_field("usernameChangeDoneText", v)?;
        }
        if let Some(v) = self.init_password_text.as_ref() {
            struct_ser.serialize_field("initPasswordText", v)?;
        }
        if let Some(v) = self.init_password_done_text.as_ref() {
            struct_ser.serialize_field("initPasswordDoneText", v)?;
        }
        if let Some(v) = self.email_verification_text.as_ref() {
            struct_ser.serialize_field("emailVerificationText", v)?;
        }
        if let Some(v) = self.email_verification_done_text.as_ref() {
            struct_ser.serialize_field("emailVerificationDoneText", v)?;
        }
        if let Some(v) = self.initialize_user_text.as_ref() {
            struct_ser.serialize_field("initializeUserText", v)?;
        }
        if let Some(v) = self.initialize_done_text.as_ref() {
            struct_ser.serialize_field("initializeDoneText", v)?;
        }
        if let Some(v) = self.init_mfa_prompt_text.as_ref() {
            struct_ser.serialize_field("initMfaPromptText", v)?;
        }
        if let Some(v) = self.init_mfa_otp_text.as_ref() {
            struct_ser.serialize_field("initMfaOtpText", v)?;
        }
        if let Some(v) = self.init_mfa_u2f_text.as_ref() {
            struct_ser.serialize_field("initMfaU2fText", v)?;
        }
        if let Some(v) = self.init_mfa_done_text.as_ref() {
            struct_ser.serialize_field("initMfaDoneText", v)?;
        }
        if let Some(v) = self.mfa_providers_text.as_ref() {
            struct_ser.serialize_field("mfaProvidersText", v)?;
        }
        if let Some(v) = self.verify_mfa_otp_text.as_ref() {
            struct_ser.serialize_field("verifyMfaOtpText", v)?;
        }
        if let Some(v) = self.verify_mfa_u2f_text.as_ref() {
            struct_ser.serialize_field("verifyMfaU2fText", v)?;
        }
        if let Some(v) = self.passwordless_text.as_ref() {
            struct_ser.serialize_field("passwordlessText", v)?;
        }
        if let Some(v) = self.password_change_text.as_ref() {
            struct_ser.serialize_field("passwordChangeText", v)?;
        }
        if let Some(v) = self.password_change_done_text.as_ref() {
            struct_ser.serialize_field("passwordChangeDoneText", v)?;
        }
        if let Some(v) = self.password_reset_done_text.as_ref() {
            struct_ser.serialize_field("passwordResetDoneText", v)?;
        }
        if let Some(v) = self.registration_option_text.as_ref() {
            struct_ser.serialize_field("registrationOptionText", v)?;
        }
        if let Some(v) = self.registration_user_text.as_ref() {
            struct_ser.serialize_field("registrationUserText", v)?;
        }
        if let Some(v) = self.registration_org_text.as_ref() {
            struct_ser.serialize_field("registrationOrgText", v)?;
        }
        if let Some(v) = self.linking_user_done_text.as_ref() {
            struct_ser.serialize_field("linkingUserDoneText", v)?;
        }
        if let Some(v) = self.external_user_not_found_text.as_ref() {
            struct_ser.serialize_field("externalUserNotFoundText", v)?;
        }
        if let Some(v) = self.success_login_text.as_ref() {
            struct_ser.serialize_field("successLoginText", v)?;
        }
        if let Some(v) = self.logout_text.as_ref() {
            struct_ser.serialize_field("logoutText", v)?;
        }
        if let Some(v) = self.footer_text.as_ref() {
            struct_ser.serialize_field("footerText", v)?;
        }
        if let Some(v) = self.passwordless_prompt_text.as_ref() {
            struct_ser.serialize_field("passwordlessPromptText", v)?;
        }
        if let Some(v) = self.passwordless_registration_text.as_ref() {
            struct_ser.serialize_field("passwordlessRegistrationText", v)?;
        }
        if let Some(v) = self.passwordless_registration_done_text.as_ref() {
            struct_ser.serialize_field("passwordlessRegistrationDoneText", v)?;
        }
        if let Some(v) = self.external_registration_user_overview_text.as_ref() {
            struct_ser.serialize_field("externalRegistrationUserOverviewText", v)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if let Some(v) = self.linking_user_prompt_text.as_ref() {
            struct_ser.serialize_field("linkingUserPromptText", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginCustomText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "select_account_text",
            "selectAccountText",
            "login_text",
            "loginText",
            "password_text",
            "passwordText",
            "username_change_text",
            "usernameChangeText",
            "username_change_done_text",
            "usernameChangeDoneText",
            "init_password_text",
            "initPasswordText",
            "init_password_done_text",
            "initPasswordDoneText",
            "email_verification_text",
            "emailVerificationText",
            "email_verification_done_text",
            "emailVerificationDoneText",
            "initialize_user_text",
            "initializeUserText",
            "initialize_done_text",
            "initializeDoneText",
            "init_mfa_prompt_text",
            "initMfaPromptText",
            "init_mfa_otp_text",
            "initMfaOtpText",
            "init_mfa_u2f_text",
            "initMfaU2fText",
            "init_mfa_done_text",
            "initMfaDoneText",
            "mfa_providers_text",
            "mfaProvidersText",
            "verify_mfa_otp_text",
            "verifyMfaOtpText",
            "verify_mfa_u2f_text",
            "verifyMfaU2fText",
            "passwordless_text",
            "passwordlessText",
            "password_change_text",
            "passwordChangeText",
            "password_change_done_text",
            "passwordChangeDoneText",
            "password_reset_done_text",
            "passwordResetDoneText",
            "registration_option_text",
            "registrationOptionText",
            "registration_user_text",
            "registrationUserText",
            "registration_org_text",
            "registrationOrgText",
            "linking_user_done_text",
            "linkingUserDoneText",
            "external_user_not_found_text",
            "externalUserNotFoundText",
            "success_login_text",
            "successLoginText",
            "logout_text",
            "logoutText",
            "footer_text",
            "footerText",
            "passwordless_prompt_text",
            "passwordlessPromptText",
            "passwordless_registration_text",
            "passwordlessRegistrationText",
            "passwordless_registration_done_text",
            "passwordlessRegistrationDoneText",
            "external_registration_user_overview_text",
            "externalRegistrationUserOverviewText",
            "is_default",
            "isDefault",
            "linking_user_prompt_text",
            "linkingUserPromptText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SelectAccountText,
            LoginText,
            PasswordText,
            UsernameChangeText,
            UsernameChangeDoneText,
            InitPasswordText,
            InitPasswordDoneText,
            EmailVerificationText,
            EmailVerificationDoneText,
            InitializeUserText,
            InitializeDoneText,
            InitMfaPromptText,
            InitMfaOtpText,
            InitMfaU2fText,
            InitMfaDoneText,
            MfaProvidersText,
            VerifyMfaOtpText,
            VerifyMfaU2fText,
            PasswordlessText,
            PasswordChangeText,
            PasswordChangeDoneText,
            PasswordResetDoneText,
            RegistrationOptionText,
            RegistrationUserText,
            RegistrationOrgText,
            LinkingUserDoneText,
            ExternalUserNotFoundText,
            SuccessLoginText,
            LogoutText,
            FooterText,
            PasswordlessPromptText,
            PasswordlessRegistrationText,
            PasswordlessRegistrationDoneText,
            ExternalRegistrationUserOverviewText,
            IsDefault,
            LinkingUserPromptText,
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
                            "selectAccountText" | "select_account_text" => Ok(GeneratedField::SelectAccountText),
                            "loginText" | "login_text" => Ok(GeneratedField::LoginText),
                            "passwordText" | "password_text" => Ok(GeneratedField::PasswordText),
                            "usernameChangeText" | "username_change_text" => Ok(GeneratedField::UsernameChangeText),
                            "usernameChangeDoneText" | "username_change_done_text" => Ok(GeneratedField::UsernameChangeDoneText),
                            "initPasswordText" | "init_password_text" => Ok(GeneratedField::InitPasswordText),
                            "initPasswordDoneText" | "init_password_done_text" => Ok(GeneratedField::InitPasswordDoneText),
                            "emailVerificationText" | "email_verification_text" => Ok(GeneratedField::EmailVerificationText),
                            "emailVerificationDoneText" | "email_verification_done_text" => Ok(GeneratedField::EmailVerificationDoneText),
                            "initializeUserText" | "initialize_user_text" => Ok(GeneratedField::InitializeUserText),
                            "initializeDoneText" | "initialize_done_text" => Ok(GeneratedField::InitializeDoneText),
                            "initMfaPromptText" | "init_mfa_prompt_text" => Ok(GeneratedField::InitMfaPromptText),
                            "initMfaOtpText" | "init_mfa_otp_text" => Ok(GeneratedField::InitMfaOtpText),
                            "initMfaU2fText" | "init_mfa_u2f_text" => Ok(GeneratedField::InitMfaU2fText),
                            "initMfaDoneText" | "init_mfa_done_text" => Ok(GeneratedField::InitMfaDoneText),
                            "mfaProvidersText" | "mfa_providers_text" => Ok(GeneratedField::MfaProvidersText),
                            "verifyMfaOtpText" | "verify_mfa_otp_text" => Ok(GeneratedField::VerifyMfaOtpText),
                            "verifyMfaU2fText" | "verify_mfa_u2f_text" => Ok(GeneratedField::VerifyMfaU2fText),
                            "passwordlessText" | "passwordless_text" => Ok(GeneratedField::PasswordlessText),
                            "passwordChangeText" | "password_change_text" => Ok(GeneratedField::PasswordChangeText),
                            "passwordChangeDoneText" | "password_change_done_text" => Ok(GeneratedField::PasswordChangeDoneText),
                            "passwordResetDoneText" | "password_reset_done_text" => Ok(GeneratedField::PasswordResetDoneText),
                            "registrationOptionText" | "registration_option_text" => Ok(GeneratedField::RegistrationOptionText),
                            "registrationUserText" | "registration_user_text" => Ok(GeneratedField::RegistrationUserText),
                            "registrationOrgText" | "registration_org_text" => Ok(GeneratedField::RegistrationOrgText),
                            "linkingUserDoneText" | "linking_user_done_text" => Ok(GeneratedField::LinkingUserDoneText),
                            "externalUserNotFoundText" | "external_user_not_found_text" => Ok(GeneratedField::ExternalUserNotFoundText),
                            "successLoginText" | "success_login_text" => Ok(GeneratedField::SuccessLoginText),
                            "logoutText" | "logout_text" => Ok(GeneratedField::LogoutText),
                            "footerText" | "footer_text" => Ok(GeneratedField::FooterText),
                            "passwordlessPromptText" | "passwordless_prompt_text" => Ok(GeneratedField::PasswordlessPromptText),
                            "passwordlessRegistrationText" | "passwordless_registration_text" => Ok(GeneratedField::PasswordlessRegistrationText),
                            "passwordlessRegistrationDoneText" | "passwordless_registration_done_text" => Ok(GeneratedField::PasswordlessRegistrationDoneText),
                            "externalRegistrationUserOverviewText" | "external_registration_user_overview_text" => Ok(GeneratedField::ExternalRegistrationUserOverviewText),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "linkingUserPromptText" | "linking_user_prompt_text" => Ok(GeneratedField::LinkingUserPromptText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginCustomText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.LoginCustomText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginCustomText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut select_account_text__ = None;
                let mut login_text__ = None;
                let mut password_text__ = None;
                let mut username_change_text__ = None;
                let mut username_change_done_text__ = None;
                let mut init_password_text__ = None;
                let mut init_password_done_text__ = None;
                let mut email_verification_text__ = None;
                let mut email_verification_done_text__ = None;
                let mut initialize_user_text__ = None;
                let mut initialize_done_text__ = None;
                let mut init_mfa_prompt_text__ = None;
                let mut init_mfa_otp_text__ = None;
                let mut init_mfa_u2f_text__ = None;
                let mut init_mfa_done_text__ = None;
                let mut mfa_providers_text__ = None;
                let mut verify_mfa_otp_text__ = None;
                let mut verify_mfa_u2f_text__ = None;
                let mut passwordless_text__ = None;
                let mut password_change_text__ = None;
                let mut password_change_done_text__ = None;
                let mut password_reset_done_text__ = None;
                let mut registration_option_text__ = None;
                let mut registration_user_text__ = None;
                let mut registration_org_text__ = None;
                let mut linking_user_done_text__ = None;
                let mut external_user_not_found_text__ = None;
                let mut success_login_text__ = None;
                let mut logout_text__ = None;
                let mut footer_text__ = None;
                let mut passwordless_prompt_text__ = None;
                let mut passwordless_registration_text__ = None;
                let mut passwordless_registration_done_text__ = None;
                let mut external_registration_user_overview_text__ = None;
                let mut is_default__ = None;
                let mut linking_user_prompt_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SelectAccountText => {
                            if select_account_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectAccountText"));
                            }
                            select_account_text__ = map_.next_value()?;
                        }
                        GeneratedField::LoginText => {
                            if login_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginText"));
                            }
                            login_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordText => {
                            if password_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordText"));
                            }
                            password_text__ = map_.next_value()?;
                        }
                        GeneratedField::UsernameChangeText => {
                            if username_change_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameChangeText"));
                            }
                            username_change_text__ = map_.next_value()?;
                        }
                        GeneratedField::UsernameChangeDoneText => {
                            if username_change_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameChangeDoneText"));
                            }
                            username_change_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitPasswordText => {
                            if init_password_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initPasswordText"));
                            }
                            init_password_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitPasswordDoneText => {
                            if init_password_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initPasswordDoneText"));
                            }
                            init_password_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::EmailVerificationText => {
                            if email_verification_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailVerificationText"));
                            }
                            email_verification_text__ = map_.next_value()?;
                        }
                        GeneratedField::EmailVerificationDoneText => {
                            if email_verification_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailVerificationDoneText"));
                            }
                            email_verification_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitializeUserText => {
                            if initialize_user_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initializeUserText"));
                            }
                            initialize_user_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitializeDoneText => {
                            if initialize_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initializeDoneText"));
                            }
                            initialize_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitMfaPromptText => {
                            if init_mfa_prompt_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initMfaPromptText"));
                            }
                            init_mfa_prompt_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitMfaOtpText => {
                            if init_mfa_otp_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initMfaOtpText"));
                            }
                            init_mfa_otp_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitMfaU2fText => {
                            if init_mfa_u2f_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initMfaU2fText"));
                            }
                            init_mfa_u2f_text__ = map_.next_value()?;
                        }
                        GeneratedField::InitMfaDoneText => {
                            if init_mfa_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initMfaDoneText"));
                            }
                            init_mfa_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::MfaProvidersText => {
                            if mfa_providers_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mfaProvidersText"));
                            }
                            mfa_providers_text__ = map_.next_value()?;
                        }
                        GeneratedField::VerifyMfaOtpText => {
                            if verify_mfa_otp_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyMfaOtpText"));
                            }
                            verify_mfa_otp_text__ = map_.next_value()?;
                        }
                        GeneratedField::VerifyMfaU2fText => {
                            if verify_mfa_u2f_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyMfaU2fText"));
                            }
                            verify_mfa_u2f_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordlessText => {
                            if passwordless_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessText"));
                            }
                            passwordless_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordChangeText => {
                            if password_change_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChangeText"));
                            }
                            password_change_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordChangeDoneText => {
                            if password_change_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChangeDoneText"));
                            }
                            password_change_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordResetDoneText => {
                            if password_reset_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordResetDoneText"));
                            }
                            password_reset_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::RegistrationOptionText => {
                            if registration_option_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registrationOptionText"));
                            }
                            registration_option_text__ = map_.next_value()?;
                        }
                        GeneratedField::RegistrationUserText => {
                            if registration_user_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registrationUserText"));
                            }
                            registration_user_text__ = map_.next_value()?;
                        }
                        GeneratedField::RegistrationOrgText => {
                            if registration_org_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registrationOrgText"));
                            }
                            registration_org_text__ = map_.next_value()?;
                        }
                        GeneratedField::LinkingUserDoneText => {
                            if linking_user_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkingUserDoneText"));
                            }
                            linking_user_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::ExternalUserNotFoundText => {
                            if external_user_not_found_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalUserNotFoundText"));
                            }
                            external_user_not_found_text__ = map_.next_value()?;
                        }
                        GeneratedField::SuccessLoginText => {
                            if success_login_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successLoginText"));
                            }
                            success_login_text__ = map_.next_value()?;
                        }
                        GeneratedField::LogoutText => {
                            if logout_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoutText"));
                            }
                            logout_text__ = map_.next_value()?;
                        }
                        GeneratedField::FooterText => {
                            if footer_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("footerText"));
                            }
                            footer_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordlessPromptText => {
                            if passwordless_prompt_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessPromptText"));
                            }
                            passwordless_prompt_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordlessRegistrationText => {
                            if passwordless_registration_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessRegistrationText"));
                            }
                            passwordless_registration_text__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordlessRegistrationDoneText => {
                            if passwordless_registration_done_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessRegistrationDoneText"));
                            }
                            passwordless_registration_done_text__ = map_.next_value()?;
                        }
                        GeneratedField::ExternalRegistrationUserOverviewText => {
                            if external_registration_user_overview_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalRegistrationUserOverviewText"));
                            }
                            external_registration_user_overview_text__ = map_.next_value()?;
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkingUserPromptText => {
                            if linking_user_prompt_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkingUserPromptText"));
                            }
                            linking_user_prompt_text__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginCustomText {
                    details: details__,
                    select_account_text: select_account_text__,
                    login_text: login_text__,
                    password_text: password_text__,
                    username_change_text: username_change_text__,
                    username_change_done_text: username_change_done_text__,
                    init_password_text: init_password_text__,
                    init_password_done_text: init_password_done_text__,
                    email_verification_text: email_verification_text__,
                    email_verification_done_text: email_verification_done_text__,
                    initialize_user_text: initialize_user_text__,
                    initialize_done_text: initialize_done_text__,
                    init_mfa_prompt_text: init_mfa_prompt_text__,
                    init_mfa_otp_text: init_mfa_otp_text__,
                    init_mfa_u2f_text: init_mfa_u2f_text__,
                    init_mfa_done_text: init_mfa_done_text__,
                    mfa_providers_text: mfa_providers_text__,
                    verify_mfa_otp_text: verify_mfa_otp_text__,
                    verify_mfa_u2f_text: verify_mfa_u2f_text__,
                    passwordless_text: passwordless_text__,
                    password_change_text: password_change_text__,
                    password_change_done_text: password_change_done_text__,
                    password_reset_done_text: password_reset_done_text__,
                    registration_option_text: registration_option_text__,
                    registration_user_text: registration_user_text__,
                    registration_org_text: registration_org_text__,
                    linking_user_done_text: linking_user_done_text__,
                    external_user_not_found_text: external_user_not_found_text__,
                    success_login_text: success_login_text__,
                    logout_text: logout_text__,
                    footer_text: footer_text__,
                    passwordless_prompt_text: passwordless_prompt_text__,
                    passwordless_registration_text: passwordless_registration_text__,
                    passwordless_registration_done_text: passwordless_registration_done_text__,
                    external_registration_user_overview_text: external_registration_user_overview_text__,
                    is_default: is_default__.unwrap_or_default(),
                    linking_user_prompt_text: linking_user_prompt_text__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.LoginCustomText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.title_linking_process.is_empty() {
            len += 1;
        }
        if !self.description_linking_process.is_empty() {
            len += 1;
        }
        if !self.user_must_be_member_of_org.is_empty() {
            len += 1;
        }
        if !self.login_name_label.is_empty() {
            len += 1;
        }
        if !self.register_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.external_user_description.is_empty() {
            len += 1;
        }
        if !self.user_name_placeholder.is_empty() {
            len += 1;
        }
        if !self.login_name_placeholder.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.LoginScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.title_linking_process.is_empty() {
            struct_ser.serialize_field("titleLinkingProcess", &self.title_linking_process)?;
        }
        if !self.description_linking_process.is_empty() {
            struct_ser.serialize_field("descriptionLinkingProcess", &self.description_linking_process)?;
        }
        if !self.user_must_be_member_of_org.is_empty() {
            struct_ser.serialize_field("userMustBeMemberOfOrg", &self.user_must_be_member_of_org)?;
        }
        if !self.login_name_label.is_empty() {
            struct_ser.serialize_field("loginNameLabel", &self.login_name_label)?;
        }
        if !self.register_button_text.is_empty() {
            struct_ser.serialize_field("registerButtonText", &self.register_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.external_user_description.is_empty() {
            struct_ser.serialize_field("externalUserDescription", &self.external_user_description)?;
        }
        if !self.user_name_placeholder.is_empty() {
            struct_ser.serialize_field("userNamePlaceholder", &self.user_name_placeholder)?;
        }
        if !self.login_name_placeholder.is_empty() {
            struct_ser.serialize_field("loginNamePlaceholder", &self.login_name_placeholder)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "title_linking_process",
            "titleLinkingProcess",
            "description_linking_process",
            "descriptionLinkingProcess",
            "user_must_be_member_of_org",
            "userMustBeMemberOfOrg",
            "login_name_label",
            "loginNameLabel",
            "register_button_text",
            "registerButtonText",
            "next_button_text",
            "nextButtonText",
            "external_user_description",
            "externalUserDescription",
            "user_name_placeholder",
            "userNamePlaceholder",
            "login_name_placeholder",
            "loginNamePlaceholder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            TitleLinkingProcess,
            DescriptionLinkingProcess,
            UserMustBeMemberOfOrg,
            LoginNameLabel,
            RegisterButtonText,
            NextButtonText,
            ExternalUserDescription,
            UserNamePlaceholder,
            LoginNamePlaceholder,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "titleLinkingProcess" | "title_linking_process" => Ok(GeneratedField::TitleLinkingProcess),
                            "descriptionLinkingProcess" | "description_linking_process" => Ok(GeneratedField::DescriptionLinkingProcess),
                            "userMustBeMemberOfOrg" | "user_must_be_member_of_org" => Ok(GeneratedField::UserMustBeMemberOfOrg),
                            "loginNameLabel" | "login_name_label" => Ok(GeneratedField::LoginNameLabel),
                            "registerButtonText" | "register_button_text" => Ok(GeneratedField::RegisterButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "externalUserDescription" | "external_user_description" => Ok(GeneratedField::ExternalUserDescription),
                            "userNamePlaceholder" | "user_name_placeholder" => Ok(GeneratedField::UserNamePlaceholder),
                            "loginNamePlaceholder" | "login_name_placeholder" => Ok(GeneratedField::LoginNamePlaceholder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.LoginScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut title_linking_process__ = None;
                let mut description_linking_process__ = None;
                let mut user_must_be_member_of_org__ = None;
                let mut login_name_label__ = None;
                let mut register_button_text__ = None;
                let mut next_button_text__ = None;
                let mut external_user_description__ = None;
                let mut user_name_placeholder__ = None;
                let mut login_name_placeholder__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TitleLinkingProcess => {
                            if title_linking_process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("titleLinkingProcess"));
                            }
                            title_linking_process__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptionLinkingProcess => {
                            if description_linking_process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionLinkingProcess"));
                            }
                            description_linking_process__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserMustBeMemberOfOrg => {
                            if user_must_be_member_of_org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userMustBeMemberOfOrg"));
                            }
                            user_must_be_member_of_org__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginNameLabel => {
                            if login_name_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginNameLabel"));
                            }
                            login_name_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegisterButtonText => {
                            if register_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerButtonText"));
                            }
                            register_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExternalUserDescription => {
                            if external_user_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalUserDescription"));
                            }
                            external_user_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNamePlaceholder => {
                            if user_name_placeholder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNamePlaceholder"));
                            }
                            user_name_placeholder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginNamePlaceholder => {
                            if login_name_placeholder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginNamePlaceholder"));
                            }
                            login_name_placeholder__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    title_linking_process: title_linking_process__.unwrap_or_default(),
                    description_linking_process: description_linking_process__.unwrap_or_default(),
                    user_must_be_member_of_org: user_must_be_member_of_org__.unwrap_or_default(),
                    login_name_label: login_name_label__.unwrap_or_default(),
                    register_button_text: register_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    external_user_description: external_user_description__.unwrap_or_default(),
                    user_name_placeholder: user_name_placeholder__.unwrap_or_default(),
                    login_name_placeholder: login_name_placeholder__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.LoginScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogoutDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.login_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.LogoutDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.login_button_text.is_empty() {
            struct_ser.serialize_field("loginButtonText", &self.login_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogoutDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "login_button_text",
            "loginButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            LoginButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "loginButtonText" | "login_button_text" => Ok(GeneratedField::LoginButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogoutDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.LogoutDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogoutDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut login_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginButtonText => {
                            if login_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginButtonText"));
                            }
                            login_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogoutDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    login_button_text: login_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.LogoutDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MfaProvidersText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.choose_other.is_empty() {
            len += 1;
        }
        if !self.otp.is_empty() {
            len += 1;
        }
        if !self.u2f.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.MFAProvidersText", len)?;
        if !self.choose_other.is_empty() {
            struct_ser.serialize_field("chooseOther", &self.choose_other)?;
        }
        if !self.otp.is_empty() {
            struct_ser.serialize_field("otp", &self.otp)?;
        }
        if !self.u2f.is_empty() {
            struct_ser.serialize_field("u2f", &self.u2f)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MfaProvidersText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "choose_other",
            "chooseOther",
            "otp",
            "u2f",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChooseOther,
            Otp,
            U2f,
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
                            "chooseOther" | "choose_other" => Ok(GeneratedField::ChooseOther),
                            "otp" => Ok(GeneratedField::Otp),
                            "u2f" => Ok(GeneratedField::U2f),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MfaProvidersText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.MFAProvidersText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MfaProvidersText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut choose_other__ = None;
                let mut otp__ = None;
                let mut u2f__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChooseOther => {
                            if choose_other__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chooseOther"));
                            }
                            choose_other__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Otp => {
                            if otp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otp"));
                            }
                            otp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::U2f => {
                            if u2f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u2f"));
                            }
                            u2f__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MfaProvidersText {
                    choose_other: choose_other__.unwrap_or_default(),
                    otp: otp__.unwrap_or_default(),
                    u2f: u2f__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.MFAProvidersText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageCustomText {
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
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.pre_header.is_empty() {
            len += 1;
        }
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.greeting.is_empty() {
            len += 1;
        }
        if !self.text.is_empty() {
            len += 1;
        }
        if !self.button_text.is_empty() {
            len += 1;
        }
        if !self.footer_text.is_empty() {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.MessageCustomText", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.pre_header.is_empty() {
            struct_ser.serialize_field("preHeader", &self.pre_header)?;
        }
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.greeting.is_empty() {
            struct_ser.serialize_field("greeting", &self.greeting)?;
        }
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if !self.button_text.is_empty() {
            struct_ser.serialize_field("buttonText", &self.button_text)?;
        }
        if !self.footer_text.is_empty() {
            struct_ser.serialize_field("footerText", &self.footer_text)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageCustomText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "title",
            "pre_header",
            "preHeader",
            "subject",
            "greeting",
            "text",
            "button_text",
            "buttonText",
            "footer_text",
            "footerText",
            "is_default",
            "isDefault",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Title,
            PreHeader,
            Subject,
            Greeting,
            Text,
            ButtonText,
            FooterText,
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
                            "title" => Ok(GeneratedField::Title),
                            "preHeader" | "pre_header" => Ok(GeneratedField::PreHeader),
                            "subject" => Ok(GeneratedField::Subject),
                            "greeting" => Ok(GeneratedField::Greeting),
                            "text" => Ok(GeneratedField::Text),
                            "buttonText" | "button_text" => Ok(GeneratedField::ButtonText),
                            "footerText" | "footer_text" => Ok(GeneratedField::FooterText),
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
            type Value = MessageCustomText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.MessageCustomText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageCustomText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut title__ = None;
                let mut pre_header__ = None;
                let mut subject__ = None;
                let mut greeting__ = None;
                let mut text__ = None;
                let mut button_text__ = None;
                let mut footer_text__ = None;
                let mut is_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreHeader => {
                            if pre_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preHeader"));
                            }
                            pre_header__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Greeting => {
                            if greeting__.is_some() {
                                return Err(serde::de::Error::duplicate_field("greeting"));
                            }
                            greeting__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ButtonText => {
                            if button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buttonText"));
                            }
                            button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FooterText => {
                            if footer_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("footerText"));
                            }
                            footer_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageCustomText {
                    details: details__,
                    title: title__.unwrap_or_default(),
                    pre_header: pre_header__.unwrap_or_default(),
                    subject: subject__.unwrap_or_default(),
                    greeting: greeting__.unwrap_or_default(),
                    text: text__.unwrap_or_default(),
                    button_text: button_text__.unwrap_or_default(),
                    footer_text: footer_text__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.MessageCustomText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordChangeDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordChangeDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordChangeDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordChangeDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordChangeDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordChangeDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordChangeDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordChangeDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordChangeScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.old_password_label.is_empty() {
            len += 1;
        }
        if !self.new_password_label.is_empty() {
            len += 1;
        }
        if !self.new_password_confirm_label.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.expired_description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordChangeScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.old_password_label.is_empty() {
            struct_ser.serialize_field("oldPasswordLabel", &self.old_password_label)?;
        }
        if !self.new_password_label.is_empty() {
            struct_ser.serialize_field("newPasswordLabel", &self.new_password_label)?;
        }
        if !self.new_password_confirm_label.is_empty() {
            struct_ser.serialize_field("newPasswordConfirmLabel", &self.new_password_confirm_label)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.expired_description.is_empty() {
            struct_ser.serialize_field("expiredDescription", &self.expired_description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordChangeScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "old_password_label",
            "oldPasswordLabel",
            "new_password_label",
            "newPasswordLabel",
            "new_password_confirm_label",
            "newPasswordConfirmLabel",
            "cancel_button_text",
            "cancelButtonText",
            "next_button_text",
            "nextButtonText",
            "expired_description",
            "expiredDescription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            OldPasswordLabel,
            NewPasswordLabel,
            NewPasswordConfirmLabel,
            CancelButtonText,
            NextButtonText,
            ExpiredDescription,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "oldPasswordLabel" | "old_password_label" => Ok(GeneratedField::OldPasswordLabel),
                            "newPasswordLabel" | "new_password_label" => Ok(GeneratedField::NewPasswordLabel),
                            "newPasswordConfirmLabel" | "new_password_confirm_label" => Ok(GeneratedField::NewPasswordConfirmLabel),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "expiredDescription" | "expired_description" => Ok(GeneratedField::ExpiredDescription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordChangeScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordChangeScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordChangeScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut old_password_label__ = None;
                let mut new_password_label__ = None;
                let mut new_password_confirm_label__ = None;
                let mut cancel_button_text__ = None;
                let mut next_button_text__ = None;
                let mut expired_description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OldPasswordLabel => {
                            if old_password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldPasswordLabel"));
                            }
                            old_password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPasswordLabel => {
                            if new_password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPasswordLabel"));
                            }
                            new_password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPasswordConfirmLabel => {
                            if new_password_confirm_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPasswordConfirmLabel"));
                            }
                            new_password_confirm_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiredDescription => {
                            if expired_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiredDescription"));
                            }
                            expired_description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordChangeScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    old_password_label: old_password_label__.unwrap_or_default(),
                    new_password_label: new_password_label__.unwrap_or_default(),
                    new_password_confirm_label: new_password_confirm_label__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    expired_description: expired_description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordChangeScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordResetDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordResetDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordResetDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordResetDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordResetDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordResetDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordResetDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordResetDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.password_label.is_empty() {
            len += 1;
        }
        if !self.reset_link_text.is_empty() {
            len += 1;
        }
        if !self.back_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.min_length.is_empty() {
            len += 1;
        }
        if !self.has_uppercase.is_empty() {
            len += 1;
        }
        if !self.has_lowercase.is_empty() {
            len += 1;
        }
        if !self.has_number.is_empty() {
            len += 1;
        }
        if !self.has_symbol.is_empty() {
            len += 1;
        }
        if !self.confirmation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.password_label.is_empty() {
            struct_ser.serialize_field("passwordLabel", &self.password_label)?;
        }
        if !self.reset_link_text.is_empty() {
            struct_ser.serialize_field("resetLinkText", &self.reset_link_text)?;
        }
        if !self.back_button_text.is_empty() {
            struct_ser.serialize_field("backButtonText", &self.back_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.min_length.is_empty() {
            struct_ser.serialize_field("minLength", &self.min_length)?;
        }
        if !self.has_uppercase.is_empty() {
            struct_ser.serialize_field("hasUppercase", &self.has_uppercase)?;
        }
        if !self.has_lowercase.is_empty() {
            struct_ser.serialize_field("hasLowercase", &self.has_lowercase)?;
        }
        if !self.has_number.is_empty() {
            struct_ser.serialize_field("hasNumber", &self.has_number)?;
        }
        if !self.has_symbol.is_empty() {
            struct_ser.serialize_field("hasSymbol", &self.has_symbol)?;
        }
        if !self.confirmation.is_empty() {
            struct_ser.serialize_field("confirmation", &self.confirmation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "password_label",
            "passwordLabel",
            "reset_link_text",
            "resetLinkText",
            "back_button_text",
            "backButtonText",
            "next_button_text",
            "nextButtonText",
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
            "confirmation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            PasswordLabel,
            ResetLinkText,
            BackButtonText,
            NextButtonText,
            MinLength,
            HasUppercase,
            HasLowercase,
            HasNumber,
            HasSymbol,
            Confirmation,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "passwordLabel" | "password_label" => Ok(GeneratedField::PasswordLabel),
                            "resetLinkText" | "reset_link_text" => Ok(GeneratedField::ResetLinkText),
                            "backButtonText" | "back_button_text" => Ok(GeneratedField::BackButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "minLength" | "min_length" => Ok(GeneratedField::MinLength),
                            "hasUppercase" | "has_uppercase" => Ok(GeneratedField::HasUppercase),
                            "hasLowercase" | "has_lowercase" => Ok(GeneratedField::HasLowercase),
                            "hasNumber" | "has_number" => Ok(GeneratedField::HasNumber),
                            "hasSymbol" | "has_symbol" => Ok(GeneratedField::HasSymbol),
                            "confirmation" => Ok(GeneratedField::Confirmation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut password_label__ = None;
                let mut reset_link_text__ = None;
                let mut back_button_text__ = None;
                let mut next_button_text__ = None;
                let mut min_length__ = None;
                let mut has_uppercase__ = None;
                let mut has_lowercase__ = None;
                let mut has_number__ = None;
                let mut has_symbol__ = None;
                let mut confirmation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordLabel => {
                            if password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordLabel"));
                            }
                            password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResetLinkText => {
                            if reset_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetLinkText"));
                            }
                            reset_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackButtonText => {
                            if back_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backButtonText"));
                            }
                            back_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinLength => {
                            if min_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLength"));
                            }
                            min_length__ = Some(map_.next_value()?);
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
                        GeneratedField::Confirmation => {
                            if confirmation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("confirmation"));
                            }
                            confirmation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    password_label: password_label__.unwrap_or_default(),
                    reset_link_text: reset_link_text__.unwrap_or_default(),
                    back_button_text: back_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    min_length: min_length__.unwrap_or_default(),
                    has_uppercase: has_uppercase__.unwrap_or_default(),
                    has_lowercase: has_lowercase__.unwrap_or_default(),
                    has_number: has_number__.unwrap_or_default(),
                    has_symbol: has_symbol__.unwrap_or_default(),
                    confirmation: confirmation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordlessPromptScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.description_init.is_empty() {
            len += 1;
        }
        if !self.passwordless_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.skip_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordlessPromptScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.description_init.is_empty() {
            struct_ser.serialize_field("descriptionInit", &self.description_init)?;
        }
        if !self.passwordless_button_text.is_empty() {
            struct_ser.serialize_field("passwordlessButtonText", &self.passwordless_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.skip_button_text.is_empty() {
            struct_ser.serialize_field("skipButtonText", &self.skip_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordlessPromptScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "description_init",
            "descriptionInit",
            "passwordless_button_text",
            "passwordlessButtonText",
            "next_button_text",
            "nextButtonText",
            "skip_button_text",
            "skipButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            DescriptionInit,
            PasswordlessButtonText,
            NextButtonText,
            SkipButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "descriptionInit" | "description_init" => Ok(GeneratedField::DescriptionInit),
                            "passwordlessButtonText" | "passwordless_button_text" => Ok(GeneratedField::PasswordlessButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "skipButtonText" | "skip_button_text" => Ok(GeneratedField::SkipButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordlessPromptScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordlessPromptScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordlessPromptScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut description_init__ = None;
                let mut passwordless_button_text__ = None;
                let mut next_button_text__ = None;
                let mut skip_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptionInit => {
                            if description_init__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionInit"));
                            }
                            description_init__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordlessButtonText => {
                            if passwordless_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessButtonText"));
                            }
                            passwordless_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipButtonText => {
                            if skip_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipButtonText"));
                            }
                            skip_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordlessPromptScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    description_init: description_init__.unwrap_or_default(),
                    passwordless_button_text: passwordless_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    skip_button_text: skip_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordlessPromptScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordlessRegistrationDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.description_close.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordlessRegistrationDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.description_close.is_empty() {
            struct_ser.serialize_field("descriptionClose", &self.description_close)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordlessRegistrationDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "next_button_text",
            "nextButtonText",
            "cancel_button_text",
            "cancelButtonText",
            "description_close",
            "descriptionClose",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NextButtonText,
            CancelButtonText,
            DescriptionClose,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "descriptionClose" | "description_close" => Ok(GeneratedField::DescriptionClose),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordlessRegistrationDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordlessRegistrationDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordlessRegistrationDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut next_button_text__ = None;
                let mut cancel_button_text__ = None;
                let mut description_close__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptionClose => {
                            if description_close__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionClose"));
                            }
                            description_close__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordlessRegistrationDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    description_close: description_close__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordlessRegistrationDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordlessRegistrationScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.token_name_label.is_empty() {
            len += 1;
        }
        if !self.not_supported.is_empty() {
            len += 1;
        }
        if !self.register_token_button_text.is_empty() {
            len += 1;
        }
        if !self.error_retry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordlessRegistrationScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.token_name_label.is_empty() {
            struct_ser.serialize_field("tokenNameLabel", &self.token_name_label)?;
        }
        if !self.not_supported.is_empty() {
            struct_ser.serialize_field("notSupported", &self.not_supported)?;
        }
        if !self.register_token_button_text.is_empty() {
            struct_ser.serialize_field("registerTokenButtonText", &self.register_token_button_text)?;
        }
        if !self.error_retry.is_empty() {
            struct_ser.serialize_field("errorRetry", &self.error_retry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordlessRegistrationScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "token_name_label",
            "tokenNameLabel",
            "not_supported",
            "notSupported",
            "register_token_button_text",
            "registerTokenButtonText",
            "error_retry",
            "errorRetry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            TokenNameLabel,
            NotSupported,
            RegisterTokenButtonText,
            ErrorRetry,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "tokenNameLabel" | "token_name_label" => Ok(GeneratedField::TokenNameLabel),
                            "notSupported" | "not_supported" => Ok(GeneratedField::NotSupported),
                            "registerTokenButtonText" | "register_token_button_text" => Ok(GeneratedField::RegisterTokenButtonText),
                            "errorRetry" | "error_retry" => Ok(GeneratedField::ErrorRetry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordlessRegistrationScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordlessRegistrationScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordlessRegistrationScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut token_name_label__ = None;
                let mut not_supported__ = None;
                let mut register_token_button_text__ = None;
                let mut error_retry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenNameLabel => {
                            if token_name_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenNameLabel"));
                            }
                            token_name_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotSupported => {
                            if not_supported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notSupported"));
                            }
                            not_supported__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RegisterTokenButtonText => {
                            if register_token_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerTokenButtonText"));
                            }
                            register_token_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorRetry => {
                            if error_retry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorRetry"));
                            }
                            error_retry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordlessRegistrationScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    token_name_label: token_name_label__.unwrap_or_default(),
                    not_supported: not_supported__.unwrap_or_default(),
                    register_token_button_text: register_token_button_text__.unwrap_or_default(),
                    error_retry: error_retry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordlessRegistrationScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordlessScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.login_with_pw_button_text.is_empty() {
            len += 1;
        }
        if !self.validate_token_button_text.is_empty() {
            len += 1;
        }
        if !self.not_supported.is_empty() {
            len += 1;
        }
        if !self.error_retry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.PasswordlessScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.login_with_pw_button_text.is_empty() {
            struct_ser.serialize_field("loginWithPwButtonText", &self.login_with_pw_button_text)?;
        }
        if !self.validate_token_button_text.is_empty() {
            struct_ser.serialize_field("validateTokenButtonText", &self.validate_token_button_text)?;
        }
        if !self.not_supported.is_empty() {
            struct_ser.serialize_field("notSupported", &self.not_supported)?;
        }
        if !self.error_retry.is_empty() {
            struct_ser.serialize_field("errorRetry", &self.error_retry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordlessScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "login_with_pw_button_text",
            "loginWithPwButtonText",
            "validate_token_button_text",
            "validateTokenButtonText",
            "not_supported",
            "notSupported",
            "error_retry",
            "errorRetry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            LoginWithPwButtonText,
            ValidateTokenButtonText,
            NotSupported,
            ErrorRetry,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "loginWithPwButtonText" | "login_with_pw_button_text" => Ok(GeneratedField::LoginWithPwButtonText),
                            "validateTokenButtonText" | "validate_token_button_text" => Ok(GeneratedField::ValidateTokenButtonText),
                            "notSupported" | "not_supported" => Ok(GeneratedField::NotSupported),
                            "errorRetry" | "error_retry" => Ok(GeneratedField::ErrorRetry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordlessScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.PasswordlessScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordlessScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut login_with_pw_button_text__ = None;
                let mut validate_token_button_text__ = None;
                let mut not_supported__ = None;
                let mut error_retry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginWithPwButtonText => {
                            if login_with_pw_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginWithPwButtonText"));
                            }
                            login_with_pw_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidateTokenButtonText => {
                            if validate_token_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateTokenButtonText"));
                            }
                            validate_token_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotSupported => {
                            if not_supported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notSupported"));
                            }
                            not_supported__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorRetry => {
                            if error_retry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorRetry"));
                            }
                            error_retry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasswordlessScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    login_with_pw_button_text: login_with_pw_button_text__.unwrap_or_default(),
                    validate_token_button_text: validate_token_button_text__.unwrap_or_default(),
                    not_supported: not_supported__.unwrap_or_default(),
                    error_retry: error_retry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.PasswordlessScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegistrationOptionScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.user_name_button_text.is_empty() {
            len += 1;
        }
        if !self.external_login_description.is_empty() {
            len += 1;
        }
        if !self.login_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.RegistrationOptionScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.user_name_button_text.is_empty() {
            struct_ser.serialize_field("userNameButtonText", &self.user_name_button_text)?;
        }
        if !self.external_login_description.is_empty() {
            struct_ser.serialize_field("externalLoginDescription", &self.external_login_description)?;
        }
        if !self.login_button_text.is_empty() {
            struct_ser.serialize_field("loginButtonText", &self.login_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegistrationOptionScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "user_name_button_text",
            "userNameButtonText",
            "external_login_description",
            "externalLoginDescription",
            "login_button_text",
            "loginButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            UserNameButtonText,
            ExternalLoginDescription,
            LoginButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "userNameButtonText" | "user_name_button_text" => Ok(GeneratedField::UserNameButtonText),
                            "externalLoginDescription" | "external_login_description" => Ok(GeneratedField::ExternalLoginDescription),
                            "loginButtonText" | "login_button_text" => Ok(GeneratedField::LoginButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegistrationOptionScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.RegistrationOptionScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegistrationOptionScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut user_name_button_text__ = None;
                let mut external_login_description__ = None;
                let mut login_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNameButtonText => {
                            if user_name_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNameButtonText"));
                            }
                            user_name_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExternalLoginDescription => {
                            if external_login_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalLoginDescription"));
                            }
                            external_login_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginButtonText => {
                            if login_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginButtonText"));
                            }
                            login_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegistrationOptionScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    user_name_button_text: user_name_button_text__.unwrap_or_default(),
                    external_login_description: external_login_description__.unwrap_or_default(),
                    login_button_text: login_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.RegistrationOptionScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegistrationOrgScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.orgname_label.is_empty() {
            len += 1;
        }
        if !self.firstname_label.is_empty() {
            len += 1;
        }
        if !self.lastname_label.is_empty() {
            len += 1;
        }
        if !self.username_label.is_empty() {
            len += 1;
        }
        if !self.email_label.is_empty() {
            len += 1;
        }
        if !self.password_label.is_empty() {
            len += 1;
        }
        if !self.password_confirm_label.is_empty() {
            len += 1;
        }
        if !self.tos_and_privacy_label.is_empty() {
            len += 1;
        }
        if !self.tos_confirm.is_empty() {
            len += 1;
        }
        if !self.tos_link_text.is_empty() {
            len += 1;
        }
        if !self.privacy_confirm.is_empty() {
            len += 1;
        }
        if !self.privacy_link_text.is_empty() {
            len += 1;
        }
        if !self.save_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.RegistrationOrgScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.orgname_label.is_empty() {
            struct_ser.serialize_field("orgnameLabel", &self.orgname_label)?;
        }
        if !self.firstname_label.is_empty() {
            struct_ser.serialize_field("firstnameLabel", &self.firstname_label)?;
        }
        if !self.lastname_label.is_empty() {
            struct_ser.serialize_field("lastnameLabel", &self.lastname_label)?;
        }
        if !self.username_label.is_empty() {
            struct_ser.serialize_field("usernameLabel", &self.username_label)?;
        }
        if !self.email_label.is_empty() {
            struct_ser.serialize_field("emailLabel", &self.email_label)?;
        }
        if !self.password_label.is_empty() {
            struct_ser.serialize_field("passwordLabel", &self.password_label)?;
        }
        if !self.password_confirm_label.is_empty() {
            struct_ser.serialize_field("passwordConfirmLabel", &self.password_confirm_label)?;
        }
        if !self.tos_and_privacy_label.is_empty() {
            struct_ser.serialize_field("tosAndPrivacyLabel", &self.tos_and_privacy_label)?;
        }
        if !self.tos_confirm.is_empty() {
            struct_ser.serialize_field("tosConfirm", &self.tos_confirm)?;
        }
        if !self.tos_link_text.is_empty() {
            struct_ser.serialize_field("tosLinkText", &self.tos_link_text)?;
        }
        if !self.privacy_confirm.is_empty() {
            struct_ser.serialize_field("privacyConfirm", &self.privacy_confirm)?;
        }
        if !self.privacy_link_text.is_empty() {
            struct_ser.serialize_field("privacyLinkText", &self.privacy_link_text)?;
        }
        if !self.save_button_text.is_empty() {
            struct_ser.serialize_field("saveButtonText", &self.save_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegistrationOrgScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "orgname_label",
            "orgnameLabel",
            "firstname_label",
            "firstnameLabel",
            "lastname_label",
            "lastnameLabel",
            "username_label",
            "usernameLabel",
            "email_label",
            "emailLabel",
            "password_label",
            "passwordLabel",
            "password_confirm_label",
            "passwordConfirmLabel",
            "tos_and_privacy_label",
            "tosAndPrivacyLabel",
            "tos_confirm",
            "tosConfirm",
            "tos_link_text",
            "tosLinkText",
            "privacy_confirm",
            "privacyConfirm",
            "privacy_link_text",
            "privacyLinkText",
            "save_button_text",
            "saveButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            OrgnameLabel,
            FirstnameLabel,
            LastnameLabel,
            UsernameLabel,
            EmailLabel,
            PasswordLabel,
            PasswordConfirmLabel,
            TosAndPrivacyLabel,
            TosConfirm,
            TosLinkText,
            PrivacyConfirm,
            PrivacyLinkText,
            SaveButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "orgnameLabel" | "orgname_label" => Ok(GeneratedField::OrgnameLabel),
                            "firstnameLabel" | "firstname_label" => Ok(GeneratedField::FirstnameLabel),
                            "lastnameLabel" | "lastname_label" => Ok(GeneratedField::LastnameLabel),
                            "usernameLabel" | "username_label" => Ok(GeneratedField::UsernameLabel),
                            "emailLabel" | "email_label" => Ok(GeneratedField::EmailLabel),
                            "passwordLabel" | "password_label" => Ok(GeneratedField::PasswordLabel),
                            "passwordConfirmLabel" | "password_confirm_label" => Ok(GeneratedField::PasswordConfirmLabel),
                            "tosAndPrivacyLabel" | "tos_and_privacy_label" => Ok(GeneratedField::TosAndPrivacyLabel),
                            "tosConfirm" | "tos_confirm" => Ok(GeneratedField::TosConfirm),
                            "tosLinkText" | "tos_link_text" => Ok(GeneratedField::TosLinkText),
                            "privacyConfirm" | "privacy_confirm" => Ok(GeneratedField::PrivacyConfirm),
                            "privacyLinkText" | "privacy_link_text" => Ok(GeneratedField::PrivacyLinkText),
                            "saveButtonText" | "save_button_text" => Ok(GeneratedField::SaveButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegistrationOrgScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.RegistrationOrgScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegistrationOrgScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut orgname_label__ = None;
                let mut firstname_label__ = None;
                let mut lastname_label__ = None;
                let mut username_label__ = None;
                let mut email_label__ = None;
                let mut password_label__ = None;
                let mut password_confirm_label__ = None;
                let mut tos_and_privacy_label__ = None;
                let mut tos_confirm__ = None;
                let mut tos_link_text__ = None;
                let mut privacy_confirm__ = None;
                let mut privacy_link_text__ = None;
                let mut save_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgnameLabel => {
                            if orgname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgnameLabel"));
                            }
                            orgname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstnameLabel => {
                            if firstname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstnameLabel"));
                            }
                            firstname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastnameLabel => {
                            if lastname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastnameLabel"));
                            }
                            lastname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsernameLabel => {
                            if username_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameLabel"));
                            }
                            username_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailLabel => {
                            if email_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailLabel"));
                            }
                            email_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordLabel => {
                            if password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordLabel"));
                            }
                            password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordConfirmLabel => {
                            if password_confirm_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordConfirmLabel"));
                            }
                            password_confirm_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosAndPrivacyLabel => {
                            if tos_and_privacy_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosAndPrivacyLabel"));
                            }
                            tos_and_privacy_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosConfirm => {
                            if tos_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosConfirm"));
                            }
                            tos_confirm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosLinkText => {
                            if tos_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosLinkText"));
                            }
                            tos_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyConfirm => {
                            if privacy_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyConfirm"));
                            }
                            privacy_confirm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyLinkText => {
                            if privacy_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyLinkText"));
                            }
                            privacy_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SaveButtonText => {
                            if save_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveButtonText"));
                            }
                            save_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegistrationOrgScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    orgname_label: orgname_label__.unwrap_or_default(),
                    firstname_label: firstname_label__.unwrap_or_default(),
                    lastname_label: lastname_label__.unwrap_or_default(),
                    username_label: username_label__.unwrap_or_default(),
                    email_label: email_label__.unwrap_or_default(),
                    password_label: password_label__.unwrap_or_default(),
                    password_confirm_label: password_confirm_label__.unwrap_or_default(),
                    tos_and_privacy_label: tos_and_privacy_label__.unwrap_or_default(),
                    tos_confirm: tos_confirm__.unwrap_or_default(),
                    tos_link_text: tos_link_text__.unwrap_or_default(),
                    privacy_confirm: privacy_confirm__.unwrap_or_default(),
                    privacy_link_text: privacy_link_text__.unwrap_or_default(),
                    save_button_text: save_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.RegistrationOrgScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegistrationUserScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.description_org_register.is_empty() {
            len += 1;
        }
        if !self.firstname_label.is_empty() {
            len += 1;
        }
        if !self.lastname_label.is_empty() {
            len += 1;
        }
        if !self.email_label.is_empty() {
            len += 1;
        }
        if !self.username_label.is_empty() {
            len += 1;
        }
        if !self.language_label.is_empty() {
            len += 1;
        }
        if !self.gender_label.is_empty() {
            len += 1;
        }
        if !self.password_label.is_empty() {
            len += 1;
        }
        if !self.password_confirm_label.is_empty() {
            len += 1;
        }
        if !self.tos_and_privacy_label.is_empty() {
            len += 1;
        }
        if !self.tos_confirm.is_empty() {
            len += 1;
        }
        if !self.tos_link_text.is_empty() {
            len += 1;
        }
        if !self.privacy_confirm.is_empty() {
            len += 1;
        }
        if !self.privacy_link_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        if !self.back_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.RegistrationUserScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.description_org_register.is_empty() {
            struct_ser.serialize_field("descriptionOrgRegister", &self.description_org_register)?;
        }
        if !self.firstname_label.is_empty() {
            struct_ser.serialize_field("firstnameLabel", &self.firstname_label)?;
        }
        if !self.lastname_label.is_empty() {
            struct_ser.serialize_field("lastnameLabel", &self.lastname_label)?;
        }
        if !self.email_label.is_empty() {
            struct_ser.serialize_field("emailLabel", &self.email_label)?;
        }
        if !self.username_label.is_empty() {
            struct_ser.serialize_field("usernameLabel", &self.username_label)?;
        }
        if !self.language_label.is_empty() {
            struct_ser.serialize_field("languageLabel", &self.language_label)?;
        }
        if !self.gender_label.is_empty() {
            struct_ser.serialize_field("genderLabel", &self.gender_label)?;
        }
        if !self.password_label.is_empty() {
            struct_ser.serialize_field("passwordLabel", &self.password_label)?;
        }
        if !self.password_confirm_label.is_empty() {
            struct_ser.serialize_field("passwordConfirmLabel", &self.password_confirm_label)?;
        }
        if !self.tos_and_privacy_label.is_empty() {
            struct_ser.serialize_field("tosAndPrivacyLabel", &self.tos_and_privacy_label)?;
        }
        if !self.tos_confirm.is_empty() {
            struct_ser.serialize_field("tosConfirm", &self.tos_confirm)?;
        }
        if !self.tos_link_text.is_empty() {
            struct_ser.serialize_field("tosLinkText", &self.tos_link_text)?;
        }
        if !self.privacy_confirm.is_empty() {
            struct_ser.serialize_field("privacyConfirm", &self.privacy_confirm)?;
        }
        if !self.privacy_link_text.is_empty() {
            struct_ser.serialize_field("privacyLinkText", &self.privacy_link_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        if !self.back_button_text.is_empty() {
            struct_ser.serialize_field("backButtonText", &self.back_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegistrationUserScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "description_org_register",
            "descriptionOrgRegister",
            "firstname_label",
            "firstnameLabel",
            "lastname_label",
            "lastnameLabel",
            "email_label",
            "emailLabel",
            "username_label",
            "usernameLabel",
            "language_label",
            "languageLabel",
            "gender_label",
            "genderLabel",
            "password_label",
            "passwordLabel",
            "password_confirm_label",
            "passwordConfirmLabel",
            "tos_and_privacy_label",
            "tosAndPrivacyLabel",
            "tos_confirm",
            "tosConfirm",
            "tos_link_text",
            "tosLinkText",
            "privacy_confirm",
            "privacyConfirm",
            "privacy_link_text",
            "privacyLinkText",
            "next_button_text",
            "nextButtonText",
            "back_button_text",
            "backButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            DescriptionOrgRegister,
            FirstnameLabel,
            LastnameLabel,
            EmailLabel,
            UsernameLabel,
            LanguageLabel,
            GenderLabel,
            PasswordLabel,
            PasswordConfirmLabel,
            TosAndPrivacyLabel,
            TosConfirm,
            TosLinkText,
            PrivacyConfirm,
            PrivacyLinkText,
            NextButtonText,
            BackButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "descriptionOrgRegister" | "description_org_register" => Ok(GeneratedField::DescriptionOrgRegister),
                            "firstnameLabel" | "firstname_label" => Ok(GeneratedField::FirstnameLabel),
                            "lastnameLabel" | "lastname_label" => Ok(GeneratedField::LastnameLabel),
                            "emailLabel" | "email_label" => Ok(GeneratedField::EmailLabel),
                            "usernameLabel" | "username_label" => Ok(GeneratedField::UsernameLabel),
                            "languageLabel" | "language_label" => Ok(GeneratedField::LanguageLabel),
                            "genderLabel" | "gender_label" => Ok(GeneratedField::GenderLabel),
                            "passwordLabel" | "password_label" => Ok(GeneratedField::PasswordLabel),
                            "passwordConfirmLabel" | "password_confirm_label" => Ok(GeneratedField::PasswordConfirmLabel),
                            "tosAndPrivacyLabel" | "tos_and_privacy_label" => Ok(GeneratedField::TosAndPrivacyLabel),
                            "tosConfirm" | "tos_confirm" => Ok(GeneratedField::TosConfirm),
                            "tosLinkText" | "tos_link_text" => Ok(GeneratedField::TosLinkText),
                            "privacyConfirm" | "privacy_confirm" => Ok(GeneratedField::PrivacyConfirm),
                            "privacyLinkText" | "privacy_link_text" => Ok(GeneratedField::PrivacyLinkText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            "backButtonText" | "back_button_text" => Ok(GeneratedField::BackButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegistrationUserScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.RegistrationUserScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegistrationUserScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut description_org_register__ = None;
                let mut firstname_label__ = None;
                let mut lastname_label__ = None;
                let mut email_label__ = None;
                let mut username_label__ = None;
                let mut language_label__ = None;
                let mut gender_label__ = None;
                let mut password_label__ = None;
                let mut password_confirm_label__ = None;
                let mut tos_and_privacy_label__ = None;
                let mut tos_confirm__ = None;
                let mut tos_link_text__ = None;
                let mut privacy_confirm__ = None;
                let mut privacy_link_text__ = None;
                let mut next_button_text__ = None;
                let mut back_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptionOrgRegister => {
                            if description_org_register__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionOrgRegister"));
                            }
                            description_org_register__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstnameLabel => {
                            if firstname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstnameLabel"));
                            }
                            firstname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastnameLabel => {
                            if lastname_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastnameLabel"));
                            }
                            lastname_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EmailLabel => {
                            if email_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailLabel"));
                            }
                            email_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsernameLabel => {
                            if username_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameLabel"));
                            }
                            username_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LanguageLabel => {
                            if language_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("languageLabel"));
                            }
                            language_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GenderLabel => {
                            if gender_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genderLabel"));
                            }
                            gender_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordLabel => {
                            if password_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordLabel"));
                            }
                            password_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordConfirmLabel => {
                            if password_confirm_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordConfirmLabel"));
                            }
                            password_confirm_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosAndPrivacyLabel => {
                            if tos_and_privacy_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosAndPrivacyLabel"));
                            }
                            tos_and_privacy_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosConfirm => {
                            if tos_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosConfirm"));
                            }
                            tos_confirm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TosLinkText => {
                            if tos_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tosLinkText"));
                            }
                            tos_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyConfirm => {
                            if privacy_confirm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyConfirm"));
                            }
                            privacy_confirm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivacyLinkText => {
                            if privacy_link_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyLinkText"));
                            }
                            privacy_link_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackButtonText => {
                            if back_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backButtonText"));
                            }
                            back_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegistrationUserScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    description_org_register: description_org_register__.unwrap_or_default(),
                    firstname_label: firstname_label__.unwrap_or_default(),
                    lastname_label: lastname_label__.unwrap_or_default(),
                    email_label: email_label__.unwrap_or_default(),
                    username_label: username_label__.unwrap_or_default(),
                    language_label: language_label__.unwrap_or_default(),
                    gender_label: gender_label__.unwrap_or_default(),
                    password_label: password_label__.unwrap_or_default(),
                    password_confirm_label: password_confirm_label__.unwrap_or_default(),
                    tos_and_privacy_label: tos_and_privacy_label__.unwrap_or_default(),
                    tos_confirm: tos_confirm__.unwrap_or_default(),
                    tos_link_text: tos_link_text__.unwrap_or_default(),
                    privacy_confirm: privacy_confirm__.unwrap_or_default(),
                    privacy_link_text: privacy_link_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                    back_button_text: back_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.RegistrationUserScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SelectAccountScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.title_linking_process.is_empty() {
            len += 1;
        }
        if !self.description_linking_process.is_empty() {
            len += 1;
        }
        if !self.other_user.is_empty() {
            len += 1;
        }
        if !self.session_state_active.is_empty() {
            len += 1;
        }
        if !self.session_state_inactive.is_empty() {
            len += 1;
        }
        if !self.user_must_be_member_of_org.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.SelectAccountScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.title_linking_process.is_empty() {
            struct_ser.serialize_field("titleLinkingProcess", &self.title_linking_process)?;
        }
        if !self.description_linking_process.is_empty() {
            struct_ser.serialize_field("descriptionLinkingProcess", &self.description_linking_process)?;
        }
        if !self.other_user.is_empty() {
            struct_ser.serialize_field("otherUser", &self.other_user)?;
        }
        if !self.session_state_active.is_empty() {
            struct_ser.serialize_field("sessionStateActive", &self.session_state_active)?;
        }
        if !self.session_state_inactive.is_empty() {
            struct_ser.serialize_field("sessionStateInactive", &self.session_state_inactive)?;
        }
        if !self.user_must_be_member_of_org.is_empty() {
            struct_ser.serialize_field("userMustBeMemberOfOrg", &self.user_must_be_member_of_org)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SelectAccountScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "title_linking_process",
            "titleLinkingProcess",
            "description_linking_process",
            "descriptionLinkingProcess",
            "other_user",
            "otherUser",
            "session_state_active",
            "sessionStateActive",
            "session_state_inactive",
            "sessionStateInactive",
            "user_must_be_member_of_org",
            "userMustBeMemberOfOrg",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            TitleLinkingProcess,
            DescriptionLinkingProcess,
            OtherUser,
            SessionStateActive,
            SessionStateInactive,
            UserMustBeMemberOfOrg,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "titleLinkingProcess" | "title_linking_process" => Ok(GeneratedField::TitleLinkingProcess),
                            "descriptionLinkingProcess" | "description_linking_process" => Ok(GeneratedField::DescriptionLinkingProcess),
                            "otherUser" | "other_user" => Ok(GeneratedField::OtherUser),
                            "sessionStateActive" | "session_state_active" => Ok(GeneratedField::SessionStateActive),
                            "sessionStateInactive" | "session_state_inactive" => Ok(GeneratedField::SessionStateInactive),
                            "userMustBeMemberOfOrg" | "user_must_be_member_of_org" => Ok(GeneratedField::UserMustBeMemberOfOrg),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SelectAccountScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.SelectAccountScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SelectAccountScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut title_linking_process__ = None;
                let mut description_linking_process__ = None;
                let mut other_user__ = None;
                let mut session_state_active__ = None;
                let mut session_state_inactive__ = None;
                let mut user_must_be_member_of_org__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TitleLinkingProcess => {
                            if title_linking_process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("titleLinkingProcess"));
                            }
                            title_linking_process__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DescriptionLinkingProcess => {
                            if description_linking_process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptionLinkingProcess"));
                            }
                            description_linking_process__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtherUser => {
                            if other_user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherUser"));
                            }
                            other_user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionStateActive => {
                            if session_state_active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionStateActive"));
                            }
                            session_state_active__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionStateInactive => {
                            if session_state_inactive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionStateInactive"));
                            }
                            session_state_inactive__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserMustBeMemberOfOrg => {
                            if user_must_be_member_of_org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userMustBeMemberOfOrg"));
                            }
                            user_must_be_member_of_org__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SelectAccountScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    title_linking_process: title_linking_process__.unwrap_or_default(),
                    description_linking_process: description_linking_process__.unwrap_or_default(),
                    other_user: other_user__.unwrap_or_default(),
                    session_state_active: session_state_active__.unwrap_or_default(),
                    session_state_inactive: session_state_inactive__.unwrap_or_default(),
                    user_must_be_member_of_org: user_must_be_member_of_org__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.SelectAccountScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SuccessLoginScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.auto_redirect_description.is_empty() {
            len += 1;
        }
        if !self.redirected_description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.SuccessLoginScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.auto_redirect_description.is_empty() {
            struct_ser.serialize_field("autoRedirectDescription", &self.auto_redirect_description)?;
        }
        if !self.redirected_description.is_empty() {
            struct_ser.serialize_field("redirectedDescription", &self.redirected_description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SuccessLoginScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "auto_redirect_description",
            "autoRedirectDescription",
            "redirected_description",
            "redirectedDescription",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            AutoRedirectDescription,
            RedirectedDescription,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "autoRedirectDescription" | "auto_redirect_description" => Ok(GeneratedField::AutoRedirectDescription),
                            "redirectedDescription" | "redirected_description" => Ok(GeneratedField::RedirectedDescription),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SuccessLoginScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.SuccessLoginScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuccessLoginScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut auto_redirect_description__ = None;
                let mut redirected_description__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutoRedirectDescription => {
                            if auto_redirect_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoRedirectDescription"));
                            }
                            auto_redirect_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RedirectedDescription => {
                            if redirected_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectedDescription"));
                            }
                            redirected_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SuccessLoginScreenText {
                    title: title__.unwrap_or_default(),
                    auto_redirect_description: auto_redirect_description__.unwrap_or_default(),
                    redirected_description: redirected_description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.SuccessLoginScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsernameChangeDoneScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.UsernameChangeDoneScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsernameChangeDoneScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsernameChangeDoneScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.UsernameChangeDoneScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsernameChangeDoneScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UsernameChangeDoneScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.UsernameChangeDoneScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsernameChangeScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.username_label.is_empty() {
            len += 1;
        }
        if !self.cancel_button_text.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.UsernameChangeScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.username_label.is_empty() {
            struct_ser.serialize_field("usernameLabel", &self.username_label)?;
        }
        if !self.cancel_button_text.is_empty() {
            struct_ser.serialize_field("cancelButtonText", &self.cancel_button_text)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsernameChangeScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "username_label",
            "usernameLabel",
            "cancel_button_text",
            "cancelButtonText",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            UsernameLabel,
            CancelButtonText,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "usernameLabel" | "username_label" => Ok(GeneratedField::UsernameLabel),
                            "cancelButtonText" | "cancel_button_text" => Ok(GeneratedField::CancelButtonText),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsernameChangeScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.UsernameChangeScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsernameChangeScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut username_label__ = None;
                let mut cancel_button_text__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsernameLabel => {
                            if username_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernameLabel"));
                            }
                            username_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CancelButtonText => {
                            if cancel_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelButtonText"));
                            }
                            cancel_button_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UsernameChangeScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    username_label: username_label__.unwrap_or_default(),
                    cancel_button_text: cancel_button_text__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.UsernameChangeScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyMfaotpScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.code_label.is_empty() {
            len += 1;
        }
        if !self.next_button_text.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.VerifyMFAOTPScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.code_label.is_empty() {
            struct_ser.serialize_field("codeLabel", &self.code_label)?;
        }
        if !self.next_button_text.is_empty() {
            struct_ser.serialize_field("nextButtonText", &self.next_button_text)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyMfaotpScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "code_label",
            "codeLabel",
            "next_button_text",
            "nextButtonText",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CodeLabel,
            NextButtonText,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "codeLabel" | "code_label" => Ok(GeneratedField::CodeLabel),
                            "nextButtonText" | "next_button_text" => Ok(GeneratedField::NextButtonText),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VerifyMfaotpScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.VerifyMFAOTPScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyMfaotpScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut code_label__ = None;
                let mut next_button_text__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CodeLabel => {
                            if code_label__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codeLabel"));
                            }
                            code_label__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextButtonText => {
                            if next_button_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextButtonText"));
                            }
                            next_button_text__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyMfaotpScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    code_label: code_label__.unwrap_or_default(),
                    next_button_text: next_button_text__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.VerifyMFAOTPScreenText", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyMfau2fScreenText {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.validate_token_text.is_empty() {
            len += 1;
        }
        if !self.not_supported.is_empty() {
            len += 1;
        }
        if !self.error_retry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.text.v1.VerifyMFAU2FScreenText", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.validate_token_text.is_empty() {
            struct_ser.serialize_field("validateTokenText", &self.validate_token_text)?;
        }
        if !self.not_supported.is_empty() {
            struct_ser.serialize_field("notSupported", &self.not_supported)?;
        }
        if !self.error_retry.is_empty() {
            struct_ser.serialize_field("errorRetry", &self.error_retry)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyMfau2fScreenText {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "validate_token_text",
            "validateTokenText",
            "not_supported",
            "notSupported",
            "error_retry",
            "errorRetry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            ValidateTokenText,
            NotSupported,
            ErrorRetry,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "validateTokenText" | "validate_token_text" => Ok(GeneratedField::ValidateTokenText),
                            "notSupported" | "not_supported" => Ok(GeneratedField::NotSupported),
                            "errorRetry" | "error_retry" => Ok(GeneratedField::ErrorRetry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VerifyMfau2fScreenText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.text.v1.VerifyMFAU2FScreenText")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyMfau2fScreenText, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut validate_token_text__ = None;
                let mut not_supported__ = None;
                let mut error_retry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidateTokenText => {
                            if validate_token_text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateTokenText"));
                            }
                            validate_token_text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotSupported => {
                            if not_supported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notSupported"));
                            }
                            not_supported__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorRetry => {
                            if error_retry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorRetry"));
                            }
                            error_retry__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyMfau2fScreenText {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    validate_token_text: validate_token_text__.unwrap_or_default(),
                    not_supported: not_supported__.unwrap_or_default(),
                    error_retry: error_retry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.text.v1.VerifyMFAU2FScreenText", FIELDS, GeneratedVisitor)
    }
}
