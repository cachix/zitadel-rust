// @generated
impl serde::Serialize for ActivateAuthorizationRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.ActivateAuthorizationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivateAuthorizationRequest {
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
            type Value = ActivateAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.ActivateAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivateAuthorizationRequest, V::Error>
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
                Ok(ActivateAuthorizationRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.ActivateAuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActivateAuthorizationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.ActivateAuthorizationResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivateAuthorizationResponse {
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
            type Value = ActivateAuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.ActivateAuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivateAuthorizationResponse, V::Error>
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
                Ok(ActivateAuthorizationResponse {
                    change_date: change_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.ActivateAuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Authorization {
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
        if !self.project_organization_id.is_empty() {
            len += 1;
        }
        if self.project_grant_id.is_some() {
            len += 1;
        }
        if self.granted_organization_id.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
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
        if self.user.is_some() {
            len += 1;
        }
        if !self.roles.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.Authorization", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.project_name.is_empty() {
            struct_ser.serialize_field("projectName", &self.project_name)?;
        }
        if !self.project_organization_id.is_empty() {
            struct_ser.serialize_field("projectOrganizationId", &self.project_organization_id)?;
        }
        if let Some(v) = self.project_grant_id.as_ref() {
            struct_ser.serialize_field("projectGrantId", v)?;
        }
        if let Some(v) = self.granted_organization_id.as_ref() {
            struct_ser.serialize_field("grantedOrganizationId", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if self.state != 0 {
            let v = State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Authorization {
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
            "project_organization_id",
            "projectOrganizationId",
            "project_grant_id",
            "projectGrantId",
            "granted_organization_id",
            "grantedOrganizationId",
            "organization_id",
            "organizationId",
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "state",
            "user",
            "roles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProjectId,
            ProjectName,
            ProjectOrganizationId,
            ProjectGrantId,
            GrantedOrganizationId,
            OrganizationId,
            CreationDate,
            ChangeDate,
            State,
            User,
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
                            "id" => Ok(GeneratedField::Id),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            "projectOrganizationId" | "project_organization_id" => Ok(GeneratedField::ProjectOrganizationId),
                            "projectGrantId" | "project_grant_id" => Ok(GeneratedField::ProjectGrantId),
                            "grantedOrganizationId" | "granted_organization_id" => Ok(GeneratedField::GrantedOrganizationId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "state" => Ok(GeneratedField::State),
                            "user" => Ok(GeneratedField::User),
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
            type Value = Authorization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.Authorization")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Authorization, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut project_id__ = None;
                let mut project_name__ = None;
                let mut project_organization_id__ = None;
                let mut project_grant_id__ = None;
                let mut granted_organization_id__ = None;
                let mut organization_id__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut state__ = None;
                let mut user__ = None;
                let mut roles__ = None;
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
                        GeneratedField::ProjectOrganizationId => {
                            if project_organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectOrganizationId"));
                            }
                            project_organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectGrantId => {
                            if project_grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            project_grant_id__ = map_.next_value()?;
                        }
                        GeneratedField::GrantedOrganizationId => {
                            if granted_organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrganizationId"));
                            }
                            granted_organization_id__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
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
                            state__ = Some(map_.next_value::<State>()? as i32);
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
                    }
                }
                Ok(Authorization {
                    id: id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    project_name: project_name__.unwrap_or_default(),
                    project_organization_id: project_organization_id__.unwrap_or_default(),
                    project_grant_id: project_grant_id__,
                    granted_organization_id: granted_organization_id__,
                    organization_id: organization_id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    change_date: change_date__,
                    state: state__.unwrap_or_default(),
                    user: user__,
                    roles: roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.Authorization", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizationFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTHORIZATION_FIELD_NAME_UNSPECIFIED",
            Self::CreatedDate => "AUTHORIZATION_FIELD_NAME_CREATED_DATE",
            Self::ChangedDate => "AUTHORIZATION_FIELD_NAME_CHANGED_DATE",
            Self::Id => "AUTHORIZATION_FIELD_NAME_ID",
            Self::UserId => "AUTHORIZATION_FIELD_NAME_USER_ID",
            Self::ProjectId => "AUTHORIZATION_FIELD_NAME_PROJECT_ID",
            Self::OrganizationId => "AUTHORIZATION_FIELD_NAME_ORGANIZATION_ID",
            Self::UserOrganizationId => "AUTHORIZATION_FIELD_NAME_USER_ORGANIZATION_ID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTHORIZATION_FIELD_NAME_UNSPECIFIED",
            "AUTHORIZATION_FIELD_NAME_CREATED_DATE",
            "AUTHORIZATION_FIELD_NAME_CHANGED_DATE",
            "AUTHORIZATION_FIELD_NAME_ID",
            "AUTHORIZATION_FIELD_NAME_USER_ID",
            "AUTHORIZATION_FIELD_NAME_PROJECT_ID",
            "AUTHORIZATION_FIELD_NAME_ORGANIZATION_ID",
            "AUTHORIZATION_FIELD_NAME_USER_ORGANIZATION_ID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationFieldName;

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
                    "AUTHORIZATION_FIELD_NAME_UNSPECIFIED" => Ok(AuthorizationFieldName::Unspecified),
                    "AUTHORIZATION_FIELD_NAME_CREATED_DATE" => Ok(AuthorizationFieldName::CreatedDate),
                    "AUTHORIZATION_FIELD_NAME_CHANGED_DATE" => Ok(AuthorizationFieldName::ChangedDate),
                    "AUTHORIZATION_FIELD_NAME_ID" => Ok(AuthorizationFieldName::Id),
                    "AUTHORIZATION_FIELD_NAME_USER_ID" => Ok(AuthorizationFieldName::UserId),
                    "AUTHORIZATION_FIELD_NAME_PROJECT_ID" => Ok(AuthorizationFieldName::ProjectId),
                    "AUTHORIZATION_FIELD_NAME_ORGANIZATION_ID" => Ok(AuthorizationFieldName::OrganizationId),
                    "AUTHORIZATION_FIELD_NAME_USER_ORGANIZATION_ID" => Ok(AuthorizationFieldName::UserOrganizationId),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizationsSearchFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.AuthorizationsSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                authorizations_search_filter::Filter::AuthorizationIds(v) => {
                    struct_ser.serialize_field("authorizationIds", v)?;
                }
                authorizations_search_filter::Filter::OrganizationId(v) => {
                    struct_ser.serialize_field("organizationId", v)?;
                }
                authorizations_search_filter::Filter::State(v) => {
                    struct_ser.serialize_field("state", v)?;
                }
                authorizations_search_filter::Filter::UserId(v) => {
                    struct_ser.serialize_field("userId", v)?;
                }
                authorizations_search_filter::Filter::UserOrganizationId(v) => {
                    struct_ser.serialize_field("userOrganizationId", v)?;
                }
                authorizations_search_filter::Filter::UserPreferredLoginName(v) => {
                    struct_ser.serialize_field("userPreferredLoginName", v)?;
                }
                authorizations_search_filter::Filter::UserDisplayName(v) => {
                    struct_ser.serialize_field("userDisplayName", v)?;
                }
                authorizations_search_filter::Filter::ProjectId(v) => {
                    struct_ser.serialize_field("projectId", v)?;
                }
                authorizations_search_filter::Filter::ProjectName(v) => {
                    struct_ser.serialize_field("projectName", v)?;
                }
                authorizations_search_filter::Filter::RoleKey(v) => {
                    struct_ser.serialize_field("roleKey", v)?;
                }
                authorizations_search_filter::Filter::ProjectGrantId(v) => {
                    struct_ser.serialize_field("projectGrantId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationsSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_ids",
            "authorizationIds",
            "organization_id",
            "organizationId",
            "state",
            "user_id",
            "userId",
            "user_organization_id",
            "userOrganizationId",
            "user_preferred_login_name",
            "userPreferredLoginName",
            "user_display_name",
            "userDisplayName",
            "project_id",
            "projectId",
            "project_name",
            "projectName",
            "role_key",
            "roleKey",
            "project_grant_id",
            "projectGrantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationIds,
            OrganizationId,
            State,
            UserId,
            UserOrganizationId,
            UserPreferredLoginName,
            UserDisplayName,
            ProjectId,
            ProjectName,
            RoleKey,
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
                            "authorizationIds" | "authorization_ids" => Ok(GeneratedField::AuthorizationIds),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "state" => Ok(GeneratedField::State),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userOrganizationId" | "user_organization_id" => Ok(GeneratedField::UserOrganizationId),
                            "userPreferredLoginName" | "user_preferred_login_name" => Ok(GeneratedField::UserPreferredLoginName),
                            "userDisplayName" | "user_display_name" => Ok(GeneratedField::UserDisplayName),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            "roleKey" | "role_key" => Ok(GeneratedField::RoleKey),
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
            type Value = AuthorizationsSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.AuthorizationsSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationsSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationIds => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationIds"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::AuthorizationIds)
;
                        }
                        GeneratedField::OrganizationId => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::OrganizationId)
;
                        }
                        GeneratedField::State => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::State)
;
                        }
                        GeneratedField::UserId => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::UserId)
;
                        }
                        GeneratedField::UserOrganizationId => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userOrganizationId"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::UserOrganizationId)
;
                        }
                        GeneratedField::UserPreferredLoginName => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userPreferredLoginName"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::UserPreferredLoginName)
;
                        }
                        GeneratedField::UserDisplayName => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDisplayName"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::UserDisplayName)
;
                        }
                        GeneratedField::ProjectId => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::ProjectId)
;
                        }
                        GeneratedField::ProjectName => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectName"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::ProjectName)
;
                        }
                        GeneratedField::RoleKey => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKey"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::RoleKey)
;
                        }
                        GeneratedField::ProjectGrantId => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantId"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(authorizations_search_filter::Filter::ProjectGrantId)
;
                        }
                    }
                }
                Ok(AuthorizationsSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.AuthorizationsSearchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAuthorizationRequest {
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
        if !self.project_id.is_empty() {
            len += 1;
        }
        if self.organization_id.is_some() {
            len += 1;
        }
        if !self.role_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.CreateAuthorizationRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.organization_id.as_ref() {
            struct_ser.serialize_field("organizationId", v)?;
        }
        if !self.role_keys.is_empty() {
            struct_ser.serialize_field("roleKeys", &self.role_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAuthorizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "project_id",
            "projectId",
            "organization_id",
            "organizationId",
            "role_keys",
            "roleKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            ProjectId,
            OrganizationId,
            RoleKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "roleKeys" | "role_keys" => Ok(GeneratedField::RoleKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.CreateAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAuthorizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut project_id__ = None;
                let mut organization_id__ = None;
                let mut role_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = map_.next_value()?;
                        }
                        GeneratedField::RoleKeys => {
                            if role_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKeys"));
                            }
                            role_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateAuthorizationRequest {
                    user_id: user_id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    organization_id: organization_id__,
                    role_keys: role_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.CreateAuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAuthorizationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.CreateAuthorizationResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAuthorizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
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
            type Value = CreateAuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.CreateAuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAuthorizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
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
                    }
                }
                Ok(CreateAuthorizationResponse {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.CreateAuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateAuthorizationRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.DeactivateAuthorizationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateAuthorizationRequest {
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
            type Value = DeactivateAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.DeactivateAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateAuthorizationRequest, V::Error>
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
                Ok(DeactivateAuthorizationRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.DeactivateAuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeactivateAuthorizationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.DeactivateAuthorizationResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeactivateAuthorizationResponse {
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
            type Value = DeactivateAuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.DeactivateAuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeactivateAuthorizationResponse, V::Error>
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
                Ok(DeactivateAuthorizationResponse {
                    change_date: change_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.DeactivateAuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAuthorizationRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.DeleteAuthorizationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAuthorizationRequest {
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
            type Value = DeleteAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.DeleteAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAuthorizationRequest, V::Error>
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
                Ok(DeleteAuthorizationRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.DeleteAuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAuthorizationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.DeleteAuthorizationResponse", len)?;
        if let Some(v) = self.deletion_date.as_ref() {
            struct_ser.serialize_field("deletionDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAuthorizationResponse {
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
            type Value = DeleteAuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.DeleteAuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAuthorizationResponse, V::Error>
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
                Ok(DeleteAuthorizationResponse {
                    deletion_date: deletion_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.DeleteAuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAuthorizationsRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.ListAuthorizationsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.sorting_column.as_ref() {
            let v = AuthorizationFieldName::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAuthorizationsRequest {
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
            type Value = ListAuthorizationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.ListAuthorizationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAuthorizationsRequest, V::Error>
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
                            sorting_column__ = map_.next_value::<::std::option::Option<AuthorizationFieldName>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAuthorizationsRequest {
                    pagination: pagination__,
                    sorting_column: sorting_column__,
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.ListAuthorizationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAuthorizationsResponse {
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
        if !self.authorizations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.ListAuthorizationsResponse", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.authorizations.is_empty() {
            struct_ser.serialize_field("authorizations", &self.authorizations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAuthorizationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "authorizations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            Authorizations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "authorizations" => Ok(GeneratedField::Authorizations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAuthorizationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.ListAuthorizationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAuthorizationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut authorizations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Authorizations => {
                            if authorizations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizations"));
                            }
                            authorizations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAuthorizationsResponse {
                    pagination: pagination__,
                    authorizations: authorizations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.ListAuthorizationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrganizationNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.OrganizationNameQuery", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2beta::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationNameQuery {
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
            type Value = OrganizationNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.OrganizationNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OrganizationNameQuery, V::Error>
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
                            method__ = Some(map_.next_value::<super::super::filter::v2beta::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(OrganizationNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.OrganizationNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.ProjectNameQuery", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2beta::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectNameQuery {
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
            type Value = ProjectNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.ProjectNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectNameQuery, V::Error>
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
                            method__ = Some(map_.next_value::<super::super::filter::v2beta::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(ProjectNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.ProjectNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoleKeyQuery {
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
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.RoleKeyQuery", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2beta::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoleKeyQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
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
            type Value = RoleKeyQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.RoleKeyQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoleKeyQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::filter::v2beta::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(RoleKeyQuery {
                    key: key__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.RoleKeyQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::Active => "STATE_ACTIVE",
            Self::Inactive => "STATE_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "STATE_ACTIVE",
            "STATE_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = State;

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
                    "STATE_UNSPECIFIED" => Ok(State::Unspecified),
                    "STATE_ACTIVE" => Ok(State::Active),
                    "STATE_INACTIVE" => Ok(State::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.StateQuery", len)?;
        if self.state != 0 {
            let v = State::try_from(self.state)
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
                formatter.write_str("struct zitadel.authorization.v2beta.StateQuery")
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
                            state__ = Some(map_.next_value::<State>()? as i32);
                        }
                    }
                }
                Ok(StateQuery {
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.StateQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAuthorizationRequest {
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
        if !self.role_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.UpdateAuthorizationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.role_keys.is_empty() {
            struct_ser.serialize_field("roleKeys", &self.role_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAuthorizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "role_keys",
            "roleKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            RoleKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "roleKeys" | "role_keys" => Ok(GeneratedField::RoleKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.UpdateAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAuthorizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut role_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoleKeys => {
                            if role_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKeys"));
                            }
                            role_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAuthorizationRequest {
                    id: id__.unwrap_or_default(),
                    role_keys: role_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.UpdateAuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAuthorizationResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.UpdateAuthorizationResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAuthorizationResponse {
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
            type Value = UpdateAuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.UpdateAuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAuthorizationResponse, V::Error>
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
                Ok(UpdateAuthorizationResponse {
                    change_date: change_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.UpdateAuthorizationResponse", FIELDS, GeneratedVisitor)
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
        if !self.avatar_url.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.User", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.preferred_login_name.is_empty() {
            struct_ser.serialize_field("preferredLoginName", &self.preferred_login_name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.avatar_url.is_empty() {
            struct_ser.serialize_field("avatarUrl", &self.avatar_url)?;
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
            "avatar_url",
            "avatarUrl",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            PreferredLoginName,
            DisplayName,
            AvatarUrl,
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
                            "avatarUrl" | "avatar_url" => Ok(GeneratedField::AvatarUrl),
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
                formatter.write_str("struct zitadel.authorization.v2beta.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut preferred_login_name__ = None;
                let mut display_name__ = None;
                let mut avatar_url__ = None;
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
                        GeneratedField::AvatarUrl => {
                            if avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avatarUrl"));
                            }
                            avatar_url__ = Some(map_.next_value()?);
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
                    avatar_url: avatar_url__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserDisplayNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.UserDisplayNameQuery", len)?;
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
impl<'de> serde::Deserialize<'de> for UserDisplayNameQuery {
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
            type Value = UserDisplayNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.UserDisplayNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserDisplayNameQuery, V::Error>
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
                Ok(UserDisplayNameQuery {
                    display_name: display_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.UserDisplayNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserPreferredLoginNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.authorization.v2beta.UserPreferredLoginNameQuery", len)?;
        if !self.login_name.is_empty() {
            struct_ser.serialize_field("loginName", &self.login_name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2beta::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserPreferredLoginNameQuery {
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
            type Value = UserPreferredLoginNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.authorization.v2beta.UserPreferredLoginNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserPreferredLoginNameQuery, V::Error>
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
                            method__ = Some(map_.next_value::<super::super::filter::v2beta::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(UserPreferredLoginNameQuery {
                    login_name: login_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.authorization.v2beta.UserPreferredLoginNameQuery", FIELDS, GeneratedVisitor)
    }
}
