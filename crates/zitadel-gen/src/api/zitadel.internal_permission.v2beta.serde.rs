// @generated
impl serde::Serialize for Administrator {
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
        if self.change_date.is_some() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if !self.roles.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.Administrator", len)?;
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        if let Some(v) = self.resource.as_ref() {
            match v {
                administrator::Resource::Instance(v) => {
                    struct_ser.serialize_field("instance", v)?;
                }
                administrator::Resource::Organization(v) => {
                    struct_ser.serialize_field("organization", v)?;
                }
                administrator::Resource::Project(v) => {
                    struct_ser.serialize_field("project", v)?;
                }
                administrator::Resource::ProjectGrant(v) => {
                    struct_ser.serialize_field("projectGrant", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Administrator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "user",
            "roles",
            "instance",
            "organization",
            "project",
            "project_grant",
            "projectGrant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreationDate,
            ChangeDate,
            User,
            Roles,
            Instance,
            Organization,
            Project,
            ProjectGrant,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "user" => Ok(GeneratedField::User),
                            "roles" => Ok(GeneratedField::Roles),
                            "instance" => Ok(GeneratedField::Instance),
                            "organization" => Ok(GeneratedField::Organization),
                            "project" => Ok(GeneratedField::Project),
                            "projectGrant" | "project_grant" => Ok(GeneratedField::ProjectGrant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Administrator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.Administrator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Administrator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut user__ = None;
                let mut roles__ = None;
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Instance => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator::Resource::Instance);
                        }
                        GeneratedField::Organization => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator::Resource::Organization)
;
                        }
                        GeneratedField::Project => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator::Resource::Project)
;
                        }
                        GeneratedField::ProjectGrant => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrant"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator::Resource::ProjectGrant)
;
                        }
                    }
                }
                Ok(Administrator {
                    creation_date: creation_date__,
                    change_date: change_date__,
                    user: user__,
                    roles: roles__.unwrap_or_default(),
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.Administrator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AdministratorFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ADMINISTRATOR_FIELD_NAME_UNSPECIFIED",
            Self::UserId => "ADMINISTRATOR_FIELD_NAME_USER_ID",
            Self::CreationDate => "ADMINISTRATOR_FIELD_NAME_CREATION_DATE",
            Self::ChangeDate => "ADMINISTRATOR_FIELD_NAME_CHANGE_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AdministratorFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ADMINISTRATOR_FIELD_NAME_UNSPECIFIED",
            "ADMINISTRATOR_FIELD_NAME_USER_ID",
            "ADMINISTRATOR_FIELD_NAME_CREATION_DATE",
            "ADMINISTRATOR_FIELD_NAME_CHANGE_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdministratorFieldName;

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
                    "ADMINISTRATOR_FIELD_NAME_UNSPECIFIED" => Ok(AdministratorFieldName::Unspecified),
                    "ADMINISTRATOR_FIELD_NAME_USER_ID" => Ok(AdministratorFieldName::UserId),
                    "ADMINISTRATOR_FIELD_NAME_CREATION_DATE" => Ok(AdministratorFieldName::CreationDate),
                    "ADMINISTRATOR_FIELD_NAME_CHANGE_DATE" => Ok(AdministratorFieldName::ChangeDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AdministratorSearchFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.AdministratorSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                administrator_search_filter::Filter::CreationDate(v) => {
                    struct_ser.serialize_field("creationDate", v)?;
                }
                administrator_search_filter::Filter::ChangeDate(v) => {
                    struct_ser.serialize_field("changeDate", v)?;
                }
                administrator_search_filter::Filter::InUserIdsFilter(v) => {
                    struct_ser.serialize_field("inUserIdsFilter", v)?;
                }
                administrator_search_filter::Filter::UserOrganizationId(v) => {
                    struct_ser.serialize_field("userOrganizationId", v)?;
                }
                administrator_search_filter::Filter::UserPreferredLoginName(v) => {
                    struct_ser.serialize_field("userPreferredLoginName", v)?;
                }
                administrator_search_filter::Filter::UserDisplayName(v) => {
                    struct_ser.serialize_field("userDisplayName", v)?;
                }
                administrator_search_filter::Filter::Resource(v) => {
                    struct_ser.serialize_field("resource", v)?;
                }
                administrator_search_filter::Filter::Role(v) => {
                    struct_ser.serialize_field("role", v)?;
                }
                administrator_search_filter::Filter::And(v) => {
                    struct_ser.serialize_field("and", v)?;
                }
                administrator_search_filter::Filter::Or(v) => {
                    struct_ser.serialize_field("or", v)?;
                }
                administrator_search_filter::Filter::Not(v) => {
                    struct_ser.serialize_field("not", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdministratorSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "in_user_ids_filter",
            "inUserIdsFilter",
            "user_organization_id",
            "userOrganizationId",
            "user_preferred_login_name",
            "userPreferredLoginName",
            "user_display_name",
            "userDisplayName",
            "resource",
            "role",
            "and",
            "or",
            "not",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreationDate,
            ChangeDate,
            InUserIdsFilter,
            UserOrganizationId,
            UserPreferredLoginName,
            UserDisplayName,
            Resource,
            Role,
            And,
            Or,
            Not,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "inUserIdsFilter" | "in_user_ids_filter" => Ok(GeneratedField::InUserIdsFilter),
                            "userOrganizationId" | "user_organization_id" => Ok(GeneratedField::UserOrganizationId),
                            "userPreferredLoginName" | "user_preferred_login_name" => Ok(GeneratedField::UserPreferredLoginName),
                            "userDisplayName" | "user_display_name" => Ok(GeneratedField::UserDisplayName),
                            "resource" => Ok(GeneratedField::Resource),
                            "role" => Ok(GeneratedField::Role),
                            "and" => Ok(GeneratedField::And),
                            "or" => Ok(GeneratedField::Or),
                            "not" => Ok(GeneratedField::Not),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdministratorSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.AdministratorSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AdministratorSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreationDate => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::CreationDate)
;
                        }
                        GeneratedField::ChangeDate => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::ChangeDate)
;
                        }
                        GeneratedField::InUserIdsFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inUserIdsFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::InUserIdsFilter)
;
                        }
                        GeneratedField::UserOrganizationId => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userOrganizationId"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::UserOrganizationId)
;
                        }
                        GeneratedField::UserPreferredLoginName => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userPreferredLoginName"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::UserPreferredLoginName)
;
                        }
                        GeneratedField::UserDisplayName => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDisplayName"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::UserDisplayName)
;
                        }
                        GeneratedField::Resource => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::Resource)
;
                        }
                        GeneratedField::Role => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::Role)
;
                        }
                        GeneratedField::And => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("and"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::And)
;
                        }
                        GeneratedField::Or => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("or"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::Or)
;
                        }
                        GeneratedField::Not => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("not"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(administrator_search_filter::Filter::Not)
;
                        }
                    }
                }
                Ok(AdministratorSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.AdministratorSearchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AndFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.AndFilter", len)?;
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AndFilter {
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
            type Value = AndFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.AndFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AndFilter, V::Error>
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
                Ok(AndFilter {
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.AndFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAdministratorRequest {
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
        if self.resource.is_some() {
            len += 1;
        }
        if !self.roles.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.CreateAdministratorRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAdministratorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "resource",
            "roles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Resource,
            Roles,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "resource" => Ok(GeneratedField::Resource),
                            "roles" => Ok(GeneratedField::Roles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAdministratorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.CreateAdministratorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAdministratorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut resource__ = None;
                let mut roles__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateAdministratorRequest {
                    user_id: user_id__.unwrap_or_default(),
                    resource: resource__,
                    roles: roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.CreateAdministratorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAdministratorResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.CreateAdministratorResponse", len)?;
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAdministratorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_date",
            "creationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateAdministratorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.CreateAdministratorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAdministratorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut creation_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateAdministratorResponse {
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.CreateAdministratorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAdministratorRequest {
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
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.DeleteAdministratorRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAdministratorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Resource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAdministratorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.DeleteAdministratorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAdministratorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteAdministratorRequest {
                    user_id: user_id__.unwrap_or_default(),
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.DeleteAdministratorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAdministratorResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.DeleteAdministratorResponse", len)?;
        if let Some(v) = self.deletion_date.as_ref() {
            struct_ser.serialize_field("deletionDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAdministratorResponse {
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
            type Value = DeleteAdministratorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.DeleteAdministratorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAdministratorResponse, V::Error>
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
                Ok(DeleteAdministratorResponse {
                    deletion_date: deletion_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.DeleteAdministratorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAdministratorRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.GetAdministratorRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAdministratorRequest {
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
            type Value = GetAdministratorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.GetAdministratorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAdministratorRequest, V::Error>
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
                Ok(GetAdministratorRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.GetAdministratorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAdministratorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.administrator.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.GetAdministratorResponse", len)?;
        if let Some(v) = self.administrator.as_ref() {
            struct_ser.serialize_field("administrator", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAdministratorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "administrator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Administrator,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "administrator" => Ok(GeneratedField::Administrator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAdministratorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.GetAdministratorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAdministratorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut administrator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Administrator => {
                            if administrator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("administrator"));
                            }
                            administrator__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAdministratorResponse {
                    administrator: administrator__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.GetAdministratorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAdministratorsRequest {
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
        if self.sorting_column.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.ListAdministratorsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.sorting_column.as_ref() {
            let v = AdministratorFieldName::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAdministratorsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "sorting_column",
            "sortingColumn",
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            SortingColumn,
            Filters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAdministratorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.ListAdministratorsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAdministratorsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut sorting_column__ = None;
                let mut filters__ = None;
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
                            sorting_column__ = map_.next_value::<::std::option::Option<AdministratorFieldName>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAdministratorsRequest {
                    pagination: pagination__,
                    sorting_column: sorting_column__,
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.ListAdministratorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAdministratorsResponse {
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
        if !self.administrators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.ListAdministratorsResponse", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.administrators.is_empty() {
            struct_ser.serialize_field("administrators", &self.administrators)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAdministratorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "administrators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            Administrators,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "administrators" => Ok(GeneratedField::Administrators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAdministratorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.ListAdministratorsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAdministratorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut administrators__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Administrators => {
                            if administrators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("administrators"));
                            }
                            administrators__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAdministratorsResponse {
                    pagination: pagination__,
                    administrators: administrators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.ListAdministratorsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.NotFilter", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotFilter {
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
            type Value = NotFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.NotFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotFilter, V::Error>
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
                Ok(NotFilter {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.NotFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.OrFilter", len)?;
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrFilter {
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
            type Value = OrFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.OrFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrFilter, V::Error>
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
                Ok(OrFilter {
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.OrFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Organization {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.Organization", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Organization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Organization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.Organization")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Organization, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
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
                    }
                }
                Ok(Organization {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.Organization", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Project {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.Project", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Project {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = Project;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.Project")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Project, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut organization_id__ = None;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Project {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.Project", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectGrant {
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
        if !self.project_name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.granted_organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.ProjectGrant", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.project_name.is_empty() {
            struct_ser.serialize_field("projectName", &self.project_name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.granted_organization_id.is_empty() {
            struct_ser.serialize_field("grantedOrganizationId", &self.granted_organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectGrant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "project_id",
            "projectId",
            "project_name",
            "projectName",
            "organization_id",
            "organizationId",
            "granted_organization_id",
            "grantedOrganizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProjectId,
            ProjectName,
            OrganizationId,
            GrantedOrganizationId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "grantedOrganizationId" | "granted_organization_id" => Ok(GeneratedField::GrantedOrganizationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectGrant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.ProjectGrant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectGrant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut project_id__ = None;
                let mut project_name__ = None;
                let mut organization_id__ = None;
                let mut granted_organization_id__ = None;
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
                        GeneratedField::ProjectName => {
                            if project_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectName"));
                            }
                            project_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedOrganizationId => {
                            if granted_organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrganizationId"));
                            }
                            granted_organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectGrant {
                    id: id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    project_name: project_name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    granted_organization_id: granted_organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.ProjectGrant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.ResourceFilter", len)?;
        if let Some(v) = self.resource.as_ref() {
            match v {
                resource_filter::Resource::Instance(v) => {
                    struct_ser.serialize_field("instance", v)?;
                }
                resource_filter::Resource::OrganizationId(v) => {
                    struct_ser.serialize_field("organizationId", v)?;
                }
                resource_filter::Resource::ProjectId(v) => {
                    struct_ser.serialize_field("projectId", v)?;
                }
                resource_filter::Resource::ProjectGrantId(v) => {
                    struct_ser.serialize_field("projectGrantId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "organization_id",
            "organizationId",
            "project_id",
            "projectId",
            "project_grant_id",
            "projectGrantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            OrganizationId,
            ProjectId,
            ProjectGrantId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instance" => Ok(GeneratedField::Instance),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.ResourceFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_filter::Resource::Instance);
                        }
                        GeneratedField::OrganizationId => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_filter::Resource::OrganizationId);
                        }
                        GeneratedField::ProjectId => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_filter::Resource::ProjectId);
                        }
                        GeneratedField::ProjectGrantId => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_filter::Resource::ProjectGrantId);
                        }
                    }
                }
                Ok(ResourceFilter {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.ResourceFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.ResourceType", len)?;
        if let Some(v) = self.resource.as_ref() {
            match v {
                resource_type::Resource::Instance(v) => {
                    struct_ser.serialize_field("instance", v)?;
                }
                resource_type::Resource::OrganizationId(v) => {
                    struct_ser.serialize_field("organizationId", v)?;
                }
                resource_type::Resource::ProjectId(v) => {
                    struct_ser.serialize_field("projectId", v)?;
                }
                resource_type::Resource::ProjectGrant(v) => {
                    struct_ser.serialize_field("projectGrant", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "organization_id",
            "organizationId",
            "project_id",
            "projectId",
            "project_grant",
            "projectGrant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            OrganizationId,
            ProjectId,
            ProjectGrant,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "instance" => Ok(GeneratedField::Instance),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectGrant" | "project_grant" => Ok(GeneratedField::ProjectGrant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.ResourceType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_type::Resource::Instance);
                        }
                        GeneratedField::OrganizationId => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_type::Resource::OrganizationId);
                        }
                        GeneratedField::ProjectId => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_type::Resource::ProjectId);
                        }
                        GeneratedField::ProjectGrant => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrant"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_type::Resource::ProjectGrant)
;
                        }
                    }
                }
                Ok(ResourceType {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.ResourceType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for resource_type::ProjectGrant {
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
        if !self.project_grant_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.ResourceType.ProjectGrant", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.project_grant_id.is_empty() {
            struct_ser.serialize_field("projectGrantId", &self.project_grant_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for resource_type::ProjectGrant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "project_grant_id",
            "projectGrantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            ProjectGrantId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = resource_type::ProjectGrant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.ResourceType.ProjectGrant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<resource_type::ProjectGrant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut project_grant_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectGrantId => {
                            if project_grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            project_grant_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(resource_type::ProjectGrant {
                    project_id: project_id__.unwrap_or_default(),
                    project_grant_id: project_grant_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.ResourceType.ProjectGrant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoleFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.role_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.RoleFilter", len)?;
        if !self.role_key.is_empty() {
            struct_ser.serialize_field("roleKey", &self.role_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoleFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "role_key",
            "roleKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoleKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "roleKey" | "role_key" => Ok(GeneratedField::RoleKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoleFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.RoleFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoleFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut role_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoleKey => {
                            if role_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKey"));
                            }
                            role_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RoleFilter {
                    role_key: role_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.RoleFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAdministratorRequest {
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
        if self.resource.is_some() {
            len += 1;
        }
        if !self.roles.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.UpdateAdministratorRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAdministratorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "resource",
            "roles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Resource,
            Roles,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "resource" => Ok(GeneratedField::Resource),
                            "roles" => Ok(GeneratedField::Roles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAdministratorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.UpdateAdministratorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAdministratorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut resource__ = None;
                let mut roles__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAdministratorRequest {
                    user_id: user_id__.unwrap_or_default(),
                    resource: resource__,
                    roles: roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.UpdateAdministratorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAdministratorResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.UpdateAdministratorResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAdministratorResponse {
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
            type Value = UpdateAdministratorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.UpdateAdministratorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAdministratorResponse, V::Error>
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
                Ok(UpdateAdministratorResponse {
                    change_date: change_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.UpdateAdministratorResponse", FIELDS, GeneratedVisitor)
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
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.preferred_login_name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.User", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
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
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "preferred_login_name",
            "preferredLoginName",
            "display_name",
            "displayName",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            PreferredLoginName,
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
                            "id" => Ok(GeneratedField::Id),
                            "preferredLoginName" | "preferred_login_name" => Ok(GeneratedField::PreferredLoginName),
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
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut preferred_login_name__ = None;
                let mut display_name__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredLoginName => {
                            if preferred_login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLoginName"));
                            }
                            preferred_login_name__ = Some(map_.next_value()?);
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
                Ok(User {
                    id: id__.unwrap_or_default(),
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserDisplayNameFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.UserDisplayNameFilter", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2beta::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserDisplayNameFilter {
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
            type Value = UserDisplayNameFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.UserDisplayNameFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserDisplayNameFilter, V::Error>
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
                            method__ = Some(map_.next_value::<super::super::filter::v2beta::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(UserDisplayNameFilter {
                    display_name: display_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.UserDisplayNameFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserPreferredLoginNameFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.preferred_login_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.internal_permission.v2beta.UserPreferredLoginNameFilter", len)?;
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2beta::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserPreferredLoginNameFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "preferred_login_name",
            "preferredLoginName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreferredLoginName,
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
                            "preferredLoginName" | "preferred_login_name" => Ok(GeneratedField::PreferredLoginName),
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
            type Value = UserPreferredLoginNameFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.internal_permission.v2beta.UserPreferredLoginNameFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserPreferredLoginNameFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut preferred_login_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PreferredLoginName => {
                            if preferred_login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLoginName"));
                            }
                            preferred_login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::filter::v2beta::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(UserPreferredLoginNameFilter {
                    preferred_login_name: preferred_login_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.internal_permission.v2beta.UserPreferredLoginNameFilter", FIELDS, GeneratedVisitor)
    }
}
