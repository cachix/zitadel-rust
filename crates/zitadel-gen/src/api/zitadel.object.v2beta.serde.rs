// @generated
impl serde::Serialize for Details {
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
        if self.change_date.is_some() {
            len += 1;
        }
        if !self.resource_owner.is_empty() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.object.v2beta.Details", len)?;
        if self.sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sequence", ToString::to_string(&self.sequence).as_str())?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if !self.resource_owner.is_empty() {
            struct_ser.serialize_field("resourceOwner", &self.resource_owner)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Details {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequence",
            "change_date",
            "changeDate",
            "resource_owner",
            "resourceOwner",
            "creation_date",
            "creationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sequence,
            ChangeDate,
            ResourceOwner,
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
                            "sequence" => Ok(GeneratedField::Sequence),
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "resourceOwner" | "resource_owner" => Ok(GeneratedField::ResourceOwner),
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
            type Value = Details;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.object.v2beta.Details")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Details, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequence__ = None;
                let mut change_date__ = None;
                let mut resource_owner__ = None;
                let mut creation_date__ = None;
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
                        GeneratedField::CreationDate => {
                            if creation_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationDate"));
                            }
                            creation_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Details {
                    sequence: sequence__.unwrap_or_default(),
                    change_date: change_date__,
                    resource_owner: resource_owner__.unwrap_or_default(),
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.object.v2beta.Details", FIELDS, GeneratedVisitor)
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
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.object.v2beta.ListDetails", len)?;
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
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
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
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalResult,
            ProcessedSequence,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

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
                            "timestamp" => Ok(GeneratedField::Timestamp),
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
                formatter.write_str("struct zitadel.object.v2beta.ListDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_result__ = None;
                let mut processed_sequence__ = None;
                let mut timestamp__ = None;
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
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListDetails {
                    total_result: total_result__.unwrap_or_default(),
                    processed_sequence: processed_sequence__.unwrap_or_default(),
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.object.v2beta.ListDetails", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.object.v2beta.ListQuery", len)?;
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
                formatter.write_str("struct zitadel.object.v2beta.ListQuery")
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
        deserializer.deserialize_struct("zitadel.object.v2beta.ListQuery", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for Organisation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.org.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.object.v2beta.Organisation", len)?;
        if let Some(v) = self.org.as_ref() {
            match v {
                organisation::Org::OrgId(v) => {
                    struct_ser.serialize_field("orgId", v)?;
                }
                organisation::Org::OrgDomain(v) => {
                    struct_ser.serialize_field("orgDomain", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Organisation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_id",
            "orgId",
            "org_domain",
            "orgDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgId,
            OrgDomain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            "orgDomain" | "org_domain" => Ok(GeneratedField::OrgDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Organisation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.object.v2beta.Organisation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Organisation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgId => {
                            if org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org__ = map_.next_value::<::std::option::Option<_>>()?.map(organisation::Org::OrgId);
                        }
                        GeneratedField::OrgDomain => {
                            if org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgDomain"));
                            }
                            org__ = map_.next_value::<::std::option::Option<_>>()?.map(organisation::Org::OrgDomain);
                        }
                    }
                }
                Ok(Organisation {
                    org: org__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.object.v2beta.Organisation", FIELDS, GeneratedVisitor)
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
        if self.org.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.object.v2beta.Organization", len)?;
        if let Some(v) = self.org.as_ref() {
            match v {
                organization::Org::OrgId(v) => {
                    struct_ser.serialize_field("orgId", v)?;
                }
                organization::Org::OrgDomain(v) => {
                    struct_ser.serialize_field("orgDomain", v)?;
                }
            }
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
            "org_id",
            "orgId",
            "org_domain",
            "orgDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgId,
            OrgDomain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            "orgDomain" | "org_domain" => Ok(GeneratedField::OrgDomain),
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
                formatter.write_str("struct zitadel.object.v2beta.Organization")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Organization, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgId => {
                            if org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org__ = map_.next_value::<::std::option::Option<_>>()?.map(organization::Org::OrgId);
                        }
                        GeneratedField::OrgDomain => {
                            if org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgDomain"));
                            }
                            org__ = map_.next_value::<::std::option::Option<_>>()?.map(organization::Org::OrgDomain);
                        }
                    }
                }
                Ok(Organization {
                    org: org__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.object.v2beta.Organization", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_owner.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.object.v2beta.RequestContext", len)?;
        if let Some(v) = self.resource_owner.as_ref() {
            match v {
                request_context::ResourceOwner::OrgId(v) => {
                    struct_ser.serialize_field("orgId", v)?;
                }
                request_context::ResourceOwner::Instance(v) => {
                    struct_ser.serialize_field("instance", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_id",
            "orgId",
            "instance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgId,
            Instance,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "orgId" | "org_id" => Ok(GeneratedField::OrgId),
                            "instance" => Ok(GeneratedField::Instance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.object.v2beta.RequestContext")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgId => {
                            if resource_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            resource_owner__ = map_.next_value::<::std::option::Option<_>>()?.map(request_context::ResourceOwner::OrgId);
                        }
                        GeneratedField::Instance => {
                            if resource_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            resource_owner__ = map_.next_value::<::std::option::Option<_>>()?.map(request_context::ResourceOwner::Instance);
                        }
                    }
                }
                Ok(RequestContext {
                    resource_owner: resource_owner__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.object.v2beta.RequestContext", FIELDS, GeneratedVisitor)
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
