// @generated
impl serde::Serialize for Change {
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
        if self.event_type.is_some() {
            len += 1;
        }
        if self.sequence != 0 {
            len += 1;
        }
        if !self.editor_id.is_empty() {
            len += 1;
        }
        if !self.editor_display_name.is_empty() {
            len += 1;
        }
        if !self.resource_owner_id.is_empty() {
            len += 1;
        }
        if !self.editor_preferred_login_name.is_empty() {
            len += 1;
        }
        if !self.editor_avatar_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.change.v1.Change", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if let Some(v) = self.event_type.as_ref() {
            struct_ser.serialize_field("eventType", v)?;
        }
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if !self.editor_id.is_empty() {
            struct_ser.serialize_field("editorId", &self.editor_id)?;
        }
        if !self.editor_display_name.is_empty() {
            struct_ser.serialize_field("editorDisplayName", &self.editor_display_name)?;
        }
        if !self.resource_owner_id.is_empty() {
            struct_ser.serialize_field("resourceOwnerId", &self.resource_owner_id)?;
        }
        if !self.editor_preferred_login_name.is_empty() {
            struct_ser.serialize_field("editorPreferredLoginName", &self.editor_preferred_login_name)?;
        }
        if !self.editor_avatar_url.is_empty() {
            struct_ser.serialize_field("editorAvatarUrl", &self.editor_avatar_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Change {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change_date",
            "changeDate",
            "event_type",
            "eventType",
            "sequence",
            "editor_id",
            "editorId",
            "editor_display_name",
            "editorDisplayName",
            "resource_owner_id",
            "resourceOwnerId",
            "editor_preferred_login_name",
            "editorPreferredLoginName",
            "editor_avatar_url",
            "editorAvatarUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChangeDate,
            EventType,
            Sequence,
            EditorId,
            EditorDisplayName,
            ResourceOwnerId,
            EditorPreferredLoginName,
            EditorAvatarUrl,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "sequence" => Ok(GeneratedField::Sequence),
                            "editorId" | "editor_id" => Ok(GeneratedField::EditorId),
                            "editorDisplayName" | "editor_display_name" => Ok(GeneratedField::EditorDisplayName),
                            "resourceOwnerId" | "resource_owner_id" => Ok(GeneratedField::ResourceOwnerId),
                            "editorPreferredLoginName" | "editor_preferred_login_name" => Ok(GeneratedField::EditorPreferredLoginName),
                            "editorAvatarUrl" | "editor_avatar_url" => Ok(GeneratedField::EditorAvatarUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Change;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.change.v1.Change")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Change, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut change_date__ = None;
                let mut event_type__ = None;
                let mut sequence__ = None;
                let mut editor_id__ = None;
                let mut editor_display_name__ = None;
                let mut resource_owner_id__ = None;
                let mut editor_preferred_login_name__ = None;
                let mut editor_avatar_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChangeDate => {
                            if change_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            change_date__ = map_.next_value()?;
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = map_.next_value()?;
                        }
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EditorId => {
                            if editor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("editorId"));
                            }
                            editor_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EditorDisplayName => {
                            if editor_display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("editorDisplayName"));
                            }
                            editor_display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceOwnerId => {
                            if resource_owner_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwnerId"));
                            }
                            resource_owner_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EditorPreferredLoginName => {
                            if editor_preferred_login_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("editorPreferredLoginName"));
                            }
                            editor_preferred_login_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EditorAvatarUrl => {
                            if editor_avatar_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("editorAvatarUrl"));
                            }
                            editor_avatar_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Change {
                    change_date: change_date__,
                    event_type: event_type__,
                    sequence: sequence__.unwrap_or_default(),
                    editor_id: editor_id__.unwrap_or_default(),
                    editor_display_name: editor_display_name__.unwrap_or_default(),
                    resource_owner_id: resource_owner_id__.unwrap_or_default(),
                    editor_preferred_login_name: editor_preferred_login_name__.unwrap_or_default(),
                    editor_avatar_url: editor_avatar_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.change.v1.Change", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangeQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sequence != 0 {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.asc {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.change.v1.ChangeQuery", len)?;
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if self.asc {
            struct_ser.serialize_field("asc", &self.asc)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChangeQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence",
            "limit",
            "asc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
            Limit,
            Asc,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sequence" => Ok(GeneratedField::Sequence),
                            "limit" => Ok(GeneratedField::Limit),
                            "asc" => Ok(GeneratedField::Asc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChangeQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.change.v1.ChangeQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChangeQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                let mut limit__ = None;
                let mut asc__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sequence => {
                            if sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Asc => {
                            if asc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asc"));
                            }
                            asc__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChangeQuery {
                    sequence: sequence__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    asc: asc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.change.v1.ChangeQuery", FIELDS, GeneratedVisitor)
    }
}
