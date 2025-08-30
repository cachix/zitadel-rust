// @generated
impl serde::Serialize for IsReachedQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reached {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.milestone.v1.IsReachedQuery", len)?;
        if self.reached {
            struct_ser.serialize_field("reached", &self.reached)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsReachedQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reached",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reached,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reached" => Ok(GeneratedField::Reached),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsReachedQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.milestone.v1.IsReachedQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IsReachedQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reached__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reached => {
                            if reached__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reached"));
                            }
                            reached__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IsReachedQuery {
                    reached: reached__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.milestone.v1.IsReachedQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Milestone {
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
        if self.reached_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.milestone.v1.Milestone", len)?;
        if self.r#type != 0 {
            let v = MilestoneType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.reached_date.as_ref() {
            struct_ser.serialize_field("reachedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Milestone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "reached_date",
            "reachedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            ReachedDate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "reachedDate" | "reached_date" => Ok(GeneratedField::ReachedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Milestone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.milestone.v1.Milestone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Milestone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut reached_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<MilestoneType>()? as i32);
                        }
                        GeneratedField::ReachedDate => {
                            if reached_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reachedDate"));
                            }
                            reached_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Milestone {
                    r#type: r#type__.unwrap_or_default(),
                    reached_date: reached_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.milestone.v1.Milestone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MilestoneFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MILESTONE_FIELD_NAME_UNSPECIFIED",
            Self::Type => "MILESTONE_FIELD_NAME_TYPE",
            Self::ReachedDate => "MILESTONE_FIELD_NAME_REACHED_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MilestoneFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MILESTONE_FIELD_NAME_UNSPECIFIED",
            "MILESTONE_FIELD_NAME_TYPE",
            "MILESTONE_FIELD_NAME_REACHED_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MilestoneFieldName;

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
                    "MILESTONE_FIELD_NAME_UNSPECIFIED" => Ok(MilestoneFieldName::Unspecified),
                    "MILESTONE_FIELD_NAME_TYPE" => Ok(MilestoneFieldName::Type),
                    "MILESTONE_FIELD_NAME_REACHED_DATE" => Ok(MilestoneFieldName::ReachedDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MilestoneQuery {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.milestone.v1.MilestoneQuery", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                milestone_query::Query::IsReachedQuery(v) => {
                    struct_ser.serialize_field("isReachedQuery", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MilestoneQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_reached_query",
            "isReachedQuery",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsReachedQuery,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "isReachedQuery" | "is_reached_query" => Ok(GeneratedField::IsReachedQuery),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MilestoneQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.milestone.v1.MilestoneQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MilestoneQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsReachedQuery => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isReachedQuery"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(milestone_query::Query::IsReachedQuery)
;
                        }
                    }
                }
                Ok(MilestoneQuery {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.milestone.v1.MilestoneQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MilestoneType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MILESTONE_TYPE_UNSPECIFIED",
            Self::InstanceCreated => "MILESTONE_TYPE_INSTANCE_CREATED",
            Self::AuthenticationSucceededOnInstance => "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_INSTANCE",
            Self::ProjectCreated => "MILESTONE_TYPE_PROJECT_CREATED",
            Self::ApplicationCreated => "MILESTONE_TYPE_APPLICATION_CREATED",
            Self::AuthenticationSucceededOnApplication => "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_APPLICATION",
            Self::InstanceDeleted => "MILESTONE_TYPE_INSTANCE_DELETED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MilestoneType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MILESTONE_TYPE_UNSPECIFIED",
            "MILESTONE_TYPE_INSTANCE_CREATED",
            "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_INSTANCE",
            "MILESTONE_TYPE_PROJECT_CREATED",
            "MILESTONE_TYPE_APPLICATION_CREATED",
            "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_APPLICATION",
            "MILESTONE_TYPE_INSTANCE_DELETED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MilestoneType;

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
                    "MILESTONE_TYPE_UNSPECIFIED" => Ok(MilestoneType::Unspecified),
                    "MILESTONE_TYPE_INSTANCE_CREATED" => Ok(MilestoneType::InstanceCreated),
                    "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_INSTANCE" => Ok(MilestoneType::AuthenticationSucceededOnInstance),
                    "MILESTONE_TYPE_PROJECT_CREATED" => Ok(MilestoneType::ProjectCreated),
                    "MILESTONE_TYPE_APPLICATION_CREATED" => Ok(MilestoneType::ApplicationCreated),
                    "MILESTONE_TYPE_AUTHENTICATION_SUCCEEDED_ON_APPLICATION" => Ok(MilestoneType::AuthenticationSucceededOnApplication),
                    "MILESTONE_TYPE_INSTANCE_DELETED" => Ok(MilestoneType::InstanceDeleted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
