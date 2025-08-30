// @generated
impl serde::Serialize for AllProjectGrantQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.AllProjectGrantQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                all_project_grant_query::Query::ProjectNameQuery(v) => {
                    struct_ser.serialize_field("projectNameQuery", v)?;
                }
                all_project_grant_query::Query::RoleKeyQuery(v) => {
                    struct_ser.serialize_field("roleKeyQuery", v)?;
                }
                all_project_grant_query::Query::ProjectIdQuery(v) => {
                    struct_ser.serialize_field("projectIdQuery", v)?;
                }
                all_project_grant_query::Query::GrantedOrgIdQuery(v) => {
                    struct_ser.serialize_field("grantedOrgIdQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllProjectGrantQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_name_query",
            "projectNameQuery",
            "role_key_query",
            "roleKeyQuery",
            "project_id_query",
            "projectIdQuery",
            "granted_org_id_query",
            "grantedOrgIdQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectNameQuery,
            RoleKeyQuery,
            ProjectIdQuery,
            GrantedOrgIdQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "projectNameQuery" | "project_name_query" => Ok(GeneratedField::ProjectNameQuery),
                            "roleKeyQuery" | "role_key_query" => Ok(GeneratedField::RoleKeyQuery),
                            "projectIdQuery" | "project_id_query" => Ok(GeneratedField::ProjectIdQuery),
                            "grantedOrgIdQuery" | "granted_org_id_query" => Ok(GeneratedField::GrantedOrgIdQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllProjectGrantQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.AllProjectGrantQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllProjectGrantQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(all_project_grant_query::Query::ProjectNameQuery)
;
                        }
                        GeneratedField::RoleKeyQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKeyQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(all_project_grant_query::Query::RoleKeyQuery)
;
                        }
                        GeneratedField::ProjectIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(all_project_grant_query::Query::ProjectIdQuery)
;
                        }
                        GeneratedField::GrantedOrgIdQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgIdQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(all_project_grant_query::Query::GrantedOrgIdQuery)
;
                        }
                    }
                }
                Ok(AllProjectGrantQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.AllProjectGrantQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrantProjectNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.GrantProjectNameQuery", len)?;
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
impl<'de> serde::Deserialize<'de> for GrantProjectNameQuery {
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
            type Value = GrantProjectNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.GrantProjectNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GrantProjectNameQuery, V::Error>
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
                Ok(GrantProjectNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.GrantProjectNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrantRoleKeyQuery {
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
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.GrantRoleKeyQuery", len)?;
        if !self.role_key.is_empty() {
            struct_ser.serialize_field("roleKey", &self.role_key)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrantRoleKeyQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "role_key",
            "roleKey",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoleKey,
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
                            "roleKey" | "role_key" => Ok(GeneratedField::RoleKey),
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
            type Value = GrantRoleKeyQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.GrantRoleKeyQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GrantRoleKeyQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut role_key__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RoleKey => {
                            if role_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKey"));
                            }
                            role_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(GrantRoleKeyQuery {
                    role_key: role_key__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.GrantRoleKeyQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrantedOrgIdQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.granted_org_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.GrantedOrgIDQuery", len)?;
        if !self.granted_org_id.is_empty() {
            struct_ser.serialize_field("grantedOrgId", &self.granted_org_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrantedOrgIdQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "granted_org_id",
            "grantedOrgId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrantedOrgId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "grantedOrgId" | "granted_org_id" => Ok(GeneratedField::GrantedOrgId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrantedOrgIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.GrantedOrgIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GrantedOrgIdQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut granted_org_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrantedOrgId => {
                            if granted_org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgId"));
                            }
                            granted_org_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GrantedOrgIdQuery {
                    granted_org_id: granted_org_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.GrantedOrgIDQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrantedProject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grant_id.is_empty() {
            len += 1;
        }
        if !self.granted_org_id.is_empty() {
            len += 1;
        }
        if !self.granted_org_name.is_empty() {
            len += 1;
        }
        if !self.granted_role_keys.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.project_name.is_empty() {
            len += 1;
        }
        if !self.project_owner_id.is_empty() {
            len += 1;
        }
        if !self.project_owner_name.is_empty() {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.GrantedProject", len)?;
        if !self.grant_id.is_empty() {
            struct_ser.serialize_field("grantId", &self.grant_id)?;
        }
        if !self.granted_org_id.is_empty() {
            struct_ser.serialize_field("grantedOrgId", &self.granted_org_id)?;
        }
        if !self.granted_org_name.is_empty() {
            struct_ser.serialize_field("grantedOrgName", &self.granted_org_name)?;
        }
        if !self.granted_role_keys.is_empty() {
            struct_ser.serialize_field("grantedRoleKeys", &self.granted_role_keys)?;
        }
        if self.state != 0 {
            let v = ProjectGrantState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.project_name.is_empty() {
            struct_ser.serialize_field("projectName", &self.project_name)?;
        }
        if !self.project_owner_id.is_empty() {
            struct_ser.serialize_field("projectOwnerId", &self.project_owner_id)?;
        }
        if !self.project_owner_name.is_empty() {
            struct_ser.serialize_field("projectOwnerName", &self.project_owner_name)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrantedProject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grant_id",
            "grantId",
            "granted_org_id",
            "grantedOrgId",
            "granted_org_name",
            "grantedOrgName",
            "granted_role_keys",
            "grantedRoleKeys",
            "state",
            "project_id",
            "projectId",
            "project_name",
            "projectName",
            "project_owner_id",
            "projectOwnerId",
            "project_owner_name",
            "projectOwnerName",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrantId,
            GrantedOrgId,
            GrantedOrgName,
            GrantedRoleKeys,
            State,
            ProjectId,
            ProjectName,
            ProjectOwnerId,
            ProjectOwnerName,
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
                            "grantId" | "grant_id" => Ok(GeneratedField::GrantId),
                            "grantedOrgId" | "granted_org_id" => Ok(GeneratedField::GrantedOrgId),
                            "grantedOrgName" | "granted_org_name" => Ok(GeneratedField::GrantedOrgName),
                            "grantedRoleKeys" | "granted_role_keys" => Ok(GeneratedField::GrantedRoleKeys),
                            "state" => Ok(GeneratedField::State),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            "projectOwnerId" | "project_owner_id" => Ok(GeneratedField::ProjectOwnerId),
                            "projectOwnerName" | "project_owner_name" => Ok(GeneratedField::ProjectOwnerName),
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
            type Value = GrantedProject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.GrantedProject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GrantedProject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grant_id__ = None;
                let mut granted_org_id__ = None;
                let mut granted_org_name__ = None;
                let mut granted_role_keys__ = None;
                let mut state__ = None;
                let mut project_id__ = None;
                let mut project_name__ = None;
                let mut project_owner_id__ = None;
                let mut project_owner_name__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrantId => {
                            if grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantId"));
                            }
                            grant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedOrgId => {
                            if granted_org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgId"));
                            }
                            granted_org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedOrgName => {
                            if granted_org_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedOrgName"));
                            }
                            granted_org_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GrantedRoleKeys => {
                            if granted_role_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantedRoleKeys"));
                            }
                            granted_role_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<ProjectGrantState>()? as i32);
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
                        GeneratedField::ProjectOwnerId => {
                            if project_owner_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectOwnerId"));
                            }
                            project_owner_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectOwnerName => {
                            if project_owner_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectOwnerName"));
                            }
                            project_owner_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GrantedProject {
                    grant_id: grant_id__.unwrap_or_default(),
                    granted_org_id: granted_org_id__.unwrap_or_default(),
                    granted_org_name: granted_org_name__.unwrap_or_default(),
                    granted_role_keys: granted_role_keys__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    project_name: project_name__.unwrap_or_default(),
                    project_owner_id: project_owner_id__.unwrap_or_default(),
                    project_owner_name: project_owner_name__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.GrantedProject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrivateLabelingSetting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRIVATE_LABELING_SETTING_UNSPECIFIED",
            Self::EnforceProjectResourceOwnerPolicy => "PRIVATE_LABELING_SETTING_ENFORCE_PROJECT_RESOURCE_OWNER_POLICY",
            Self::AllowLoginUserResourceOwnerPolicy => "PRIVATE_LABELING_SETTING_ALLOW_LOGIN_USER_RESOURCE_OWNER_POLICY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PrivateLabelingSetting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRIVATE_LABELING_SETTING_UNSPECIFIED",
            "PRIVATE_LABELING_SETTING_ENFORCE_PROJECT_RESOURCE_OWNER_POLICY",
            "PRIVATE_LABELING_SETTING_ALLOW_LOGIN_USER_RESOURCE_OWNER_POLICY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrivateLabelingSetting;

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
                    "PRIVATE_LABELING_SETTING_UNSPECIFIED" => Ok(PrivateLabelingSetting::Unspecified),
                    "PRIVATE_LABELING_SETTING_ENFORCE_PROJECT_RESOURCE_OWNER_POLICY" => Ok(PrivateLabelingSetting::EnforceProjectResourceOwnerPolicy),
                    "PRIVATE_LABELING_SETTING_ALLOW_LOGIN_USER_RESOURCE_OWNER_POLICY" => Ok(PrivateLabelingSetting::AllowLoginUserResourceOwnerPolicy),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if self.details.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.project_role_assertion {
            len += 1;
        }
        if self.project_role_check {
            len += 1;
        }
        if self.has_project_check {
            len += 1;
        }
        if self.private_labeling_setting != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.Project", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.state != 0 {
            let v = ProjectState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if self.project_role_assertion {
            struct_ser.serialize_field("projectRoleAssertion", &self.project_role_assertion)?;
        }
        if self.project_role_check {
            struct_ser.serialize_field("projectRoleCheck", &self.project_role_check)?;
        }
        if self.has_project_check {
            struct_ser.serialize_field("hasProjectCheck", &self.has_project_check)?;
        }
        if self.private_labeling_setting != 0 {
            let v = PrivateLabelingSetting::try_from(self.private_labeling_setting)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.private_labeling_setting)))?;
            struct_ser.serialize_field("privateLabelingSetting", &v)?;
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
            "details",
            "name",
            "state",
            "project_role_assertion",
            "projectRoleAssertion",
            "project_role_check",
            "projectRoleCheck",
            "has_project_check",
            "hasProjectCheck",
            "private_labeling_setting",
            "privateLabelingSetting",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Details,
            Name,
            State,
            ProjectRoleAssertion,
            ProjectRoleCheck,
            HasProjectCheck,
            PrivateLabelingSetting,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "name" => Ok(GeneratedField::Name),
                            "state" => Ok(GeneratedField::State),
                            "projectRoleAssertion" | "project_role_assertion" => Ok(GeneratedField::ProjectRoleAssertion),
                            "projectRoleCheck" | "project_role_check" => Ok(GeneratedField::ProjectRoleCheck),
                            "hasProjectCheck" | "has_project_check" => Ok(GeneratedField::HasProjectCheck),
                            "privateLabelingSetting" | "private_labeling_setting" => Ok(GeneratedField::PrivateLabelingSetting),
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
                formatter.write_str("struct zitadel.project.v1.Project")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Project, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut details__ = None;
                let mut name__ = None;
                let mut state__ = None;
                let mut project_role_assertion__ = None;
                let mut project_role_check__ = None;
                let mut has_project_check__ = None;
                let mut private_labeling_setting__ = None;
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
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<ProjectState>()? as i32);
                        }
                        GeneratedField::ProjectRoleAssertion => {
                            if project_role_assertion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoleAssertion"));
                            }
                            project_role_assertion__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectRoleCheck => {
                            if project_role_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoleCheck"));
                            }
                            project_role_check__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HasProjectCheck => {
                            if has_project_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasProjectCheck"));
                            }
                            has_project_check__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivateLabelingSetting => {
                            if private_labeling_setting__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateLabelingSetting"));
                            }
                            private_labeling_setting__ = Some(map_.next_value::<PrivateLabelingSetting>()? as i32);
                        }
                    }
                }
                Ok(Project {
                    id: id__.unwrap_or_default(),
                    details: details__,
                    name: name__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    project_role_assertion: project_role_assertion__.unwrap_or_default(),
                    project_role_check: project_role_check__.unwrap_or_default(),
                    has_project_check: has_project_check__.unwrap_or_default(),
                    private_labeling_setting: private_labeling_setting__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.Project", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectGrantQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.ProjectGrantQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                project_grant_query::Query::ProjectNameQuery(v) => {
                    struct_ser.serialize_field("projectNameQuery", v)?;
                }
                project_grant_query::Query::RoleKeyQuery(v) => {
                    struct_ser.serialize_field("roleKeyQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectGrantQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_name_query",
            "projectNameQuery",
            "role_key_query",
            "roleKeyQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectNameQuery,
            RoleKeyQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "projectNameQuery" | "project_name_query" => Ok(GeneratedField::ProjectNameQuery),
                            "roleKeyQuery" | "role_key_query" => Ok(GeneratedField::RoleKeyQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectGrantQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.ProjectGrantQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectGrantQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(project_grant_query::Query::ProjectNameQuery)
;
                        }
                        GeneratedField::RoleKeyQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleKeyQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(project_grant_query::Query::RoleKeyQuery)
;
                        }
                    }
                }
                Ok(ProjectGrantQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.ProjectGrantQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectGrantState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROJECT_GRANT_STATE_UNSPECIFIED",
            Self::Active => "PROJECT_GRANT_STATE_ACTIVE",
            Self::Inactive => "PROJECT_GRANT_STATE_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProjectGrantState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROJECT_GRANT_STATE_UNSPECIFIED",
            "PROJECT_GRANT_STATE_ACTIVE",
            "PROJECT_GRANT_STATE_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectGrantState;

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
                    "PROJECT_GRANT_STATE_UNSPECIFIED" => Ok(ProjectGrantState::Unspecified),
                    "PROJECT_GRANT_STATE_ACTIVE" => Ok(ProjectGrantState::Active),
                    "PROJECT_GRANT_STATE_INACTIVE" => Ok(ProjectGrantState::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectIdQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.ProjectIDQuery", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectIdQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectIdQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.ProjectIDQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectIdQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectIdQuery {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.ProjectIDQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.ProjectNameQuery", len)?;
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
                formatter.write_str("struct zitadel.project.v1.ProjectNameQuery")
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
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(ProjectNameQuery {
                    name: name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.ProjectNameQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.ProjectQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                project_query::Query::NameQuery(v) => {
                    struct_ser.serialize_field("nameQuery", v)?;
                }
                project_query::Query::ProjectResourceOwnerQuery(v) => {
                    struct_ser.serialize_field("projectResourceOwnerQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name_query",
            "nameQuery",
            "project_resource_owner_query",
            "projectResourceOwnerQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NameQuery,
            ProjectResourceOwnerQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nameQuery" | "name_query" => Ok(GeneratedField::NameQuery),
                            "projectResourceOwnerQuery" | "project_resource_owner_query" => Ok(GeneratedField::ProjectResourceOwnerQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.ProjectQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(project_query::Query::NameQuery)
;
                        }
                        GeneratedField::ProjectResourceOwnerQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectResourceOwnerQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(project_query::Query::ProjectResourceOwnerQuery)
;
                        }
                    }
                }
                Ok(ProjectQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.ProjectQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectResourceOwnerQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.ProjectResourceOwnerQuery", len)?;
        if !self.resource_owner.is_empty() {
            struct_ser.serialize_field("resourceOwner", &self.resource_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectResourceOwnerQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_owner",
            "resourceOwner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceOwner,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "resourceOwner" | "resource_owner" => Ok(GeneratedField::ResourceOwner),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectResourceOwnerQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.ProjectResourceOwnerQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectResourceOwnerQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceOwner => {
                            if resource_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwner"));
                            }
                            resource_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectResourceOwnerQuery {
                    resource_owner: resource_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.ProjectResourceOwnerQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROJECT_STATE_UNSPECIFIED",
            Self::Active => "PROJECT_STATE_ACTIVE",
            Self::Inactive => "PROJECT_STATE_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProjectState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROJECT_STATE_UNSPECIFIED",
            "PROJECT_STATE_ACTIVE",
            "PROJECT_STATE_INACTIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectState;

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
                    "PROJECT_STATE_UNSPECIFIED" => Ok(ProjectState::Unspecified),
                    "PROJECT_STATE_ACTIVE" => Ok(ProjectState::Active),
                    "PROJECT_STATE_INACTIVE" => Ok(ProjectState::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Role {
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
        if self.details.is_some() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.group.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.Role", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.group.is_empty() {
            struct_ser.serialize_field("group", &self.group)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Role {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "details",
            "display_name",
            "displayName",
            "group",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Details,
            DisplayName,
            Group,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "details" => Ok(GeneratedField::Details),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "group" => Ok(GeneratedField::Group),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Role;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.Role")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Role, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut details__ = None;
                let mut display_name__ = None;
                let mut group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Role {
                    key: key__.unwrap_or_default(),
                    details: details__,
                    display_name: display_name__.unwrap_or_default(),
                    group: group__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.Role", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoleDisplayNameQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.RoleDisplayNameQuery", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoleDisplayNameQuery {
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
            type Value = RoleDisplayNameQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.RoleDisplayNameQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoleDisplayNameQuery, V::Error>
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
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(RoleDisplayNameQuery {
                    display_name: display_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.RoleDisplayNameQuery", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.RoleKeyQuery", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.method != 0 {
            let v = super::super::v1::TextQueryMethod::try_from(self.method)
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
                formatter.write_str("struct zitadel.project.v1.RoleKeyQuery")
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
                            method__ = Some(map_.next_value::<super::super::v1::TextQueryMethod>()? as i32);
                        }
                    }
                }
                Ok(RoleKeyQuery {
                    key: key__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.RoleKeyQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoleQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.project.v1.RoleQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                role_query::Query::KeyQuery(v) => {
                    struct_ser.serialize_field("keyQuery", v)?;
                }
                role_query::Query::DisplayNameQuery(v) => {
                    struct_ser.serialize_field("displayNameQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoleQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_query",
            "keyQuery",
            "display_name_query",
            "displayNameQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyQuery,
            DisplayNameQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keyQuery" | "key_query" => Ok(GeneratedField::KeyQuery),
                            "displayNameQuery" | "display_name_query" => Ok(GeneratedField::DisplayNameQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoleQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.project.v1.RoleQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RoleQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(role_query::Query::KeyQuery)
;
                        }
                        GeneratedField::DisplayNameQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayNameQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(role_query::Query::DisplayNameQuery)
;
                        }
                    }
                }
                Ok(RoleQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.project.v1.RoleQuery", FIELDS, GeneratedVisitor)
    }
}
