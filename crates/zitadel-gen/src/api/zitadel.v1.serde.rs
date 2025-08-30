// @generated
impl serde::Serialize for AuthOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permission.is_empty() {
            len += 1;
        }
        if !self.check_field_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.AuthOption", len)?;
        if !self.permission.is_empty() {
            struct_ser.serialize_field("permission", &self.permission)?;
        }
        if !self.check_field_name.is_empty() {
            struct_ser.serialize_field("checkFieldName", &self.check_field_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permission",
            "check_field_name",
            "checkFieldName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permission,
            CheckFieldName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "permission" => Ok(GeneratedField::Permission),
                            "checkFieldName" | "check_field_name" => Ok(GeneratedField::CheckFieldName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.AuthOption")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthOption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permission__ = None;
                let mut check_field_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CheckFieldName => {
                            if check_field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkFieldName"));
                            }
                            check_field_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthOption {
                    permission: permission__.unwrap_or_default(),
                    check_field_name: check_field_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.AuthOption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CredentialsCheckError {
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
        if !self.message.is_empty() {
            len += 1;
        }
        if self.failed_attempts != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.CredentialsCheckError", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if self.failed_attempts != 0 {
            struct_ser.serialize_field("failedAttempts", &self.failed_attempts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CredentialsCheckError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "message",
            "failed_attempts",
            "failedAttempts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Message,
            FailedAttempts,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "message" => Ok(GeneratedField::Message),
                            "failedAttempts" | "failed_attempts" => Ok(GeneratedField::FailedAttempts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CredentialsCheckError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.CredentialsCheckError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CredentialsCheckError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut message__ = None;
                let mut failed_attempts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FailedAttempts => {
                            if failed_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedAttempts"));
                            }
                            failed_attempts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CredentialsCheckError {
                    id: id__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    failed_attempts: failed_attempts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.CredentialsCheckError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorDetail {
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
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.ErrorDetail", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorDetail {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorDetail;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.ErrorDetail")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorDetail, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ErrorDetail {
                    id: id__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.ErrorDetail", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_result != 0 {
            len += 1;
        }
        if self.processed_sequence != 0 {
            len += 1;
        }
        if self.view_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.ListDetails", len)?;
        if self.total_result != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("totalResult", ToString::to_string(&self.total_result).as_str())?;
        }
        if self.processed_sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("processedSequence", ToString::to_string(&self.processed_sequence).as_str())?;
        }
        if let Some(v) = self.view_timestamp.as_ref() {
            struct_ser.serialize_field("viewTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_result",
            "totalResult",
            "processed_sequence",
            "processedSequence",
            "view_timestamp",
            "viewTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalResult,
            ProcessedSequence,
            ViewTimestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "totalResult" | "total_result" => Ok(GeneratedField::TotalResult),
                            "processedSequence" | "processed_sequence" => Ok(GeneratedField::ProcessedSequence),
                            "viewTimestamp" | "view_timestamp" => Ok(GeneratedField::ViewTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.ListDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_result__ = None;
                let mut processed_sequence__ = None;
                let mut view_timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalResult => {
                            if total_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalResult"));
                            }
                            total_result__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProcessedSequence => {
                            if processed_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processedSequence"));
                            }
                            processed_sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ViewTimestamp => {
                            if view_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewTimestamp"));
                            }
                            view_timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListDetails {
                    total_result: total_result__.unwrap_or_default(),
                    processed_sequence: processed_sequence__.unwrap_or_default(),
                    view_timestamp: view_timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.ListDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offset != 0 {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        if self.asc {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.ListQuery", len)?;
        if self.offset != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("offset", ToString::to_string(&self.offset).as_str())?;
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
impl<'de> serde::Deserialize<'de> for ListQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offset",
            "limit",
            "asc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offset,
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
                            "offset" => Ok(GeneratedField::Offset),
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
            type Value = ListQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.ListQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offset__ = None;
                let mut limit__ = None;
                let mut asc__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
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
                Ok(ListQuery {
                    offset: offset__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    asc: asc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.ListQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListQueryMethod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::In => "LIST_QUERY_METHOD_IN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ListQueryMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LIST_QUERY_METHOD_IN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListQueryMethod;

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
                    "LIST_QUERY_METHOD_IN" => Ok(ListQueryMethod::In),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LocalizedMessage {
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
        if !self.localized_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.LocalizedMessage", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.localized_message.is_empty() {
            struct_ser.serialize_field("localizedMessage", &self.localized_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalizedMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "localized_message",
            "localizedMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            LocalizedMessage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "localizedMessage" | "localized_message" => Ok(GeneratedField::LocalizedMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalizedMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.LocalizedMessage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LocalizedMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut localized_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalizedMessage => {
                            if localized_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localizedMessage"));
                            }
                            localized_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LocalizedMessage {
                    key: key__.unwrap_or_default(),
                    localized_message: localized_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.LocalizedMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectDetails {
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
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.change_date.is_some() {
            len += 1;
        }
        if !self.resource_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.ObjectDetails", len)?;
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if !self.resource_owner.is_empty() {
            struct_ser.serialize_field("resourceOwner", &self.resource_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence",
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "resource_owner",
            "resourceOwner",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
            CreationDate,
            ChangeDate,
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
                            "sequence" => Ok(GeneratedField::Sequence),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
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
            type Value = ObjectDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.ObjectDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ObjectDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut resource_owner__ = None;
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
                        GeneratedField::ResourceOwner => {
                            if resource_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceOwner"));
                            }
                            resource_owner__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ObjectDetails {
                    sequence: sequence__.unwrap_or_default(),
                    creation_date: creation_date__,
                    change_date: change_date__,
                    resource_owner: resource_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.ObjectDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TextQueryMethod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Equals => "TEXT_QUERY_METHOD_EQUALS",
            Self::EqualsIgnoreCase => "TEXT_QUERY_METHOD_EQUALS_IGNORE_CASE",
            Self::StartsWith => "TEXT_QUERY_METHOD_STARTS_WITH",
            Self::StartsWithIgnoreCase => "TEXT_QUERY_METHOD_STARTS_WITH_IGNORE_CASE",
            Self::Contains => "TEXT_QUERY_METHOD_CONTAINS",
            Self::ContainsIgnoreCase => "TEXT_QUERY_METHOD_CONTAINS_IGNORE_CASE",
            Self::EndsWith => "TEXT_QUERY_METHOD_ENDS_WITH",
            Self::EndsWithIgnoreCase => "TEXT_QUERY_METHOD_ENDS_WITH_IGNORE_CASE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TextQueryMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TEXT_QUERY_METHOD_EQUALS",
            "TEXT_QUERY_METHOD_EQUALS_IGNORE_CASE",
            "TEXT_QUERY_METHOD_STARTS_WITH",
            "TEXT_QUERY_METHOD_STARTS_WITH_IGNORE_CASE",
            "TEXT_QUERY_METHOD_CONTAINS",
            "TEXT_QUERY_METHOD_CONTAINS_IGNORE_CASE",
            "TEXT_QUERY_METHOD_ENDS_WITH",
            "TEXT_QUERY_METHOD_ENDS_WITH_IGNORE_CASE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TextQueryMethod;

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
                    "TEXT_QUERY_METHOD_EQUALS" => Ok(TextQueryMethod::Equals),
                    "TEXT_QUERY_METHOD_EQUALS_IGNORE_CASE" => Ok(TextQueryMethod::EqualsIgnoreCase),
                    "TEXT_QUERY_METHOD_STARTS_WITH" => Ok(TextQueryMethod::StartsWith),
                    "TEXT_QUERY_METHOD_STARTS_WITH_IGNORE_CASE" => Ok(TextQueryMethod::StartsWithIgnoreCase),
                    "TEXT_QUERY_METHOD_CONTAINS" => Ok(TextQueryMethod::Contains),
                    "TEXT_QUERY_METHOD_CONTAINS_IGNORE_CASE" => Ok(TextQueryMethod::ContainsIgnoreCase),
                    "TEXT_QUERY_METHOD_ENDS_WITH" => Ok(TextQueryMethod::EndsWith),
                    "TEXT_QUERY_METHOD_ENDS_WITH_IGNORE_CASE" => Ok(TextQueryMethod::EndsWithIgnoreCase),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampQueryMethod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Equals => "TIMESTAMP_QUERY_METHOD_EQUALS",
            Self::Greater => "TIMESTAMP_QUERY_METHOD_GREATER",
            Self::GreaterOrEquals => "TIMESTAMP_QUERY_METHOD_GREATER_OR_EQUALS",
            Self::Less => "TIMESTAMP_QUERY_METHOD_LESS",
            Self::LessOrEquals => "TIMESTAMP_QUERY_METHOD_LESS_OR_EQUALS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TimestampQueryMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TIMESTAMP_QUERY_METHOD_EQUALS",
            "TIMESTAMP_QUERY_METHOD_GREATER",
            "TIMESTAMP_QUERY_METHOD_GREATER_OR_EQUALS",
            "TIMESTAMP_QUERY_METHOD_LESS",
            "TIMESTAMP_QUERY_METHOD_LESS_OR_EQUALS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampQueryMethod;

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
                    "TIMESTAMP_QUERY_METHOD_EQUALS" => Ok(TimestampQueryMethod::Equals),
                    "TIMESTAMP_QUERY_METHOD_GREATER" => Ok(TimestampQueryMethod::Greater),
                    "TIMESTAMP_QUERY_METHOD_GREATER_OR_EQUALS" => Ok(TimestampQueryMethod::GreaterOrEquals),
                    "TIMESTAMP_QUERY_METHOD_LESS" => Ok(TimestampQueryMethod::Less),
                    "TIMESTAMP_QUERY_METHOD_LESS_OR_EQUALS" => Ok(TimestampQueryMethod::LessOrEquals),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
