// @generated
impl serde::Serialize for Challenges {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.web_auth_n.is_some() {
            len += 1;
        }
        if self.otp_sms.is_some() {
            len += 1;
        }
        if self.otp_email.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.Challenges", len)?;
        if let Some(v) = self.web_auth_n.as_ref() {
            struct_ser.serialize_field("webAuthN", v)?;
        }
        if let Some(v) = self.otp_sms.as_ref() {
            struct_ser.serialize_field("otpSms", v)?;
        }
        if let Some(v) = self.otp_email.as_ref() {
            struct_ser.serialize_field("otpEmail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Challenges {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "web_auth_n",
            "webAuthN",
            "otp_sms",
            "otpSms",
            "otp_email",
            "otpEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebAuthN,
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
                            "webAuthN" | "web_auth_n" => Ok(GeneratedField::WebAuthN),
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
            type Value = Challenges;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.Challenges")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Challenges, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut web_auth_n__ = None;
                let mut otp_sms__ = None;
                let mut otp_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebAuthN => {
                            if web_auth_n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webAuthN"));
                            }
                            web_auth_n__ = map_.next_value()?;
                        }
                        GeneratedField::OtpSms => {
                            if otp_sms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpSms"));
                            }
                            otp_sms__ = map_.next_value()?;
                        }
                        GeneratedField::OtpEmail => {
                            if otp_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpEmail"));
                            }
                            otp_email__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Challenges {
                    web_auth_n: web_auth_n__,
                    otp_sms: otp_sms__,
                    otp_email: otp_email__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.Challenges", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for challenges::WebAuthN {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.public_key_credential_request_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.Challenges.WebAuthN", len)?;
        if let Some(v) = self.public_key_credential_request_options.as_ref() {
            struct_ser.serialize_field("publicKeyCredentialRequestOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for challenges::WebAuthN {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_key_credential_request_options",
            "publicKeyCredentialRequestOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKeyCredentialRequestOptions,
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
                            "publicKeyCredentialRequestOptions" | "public_key_credential_request_options" => Ok(GeneratedField::PublicKeyCredentialRequestOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = challenges::WebAuthN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.Challenges.WebAuthN")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<challenges::WebAuthN, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key_credential_request_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PublicKeyCredentialRequestOptions => {
                            if public_key_credential_request_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCredentialRequestOptions"));
                            }
                            public_key_credential_request_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(challenges::WebAuthN {
                    public_key_credential_request_options: public_key_credential_request_options__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.Challenges.WebAuthN", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckIdpIntent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.idp_intent_id.is_empty() {
            len += 1;
        }
        if !self.idp_intent_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CheckIDPIntent", len)?;
        if !self.idp_intent_id.is_empty() {
            struct_ser.serialize_field("idpIntentId", &self.idp_intent_id)?;
        }
        if !self.idp_intent_token.is_empty() {
            struct_ser.serialize_field("idpIntentToken", &self.idp_intent_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckIdpIntent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_intent_id",
            "idpIntentId",
            "idp_intent_token",
            "idpIntentToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpIntentId,
            IdpIntentToken,
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
                            "idpIntentId" | "idp_intent_id" => Ok(GeneratedField::IdpIntentId),
                            "idpIntentToken" | "idp_intent_token" => Ok(GeneratedField::IdpIntentToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckIdpIntent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CheckIDPIntent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckIdpIntent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_intent_id__ = None;
                let mut idp_intent_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpIntentId => {
                            if idp_intent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpIntentId"));
                            }
                            idp_intent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpIntentToken => {
                            if idp_intent_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpIntentToken"));
                            }
                            idp_intent_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckIdpIntent {
                    idp_intent_id: idp_intent_id__.unwrap_or_default(),
                    idp_intent_token: idp_intent_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CheckIDPIntent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckOtp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CheckOTP", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckOtp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckOtp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CheckOTP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckOtp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckOtp {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CheckOTP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPassword {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CheckPassword", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPassword {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
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
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPassword;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CheckPassword")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckPassword, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckPassword {
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CheckPassword", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckTotp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CheckTOTP", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckTotp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckTotp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CheckTOTP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckTotp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckTotp {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CheckTOTP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CheckUser", len)?;
        if let Some(v) = self.search.as_ref() {
            match v {
                check_user::Search::UserId(v) => {
                    struct_ser.serialize_field("userId", v)?;
                }
                check_user::Search::LoginName(v) => {
                    struct_ser.serialize_field("loginName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "login_name",
            "loginName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            LoginName,
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
                            "loginName" | "login_name" => Ok(GeneratedField::LoginName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CheckUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            search__ = map_.next_value::<::std::option::Option<_>>()?.map(check_user::Search::UserId);
                        }
                        GeneratedField::LoginName => {
                            if search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginName"));
                            }
                            search__ = map_.next_value::<::std::option::Option<_>>()?.map(check_user::Search::LoginName);
                        }
                    }
                }
                Ok(CheckUser {
                    search: search__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CheckUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckWebAuthN {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credential_assertion_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CheckWebAuthN", len)?;
        if let Some(v) = self.credential_assertion_data.as_ref() {
            struct_ser.serialize_field("credentialAssertionData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckWebAuthN {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credential_assertion_data",
            "credentialAssertionData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CredentialAssertionData,
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
                            "credentialAssertionData" | "credential_assertion_data" => Ok(GeneratedField::CredentialAssertionData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckWebAuthN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CheckWebAuthN")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckWebAuthN, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential_assertion_data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CredentialAssertionData => {
                            if credential_assertion_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentialAssertionData"));
                            }
                            credential_assertion_data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckWebAuthN {
                    credential_assertion_data: credential_assertion_data__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CheckWebAuthN", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Checks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if self.password.is_some() {
            len += 1;
        }
        if self.web_auth_n.is_some() {
            len += 1;
        }
        if self.idp_intent.is_some() {
            len += 1;
        }
        if self.totp.is_some() {
            len += 1;
        }
        if self.otp_sms.is_some() {
            len += 1;
        }
        if self.otp_email.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.Checks", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.password.as_ref() {
            struct_ser.serialize_field("password", v)?;
        }
        if let Some(v) = self.web_auth_n.as_ref() {
            struct_ser.serialize_field("webAuthN", v)?;
        }
        if let Some(v) = self.idp_intent.as_ref() {
            struct_ser.serialize_field("idpIntent", v)?;
        }
        if let Some(v) = self.totp.as_ref() {
            struct_ser.serialize_field("totp", v)?;
        }
        if let Some(v) = self.otp_sms.as_ref() {
            struct_ser.serialize_field("otpSms", v)?;
        }
        if let Some(v) = self.otp_email.as_ref() {
            struct_ser.serialize_field("otpEmail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Checks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "password",
            "web_auth_n",
            "webAuthN",
            "idp_intent",
            "idpIntent",
            "totp",
            "otp_sms",
            "otpSms",
            "otp_email",
            "otpEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Password,
            WebAuthN,
            IdpIntent,
            Totp,
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
                            "user" => Ok(GeneratedField::User),
                            "password" => Ok(GeneratedField::Password),
                            "webAuthN" | "web_auth_n" => Ok(GeneratedField::WebAuthN),
                            "idpIntent" | "idp_intent" => Ok(GeneratedField::IdpIntent),
                            "totp" => Ok(GeneratedField::Totp),
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
            type Value = Checks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.Checks")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Checks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut password__ = None;
                let mut web_auth_n__ = None;
                let mut idp_intent__ = None;
                let mut totp__ = None;
                let mut otp_sms__ = None;
                let mut otp_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = map_.next_value()?;
                        }
                        GeneratedField::WebAuthN => {
                            if web_auth_n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webAuthN"));
                            }
                            web_auth_n__ = map_.next_value()?;
                        }
                        GeneratedField::IdpIntent => {
                            if idp_intent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpIntent"));
                            }
                            idp_intent__ = map_.next_value()?;
                        }
                        GeneratedField::Totp => {
                            if totp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totp"));
                            }
                            totp__ = map_.next_value()?;
                        }
                        GeneratedField::OtpSms => {
                            if otp_sms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpSms"));
                            }
                            otp_sms__ = map_.next_value()?;
                        }
                        GeneratedField::OtpEmail => {
                            if otp_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpEmail"));
                            }
                            otp_email__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Checks {
                    user: user__,
                    password: password__,
                    web_auth_n: web_auth_n__,
                    idp_intent: idp_intent__,
                    totp: totp__,
                    otp_sms: otp_sms__,
                    otp_email: otp_email__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.Checks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSessionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.checks.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.challenges.is_some() {
            len += 1;
        }
        if self.user_agent.is_some() {
            len += 1;
        }
        if self.lifetime.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CreateSessionRequest", len)?;
        if let Some(v) = self.checks.as_ref() {
            struct_ser.serialize_field("checks", v)?;
        }
        if !self.metadata.is_empty() {
            let v: std::collections::HashMap<_, _> = self.metadata.iter()
                .map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();
            struct_ser.serialize_field("metadata", &v)?;
        }
        if let Some(v) = self.challenges.as_ref() {
            struct_ser.serialize_field("challenges", v)?;
        }
        if let Some(v) = self.user_agent.as_ref() {
            struct_ser.serialize_field("userAgent", v)?;
        }
        if let Some(v) = self.lifetime.as_ref() {
            struct_ser.serialize_field("lifetime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "checks",
            "metadata",
            "challenges",
            "user_agent",
            "userAgent",
            "lifetime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Checks,
            Metadata,
            Challenges,
            UserAgent,
            Lifetime,
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
                            "checks" => Ok(GeneratedField::Checks),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "challenges" => Ok(GeneratedField::Challenges),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
                            "lifetime" => Ok(GeneratedField::Lifetime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CreateSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut checks__ = None;
                let mut metadata__ = None;
                let mut challenges__ = None;
                let mut user_agent__ = None;
                let mut lifetime__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Checks => {
                            if checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checks"));
                            }
                            checks__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::Challenges => {
                            if challenges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("challenges"));
                            }
                            challenges__ = map_.next_value()?;
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = map_.next_value()?;
                        }
                        GeneratedField::Lifetime => {
                            if lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifetime"));
                            }
                            lifetime__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateSessionRequest {
                    checks: checks__,
                    metadata: metadata__.unwrap_or_default(),
                    challenges: challenges__,
                    user_agent: user_agent__,
                    lifetime: lifetime__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CreateSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSessionResponse {
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
        if !self.session_id.is_empty() {
            len += 1;
        }
        if !self.session_token.is_empty() {
            len += 1;
        }
        if self.challenges.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CreateSessionResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if !self.session_token.is_empty() {
            struct_ser.serialize_field("sessionToken", &self.session_token)?;
        }
        if let Some(v) = self.challenges.as_ref() {
            struct_ser.serialize_field("challenges", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSessionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "session_id",
            "sessionId",
            "session_token",
            "sessionToken",
            "challenges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SessionId,
            SessionToken,
            Challenges,
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
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            "sessionToken" | "session_token" => Ok(GeneratedField::SessionToken),
                            "challenges" => Ok(GeneratedField::Challenges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CreateSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSessionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut session_id__ = None;
                let mut session_token__ = None;
                let mut challenges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionToken => {
                            if session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionToken"));
                            }
                            session_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Challenges => {
                            if challenges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("challenges"));
                            }
                            challenges__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateSessionResponse {
                    details: details__,
                    session_id: session_id__.unwrap_or_default(),
                    session_token: session_token__.unwrap_or_default(),
                    challenges: challenges__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CreateSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreationDateQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.CreationDateQuery", len)?;
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TimestampQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreationDateQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_date",
            "creationDate",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreationDate,
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
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
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
            type Value = CreationDateQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.CreationDateQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreationDateQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut creation_date__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TimestampQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(CreationDateQuery {
                    creation_date: creation_date__,
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.CreationDateQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSessionRequest {
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
        if self.session_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.DeleteSessionRequest", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if let Some(v) = self.session_token.as_ref() {
            struct_ser.serialize_field("sessionToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "session_token",
            "sessionToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            SessionToken,
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
                            "sessionToken" | "session_token" => Ok(GeneratedField::SessionToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.DeleteSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut session_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionToken => {
                            if session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionToken"));
                            }
                            session_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteSessionRequest {
                    session_id: session_id__.unwrap_or_default(),
                    session_token: session_token__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.DeleteSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSessionResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.DeleteSessionResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSessionResponse {
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
            type Value = DeleteSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.DeleteSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSessionResponse, V::Error>
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
                Ok(DeleteSessionResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.DeleteSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Factors {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if self.password.is_some() {
            len += 1;
        }
        if self.web_auth_n.is_some() {
            len += 1;
        }
        if self.intent.is_some() {
            len += 1;
        }
        if self.totp.is_some() {
            len += 1;
        }
        if self.otp_sms.is_some() {
            len += 1;
        }
        if self.otp_email.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.Factors", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.password.as_ref() {
            struct_ser.serialize_field("password", v)?;
        }
        if let Some(v) = self.web_auth_n.as_ref() {
            struct_ser.serialize_field("webAuthN", v)?;
        }
        if let Some(v) = self.intent.as_ref() {
            struct_ser.serialize_field("intent", v)?;
        }
        if let Some(v) = self.totp.as_ref() {
            struct_ser.serialize_field("totp", v)?;
        }
        if let Some(v) = self.otp_sms.as_ref() {
            struct_ser.serialize_field("otpSms", v)?;
        }
        if let Some(v) = self.otp_email.as_ref() {
            struct_ser.serialize_field("otpEmail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Factors {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "password",
            "web_auth_n",
            "webAuthN",
            "intent",
            "totp",
            "otp_sms",
            "otpSms",
            "otp_email",
            "otpEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Password,
            WebAuthN,
            Intent,
            Totp,
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
                            "user" => Ok(GeneratedField::User),
                            "password" => Ok(GeneratedField::Password),
                            "webAuthN" | "web_auth_n" => Ok(GeneratedField::WebAuthN),
                            "intent" => Ok(GeneratedField::Intent),
                            "totp" => Ok(GeneratedField::Totp),
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
            type Value = Factors;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.Factors")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Factors, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut password__ = None;
                let mut web_auth_n__ = None;
                let mut intent__ = None;
                let mut totp__ = None;
                let mut otp_sms__ = None;
                let mut otp_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = map_.next_value()?;
                        }
                        GeneratedField::WebAuthN => {
                            if web_auth_n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webAuthN"));
                            }
                            web_auth_n__ = map_.next_value()?;
                        }
                        GeneratedField::Intent => {
                            if intent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            intent__ = map_.next_value()?;
                        }
                        GeneratedField::Totp => {
                            if totp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totp"));
                            }
                            totp__ = map_.next_value()?;
                        }
                        GeneratedField::OtpSms => {
                            if otp_sms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpSms"));
                            }
                            otp_sms__ = map_.next_value()?;
                        }
                        GeneratedField::OtpEmail => {
                            if otp_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpEmail"));
                            }
                            otp_email__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Factors {
                    user: user__,
                    password: password__,
                    web_auth_n: web_auth_n__,
                    intent: intent__,
                    totp: totp__,
                    otp_sms: otp_sms__,
                    otp_email: otp_email__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.Factors", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSessionRequest {
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
        if self.session_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.GetSessionRequest", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if let Some(v) = self.session_token.as_ref() {
            struct_ser.serialize_field("sessionToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "session_token",
            "sessionToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            SessionToken,
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
                            "sessionToken" | "session_token" => Ok(GeneratedField::SessionToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.GetSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut session_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionToken => {
                            if session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionToken"));
                            }
                            session_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSessionRequest {
                    session_id: session_id__.unwrap_or_default(),
                    session_token: session_token__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.GetSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSessionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.session.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.GetSessionResponse", len)?;
        if let Some(v) = self.session.as_ref() {
            struct_ser.serialize_field("session", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSessionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Session,
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
                            "session" => Ok(GeneratedField::Session),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.GetSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSessionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Session => {
                            if session__.is_some() {
                                return Err(serde::de::Error::duplicate_field("session"));
                            }
                            session__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSessionResponse {
                    session: session__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.GetSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IDsQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.IDsQuery", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field("ids", &self.ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IDsQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IDsQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.IDsQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IDsQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IDsQuery {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.IDsQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntentFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.IntentFactor", len)?;
        if let Some(v) = self.verified_at.as_ref() {
            struct_ser.serialize_field("verifiedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntentFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verified_at",
            "verifiedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifiedAt,
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
                            "verifiedAt" | "verified_at" => Ok(GeneratedField::VerifiedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntentFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.IntentFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntentFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verified_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerifiedAt => {
                            if verified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedAt"));
                            }
                            verified_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IntentFactor {
                    verified_at: verified_at__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.IntentFactor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSessionsRequest {
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
        if !self.queries.is_empty() {
            len += 1;
        }
        if self.sorting_column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.ListSessionsRequest", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        if self.sorting_column != 0 {
            let v = SessionFieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSessionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "queries",
            "sorting_column",
            "sortingColumn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            Queries,
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
                            "query" => Ok(GeneratedField::Query),
                            "queries" => Ok(GeneratedField::Queries),
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
            type Value = ListSessionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.ListSessionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSessionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut queries__ = None;
                let mut sorting_column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<SessionFieldName>()? as i32);
                        }
                    }
                }
                Ok(ListSessionsRequest {
                    query: query__,
                    queries: queries__.unwrap_or_default(),
                    sorting_column: sorting_column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.ListSessionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSessionsResponse {
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
        if !self.sessions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.ListSessionsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.sessions.is_empty() {
            struct_ser.serialize_field("sessions", &self.sessions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSessionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "sessions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Sessions,
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
                            "sessions" => Ok(GeneratedField::Sessions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSessionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.ListSessionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSessionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut sessions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Sessions => {
                            if sessions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessions"));
                            }
                            sessions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSessionsResponse {
                    details: details__,
                    sessions: sessions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.ListSessionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OtpFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.OTPFactor", len)?;
        if let Some(v) = self.verified_at.as_ref() {
            struct_ser.serialize_field("verifiedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OtpFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verified_at",
            "verifiedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifiedAt,
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
                            "verifiedAt" | "verified_at" => Ok(GeneratedField::VerifiedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OtpFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.OTPFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OtpFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verified_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerifiedAt => {
                            if verified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedAt"));
                            }
                            verified_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OtpFactor {
                    verified_at: verified_at__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.OTPFactor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.PasswordFactor", len)?;
        if let Some(v) = self.verified_at.as_ref() {
            struct_ser.serialize_field("verifiedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verified_at",
            "verifiedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifiedAt,
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
                            "verifiedAt" | "verified_at" => Ok(GeneratedField::VerifiedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.PasswordFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verified_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerifiedAt => {
                            if verified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedAt"));
                            }
                            verified_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PasswordFactor {
                    verified_at: verified_at__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.PasswordFactor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestChallenges {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.web_auth_n.is_some() {
            len += 1;
        }
        if self.otp_sms.is_some() {
            len += 1;
        }
        if self.otp_email.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.RequestChallenges", len)?;
        if let Some(v) = self.web_auth_n.as_ref() {
            struct_ser.serialize_field("webAuthN", v)?;
        }
        if let Some(v) = self.otp_sms.as_ref() {
            struct_ser.serialize_field("otpSms", v)?;
        }
        if let Some(v) = self.otp_email.as_ref() {
            struct_ser.serialize_field("otpEmail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestChallenges {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "web_auth_n",
            "webAuthN",
            "otp_sms",
            "otpSms",
            "otp_email",
            "otpEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebAuthN,
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
                            "webAuthN" | "web_auth_n" => Ok(GeneratedField::WebAuthN),
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
            type Value = RequestChallenges;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.RequestChallenges")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestChallenges, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut web_auth_n__ = None;
                let mut otp_sms__ = None;
                let mut otp_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebAuthN => {
                            if web_auth_n__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webAuthN"));
                            }
                            web_auth_n__ = map_.next_value()?;
                        }
                        GeneratedField::OtpSms => {
                            if otp_sms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpSms"));
                            }
                            otp_sms__ = map_.next_value()?;
                        }
                        GeneratedField::OtpEmail => {
                            if otp_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpEmail"));
                            }
                            otp_email__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RequestChallenges {
                    web_auth_n: web_auth_n__,
                    otp_sms: otp_sms__,
                    otp_email: otp_email__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.RequestChallenges", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for request_challenges::OtpEmail {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delivery_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.RequestChallenges.OTPEmail", len)?;
        if let Some(v) = self.delivery_type.as_ref() {
            match v {
                request_challenges::otp_email::DeliveryType::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                request_challenges::otp_email::DeliveryType::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for request_challenges::OtpEmail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SendCode,
            ReturnCode,
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
                            "sendCode" | "send_code" => Ok(GeneratedField::SendCode),
                            "returnCode" | "return_code" => Ok(GeneratedField::ReturnCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = request_challenges::OtpEmail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.RequestChallenges.OTPEmail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<request_challenges::OtpEmail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delivery_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SendCode => {
                            if delivery_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            delivery_type__ = map_.next_value::<::std::option::Option<_>>()?.map(request_challenges::otp_email::DeliveryType::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if delivery_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            delivery_type__ = map_.next_value::<::std::option::Option<_>>()?.map(request_challenges::otp_email::DeliveryType::ReturnCode)
;
                        }
                    }
                }
                Ok(request_challenges::OtpEmail {
                    delivery_type: delivery_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.RequestChallenges.OTPEmail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for request_challenges::otp_email::ReturnCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.session.v2beta.RequestChallenges.OTPEmail.ReturnCode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for request_challenges::otp_email::ReturnCode {
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
            type Value = request_challenges::otp_email::ReturnCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.RequestChallenges.OTPEmail.ReturnCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<request_challenges::otp_email::ReturnCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(request_challenges::otp_email::ReturnCode {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.RequestChallenges.OTPEmail.ReturnCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for request_challenges::otp_email::SendCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.url_template.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.RequestChallenges.OTPEmail.SendCode", len)?;
        if let Some(v) = self.url_template.as_ref() {
            struct_ser.serialize_field("urlTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for request_challenges::otp_email::SendCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url_template",
            "urlTemplate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UrlTemplate,
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
                            "urlTemplate" | "url_template" => Ok(GeneratedField::UrlTemplate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = request_challenges::otp_email::SendCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.RequestChallenges.OTPEmail.SendCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<request_challenges::otp_email::SendCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url_template__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UrlTemplate => {
                            if url_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlTemplate"));
                            }
                            url_template__ = map_.next_value()?;
                        }
                    }
                }
                Ok(request_challenges::otp_email::SendCode {
                    url_template: url_template__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.RequestChallenges.OTPEmail.SendCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for request_challenges::Otpsms {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.return_code {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.RequestChallenges.OTPSMS", len)?;
        if self.return_code {
            struct_ser.serialize_field("returnCode", &self.return_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for request_challenges::Otpsms {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "return_code",
            "returnCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReturnCode,
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
                            "returnCode" | "return_code" => Ok(GeneratedField::ReturnCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = request_challenges::Otpsms;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.RequestChallenges.OTPSMS")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<request_challenges::Otpsms, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut return_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReturnCode => {
                            if return_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            return_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(request_challenges::Otpsms {
                    return_code: return_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.RequestChallenges.OTPSMS", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for request_challenges::WebAuthN {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        if self.user_verification_requirement != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.RequestChallenges.WebAuthN", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if self.user_verification_requirement != 0 {
            let v = UserVerificationRequirement::try_from(self.user_verification_requirement)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.user_verification_requirement)))?;
            struct_ser.serialize_field("userVerificationRequirement", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for request_challenges::WebAuthN {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "user_verification_requirement",
            "userVerificationRequirement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            UserVerificationRequirement,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "userVerificationRequirement" | "user_verification_requirement" => Ok(GeneratedField::UserVerificationRequirement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = request_challenges::WebAuthN;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.RequestChallenges.WebAuthN")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<request_challenges::WebAuthN, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut user_verification_requirement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserVerificationRequirement => {
                            if user_verification_requirement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userVerificationRequirement"));
                            }
                            user_verification_requirement__ = Some(map_.next_value::<UserVerificationRequirement>()? as i32);
                        }
                    }
                }
                Ok(request_challenges::WebAuthN {
                    domain: domain__.unwrap_or_default(),
                    user_verification_requirement: user_verification_requirement__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.RequestChallenges.WebAuthN", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.SearchQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                search_query::Query::IdsQuery(v) => {
                    struct_ser.serialize_field("idsQuery", v)?;
                }
                search_query::Query::UserIdQuery(v) => {
                    struct_ser.serialize_field("userIdQuery", v)?;
                }
                search_query::Query::CreationDateQuery(v) => {
                    struct_ser.serialize_field("creationDateQuery", v)?;
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
            "ids_query",
            "idsQuery",
            "user_id_query",
            "userIdQuery",
            "creation_date_query",
            "creationDateQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdsQuery,
            UserIdQuery,
            CreationDateQuery,
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
                            "idsQuery" | "ids_query" => Ok(GeneratedField::IdsQuery),
                            "userIdQuery" | "user_id_query" => Ok(GeneratedField::UserIdQuery),
                            "creationDateQuery" | "creation_date_query" => Ok(GeneratedField::CreationDateQuery),
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
                formatter.write_str("struct zitadel.session.v2beta.SearchQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdsQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idsQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::IdsQuery)
;
                        }
                        GeneratedField::UserIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::UserIdQuery)
;
                        }
                        GeneratedField::CreationDateQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDateQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::CreationDateQuery)
;
                        }
                    }
                }
                Ok(SearchQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.SearchQuery", FIELDS, GeneratedVisitor)
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
        if !self.id.is_empty() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.change_date.is_some() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if self.factors.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.user_agent.is_some() {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.Session", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if let Some(v) = self.factors.as_ref() {
            struct_ser.serialize_field("factors", v)?;
        }
        if !self.metadata.is_empty() {
            let v: std::collections::HashMap<_, _> = self.metadata.iter()
                .map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();
            struct_ser.serialize_field("metadata", &v)?;
        }
        if let Some(v) = self.user_agent.as_ref() {
            struct_ser.serialize_field("userAgent", v)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
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
            "id",
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "sequence",
            "factors",
            "metadata",
            "user_agent",
            "userAgent",
            "expiration_date",
            "expirationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            ChangeDate,
            Sequence,
            Factors,
            Metadata,
            UserAgent,
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
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "factors" => Ok(GeneratedField::Factors),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
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
            type Value = Session;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.Session")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Session, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut sequence__ = None;
                let mut factors__ = None;
                let mut metadata__ = None;
                let mut user_agent__ = None;
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
                        GeneratedField::ChangeDate => {
                            if change_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            change_date__ = map_.next_value()?;
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Factors => {
                            if factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("factors"));
                            }
                            factors__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = map_.next_value()?;
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Session {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    change_date: change_date__,
                    sequence: sequence__.unwrap_or_default(),
                    factors: factors__,
                    metadata: metadata__.unwrap_or_default(),
                    user_agent: user_agent__,
                    expiration_date: expiration_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.Session", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SessionFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SESSION_FIELD_NAME_UNSPECIFIED",
            Self::CreationDate => "SESSION_FIELD_NAME_CREATION_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SessionFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SESSION_FIELD_NAME_UNSPECIFIED",
            "SESSION_FIELD_NAME_CREATION_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionFieldName;

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
                    "SESSION_FIELD_NAME_UNSPECIFIED" => Ok(SessionFieldName::Unspecified),
                    "SESSION_FIELD_NAME_CREATION_DATE" => Ok(SessionFieldName::CreationDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SetSessionRequest {
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
        if !self.session_token.is_empty() {
            len += 1;
        }
        if self.checks.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.challenges.is_some() {
            len += 1;
        }
        if self.lifetime.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.SetSessionRequest", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if !self.session_token.is_empty() {
            struct_ser.serialize_field("sessionToken", &self.session_token)?;
        }
        if let Some(v) = self.checks.as_ref() {
            struct_ser.serialize_field("checks", v)?;
        }
        if !self.metadata.is_empty() {
            let v: std::collections::HashMap<_, _> = self.metadata.iter()
                .map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();
            struct_ser.serialize_field("metadata", &v)?;
        }
        if let Some(v) = self.challenges.as_ref() {
            struct_ser.serialize_field("challenges", v)?;
        }
        if let Some(v) = self.lifetime.as_ref() {
            struct_ser.serialize_field("lifetime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSessionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "session_token",
            "sessionToken",
            "checks",
            "metadata",
            "challenges",
            "lifetime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            SessionToken,
            Checks,
            Metadata,
            Challenges,
            Lifetime,
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
                            "sessionToken" | "session_token" => Ok(GeneratedField::SessionToken),
                            "checks" => Ok(GeneratedField::Checks),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "challenges" => Ok(GeneratedField::Challenges),
                            "lifetime" => Ok(GeneratedField::Lifetime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.SetSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSessionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut session_token__ = None;
                let mut checks__ = None;
                let mut metadata__ = None;
                let mut challenges__ = None;
                let mut lifetime__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionToken => {
                            if session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionToken"));
                            }
                            session_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Checks => {
                            if checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checks"));
                            }
                            checks__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(
                                map_.next_value::<std::collections::HashMap<_, ::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                        GeneratedField::Challenges => {
                            if challenges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("challenges"));
                            }
                            challenges__ = map_.next_value()?;
                        }
                        GeneratedField::Lifetime => {
                            if lifetime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifetime"));
                            }
                            lifetime__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetSessionRequest {
                    session_id: session_id__.unwrap_or_default(),
                    session_token: session_token__.unwrap_or_default(),
                    checks: checks__,
                    metadata: metadata__.unwrap_or_default(),
                    challenges: challenges__,
                    lifetime: lifetime__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.SetSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSessionResponse {
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
        if !self.session_token.is_empty() {
            len += 1;
        }
        if self.challenges.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.SetSessionResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.session_token.is_empty() {
            struct_ser.serialize_field("sessionToken", &self.session_token)?;
        }
        if let Some(v) = self.challenges.as_ref() {
            struct_ser.serialize_field("challenges", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSessionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "session_token",
            "sessionToken",
            "challenges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SessionToken,
            Challenges,
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
                            "sessionToken" | "session_token" => Ok(GeneratedField::SessionToken),
                            "challenges" => Ok(GeneratedField::Challenges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetSessionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.SetSessionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSessionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut session_token__ = None;
                let mut challenges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SessionToken => {
                            if session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionToken"));
                            }
                            session_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Challenges => {
                            if challenges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("challenges"));
                            }
                            challenges__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetSessionResponse {
                    details: details__,
                    session_token: session_token__.unwrap_or_default(),
                    challenges: challenges__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.SetSessionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TotpFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.TOTPFactor", len)?;
        if let Some(v) = self.verified_at.as_ref() {
            struct_ser.serialize_field("verifiedAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TotpFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verified_at",
            "verifiedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifiedAt,
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
                            "verifiedAt" | "verified_at" => Ok(GeneratedField::VerifiedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TotpFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.TOTPFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TotpFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verified_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerifiedAt => {
                            if verified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedAt"));
                            }
                            verified_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TotpFactor {
                    verified_at: verified_at__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.TOTPFactor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserAgent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fingerprint_id.is_some() {
            len += 1;
        }
        if self.ip.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.header.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.UserAgent", len)?;
        if let Some(v) = self.fingerprint_id.as_ref() {
            struct_ser.serialize_field("fingerprintId", v)?;
        }
        if let Some(v) = self.ip.as_ref() {
            struct_ser.serialize_field("ip", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserAgent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fingerprint_id",
            "fingerprintId",
            "ip",
            "description",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FingerprintId,
            Ip,
            Description,
            Header,
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
                            "fingerprintId" | "fingerprint_id" => Ok(GeneratedField::FingerprintId),
                            "ip" => Ok(GeneratedField::Ip),
                            "description" => Ok(GeneratedField::Description),
                            "header" => Ok(GeneratedField::Header),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserAgent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.UserAgent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserAgent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fingerprint_id__ = None;
                let mut ip__ = None;
                let mut description__ = None;
                let mut header__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FingerprintId => {
                            if fingerprint_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fingerprintId"));
                            }
                            fingerprint_id__ = map_.next_value()?;
                        }
                        GeneratedField::Ip => {
                            if ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            ip__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(UserAgent {
                    fingerprint_id: fingerprint_id__,
                    ip: ip__,
                    description: description__,
                    header: header__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.UserAgent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for user_agent::HeaderValues {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.UserAgent.HeaderValues", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for user_agent::HeaderValues {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = user_agent::HeaderValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.UserAgent.HeaderValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<user_agent::HeaderValues, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(user_agent::HeaderValues {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.UserAgent.HeaderValues", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified_at.is_some() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.login_name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.UserFactor", len)?;
        if let Some(v) = self.verified_at.as_ref() {
            struct_ser.serialize_field("verifiedAt", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.login_name.is_empty() {
            struct_ser.serialize_field("loginName", &self.login_name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verified_at",
            "verifiedAt",
            "id",
            "login_name",
            "loginName",
            "display_name",
            "displayName",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifiedAt,
            Id,
            LoginName,
            DisplayName,
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
                            "verifiedAt" | "verified_at" => Ok(GeneratedField::VerifiedAt),
                            "id" => Ok(GeneratedField::Id),
                            "loginName" | "login_name" => Ok(GeneratedField::LoginName),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
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
            type Value = UserFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.UserFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verified_at__ = None;
                let mut id__ = None;
                let mut login_name__ = None;
                let mut display_name__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerifiedAt => {
                            if verified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedAt"));
                            }
                            verified_at__ = map_.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserFactor {
                    verified_at: verified_at__,
                    id: id__.unwrap_or_default(),
                    login_name: login_name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.UserFactor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserIdQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.UserIDQuery", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserIdQuery {
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
            type Value = UserIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.UserIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserIdQuery, V::Error>
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
                Ok(UserIdQuery {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.UserIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserVerificationRequirement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "USER_VERIFICATION_REQUIREMENT_UNSPECIFIED",
            Self::Required => "USER_VERIFICATION_REQUIREMENT_REQUIRED",
            Self::Preferred => "USER_VERIFICATION_REQUIREMENT_PREFERRED",
            Self::Discouraged => "USER_VERIFICATION_REQUIREMENT_DISCOURAGED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UserVerificationRequirement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USER_VERIFICATION_REQUIREMENT_UNSPECIFIED",
            "USER_VERIFICATION_REQUIREMENT_REQUIRED",
            "USER_VERIFICATION_REQUIREMENT_PREFERRED",
            "USER_VERIFICATION_REQUIREMENT_DISCOURAGED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserVerificationRequirement;

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
                    "USER_VERIFICATION_REQUIREMENT_UNSPECIFIED" => Ok(UserVerificationRequirement::Unspecified),
                    "USER_VERIFICATION_REQUIREMENT_REQUIRED" => Ok(UserVerificationRequirement::Required),
                    "USER_VERIFICATION_REQUIREMENT_PREFERRED" => Ok(UserVerificationRequirement::Preferred),
                    "USER_VERIFICATION_REQUIREMENT_DISCOURAGED" => Ok(UserVerificationRequirement::Discouraged),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebAuthNFactor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verified_at.is_some() {
            len += 1;
        }
        if self.user_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.session.v2beta.WebAuthNFactor", len)?;
        if let Some(v) = self.verified_at.as_ref() {
            struct_ser.serialize_field("verifiedAt", v)?;
        }
        if self.user_verified {
            struct_ser.serialize_field("userVerified", &self.user_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebAuthNFactor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verified_at",
            "verifiedAt",
            "user_verified",
            "userVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifiedAt,
            UserVerified,
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
                            "verifiedAt" | "verified_at" => Ok(GeneratedField::VerifiedAt),
                            "userVerified" | "user_verified" => Ok(GeneratedField::UserVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebAuthNFactor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.session.v2beta.WebAuthNFactor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebAuthNFactor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verified_at__ = None;
                let mut user_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VerifiedAt => {
                            if verified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedAt"));
                            }
                            verified_at__ = map_.next_value()?;
                        }
                        GeneratedField::UserVerified => {
                            if user_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userVerified"));
                            }
                            user_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WebAuthNFactor {
                    verified_at: verified_at__,
                    user_verified: user_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.session.v2beta.WebAuthNFactor", FIELDS, GeneratedVisitor)
    }
}
