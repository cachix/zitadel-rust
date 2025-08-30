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
impl serde::Serialize for AddHumanUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_id.is_some() {
            len += 1;
        }
        if self.username.is_some() {
            len += 1;
        }
        if self.organization.is_some() {
            len += 1;
        }
        if self.profile.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.idp_links.is_empty() {
            len += 1;
        }
        if self.totp_secret.is_some() {
            len += 1;
        }
        if self.password_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddHumanUserRequest", len)?;
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.username.as_ref() {
            struct_ser.serialize_field("username", v)?;
        }
        if let Some(v) = self.organization.as_ref() {
            struct_ser.serialize_field("organization", v)?;
        }
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.idp_links.is_empty() {
            struct_ser.serialize_field("idpLinks", &self.idp_links)?;
        }
        if let Some(v) = self.totp_secret.as_ref() {
            struct_ser.serialize_field("totpSecret", v)?;
        }
        if let Some(v) = self.password_type.as_ref() {
            match v {
                add_human_user_request::PasswordType::Password(v) => {
                    struct_ser.serialize_field("password", v)?;
                }
                add_human_user_request::PasswordType::HashedPassword(v) => {
                    struct_ser.serialize_field("hashedPassword", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddHumanUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "username",
            "organization",
            "profile",
            "email",
            "phone",
            "metadata",
            "idp_links",
            "idpLinks",
            "totp_secret",
            "totpSecret",
            "password",
            "hashed_password",
            "hashedPassword",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Username,
            Organization,
            Profile,
            Email,
            Phone,
            Metadata,
            IdpLinks,
            TotpSecret,
            Password,
            HashedPassword,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "username" => Ok(GeneratedField::Username),
                            "organization" => Ok(GeneratedField::Organization),
                            "profile" => Ok(GeneratedField::Profile),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "idpLinks" | "idp_links" => Ok(GeneratedField::IdpLinks),
                            "totpSecret" | "totp_secret" => Ok(GeneratedField::TotpSecret),
                            "password" => Ok(GeneratedField::Password),
                            "hashedPassword" | "hashed_password" => Ok(GeneratedField::HashedPassword),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddHumanUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddHumanUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddHumanUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut username__ = None;
                let mut organization__ = None;
                let mut profile__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut metadata__ = None;
                let mut idp_links__ = None;
                let mut totp_secret__ = None;
                let mut password_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = map_.next_value()?;
                        }
                        GeneratedField::Organization => {
                            if organization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            organization__ = map_.next_value()?;
                        }
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
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpLinks => {
                            if idp_links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpLinks"));
                            }
                            idp_links__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotpSecret => {
                            if totp_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totpSecret"));
                            }
                            totp_secret__ = map_.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password_type__ = map_.next_value::<::std::option::Option<_>>()?.map(add_human_user_request::PasswordType::Password)
;
                        }
                        GeneratedField::HashedPassword => {
                            if password_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashedPassword"));
                            }
                            password_type__ = map_.next_value::<::std::option::Option<_>>()?.map(add_human_user_request::PasswordType::HashedPassword)
;
                        }
                    }
                }
                Ok(AddHumanUserRequest {
                    user_id: user_id__,
                    username: username__,
                    organization: organization__,
                    profile: profile__,
                    email: email__,
                    phone: phone__,
                    metadata: metadata__.unwrap_or_default(),
                    idp_links: idp_links__.unwrap_or_default(),
                    totp_secret: totp_secret__,
                    password_type: password_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddHumanUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddHumanUserResponse {
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
        if self.email_code.is_some() {
            len += 1;
        }
        if self.phone_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddHumanUserResponse", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.email_code.as_ref() {
            struct_ser.serialize_field("emailCode", v)?;
        }
        if let Some(v) = self.phone_code.as_ref() {
            struct_ser.serialize_field("phoneCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddHumanUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "details",
            "email_code",
            "emailCode",
            "phone_code",
            "phoneCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Details,
            EmailCode,
            PhoneCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "emailCode" | "email_code" => Ok(GeneratedField::EmailCode),
                            "phoneCode" | "phone_code" => Ok(GeneratedField::PhoneCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddHumanUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddHumanUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddHumanUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut details__ = None;
                let mut email_code__ = None;
                let mut phone_code__ = None;
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
                        GeneratedField::EmailCode => {
                            if email_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailCode"));
                            }
                            email_code__ = map_.next_value()?;
                        }
                        GeneratedField::PhoneCode => {
                            if phone_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneCode"));
                            }
                            phone_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddHumanUserResponse {
                    user_id: user_id__.unwrap_or_default(),
                    details: details__,
                    email_code: email_code__,
                    phone_code: phone_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddHumanUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddIdpLinkRequest {
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
        if self.idp_link.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddIDPLinkRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.idp_link.as_ref() {
            struct_ser.serialize_field("idpLink", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddIdpLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "idp_link",
            "idpLink",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            IdpLink,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "idpLink" | "idp_link" => Ok(GeneratedField::IdpLink),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddIdpLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddIDPLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddIdpLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut idp_link__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdpLink => {
                            if idp_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpLink"));
                            }
                            idp_link__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddIdpLinkRequest {
                    user_id: user_id__.unwrap_or_default(),
                    idp_link: idp_link__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddIDPLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddIdpLinkResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddIDPLinkResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddIdpLinkResponse {
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
            type Value = AddIdpLinkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddIDPLinkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddIdpLinkResponse, V::Error>
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
                Ok(AddIdpLinkResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddIDPLinkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOtpEmailRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddOTPEmailRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOtpEmailRequest {
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
            type Value = AddOtpEmailRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddOTPEmailRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOtpEmailRequest, V::Error>
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
                Ok(AddOtpEmailRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddOTPEmailRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOtpEmailResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddOTPEmailResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOtpEmailResponse {
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
            type Value = AddOtpEmailResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddOTPEmailResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOtpEmailResponse, V::Error>
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
                Ok(AddOtpEmailResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddOTPEmailResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOtpsmsRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddOTPSMSRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOtpsmsRequest {
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
            type Value = AddOtpsmsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddOTPSMSRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOtpsmsRequest, V::Error>
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
                Ok(AddOtpsmsRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddOTPSMSRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddOtpsmsResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AddOTPSMSResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddOtpsmsResponse {
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
            type Value = AddOtpsmsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.AddOTPSMSResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddOtpsmsResponse, V::Error>
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
                Ok(AddOtpsmsResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.AddOTPSMSResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.AndQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.AndQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.AndQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationMethodType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED",
            Self::Password => "AUTHENTICATION_METHOD_TYPE_PASSWORD",
            Self::Passkey => "AUTHENTICATION_METHOD_TYPE_PASSKEY",
            Self::Idp => "AUTHENTICATION_METHOD_TYPE_IDP",
            Self::Totp => "AUTHENTICATION_METHOD_TYPE_TOTP",
            Self::U2f => "AUTHENTICATION_METHOD_TYPE_U2F",
            Self::OtpSms => "AUTHENTICATION_METHOD_TYPE_OTP_SMS",
            Self::OtpEmail => "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationMethodType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED",
            "AUTHENTICATION_METHOD_TYPE_PASSWORD",
            "AUTHENTICATION_METHOD_TYPE_PASSKEY",
            "AUTHENTICATION_METHOD_TYPE_IDP",
            "AUTHENTICATION_METHOD_TYPE_TOTP",
            "AUTHENTICATION_METHOD_TYPE_U2F",
            "AUTHENTICATION_METHOD_TYPE_OTP_SMS",
            "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationMethodType;

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
                    "AUTHENTICATION_METHOD_TYPE_UNSPECIFIED" => Ok(AuthenticationMethodType::Unspecified),
                    "AUTHENTICATION_METHOD_TYPE_PASSWORD" => Ok(AuthenticationMethodType::Password),
                    "AUTHENTICATION_METHOD_TYPE_PASSKEY" => Ok(AuthenticationMethodType::Passkey),
                    "AUTHENTICATION_METHOD_TYPE_IDP" => Ok(AuthenticationMethodType::Idp),
                    "AUTHENTICATION_METHOD_TYPE_TOTP" => Ok(AuthenticationMethodType::Totp),
                    "AUTHENTICATION_METHOD_TYPE_U2F" => Ok(AuthenticationMethodType::U2f),
                    "AUTHENTICATION_METHOD_TYPE_OTP_SMS" => Ok(AuthenticationMethodType::OtpSms),
                    "AUTHENTICATION_METHOD_TYPE_OTP_EMAIL" => Ok(AuthenticationMethodType::OtpEmail),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePasskeyRegistrationLinkRequest {
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
        if self.medium.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.CreatePasskeyRegistrationLinkRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.medium.as_ref() {
            match v {
                create_passkey_registration_link_request::Medium::SendLink(v) => {
                    struct_ser.serialize_field("sendLink", v)?;
                }
                create_passkey_registration_link_request::Medium::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePasskeyRegistrationLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "send_link",
            "sendLink",
            "return_code",
            "returnCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            SendLink,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "sendLink" | "send_link" => Ok(GeneratedField::SendLink),
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
            type Value = CreatePasskeyRegistrationLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.CreatePasskeyRegistrationLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePasskeyRegistrationLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut medium__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendLink => {
                            if medium__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendLink"));
                            }
                            medium__ = map_.next_value::<::std::option::Option<_>>()?.map(create_passkey_registration_link_request::Medium::SendLink)
;
                        }
                        GeneratedField::ReturnCode => {
                            if medium__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            medium__ = map_.next_value::<::std::option::Option<_>>()?.map(create_passkey_registration_link_request::Medium::ReturnCode)
;
                        }
                    }
                }
                Ok(CreatePasskeyRegistrationLinkRequest {
                    user_id: user_id__.unwrap_or_default(),
                    medium: medium__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.CreatePasskeyRegistrationLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePasskeyRegistrationLinkResponse {
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
        if self.code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.CreatePasskeyRegistrationLinkResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePasskeyRegistrationLinkResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
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
                            "details" => Ok(GeneratedField::Details),
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
            type Value = CreatePasskeyRegistrationLinkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.CreatePasskeyRegistrationLinkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePasskeyRegistrationLinkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePasskeyRegistrationLinkResponse {
                    details: details__,
                    code: code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.CreatePasskeyRegistrationLinkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.DeactivateUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateUserRequest {
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
            type Value = DeactivateUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.DeactivateUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateUserRequest, V::Error>
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
                Ok(DeactivateUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.DeactivateUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.DeactivateUserResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateUserResponse {
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
            type Value = DeactivateUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.DeactivateUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateUserResponse, V::Error>
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
                Ok(DeactivateUserResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.DeactivateUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.DeleteUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserRequest {
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
            type Value = DeleteUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.DeleteUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserRequest, V::Error>
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
                Ok(DeleteUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.DeleteUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.DeleteUserResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserResponse {
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
            type Value = DeleteUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.DeleteUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserResponse, V::Error>
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
                Ok(DeleteUserResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.DeleteUserResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.DisplayNameQuery", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.DisplayNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(DisplayNameQuery {
                    display_name: display_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.DisplayNameQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.EmailQuery", len)?;
        if !self.email_address.is_empty() {
            struct_ser.serialize_field("emailAddress", &self.email_address)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.EmailQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(EmailQuery {
                    email_address: email_address__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.EmailQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.FirstNameQuery", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.FirstNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(FirstNameQuery {
                    first_name: first_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.FirstNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FormData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.FormData", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FormData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "fields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            Fields,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FormData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.FormData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FormData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(FormData {
                    url: url__.unwrap_or_default(),
                    fields: fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.FormData", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for GetUserByIdRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.GetUserByIDRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserByIdRequest {
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
            type Value = GetUserByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.GetUserByIDRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserByIdRequest, V::Error>
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
                Ok(GetUserByIdRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.GetUserByIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserByIdResponse {
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
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.GetUserByIDResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserByIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserByIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.GetUserByIDResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserByIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserByIdResponse {
                    details: details__,
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.GetUserByIDResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HashedPassword {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.change_required {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.HashedPassword", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if self.change_required {
            struct_ser.serialize_field("changeRequired", &self.change_required)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HashedPassword {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "change_required",
            "changeRequired",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            ChangeRequired,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hash" => Ok(GeneratedField::Hash),
                            "changeRequired" | "change_required" => Ok(GeneratedField::ChangeRequired),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HashedPassword;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.HashedPassword")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HashedPassword, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut change_required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChangeRequired => {
                            if change_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeRequired"));
                            }
                            change_required__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HashedPassword {
                    hash: hash__.unwrap_or_default(),
                    change_required: change_required__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.HashedPassword", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HumanEmail {
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
        if self.is_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.HumanEmail", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.is_verified {
            struct_ser.serialize_field("isVerified", &self.is_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HumanEmail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "is_verified",
            "isVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            IsVerified,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "isVerified" | "is_verified" => Ok(GeneratedField::IsVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HumanEmail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.HumanEmail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HumanEmail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut is_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsVerified => {
                            if is_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVerified"));
                            }
                            is_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HumanEmail {
                    email: email__.unwrap_or_default(),
                    is_verified: is_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.HumanEmail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HumanPhone {
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
        if self.is_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.HumanPhone", len)?;
        if !self.phone.is_empty() {
            struct_ser.serialize_field("phone", &self.phone)?;
        }
        if self.is_verified {
            struct_ser.serialize_field("isVerified", &self.is_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HumanPhone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phone",
            "is_verified",
            "isVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phone,
            IsVerified,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "isVerified" | "is_verified" => Ok(GeneratedField::IsVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HumanPhone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.HumanPhone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HumanPhone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone__ = None;
                let mut is_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsVerified => {
                            if is_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVerified"));
                            }
                            is_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HumanPhone {
                    phone: phone__.unwrap_or_default(),
                    is_verified: is_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.HumanPhone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HumanProfile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.given_name.is_empty() {
            len += 1;
        }
        if !self.family_name.is_empty() {
            len += 1;
        }
        if self.nick_name.is_some() {
            len += 1;
        }
        if self.display_name.is_some() {
            len += 1;
        }
        if self.preferred_language.is_some() {
            len += 1;
        }
        if self.gender.is_some() {
            len += 1;
        }
        if !self.avatar_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.HumanProfile", len)?;
        if !self.given_name.is_empty() {
            struct_ser.serialize_field("givenName", &self.given_name)?;
        }
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if let Some(v) = self.nick_name.as_ref() {
            struct_ser.serialize_field("nickName", v)?;
        }
        if let Some(v) = self.display_name.as_ref() {
            struct_ser.serialize_field("displayName", v)?;
        }
        if let Some(v) = self.preferred_language.as_ref() {
            struct_ser.serialize_field("preferredLanguage", v)?;
        }
        if let Some(v) = self.gender.as_ref() {
            let v = Gender::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("gender", &v)?;
        }
        if !self.avatar_url.is_empty() {
            struct_ser.serialize_field("avatarUrl", &self.avatar_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HumanProfile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "given_name",
            "givenName",
            "family_name",
            "familyName",
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
            GivenName,
            FamilyName,
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
                            "givenName" | "given_name" => Ok(GeneratedField::GivenName),
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
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
            type Value = HumanProfile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.HumanProfile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HumanProfile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut given_name__ = None;
                let mut family_name__ = None;
                let mut nick_name__ = None;
                let mut display_name__ = None;
                let mut preferred_language__ = None;
                let mut gender__ = None;
                let mut avatar_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GivenName => {
                            if given_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("givenName"));
                            }
                            given_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NickName => {
                            if nick_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickName"));
                            }
                            nick_name__ = map_.next_value()?;
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = map_.next_value()?;
                        }
                        GeneratedField::PreferredLanguage => {
                            if preferred_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguage"));
                            }
                            preferred_language__ = map_.next_value()?;
                        }
                        GeneratedField::Gender => {
                            if gender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            gender__ = map_.next_value::<::std::option::Option<Gender>>()?.map(|x| x as i32);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HumanProfile {
                    given_name: given_name__.unwrap_or_default(),
                    family_name: family_name__.unwrap_or_default(),
                    nick_name: nick_name__,
                    display_name: display_name__,
                    preferred_language: preferred_language__,
                    gender: gender__,
                    avatar_url: avatar_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.HumanProfile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HumanUser {
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
        if self.state != 0 {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.login_names.is_empty() {
            len += 1;
        }
        if !self.preferred_login_name.is_empty() {
            len += 1;
        }
        if self.profile.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.password_change_required {
            len += 1;
        }
        if self.password_changed.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.HumanUser", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.state != 0 {
            let v = UserState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.login_names.is_empty() {
            struct_ser.serialize_field("loginNames", &self.login_names)?;
        }
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
        }
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if self.password_change_required {
            struct_ser.serialize_field("passwordChangeRequired", &self.password_change_required)?;
        }
        if let Some(v) = self.password_changed.as_ref() {
            struct_ser.serialize_field("passwordChanged", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HumanUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "state",
            "username",
            "login_names",
            "loginNames",
            "preferred_login_name",
            "preferredLoginName",
            "profile",
            "email",
            "phone",
            "password_change_required",
            "passwordChangeRequired",
            "password_changed",
            "passwordChanged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            State,
            Username,
            LoginNames,
            PreferredLoginName,
            Profile,
            Email,
            Phone,
            PasswordChangeRequired,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "state" => Ok(GeneratedField::State),
                            "username" => Ok(GeneratedField::Username),
                            "loginNames" | "login_names" => Ok(GeneratedField::LoginNames),
                            "preferredLoginName" | "preferred_login_name" => Ok(GeneratedField::PreferredLoginName),
                            "profile" => Ok(GeneratedField::Profile),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "passwordChangeRequired" | "password_change_required" => Ok(GeneratedField::PasswordChangeRequired),
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
            type Value = HumanUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.HumanUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HumanUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut state__ = None;
                let mut username__ = None;
                let mut login_names__ = None;
                let mut preferred_login_name__ = None;
                let mut profile__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut password_change_required__ = None;
                let mut password_changed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<UserState>()? as i32);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
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
                        GeneratedField::PasswordChangeRequired => {
                            if password_change_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChangeRequired"));
                            }
                            password_change_required__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordChanged => {
                            if password_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChanged"));
                            }
                            password_changed__ = map_.next_value()?;
                        }
                    }
                }
                Ok(HumanUser {
                    user_id: user_id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    login_names: login_names__.unwrap_or_default(),
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    profile: profile__,
                    email: email__,
                    phone: phone__,
                    password_change_required: password_change_required__.unwrap_or_default(),
                    password_changed: password_changed__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.HumanUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpInformation {
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.raw_information.is_some() {
            len += 1;
        }
        if self.access.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.IDPInformation", len)?;
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.raw_information.as_ref() {
            struct_ser.serialize_field("rawInformation", v)?;
        }
        if let Some(v) = self.access.as_ref() {
            match v {
                idp_information::Access::Oauth(v) => {
                    struct_ser.serialize_field("oauth", v)?;
                }
                idp_information::Access::Ldap(v) => {
                    struct_ser.serialize_field("ldap", v)?;
                }
                idp_information::Access::Saml(v) => {
                    struct_ser.serialize_field("saml", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_id",
            "idpId",
            "user_id",
            "userId",
            "user_name",
            "userName",
            "raw_information",
            "rawInformation",
            "oauth",
            "ldap",
            "saml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpId,
            UserId,
            UserName,
            RawInformation,
            Oauth,
            Ldap,
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
                            "idpId" | "idp_id" => Ok(GeneratedField::IdpId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "rawInformation" | "raw_information" => Ok(GeneratedField::RawInformation),
                            "oauth" => Ok(GeneratedField::Oauth),
                            "ldap" => Ok(GeneratedField::Ldap),
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
            type Value = IdpInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.IDPInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_id__ = None;
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut raw_information__ = None;
                let mut access__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
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
                        GeneratedField::RawInformation => {
                            if raw_information__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawInformation"));
                            }
                            raw_information__ = map_.next_value()?;
                        }
                        GeneratedField::Oauth => {
                            if access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oauth"));
                            }
                            access__ = map_.next_value::<::std::option::Option<_>>()?.map(idp_information::Access::Oauth)
;
                        }
                        GeneratedField::Ldap => {
                            if access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ldap"));
                            }
                            access__ = map_.next_value::<::std::option::Option<_>>()?.map(idp_information::Access::Ldap)
;
                        }
                        GeneratedField::Saml => {
                            if access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saml"));
                            }
                            access__ = map_.next_value::<::std::option::Option<_>>()?.map(idp_information::Access::Saml)
;
                        }
                    }
                }
                Ok(IdpInformation {
                    idp_id: idp_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    raw_information: raw_information__,
                    access: access__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.IDPInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpIntent {
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.IDPIntent", len)?;
        if !self.idp_intent_id.is_empty() {
            struct_ser.serialize_field("idpIntentId", &self.idp_intent_id)?;
        }
        if !self.idp_intent_token.is_empty() {
            struct_ser.serialize_field("idpIntentToken", &self.idp_intent_token)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpIntent {
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
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpIntentId,
            IdpIntentToken,
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
                            "idpIntentId" | "idp_intent_id" => Ok(GeneratedField::IdpIntentId),
                            "idpIntentToken" | "idp_intent_token" => Ok(GeneratedField::IdpIntentToken),
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
            type Value = IdpIntent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.IDPIntent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpIntent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_intent_id__ = None;
                let mut idp_intent_token__ = None;
                let mut user_id__ = None;
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
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IdpIntent {
                    idp_intent_id: idp_intent_id__.unwrap_or_default(),
                    idp_intent_token: idp_intent_token__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.IDPIntent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpldapAccessInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.IDPLDAPAccessInformation", len)?;
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpldapAccessInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attributes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpldapAccessInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.IDPLDAPAccessInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpldapAccessInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IdpldapAccessInformation {
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.IDPLDAPAccessInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpLink {
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.IDPLink", len)?;
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpLink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_id",
            "idpId",
            "user_id",
            "userId",
            "user_name",
            "userName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpId,
            UserId,
            UserName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpLink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.IDPLink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpLink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_id__ = None;
                let mut user_id__ = None;
                let mut user_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(IdpLink {
                    idp_id: idp_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.IDPLink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpoAuthAccessInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if self.id_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.IDPOAuthAccessInformation", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if let Some(v) = self.id_token.as_ref() {
            struct_ser.serialize_field("idToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpoAuthAccessInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "id_token",
            "idToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            IdToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "idToken" | "id_token" => Ok(GeneratedField::IdToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpoAuthAccessInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.IDPOAuthAccessInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpoAuthAccessInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut id_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IdToken => {
                            if id_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idToken"));
                            }
                            id_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(IdpoAuthAccessInformation {
                    access_token: access_token__.unwrap_or_default(),
                    id_token: id_token__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.IDPOAuthAccessInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdpsamlAccessInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assertion.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.IDPSAMLAccessInformation", len)?;
        if !self.assertion.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("assertion", pbjson::private::base64::encode(&self.assertion).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdpsamlAccessInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assertion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assertion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assertion" => Ok(GeneratedField::Assertion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdpsamlAccessInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.IDPSAMLAccessInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdpsamlAccessInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assertion__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assertion => {
                            if assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertion"));
                            }
                            assertion__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(IdpsamlAccessInformation {
                    assertion: assertion__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.IDPSAMLAccessInformation", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.InUserEmailsQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.InUserEmailsQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.InUserEmailsQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.InUserIDQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.InUserIDQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.InUserIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LdapCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.LDAPCredentials", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LdapCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
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
                            "username" => Ok(GeneratedField::Username),
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
            type Value = LdapCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.LDAPCredentials")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LdapCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LdapCredentials {
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.LDAPCredentials", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.LastNameQuery", len)?;
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.LastNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(LastNameQuery {
                    last_name: last_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.LastNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAuthenticationMethodTypesRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ListAuthenticationMethodTypesRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAuthenticationMethodTypesRequest {
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
            type Value = ListAuthenticationMethodTypesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ListAuthenticationMethodTypesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAuthenticationMethodTypesRequest, V::Error>
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
                Ok(ListAuthenticationMethodTypesRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ListAuthenticationMethodTypesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAuthenticationMethodTypesResponse {
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
        if !self.auth_method_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ListAuthenticationMethodTypesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.auth_method_types.is_empty() {
            let v = self.auth_method_types.iter().cloned().map(|v| {
                AuthenticationMethodType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("authMethodTypes", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAuthenticationMethodTypesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "auth_method_types",
            "authMethodTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            AuthMethodTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "authMethodTypes" | "auth_method_types" => Ok(GeneratedField::AuthMethodTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAuthenticationMethodTypesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ListAuthenticationMethodTypesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAuthenticationMethodTypesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut auth_method_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::AuthMethodTypes => {
                            if auth_method_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authMethodTypes"));
                            }
                            auth_method_types__ = Some(map_.next_value::<Vec<AuthenticationMethodType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(ListAuthenticationMethodTypesResponse {
                    details: details__,
                    auth_method_types: auth_method_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ListAuthenticationMethodTypesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersRequest {
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
        if self.sorting_column != 0 {
            len += 1;
        }
        if !self.queries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ListUsersRequest", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if self.sorting_column != 0 {
            let v = UserFieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "sorting_column",
            "sortingColumn",
            "queries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            SortingColumn,
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
                            "query" => Ok(GeneratedField::Query),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
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
            type Value = ListUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ListUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut sorting_column__ = None;
                let mut queries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<UserFieldName>()? as i32);
                        }
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUsersRequest {
                    query: query__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ListUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersResponse {
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
        if self.sorting_column != 0 {
            len += 1;
        }
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ListUsersResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.sorting_column != 0 {
            let v = UserFieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "sorting_column",
            "sortingColumn",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SortingColumn,
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ListUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut sorting_column__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<UserFieldName>()? as i32);
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUsersResponse {
                    details: details__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ListUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LockUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.LockUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LockUserRequest {
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
            type Value = LockUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.LockUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LockUserRequest, V::Error>
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
                Ok(LockUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.LockUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LockUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.LockUserResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LockUserResponse {
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
            type Value = LockUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.LockUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LockUserResponse, V::Error>
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
                Ok(LockUserResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.LockUserResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.LoginNameQuery", len)?;
        if !self.login_name.is_empty() {
            struct_ser.serialize_field("loginName", &self.login_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.LoginNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(LoginNameQuery {
                    login_name: login_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.LoginNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MachineUser {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.MachineUser", len)?;
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
impl<'de> serde::Deserialize<'de> for MachineUser {
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
            type Value = MachineUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.MachineUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MachineUser, V::Error>
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
                Ok(MachineUser {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    has_secret: has_secret__.unwrap_or_default(),
                    access_token_type: access_token_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.MachineUser", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.NickNameQuery", len)?;
        if !self.nick_name.is_empty() {
            struct_ser.serialize_field("nickName", &self.nick_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.NickNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(NickNameQuery {
                    nick_name: nick_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.NickNameQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.NotQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.NotQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.NotQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "NOTIFICATION_TYPE_Unspecified",
            Self::Email => "NOTIFICATION_TYPE_Email",
            Self::Sms => "NOTIFICATION_TYPE_SMS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NotificationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NOTIFICATION_TYPE_Unspecified",
            "NOTIFICATION_TYPE_Email",
            "NOTIFICATION_TYPE_SMS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationType;

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
                    "NOTIFICATION_TYPE_Unspecified" => Ok(NotificationType::Unspecified),
                    "NOTIFICATION_TYPE_Email" => Ok(NotificationType::Email),
                    "NOTIFICATION_TYPE_SMS" => Ok(NotificationType::Sms),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.OrQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.OrQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.OrQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrganizationIdQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.OrganizationIdQuery", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationIdQuery {
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
            type Value = OrganizationIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.OrganizationIdQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrganizationIdQuery, V::Error>
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
                Ok(OrganizationIdQuery {
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.OrganizationIdQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasskeyAuthenticator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PASSKEY_AUTHENTICATOR_UNSPECIFIED",
            Self::Platform => "PASSKEY_AUTHENTICATOR_PLATFORM",
            Self::CrossPlatform => "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PasskeyAuthenticator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PASSKEY_AUTHENTICATOR_UNSPECIFIED",
            "PASSKEY_AUTHENTICATOR_PLATFORM",
            "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasskeyAuthenticator;

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
                    "PASSKEY_AUTHENTICATOR_UNSPECIFIED" => Ok(PasskeyAuthenticator::Unspecified),
                    "PASSKEY_AUTHENTICATOR_PLATFORM" => Ok(PasskeyAuthenticator::Platform),
                    "PASSKEY_AUTHENTICATOR_CROSS_PLATFORM" => Ok(PasskeyAuthenticator::CrossPlatform),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PasskeyRegistrationCode {
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
        if !self.code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.PasskeyRegistrationCode", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasskeyRegistrationCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
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
            type Value = PasskeyRegistrationCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.PasskeyRegistrationCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasskeyRegistrationCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PasskeyRegistrationCode {
                    id: id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.PasskeyRegistrationCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Password {
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
        if self.change_required {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.Password", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if self.change_required {
            struct_ser.serialize_field("changeRequired", &self.change_required)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Password {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
            "change_required",
            "changeRequired",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
            ChangeRequired,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "changeRequired" | "change_required" => Ok(GeneratedField::ChangeRequired),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Password;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.Password")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Password, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                let mut change_required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChangeRequired => {
                            if change_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeRequired"));
                            }
                            change_required__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Password {
                    password: password__.unwrap_or_default(),
                    change_required: change_required__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.Password", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordResetRequest {
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
        if self.medium.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.PasswordResetRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.medium.as_ref() {
            match v {
                password_reset_request::Medium::SendLink(v) => {
                    struct_ser.serialize_field("sendLink", v)?;
                }
                password_reset_request::Medium::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordResetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "send_link",
            "sendLink",
            "return_code",
            "returnCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            SendLink,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "sendLink" | "send_link" => Ok(GeneratedField::SendLink),
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
            type Value = PasswordResetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.PasswordResetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordResetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut medium__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendLink => {
                            if medium__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendLink"));
                            }
                            medium__ = map_.next_value::<::std::option::Option<_>>()?.map(password_reset_request::Medium::SendLink)
;
                        }
                        GeneratedField::ReturnCode => {
                            if medium__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            medium__ = map_.next_value::<::std::option::Option<_>>()?.map(password_reset_request::Medium::ReturnCode)
;
                        }
                    }
                }
                Ok(PasswordResetRequest {
                    user_id: user_id__.unwrap_or_default(),
                    medium: medium__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.PasswordResetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PasswordResetResponse {
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
        if self.verification_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.PasswordResetResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.verification_code.as_ref() {
            struct_ser.serialize_field("verificationCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PasswordResetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PasswordResetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.PasswordResetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PasswordResetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PasswordResetResponse {
                    details: details__,
                    verification_code: verification_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.PasswordResetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PhoneQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.number.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.PhoneQuery", len)?;
        if !self.number.is_empty() {
            struct_ser.serialize_field("number", &self.number)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PhoneQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "number",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Number,
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
                            "number" => Ok(GeneratedField::Number),
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
            type Value = PhoneQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.PhoneQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PhoneQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut number__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(PhoneQuery {
                    number: number__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.PhoneQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactivateUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ReactivateUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactivateUserRequest {
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
            type Value = ReactivateUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ReactivateUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactivateUserRequest, V::Error>
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
                Ok(ReactivateUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ReactivateUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactivateUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ReactivateUserResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactivateUserResponse {
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
            type Value = ReactivateUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ReactivateUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactivateUserResponse, V::Error>
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
                Ok(ReactivateUserResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ReactivateUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedirectUrLs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.success_url.is_empty() {
            len += 1;
        }
        if !self.failure_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RedirectURLs", len)?;
        if !self.success_url.is_empty() {
            struct_ser.serialize_field("successUrl", &self.success_url)?;
        }
        if !self.failure_url.is_empty() {
            struct_ser.serialize_field("failureUrl", &self.failure_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedirectUrLs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success_url",
            "successUrl",
            "failure_url",
            "failureUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SuccessUrl,
            FailureUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "successUrl" | "success_url" => Ok(GeneratedField::SuccessUrl),
                            "failureUrl" | "failure_url" => Ok(GeneratedField::FailureUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedirectUrLs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RedirectURLs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RedirectUrLs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success_url__ = None;
                let mut failure_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SuccessUrl => {
                            if success_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successUrl"));
                            }
                            success_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FailureUrl => {
                            if failure_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureUrl"));
                            }
                            failure_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RedirectUrLs {
                    success_url: success_url__.unwrap_or_default(),
                    failure_url: failure_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RedirectURLs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterPasskeyRequest {
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
        if self.code.is_some() {
            len += 1;
        }
        if self.authenticator != 0 {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RegisterPasskeyRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if self.authenticator != 0 {
            let v = PasskeyAuthenticator::try_from(self.authenticator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.authenticator)))?;
            struct_ser.serialize_field("authenticator", &v)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterPasskeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "code",
            "authenticator",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Code,
            Authenticator,
            Domain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "code" => Ok(GeneratedField::Code),
                            "authenticator" => Ok(GeneratedField::Authenticator),
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterPasskeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RegisterPasskeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterPasskeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut code__ = None;
                let mut authenticator__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map_.next_value()?;
                        }
                        GeneratedField::Authenticator => {
                            if authenticator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authenticator"));
                            }
                            authenticator__ = Some(map_.next_value::<PasskeyAuthenticator>()? as i32);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegisterPasskeyRequest {
                    user_id: user_id__.unwrap_or_default(),
                    code: code__,
                    authenticator: authenticator__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RegisterPasskeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterPasskeyResponse {
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
        if !self.passkey_id.is_empty() {
            len += 1;
        }
        if self.public_key_credential_creation_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RegisterPasskeyResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.passkey_id.is_empty() {
            struct_ser.serialize_field("passkeyId", &self.passkey_id)?;
        }
        if let Some(v) = self.public_key_credential_creation_options.as_ref() {
            struct_ser.serialize_field("publicKeyCredentialCreationOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterPasskeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "passkey_id",
            "passkeyId",
            "public_key_credential_creation_options",
            "publicKeyCredentialCreationOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            PasskeyId,
            PublicKeyCredentialCreationOptions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "passkeyId" | "passkey_id" => Ok(GeneratedField::PasskeyId),
                            "publicKeyCredentialCreationOptions" | "public_key_credential_creation_options" => Ok(GeneratedField::PublicKeyCredentialCreationOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterPasskeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RegisterPasskeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterPasskeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut passkey_id__ = None;
                let mut public_key_credential_creation_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::PasskeyId => {
                            if passkey_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passkeyId"));
                            }
                            passkey_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKeyCredentialCreationOptions => {
                            if public_key_credential_creation_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCredentialCreationOptions"));
                            }
                            public_key_credential_creation_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisterPasskeyResponse {
                    details: details__,
                    passkey_id: passkey_id__.unwrap_or_default(),
                    public_key_credential_creation_options: public_key_credential_creation_options__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RegisterPasskeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterTotpRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RegisterTOTPRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterTotpRequest {
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
            type Value = RegisterTotpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RegisterTOTPRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterTotpRequest, V::Error>
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
                Ok(RegisterTotpRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RegisterTOTPRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterTotpResponse {
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
        if !self.uri.is_empty() {
            len += 1;
        }
        if !self.secret.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RegisterTOTPResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if !self.secret.is_empty() {
            struct_ser.serialize_field("secret", &self.secret)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterTotpResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "uri",
            "secret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Uri,
            Secret,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "uri" => Ok(GeneratedField::Uri),
                            "secret" => Ok(GeneratedField::Secret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterTotpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RegisterTOTPResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterTotpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut uri__ = None;
                let mut secret__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegisterTotpResponse {
                    details: details__,
                    uri: uri__.unwrap_or_default(),
                    secret: secret__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RegisterTOTPResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterU2fRequest {
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
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RegisterU2FRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterU2fRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Domain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterU2fRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RegisterU2FRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterU2fRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegisterU2fRequest {
                    user_id: user_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RegisterU2FRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterU2fResponse {
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
        if !self.u2f_id.is_empty() {
            len += 1;
        }
        if self.public_key_credential_creation_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RegisterU2FResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.u2f_id.is_empty() {
            struct_ser.serialize_field("u2fId", &self.u2f_id)?;
        }
        if let Some(v) = self.public_key_credential_creation_options.as_ref() {
            struct_ser.serialize_field("publicKeyCredentialCreationOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterU2fResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "u2f_id",
            "u2fId",
            "public_key_credential_creation_options",
            "publicKeyCredentialCreationOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            U2fId,
            PublicKeyCredentialCreationOptions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "u2fId" | "u2f_id" => Ok(GeneratedField::U2fId),
                            "publicKeyCredentialCreationOptions" | "public_key_credential_creation_options" => Ok(GeneratedField::PublicKeyCredentialCreationOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterU2fResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RegisterU2FResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterU2fResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut u2f_id__ = None;
                let mut public_key_credential_creation_options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::U2fId => {
                            if u2f_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u2fId"));
                            }
                            u2f_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKeyCredentialCreationOptions => {
                            if public_key_credential_creation_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCredentialCreationOptions"));
                            }
                            public_key_credential_creation_options__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RegisterU2fResponse {
                    details: details__,
                    u2f_id: u2f_id__.unwrap_or_default(),
                    public_key_credential_creation_options: public_key_credential_creation_options__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RegisterU2FResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOtpEmailRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemoveOTPEmailRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOtpEmailRequest {
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
            type Value = RemoveOtpEmailRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemoveOTPEmailRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOtpEmailRequest, V::Error>
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
                Ok(RemoveOtpEmailRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemoveOTPEmailRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOtpEmailResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemoveOTPEmailResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOtpEmailResponse {
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
            type Value = RemoveOtpEmailResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemoveOTPEmailResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOtpEmailResponse, V::Error>
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
                Ok(RemoveOtpEmailResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemoveOTPEmailResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOtpsmsRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemoveOTPSMSRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOtpsmsRequest {
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
            type Value = RemoveOtpsmsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemoveOTPSMSRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOtpsmsRequest, V::Error>
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
                Ok(RemoveOtpsmsRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemoveOTPSMSRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveOtpsmsResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemoveOTPSMSResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveOtpsmsResponse {
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
            type Value = RemoveOtpsmsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemoveOTPSMSResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveOtpsmsResponse, V::Error>
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
                Ok(RemoveOtpsmsResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemoveOTPSMSResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePhoneRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemovePhoneRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePhoneRequest {
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
            type Value = RemovePhoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemovePhoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePhoneRequest, V::Error>
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
                Ok(RemovePhoneRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemovePhoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovePhoneResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemovePhoneResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovePhoneResponse {
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
            type Value = RemovePhoneResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemovePhoneResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovePhoneResponse, V::Error>
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
                Ok(RemovePhoneResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemovePhoneResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveTotpRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemoveTOTPRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveTotpRequest {
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
            type Value = RemoveTotpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemoveTOTPRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveTotpRequest, V::Error>
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
                Ok(RemoveTotpRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemoveTOTPRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveTotpResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RemoveTOTPResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveTotpResponse {
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
            type Value = RemoveTotpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RemoveTOTPResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveTotpResponse, V::Error>
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
                Ok(RemoveTotpResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RemoveTOTPResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResendEmailCodeRequest {
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
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ResendEmailCodeRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                resend_email_code_request::Verification::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                resend_email_code_request::Verification::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResendEmailCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = ResendEmailCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ResendEmailCodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResendEmailCodeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(resend_email_code_request::Verification::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(resend_email_code_request::Verification::ReturnCode)
;
                        }
                    }
                }
                Ok(ResendEmailCodeRequest {
                    user_id: user_id__.unwrap_or_default(),
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ResendEmailCodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResendEmailCodeResponse {
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
        if self.verification_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ResendEmailCodeResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.verification_code.as_ref() {
            struct_ser.serialize_field("verificationCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResendEmailCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResendEmailCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ResendEmailCodeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResendEmailCodeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResendEmailCodeResponse {
                    details: details__,
                    verification_code: verification_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ResendEmailCodeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResendPhoneCodeRequest {
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
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ResendPhoneCodeRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                resend_phone_code_request::Verification::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                resend_phone_code_request::Verification::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResendPhoneCodeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = ResendPhoneCodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ResendPhoneCodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResendPhoneCodeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(resend_phone_code_request::Verification::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(resend_phone_code_request::Verification::ReturnCode)
;
                        }
                    }
                }
                Ok(ResendPhoneCodeRequest {
                    user_id: user_id__.unwrap_or_default(),
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ResendPhoneCodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResendPhoneCodeResponse {
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
        if self.verification_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ResendPhoneCodeResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.verification_code.as_ref() {
            struct_ser.serialize_field("verificationCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResendPhoneCodeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResendPhoneCodeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ResendPhoneCodeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResendPhoneCodeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResendPhoneCodeResponse {
                    details: details__,
                    verification_code: verification_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ResendPhoneCodeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetrieveIdentityProviderIntentRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RetrieveIdentityProviderIntentRequest", len)?;
        if !self.idp_intent_id.is_empty() {
            struct_ser.serialize_field("idpIntentId", &self.idp_intent_id)?;
        }
        if !self.idp_intent_token.is_empty() {
            struct_ser.serialize_field("idpIntentToken", &self.idp_intent_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetrieveIdentityProviderIntentRequest {
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
            type Value = RetrieveIdentityProviderIntentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RetrieveIdentityProviderIntentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetrieveIdentityProviderIntentRequest, V::Error>
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
                Ok(RetrieveIdentityProviderIntentRequest {
                    idp_intent_id: idp_intent_id__.unwrap_or_default(),
                    idp_intent_token: idp_intent_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RetrieveIdentityProviderIntentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetrieveIdentityProviderIntentResponse {
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
        if self.idp_information.is_some() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.RetrieveIdentityProviderIntentResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.idp_information.as_ref() {
            struct_ser.serialize_field("idpInformation", v)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetrieveIdentityProviderIntentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "idp_information",
            "idpInformation",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            IdpInformation,
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
                            "details" => Ok(GeneratedField::Details),
                            "idpInformation" | "idp_information" => Ok(GeneratedField::IdpInformation),
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
            type Value = RetrieveIdentityProviderIntentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.RetrieveIdentityProviderIntentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetrieveIdentityProviderIntentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut idp_information__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::IdpInformation => {
                            if idp_information__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpInformation"));
                            }
                            idp_information__ = map_.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RetrieveIdentityProviderIntentResponse {
                    details: details__,
                    idp_information: idp_information__,
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.RetrieveIdentityProviderIntentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnEmailVerificationCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ReturnEmailVerificationCode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnEmailVerificationCode {
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
            type Value = ReturnEmailVerificationCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ReturnEmailVerificationCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnEmailVerificationCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReturnEmailVerificationCode {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ReturnEmailVerificationCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnPasskeyRegistrationCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ReturnPasskeyRegistrationCode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnPasskeyRegistrationCode {
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
            type Value = ReturnPasskeyRegistrationCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ReturnPasskeyRegistrationCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnPasskeyRegistrationCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReturnPasskeyRegistrationCode {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ReturnPasskeyRegistrationCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnPasswordResetCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ReturnPasswordResetCode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnPasswordResetCode {
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
            type Value = ReturnPasswordResetCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ReturnPasswordResetCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnPasswordResetCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReturnPasswordResetCode {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ReturnPasswordResetCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnPhoneVerificationCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v2beta.ReturnPhoneVerificationCode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnPhoneVerificationCode {
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
            type Value = ReturnPhoneVerificationCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.ReturnPhoneVerificationCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnPhoneVerificationCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReturnPhoneVerificationCode {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.ReturnPhoneVerificationCode", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SearchQuery", len)?;
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
                search_query::Query::OrganizationIdQuery(v) => {
                    struct_ser.serialize_field("organizationIdQuery", v)?;
                }
                search_query::Query::PhoneQuery(v) => {
                    struct_ser.serialize_field("phoneQuery", v)?;
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
            "organization_id_query",
            "organizationIdQuery",
            "phone_query",
            "phoneQuery",
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
            OrganizationIdQuery,
            PhoneQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "organizationIdQuery" | "organization_id_query" => Ok(GeneratedField::OrganizationIdQuery),
                            "phoneQuery" | "phone_query" => Ok(GeneratedField::PhoneQuery),
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
                formatter.write_str("struct zitadel.user.v2beta.SearchQuery")
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
                        GeneratedField::OrganizationIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::OrganizationIdQuery)
;
                        }
                        GeneratedField::PhoneQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::PhoneQuery)
;
                        }
                    }
                }
                Ok(SearchQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SearchQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendEmailVerificationCode {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SendEmailVerificationCode", len)?;
        if let Some(v) = self.url_template.as_ref() {
            struct_ser.serialize_field("urlTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendEmailVerificationCode {
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
            type Value = SendEmailVerificationCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SendEmailVerificationCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendEmailVerificationCode, V::Error>
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
                Ok(SendEmailVerificationCode {
                    url_template: url_template__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SendEmailVerificationCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendPasskeyRegistrationLink {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SendPasskeyRegistrationLink", len)?;
        if let Some(v) = self.url_template.as_ref() {
            struct_ser.serialize_field("urlTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendPasskeyRegistrationLink {
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
            type Value = SendPasskeyRegistrationLink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SendPasskeyRegistrationLink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendPasskeyRegistrationLink, V::Error>
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
                Ok(SendPasskeyRegistrationLink {
                    url_template: url_template__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SendPasskeyRegistrationLink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendPasswordResetLink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.notification_type != 0 {
            len += 1;
        }
        if self.url_template.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SendPasswordResetLink", len)?;
        if self.notification_type != 0 {
            let v = NotificationType::try_from(self.notification_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.notification_type)))?;
            struct_ser.serialize_field("notificationType", &v)?;
        }
        if let Some(v) = self.url_template.as_ref() {
            struct_ser.serialize_field("urlTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendPasswordResetLink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification_type",
            "notificationType",
            "url_template",
            "urlTemplate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotificationType,
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
                            "notificationType" | "notification_type" => Ok(GeneratedField::NotificationType),
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
            type Value = SendPasswordResetLink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SendPasswordResetLink")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendPasswordResetLink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut notification_type__ = None;
                let mut url_template__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NotificationType => {
                            if notification_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationType"));
                            }
                            notification_type__ = Some(map_.next_value::<NotificationType>()? as i32);
                        }
                        GeneratedField::UrlTemplate => {
                            if url_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlTemplate"));
                            }
                            url_template__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SendPasswordResetLink {
                    notification_type: notification_type__.unwrap_or_default(),
                    url_template: url_template__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SendPasswordResetLink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SendPhoneVerificationCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SendPhoneVerificationCode", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SendPhoneVerificationCode {
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
            type Value = SendPhoneVerificationCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SendPhoneVerificationCode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SendPhoneVerificationCode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SendPhoneVerificationCode {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SendPhoneVerificationCode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetEmailRequest {
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
        if !self.email.is_empty() {
            len += 1;
        }
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetEmailRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                set_email_request::Verification::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                set_email_request::Verification::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
                set_email_request::Verification::IsVerified(v) => {
                    struct_ser.serialize_field("isVerified", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetEmailRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "email",
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
            "is_verified",
            "isVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Email,
            SendCode,
            ReturnCode,
            IsVerified,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "email" => Ok(GeneratedField::Email),
                            "sendCode" | "send_code" => Ok(GeneratedField::SendCode),
                            "returnCode" | "return_code" => Ok(GeneratedField::ReturnCode),
                            "isVerified" | "is_verified" => Ok(GeneratedField::IsVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetEmailRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetEmailRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetEmailRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut email__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_email_request::Verification::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_email_request::Verification::ReturnCode)
;
                        }
                        GeneratedField::IsVerified => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVerified"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_email_request::Verification::IsVerified);
                        }
                    }
                }
                Ok(SetEmailRequest {
                    user_id: user_id__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetEmailRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetEmailResponse {
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
        if self.verification_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetEmailResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.verification_code.as_ref() {
            struct_ser.serialize_field("verificationCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetEmailResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetEmailResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetEmailResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetEmailResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetEmailResponse {
                    details: details__,
                    verification_code: verification_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetEmailResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetHumanEmail {
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
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetHumanEmail", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                set_human_email::Verification::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                set_human_email::Verification::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
                set_human_email::Verification::IsVerified(v) => {
                    struct_ser.serialize_field("isVerified", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetHumanEmail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
            "is_verified",
            "isVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            SendCode,
            ReturnCode,
            IsVerified,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "sendCode" | "send_code" => Ok(GeneratedField::SendCode),
                            "returnCode" | "return_code" => Ok(GeneratedField::ReturnCode),
                            "isVerified" | "is_verified" => Ok(GeneratedField::IsVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetHumanEmail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetHumanEmail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetHumanEmail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_human_email::Verification::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_human_email::Verification::ReturnCode)
;
                        }
                        GeneratedField::IsVerified => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVerified"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_human_email::Verification::IsVerified);
                        }
                    }
                }
                Ok(SetHumanEmail {
                    email: email__.unwrap_or_default(),
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetHumanEmail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetHumanPhone {
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
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetHumanPhone", len)?;
        if !self.phone.is_empty() {
            struct_ser.serialize_field("phone", &self.phone)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                set_human_phone::Verification::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                set_human_phone::Verification::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
                set_human_phone::Verification::IsVerified(v) => {
                    struct_ser.serialize_field("isVerified", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetHumanPhone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phone",
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
            "is_verified",
            "isVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phone,
            SendCode,
            ReturnCode,
            IsVerified,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "sendCode" | "send_code" => Ok(GeneratedField::SendCode),
                            "returnCode" | "return_code" => Ok(GeneratedField::ReturnCode),
                            "isVerified" | "is_verified" => Ok(GeneratedField::IsVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetHumanPhone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetHumanPhone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetHumanPhone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_human_phone::Verification::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_human_phone::Verification::ReturnCode)
;
                        }
                        GeneratedField::IsVerified => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVerified"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_human_phone::Verification::IsVerified);
                        }
                    }
                }
                Ok(SetHumanPhone {
                    phone: phone__.unwrap_or_default(),
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetHumanPhone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetHumanProfile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.given_name.is_empty() {
            len += 1;
        }
        if !self.family_name.is_empty() {
            len += 1;
        }
        if self.nick_name.is_some() {
            len += 1;
        }
        if self.display_name.is_some() {
            len += 1;
        }
        if self.preferred_language.is_some() {
            len += 1;
        }
        if self.gender.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetHumanProfile", len)?;
        if !self.given_name.is_empty() {
            struct_ser.serialize_field("givenName", &self.given_name)?;
        }
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        if let Some(v) = self.nick_name.as_ref() {
            struct_ser.serialize_field("nickName", v)?;
        }
        if let Some(v) = self.display_name.as_ref() {
            struct_ser.serialize_field("displayName", v)?;
        }
        if let Some(v) = self.preferred_language.as_ref() {
            struct_ser.serialize_field("preferredLanguage", v)?;
        }
        if let Some(v) = self.gender.as_ref() {
            let v = Gender::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("gender", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetHumanProfile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "given_name",
            "givenName",
            "family_name",
            "familyName",
            "nick_name",
            "nickName",
            "display_name",
            "displayName",
            "preferred_language",
            "preferredLanguage",
            "gender",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GivenName,
            FamilyName,
            NickName,
            DisplayName,
            PreferredLanguage,
            Gender,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "givenName" | "given_name" => Ok(GeneratedField::GivenName),
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            "nickName" | "nick_name" => Ok(GeneratedField::NickName),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "preferredLanguage" | "preferred_language" => Ok(GeneratedField::PreferredLanguage),
                            "gender" => Ok(GeneratedField::Gender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetHumanProfile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetHumanProfile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetHumanProfile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut given_name__ = None;
                let mut family_name__ = None;
                let mut nick_name__ = None;
                let mut display_name__ = None;
                let mut preferred_language__ = None;
                let mut gender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GivenName => {
                            if given_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("givenName"));
                            }
                            given_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NickName => {
                            if nick_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickName"));
                            }
                            nick_name__ = map_.next_value()?;
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = map_.next_value()?;
                        }
                        GeneratedField::PreferredLanguage => {
                            if preferred_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguage"));
                            }
                            preferred_language__ = map_.next_value()?;
                        }
                        GeneratedField::Gender => {
                            if gender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            gender__ = map_.next_value::<::std::option::Option<Gender>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(SetHumanProfile {
                    given_name: given_name__.unwrap_or_default(),
                    family_name: family_name__.unwrap_or_default(),
                    nick_name: nick_name__,
                    display_name: display_name__,
                    preferred_language: preferred_language__,
                    gender: gender__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetHumanProfile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetMetadataEntry {
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
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetMetadataEntry", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetMetadataEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetMetadataEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetMetadataEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetMetadataEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SetMetadataEntry {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetMetadataEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPassword {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.password_type.is_some() {
            len += 1;
        }
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetPassword", len)?;
        if let Some(v) = self.password_type.as_ref() {
            match v {
                set_password::PasswordType::Password(v) => {
                    struct_ser.serialize_field("password", v)?;
                }
                set_password::PasswordType::HashedPassword(v) => {
                    struct_ser.serialize_field("hashedPassword", v)?;
                }
            }
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                set_password::Verification::CurrentPassword(v) => {
                    struct_ser.serialize_field("currentPassword", v)?;
                }
                set_password::Verification::VerificationCode(v) => {
                    struct_ser.serialize_field("verificationCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPassword {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
            "hashed_password",
            "hashedPassword",
            "current_password",
            "currentPassword",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
            HashedPassword,
            CurrentPassword,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "hashedPassword" | "hashed_password" => Ok(GeneratedField::HashedPassword),
                            "currentPassword" | "current_password" => Ok(GeneratedField::CurrentPassword),
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPassword;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetPassword")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPassword, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password_type__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password_type__ = map_.next_value::<::std::option::Option<_>>()?.map(set_password::PasswordType::Password)
;
                        }
                        GeneratedField::HashedPassword => {
                            if password_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashedPassword"));
                            }
                            password_type__ = map_.next_value::<::std::option::Option<_>>()?.map(set_password::PasswordType::HashedPassword)
;
                        }
                        GeneratedField::CurrentPassword => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPassword"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_password::Verification::CurrentPassword);
                        }
                        GeneratedField::VerificationCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_password::Verification::VerificationCode);
                        }
                    }
                }
                Ok(SetPassword {
                    password_type: password_type__,
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetPassword", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPasswordRequest {
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
        if self.new_password.is_some() {
            len += 1;
        }
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetPasswordRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.new_password.as_ref() {
            struct_ser.serialize_field("newPassword", v)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                set_password_request::Verification::CurrentPassword(v) => {
                    struct_ser.serialize_field("currentPassword", v)?;
                }
                set_password_request::Verification::VerificationCode(v) => {
                    struct_ser.serialize_field("verificationCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPasswordRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "new_password",
            "newPassword",
            "current_password",
            "currentPassword",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            NewPassword,
            CurrentPassword,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "newPassword" | "new_password" => Ok(GeneratedField::NewPassword),
                            "currentPassword" | "current_password" => Ok(GeneratedField::CurrentPassword),
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPasswordRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetPasswordRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPasswordRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut new_password__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewPassword => {
                            if new_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newPassword"));
                            }
                            new_password__ = map_.next_value()?;
                        }
                        GeneratedField::CurrentPassword => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentPassword"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_password_request::Verification::CurrentPassword);
                        }
                        GeneratedField::VerificationCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_password_request::Verification::VerificationCode);
                        }
                    }
                }
                Ok(SetPasswordRequest {
                    user_id: user_id__.unwrap_or_default(),
                    new_password: new_password__,
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetPasswordRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPasswordResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetPasswordResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPasswordResponse {
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
            type Value = SetPasswordResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetPasswordResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPasswordResponse, V::Error>
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
                Ok(SetPasswordResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetPasswordResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPhoneRequest {
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
        if !self.phone.is_empty() {
            len += 1;
        }
        if self.verification.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetPhoneRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.phone.is_empty() {
            struct_ser.serialize_field("phone", &self.phone)?;
        }
        if let Some(v) = self.verification.as_ref() {
            match v {
                set_phone_request::Verification::SendCode(v) => {
                    struct_ser.serialize_field("sendCode", v)?;
                }
                set_phone_request::Verification::ReturnCode(v) => {
                    struct_ser.serialize_field("returnCode", v)?;
                }
                set_phone_request::Verification::IsVerified(v) => {
                    struct_ser.serialize_field("isVerified", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPhoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "phone",
            "send_code",
            "sendCode",
            "return_code",
            "returnCode",
            "is_verified",
            "isVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Phone,
            SendCode,
            ReturnCode,
            IsVerified,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "phone" => Ok(GeneratedField::Phone),
                            "sendCode" | "send_code" => Ok(GeneratedField::SendCode),
                            "returnCode" | "return_code" => Ok(GeneratedField::ReturnCode),
                            "isVerified" | "is_verified" => Ok(GeneratedField::IsVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPhoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetPhoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPhoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut phone__ = None;
                let mut verification__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SendCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_phone_request::Verification::SendCode)
;
                        }
                        GeneratedField::ReturnCode => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnCode"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_phone_request::Verification::ReturnCode)
;
                        }
                        GeneratedField::IsVerified => {
                            if verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isVerified"));
                            }
                            verification__ = map_.next_value::<::std::option::Option<_>>()?.map(set_phone_request::Verification::IsVerified);
                        }
                    }
                }
                Ok(SetPhoneRequest {
                    user_id: user_id__.unwrap_or_default(),
                    phone: phone__.unwrap_or_default(),
                    verification: verification__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetPhoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPhoneResponse {
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
        if self.verification_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.SetPhoneResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.verification_code.as_ref() {
            struct_ser.serialize_field("verificationCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPhoneResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPhoneResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.SetPhoneResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPhoneResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetPhoneResponse {
                    details: details__,
                    verification_code: verification_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.SetPhoneResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartIdentityProviderIntentRequest {
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
        if self.content.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.StartIdentityProviderIntentRequest", len)?;
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if let Some(v) = self.content.as_ref() {
            match v {
                start_identity_provider_intent_request::Content::Urls(v) => {
                    struct_ser.serialize_field("urls", v)?;
                }
                start_identity_provider_intent_request::Content::Ldap(v) => {
                    struct_ser.serialize_field("ldap", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartIdentityProviderIntentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_id",
            "idpId",
            "urls",
            "ldap",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpId,
            Urls,
            Ldap,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "urls" => Ok(GeneratedField::Urls),
                            "ldap" => Ok(GeneratedField::Ldap),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartIdentityProviderIntentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.StartIdentityProviderIntentRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartIdentityProviderIntentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_id__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Urls => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urls"));
                            }
                            content__ = map_.next_value::<::std::option::Option<_>>()?.map(start_identity_provider_intent_request::Content::Urls)
;
                        }
                        GeneratedField::Ldap => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ldap"));
                            }
                            content__ = map_.next_value::<::std::option::Option<_>>()?.map(start_identity_provider_intent_request::Content::Ldap)
;
                        }
                    }
                }
                Ok(StartIdentityProviderIntentRequest {
                    idp_id: idp_id__.unwrap_or_default(),
                    content: content__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.StartIdentityProviderIntentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartIdentityProviderIntentResponse {
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
        if self.next_step.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.StartIdentityProviderIntentResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.next_step.as_ref() {
            match v {
                start_identity_provider_intent_response::NextStep::AuthUrl(v) => {
                    struct_ser.serialize_field("authUrl", v)?;
                }
                start_identity_provider_intent_response::NextStep::IdpIntent(v) => {
                    struct_ser.serialize_field("idpIntent", v)?;
                }
                start_identity_provider_intent_response::NextStep::PostForm(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("postForm", pbjson::private::base64::encode(&v).as_str())?;
                }
                start_identity_provider_intent_response::NextStep::FormData(v) => {
                    struct_ser.serialize_field("formData", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartIdentityProviderIntentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "auth_url",
            "authUrl",
            "idp_intent",
            "idpIntent",
            "post_form",
            "postForm",
            "form_data",
            "formData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            AuthUrl,
            IdpIntent,
            PostForm,
            FormData,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "authUrl" | "auth_url" => Ok(GeneratedField::AuthUrl),
                            "idpIntent" | "idp_intent" => Ok(GeneratedField::IdpIntent),
                            "postForm" | "post_form" => Ok(GeneratedField::PostForm),
                            "formData" | "form_data" => Ok(GeneratedField::FormData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartIdentityProviderIntentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.StartIdentityProviderIntentResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartIdentityProviderIntentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut next_step__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::AuthUrl => {
                            if next_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authUrl"));
                            }
                            next_step__ = map_.next_value::<::std::option::Option<_>>()?.map(start_identity_provider_intent_response::NextStep::AuthUrl);
                        }
                        GeneratedField::IdpIntent => {
                            if next_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpIntent"));
                            }
                            next_step__ = map_.next_value::<::std::option::Option<_>>()?.map(start_identity_provider_intent_response::NextStep::IdpIntent)
;
                        }
                        GeneratedField::PostForm => {
                            if next_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postForm"));
                            }
                            next_step__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| start_identity_provider_intent_response::NextStep::PostForm(x.0));
                        }
                        GeneratedField::FormData => {
                            if next_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("formData"));
                            }
                            next_step__ = map_.next_value::<::std::option::Option<_>>()?.map(start_identity_provider_intent_response::NextStep::FormData)
;
                        }
                    }
                }
                Ok(StartIdentityProviderIntentResponse {
                    details: details__,
                    next_step: next_step__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.StartIdentityProviderIntentResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.StateQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.StateQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.StateQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.TypeQuery", len)?;
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
                formatter.write_str("struct zitadel.user.v2beta.TypeQuery")
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
        deserializer.deserialize_struct("zitadel.user.v2beta.TypeQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnlockUserRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.UnlockUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnlockUserRequest {
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
            type Value = UnlockUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.UnlockUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnlockUserRequest, V::Error>
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
                Ok(UnlockUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.UnlockUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnlockUserResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.UnlockUserResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnlockUserResponse {
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
            type Value = UnlockUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.UnlockUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnlockUserResponse, V::Error>
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
                Ok(UnlockUserResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.UnlockUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateHumanUserRequest {
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
        if self.username.is_some() {
            len += 1;
        }
        if self.profile.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if self.password.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.UpdateHumanUserRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.username.as_ref() {
            struct_ser.serialize_field("username", v)?;
        }
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if let Some(v) = self.password.as_ref() {
            struct_ser.serialize_field("password", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateHumanUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "username",
            "profile",
            "email",
            "phone",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Username,
            Profile,
            Email,
            Phone,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "username" => Ok(GeneratedField::Username),
                            "profile" => Ok(GeneratedField::Profile),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
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
            type Value = UpdateHumanUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.UpdateHumanUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateHumanUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut username__ = None;
                let mut profile__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = map_.next_value()?;
                        }
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
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateHumanUserRequest {
                    user_id: user_id__.unwrap_or_default(),
                    username: username__,
                    profile: profile__,
                    email: email__,
                    phone: phone__,
                    password: password__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.UpdateHumanUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateHumanUserResponse {
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
        if self.email_code.is_some() {
            len += 1;
        }
        if self.phone_code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.UpdateHumanUserResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if let Some(v) = self.email_code.as_ref() {
            struct_ser.serialize_field("emailCode", v)?;
        }
        if let Some(v) = self.phone_code.as_ref() {
            struct_ser.serialize_field("phoneCode", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateHumanUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "email_code",
            "emailCode",
            "phone_code",
            "phoneCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            EmailCode,
            PhoneCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "emailCode" | "email_code" => Ok(GeneratedField::EmailCode),
                            "phoneCode" | "phone_code" => Ok(GeneratedField::PhoneCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateHumanUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.UpdateHumanUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateHumanUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut email_code__ = None;
                let mut phone_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::EmailCode => {
                            if email_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailCode"));
                            }
                            email_code__ = map_.next_value()?;
                        }
                        GeneratedField::PhoneCode => {
                            if phone_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneCode"));
                            }
                            phone_code__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateHumanUserResponse {
                    details: details__,
                    email_code: email_code__,
                    phone_code: phone_code__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.UpdateHumanUserResponse", FIELDS, GeneratedVisitor)
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.username.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.User", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.state != 0 {
            let v = UserState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
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
            "user_id",
            "userId",
            "details",
            "state",
            "username",
            "login_names",
            "loginNames",
            "preferred_login_name",
            "preferredLoginName",
            "human",
            "machine",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Details,
            State,
            Username,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "details" => Ok(GeneratedField::Details),
                            "state" => Ok(GeneratedField::State),
                            "username" => Ok(GeneratedField::Username),
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
                formatter.write_str("struct zitadel.user.v2beta.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut details__ = None;
                let mut state__ = None;
                let mut username__ = None;
                let mut login_names__ = None;
                let mut preferred_login_name__ = None;
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
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<UserState>()? as i32);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
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
                    user_id: user_id__.unwrap_or_default(),
                    details: details__,
                    state: state__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    login_names: login_names__.unwrap_or_default(),
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.User", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.UserNameQuery", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if self.method != 0 {
            let v = super::super::object::v2beta::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.user.v2beta.UserNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::object::v2beta::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(UserNameQuery {
                    user_name: user_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.UserNameQuery", FIELDS, GeneratedVisitor)
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
                    "USER_STATE_INITIAL" => Ok(UserState::Initial),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyEmailRequest {
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
        if !self.verification_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyEmailRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.verification_code.is_empty() {
            struct_ser.serialize_field("verificationCode", &self.verification_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyEmailRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VerifyEmailRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyEmailRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyEmailRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyEmailRequest {
                    user_id: user_id__.unwrap_or_default(),
                    verification_code: verification_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyEmailRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyEmailResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyEmailResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyEmailResponse {
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
            type Value = VerifyEmailResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyEmailResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyEmailResponse, V::Error>
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
                Ok(VerifyEmailResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyEmailResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyPasskeyRegistrationRequest {
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
        if !self.passkey_id.is_empty() {
            len += 1;
        }
        if self.public_key_credential.is_some() {
            len += 1;
        }
        if !self.passkey_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyPasskeyRegistrationRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.passkey_id.is_empty() {
            struct_ser.serialize_field("passkeyId", &self.passkey_id)?;
        }
        if let Some(v) = self.public_key_credential.as_ref() {
            struct_ser.serialize_field("publicKeyCredential", v)?;
        }
        if !self.passkey_name.is_empty() {
            struct_ser.serialize_field("passkeyName", &self.passkey_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyPasskeyRegistrationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "passkey_id",
            "passkeyId",
            "public_key_credential",
            "publicKeyCredential",
            "passkey_name",
            "passkeyName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            PasskeyId,
            PublicKeyCredential,
            PasskeyName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "passkeyId" | "passkey_id" => Ok(GeneratedField::PasskeyId),
                            "publicKeyCredential" | "public_key_credential" => Ok(GeneratedField::PublicKeyCredential),
                            "passkeyName" | "passkey_name" => Ok(GeneratedField::PasskeyName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VerifyPasskeyRegistrationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyPasskeyRegistrationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyPasskeyRegistrationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut passkey_id__ = None;
                let mut public_key_credential__ = None;
                let mut passkey_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasskeyId => {
                            if passkey_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passkeyId"));
                            }
                            passkey_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKeyCredential => {
                            if public_key_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCredential"));
                            }
                            public_key_credential__ = map_.next_value()?;
                        }
                        GeneratedField::PasskeyName => {
                            if passkey_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passkeyName"));
                            }
                            passkey_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyPasskeyRegistrationRequest {
                    user_id: user_id__.unwrap_or_default(),
                    passkey_id: passkey_id__.unwrap_or_default(),
                    public_key_credential: public_key_credential__,
                    passkey_name: passkey_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyPasskeyRegistrationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyPasskeyRegistrationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyPasskeyRegistrationResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyPasskeyRegistrationResponse {
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
            type Value = VerifyPasskeyRegistrationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyPasskeyRegistrationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyPasskeyRegistrationResponse, V::Error>
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
                Ok(VerifyPasskeyRegistrationResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyPasskeyRegistrationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyPhoneRequest {
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
        if !self.verification_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyPhoneRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.verification_code.is_empty() {
            struct_ser.serialize_field("verificationCode", &self.verification_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyPhoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "verification_code",
            "verificationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            VerificationCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "verificationCode" | "verification_code" => Ok(GeneratedField::VerificationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VerifyPhoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyPhoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyPhoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut verification_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VerificationCode => {
                            if verification_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationCode"));
                            }
                            verification_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyPhoneRequest {
                    user_id: user_id__.unwrap_or_default(),
                    verification_code: verification_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyPhoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyPhoneResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyPhoneResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyPhoneResponse {
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
            type Value = VerifyPhoneResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyPhoneResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyPhoneResponse, V::Error>
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
                Ok(VerifyPhoneResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyPhoneResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyTotpRegistrationRequest {
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
        if !self.code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyTOTPRegistrationRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyTotpRegistrationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = VerifyTotpRegistrationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyTOTPRegistrationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyTotpRegistrationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyTotpRegistrationRequest {
                    user_id: user_id__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyTOTPRegistrationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyTotpRegistrationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyTOTPRegistrationResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyTotpRegistrationResponse {
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
            type Value = VerifyTotpRegistrationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyTOTPRegistrationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyTotpRegistrationResponse, V::Error>
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
                Ok(VerifyTotpRegistrationResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyTOTPRegistrationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyU2fRegistrationRequest {
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
        if !self.u2f_id.is_empty() {
            len += 1;
        }
        if self.public_key_credential.is_some() {
            len += 1;
        }
        if !self.token_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyU2FRegistrationRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.u2f_id.is_empty() {
            struct_ser.serialize_field("u2fId", &self.u2f_id)?;
        }
        if let Some(v) = self.public_key_credential.as_ref() {
            struct_ser.serialize_field("publicKeyCredential", v)?;
        }
        if !self.token_name.is_empty() {
            struct_ser.serialize_field("tokenName", &self.token_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyU2fRegistrationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "u2f_id",
            "u2fId",
            "public_key_credential",
            "publicKeyCredential",
            "token_name",
            "tokenName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            U2fId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "u2fId" | "u2f_id" => Ok(GeneratedField::U2fId),
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
            type Value = VerifyU2fRegistrationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyU2FRegistrationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyU2fRegistrationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut u2f_id__ = None;
                let mut public_key_credential__ = None;
                let mut token_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::U2fId => {
                            if u2f_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("u2fId"));
                            }
                            u2f_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicKeyCredential => {
                            if public_key_credential__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeyCredential"));
                            }
                            public_key_credential__ = map_.next_value()?;
                        }
                        GeneratedField::TokenName => {
                            if token_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenName"));
                            }
                            token_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(VerifyU2fRegistrationRequest {
                    user_id: user_id__.unwrap_or_default(),
                    u2f_id: u2f_id__.unwrap_or_default(),
                    public_key_credential: public_key_credential__,
                    token_name: token_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyU2FRegistrationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VerifyU2fRegistrationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.user.v2beta.VerifyU2FRegistrationResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VerifyU2fRegistrationResponse {
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
            type Value = VerifyU2fRegistrationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.user.v2beta.VerifyU2FRegistrationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VerifyU2fRegistrationResponse, V::Error>
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
                Ok(VerifyU2fRegistrationResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.user.v2beta.VerifyU2FRegistrationResponse", FIELDS, GeneratedVisitor)
    }
}
