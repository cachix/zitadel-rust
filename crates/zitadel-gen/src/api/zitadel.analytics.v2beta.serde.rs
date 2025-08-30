// @generated
impl serde::Serialize for CountParentType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "COUNT_PARENT_TYPE_UNSPECIFIED",
            Self::Instance => "COUNT_PARENT_TYPE_INSTANCE",
            Self::Organization => "COUNT_PARENT_TYPE_ORGANIZATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CountParentType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COUNT_PARENT_TYPE_UNSPECIFIED",
            "COUNT_PARENT_TYPE_INSTANCE",
            "COUNT_PARENT_TYPE_ORGANIZATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CountParentType;

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
                    "COUNT_PARENT_TYPE_UNSPECIFIED" => Ok(CountParentType::Unspecified),
                    "COUNT_PARENT_TYPE_INSTANCE" => Ok(CountParentType::Instance),
                    "COUNT_PARENT_TYPE_ORGANIZATION" => Ok(CountParentType::Organization),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for InstanceInformation {
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
        if !self.domains.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.analytics.v2beta.InstanceInformation", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InstanceInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "domains",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Domains,
            CreatedAt,
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
                            "domains" => Ok(GeneratedField::Domains),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InstanceInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.analytics.v2beta.InstanceInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InstanceInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut domains__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(InstanceInformation {
                    id: id__.unwrap_or_default(),
                    domains: domains__.unwrap_or_default(),
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.analytics.v2beta.InstanceInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportBaseInformationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.system_id.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.instances.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.analytics.v2beta.ReportBaseInformationRequest", len)?;
        if !self.system_id.is_empty() {
            struct_ser.serialize_field("systemId", &self.system_id)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.instances.is_empty() {
            struct_ser.serialize_field("instances", &self.instances)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportBaseInformationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system_id",
            "systemId",
            "version",
            "instances",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SystemId,
            Version,
            Instances,
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
                            "systemId" | "system_id" => Ok(GeneratedField::SystemId),
                            "version" => Ok(GeneratedField::Version),
                            "instances" => Ok(GeneratedField::Instances),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportBaseInformationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.analytics.v2beta.ReportBaseInformationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportBaseInformationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system_id__ = None;
                let mut version__ = None;
                let mut instances__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SystemId => {
                            if system_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemId"));
                            }
                            system_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Instances => {
                            if instances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instances"));
                            }
                            instances__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportBaseInformationRequest {
                    system_id: system_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    instances: instances__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.analytics.v2beta.ReportBaseInformationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportBaseInformationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.analytics.v2beta.ReportBaseInformationResponse", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportBaseInformationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportBaseInformationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.analytics.v2beta.ReportBaseInformationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportBaseInformationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportBaseInformationResponse {
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.analytics.v2beta.ReportBaseInformationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportResourceCountsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.system_id.is_empty() {
            len += 1;
        }
        if self.report_id.is_some() {
            len += 1;
        }
        if !self.resource_counts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.analytics.v2beta.ReportResourceCountsRequest", len)?;
        if !self.system_id.is_empty() {
            struct_ser.serialize_field("systemId", &self.system_id)?;
        }
        if let Some(v) = self.report_id.as_ref() {
            struct_ser.serialize_field("reportId", v)?;
        }
        if !self.resource_counts.is_empty() {
            struct_ser.serialize_field("resourceCounts", &self.resource_counts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportResourceCountsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system_id",
            "systemId",
            "report_id",
            "reportId",
            "resource_counts",
            "resourceCounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SystemId,
            ReportId,
            ResourceCounts,
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
                            "systemId" | "system_id" => Ok(GeneratedField::SystemId),
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            "resourceCounts" | "resource_counts" => Ok(GeneratedField::ResourceCounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportResourceCountsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.analytics.v2beta.ReportResourceCountsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportResourceCountsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system_id__ = None;
                let mut report_id__ = None;
                let mut resource_counts__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SystemId => {
                            if system_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemId"));
                            }
                            system_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceCounts => {
                            if resource_counts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceCounts"));
                            }
                            resource_counts__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportResourceCountsRequest {
                    system_id: system_id__.unwrap_or_default(),
                    report_id: report_id__,
                    resource_counts: resource_counts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.analytics.v2beta.ReportResourceCountsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportResourceCountsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.analytics.v2beta.ReportResourceCountsResponse", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportResourceCountsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportResourceCountsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.analytics.v2beta.ReportResourceCountsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportResourceCountsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportResourceCountsResponse {
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.analytics.v2beta.ReportResourceCountsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceCount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_id.is_empty() {
            len += 1;
        }
        if self.parent_type != 0 {
            len += 1;
        }
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.analytics.v2beta.ResourceCount", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if self.parent_type != 0 {
            let v = CountParentType::try_from(self.parent_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.parent_type)))?;
            struct_ser.serialize_field("parentType", &v)?;
        }
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resourceName", &self.resource_name)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if self.amount != 0 {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceCount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "parent_type",
            "parentType",
            "parent_id",
            "parentId",
            "resource_name",
            "resourceName",
            "table_name",
            "tableName",
            "updated_at",
            "updatedAt",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            ParentType,
            ParentId,
            ResourceName,
            TableName,
            UpdatedAt,
            Amount,
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
                            "instanceId" | "instance_id" => Ok(GeneratedField::InstanceId),
                            "parentType" | "parent_type" => Ok(GeneratedField::ParentType),
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "resourceName" | "resource_name" => Ok(GeneratedField::ResourceName),
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceCount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.analytics.v2beta.ResourceCount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceCount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut parent_type__ = None;
                let mut parent_id__ = None;
                let mut resource_name__ = None;
                let mut table_name__ = None;
                let mut updated_at__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentType => {
                            if parent_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentType"));
                            }
                            parent_type__ = Some(map_.next_value::<CountParentType>()? as i32);
                        }
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceName"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ResourceCount {
                    instance_id: instance_id__.unwrap_or_default(),
                    parent_type: parent_type__.unwrap_or_default(),
                    parent_id: parent_id__.unwrap_or_default(),
                    resource_name: resource_name__.unwrap_or_default(),
                    table_name: table_name__.unwrap_or_default(),
                    updated_at: updated_at__,
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.analytics.v2beta.ResourceCount", FIELDS, GeneratedVisitor)
    }
}
