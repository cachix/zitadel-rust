// @generated
impl serde::Serialize for EmailQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.member.v1.EmailQuery", len)?;
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
impl<'de> serde::Deserialize<'de> for EmailQuery {
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
            type Value = EmailQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.member.v1.EmailQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EmailQuery, V::Error>
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
                Ok(EmailQuery {
                    email: email__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.member.v1.EmailQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.member.v1.FirstNameQuery", len)?;
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
                formatter.write_str("struct zitadel.member.v1.FirstNameQuery")
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
        deserializer.deserialize_struct("zitadel.member.v1.FirstNameQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.member.v1.LastNameQuery", len)?;
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
                formatter.write_str("struct zitadel.member.v1.LastNameQuery")
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
        deserializer.deserialize_struct("zitadel.member.v1.LastNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Member {
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
        if !self.preferred_login_name.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.first_name.is_empty() {
            len += 1;
        }
        if !self.last_name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.avatar_url.is_empty() {
            len += 1;
        }
        if self.user_type != 0 {
            len += 1;
        }
        if !self.user_resource_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.member.v1.Member", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.avatar_url.is_empty() {
            struct_ser.serialize_field("avatarUrl", &self.avatar_url)?;
        }
        if self.user_type != 0 {
            let v = super::super::user::v1::Type::try_from(self.user_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.user_type)))?;
            struct_ser.serialize_field("userType", &v)?;
        }
        if !self.user_resource_owner.is_empty() {
            struct_ser.serialize_field("userResourceOwner", &self.user_resource_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Member {
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
            "preferred_login_name",
            "preferredLoginName",
            "email",
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "display_name",
            "displayName",
            "avatar_url",
            "avatarUrl",
            "user_type",
            "userType",
            "user_resource_owner",
            "userResourceOwner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Details,
            Roles,
            PreferredLoginName,
            Email,
            FirstName,
            LastName,
            DisplayName,
            AvatarUrl,
            UserType,
            UserResourceOwner,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "preferredLoginName" | "preferred_login_name" => Ok(GeneratedField::PreferredLoginName),
                            "email" => Ok(GeneratedField::Email),
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
                            "userType" | "user_type" => Ok(GeneratedField::UserType),
                            "userResourceOwner" | "user_resource_owner" => Ok(GeneratedField::UserResourceOwner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Member;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.member.v1.Member")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Member, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut details__ = None;
                let mut roles__ = None;
                let mut preferred_login_name__ = None;
                let mut email__ = None;
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut display_name__ = None;
                let mut avatar_url__ = None;
                let mut user_type__ = None;
                let mut user_resource_owner__ = None;
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
                        GeneratedField::PreferredLoginName => {
                            if preferred_login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLoginName"));
                            }
                            preferred_login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
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
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserType => {
                            if user_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userType"));
                            }
                            user_type__ = Some(map_.next_value::<super::super::user::v1::Type>()? as i32);
                        }
                        GeneratedField::UserResourceOwner => {
                            if user_resource_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userResourceOwner"));
                            }
                            user_resource_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Member {
                    user_id: user_id__.unwrap_or_default(),
                    details: details__,
                    roles: roles__.unwrap_or_default(),
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    first_name: first_name__.unwrap_or_default(),
                    last_name: last_name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    avatar_url: avatar_url__.unwrap_or_default(),
                    user_type: user_type__.unwrap_or_default(),
                    user_resource_owner: user_resource_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.member.v1.Member", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MemberFieldColumnName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MemberFieldNameUnspecified => "MEMBER_FIELD_NAME_UNSPECIFIED",
            Self::MemberFieldNameUserId => "MEMBER_FIELD_NAME_USER_ID",
            Self::MemberFieldNameCreationDate => "MEMBER_FIELD_NAME_CREATION_DATE",
            Self::MemberFieldNameChangeDate => "MEMBER_FIELD_NAME_CHANGE_DATE",
            Self::MemberFieldNameUserResourceOwner => "MEMBER_FIELD_NAME_USER_RESOURCE_OWNER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MemberFieldColumnName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MEMBER_FIELD_NAME_UNSPECIFIED",
            "MEMBER_FIELD_NAME_USER_ID",
            "MEMBER_FIELD_NAME_CREATION_DATE",
            "MEMBER_FIELD_NAME_CHANGE_DATE",
            "MEMBER_FIELD_NAME_USER_RESOURCE_OWNER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MemberFieldColumnName;

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
                    "MEMBER_FIELD_NAME_UNSPECIFIED" => Ok(MemberFieldColumnName::MemberFieldNameUnspecified),
                    "MEMBER_FIELD_NAME_USER_ID" => Ok(MemberFieldColumnName::MemberFieldNameUserId),
                    "MEMBER_FIELD_NAME_CREATION_DATE" => Ok(MemberFieldColumnName::MemberFieldNameCreationDate),
                    "MEMBER_FIELD_NAME_CHANGE_DATE" => Ok(MemberFieldColumnName::MemberFieldNameChangeDate),
                    "MEMBER_FIELD_NAME_USER_RESOURCE_OWNER" => Ok(MemberFieldColumnName::MemberFieldNameUserResourceOwner),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.member.v1.SearchQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                search_query::Query::FirstNameQuery(v) => {
                    struct_ser.serialize_field("firstNameQuery", v)?;
                }
                search_query::Query::LastNameQuery(v) => {
                    struct_ser.serialize_field("lastNameQuery", v)?;
                }
                search_query::Query::EmailQuery(v) => {
                    struct_ser.serialize_field("emailQuery", v)?;
                }
                search_query::Query::UserIdQuery(v) => {
                    struct_ser.serialize_field("userIdQuery", v)?;
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
            "first_name_query",
            "firstNameQuery",
            "last_name_query",
            "lastNameQuery",
            "email_query",
            "emailQuery",
            "user_id_query",
            "userIdQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstNameQuery,
            LastNameQuery,
            EmailQuery,
            UserIdQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "firstNameQuery" | "first_name_query" => Ok(GeneratedField::FirstNameQuery),
                            "lastNameQuery" | "last_name_query" => Ok(GeneratedField::LastNameQuery),
                            "emailQuery" | "email_query" => Ok(GeneratedField::EmailQuery),
                            "userIdQuery" | "user_id_query" => Ok(GeneratedField::UserIdQuery),
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
                formatter.write_str("struct zitadel.member.v1.SearchQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::EmailQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::EmailQuery)
;
                        }
                        GeneratedField::UserIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(search_query::Query::UserIdQuery)
;
                        }
                    }
                }
                Ok(SearchQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.member.v1.SearchQuery", FIELDS, GeneratedVisitor)
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.member.v1.UserIDQuery", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
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
            type Value = UserIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.member.v1.UserIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserIdQuery, V::Error>
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
                Ok(UserIdQuery {
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.member.v1.UserIDQuery", FIELDS, GeneratedVisitor)
    }
}
