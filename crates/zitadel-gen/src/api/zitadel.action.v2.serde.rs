// @generated
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.Condition", len)?;
        if let Some(v) = self.condition_type.as_ref() {
            match v {
                condition::ConditionType::Request(v) => {
                    struct_ser.serialize_field("request", v)?;
                }
                condition::ConditionType::Response(v) => {
                    struct_ser.serialize_field("response", v)?;
                }
                condition::ConditionType::Function(v) => {
                    struct_ser.serialize_field("function", v)?;
                }
                condition::ConditionType::Event(v) => {
                    struct_ser.serialize_field("event", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "response",
            "function",
            "event",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            Response,
            Function,
            Event,
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
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            "function" => Ok(GeneratedField::Function),
                            "event" => Ok(GeneratedField::Event),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Request)
;
                        }
                        GeneratedField::Response => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Response)
;
                        }
                        GeneratedField::Function => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Function)
;
                        }
                        GeneratedField::Event => {
                            if condition_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            condition_type__ = map_.next_value::<::std::option::Option<_>>()?.map(condition::ConditionType::Event)
;
                        }
                    }
                }
                Ok(Condition {
                    condition_type: condition_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTargetRequest {
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
        if self.timeout.is_some() {
            len += 1;
        }
        if !self.endpoint.is_empty() {
            len += 1;
        }
        if self.target_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.CreateTargetRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        if let Some(v) = self.target_type.as_ref() {
            match v {
                create_target_request::TargetType::RestWebhook(v) => {
                    struct_ser.serialize_field("restWebhook", v)?;
                }
                create_target_request::TargetType::RestCall(v) => {
                    struct_ser.serialize_field("restCall", v)?;
                }
                create_target_request::TargetType::RestAsync(v) => {
                    struct_ser.serialize_field("restAsync", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "timeout",
            "endpoint",
            "rest_webhook",
            "restWebhook",
            "rest_call",
            "restCall",
            "rest_async",
            "restAsync",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Timeout,
            Endpoint,
            RestWebhook,
            RestCall,
            RestAsync,
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
                            "timeout" => Ok(GeneratedField::Timeout),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "restWebhook" | "rest_webhook" => Ok(GeneratedField::RestWebhook),
                            "restCall" | "rest_call" => Ok(GeneratedField::RestCall),
                            "restAsync" | "rest_async" => Ok(GeneratedField::RestAsync),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.CreateTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTargetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut timeout__ = None;
                let mut endpoint__ = None;
                let mut target_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RestWebhook => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restWebhook"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_target_request::TargetType::RestWebhook)
;
                        }
                        GeneratedField::RestCall => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restCall"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_target_request::TargetType::RestCall)
;
                        }
                        GeneratedField::RestAsync => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restAsync"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(create_target_request::TargetType::RestAsync)
;
                        }
                    }
                }
                Ok(CreateTargetRequest {
                    name: name__.unwrap_or_default(),
                    timeout: timeout__,
                    endpoint: endpoint__.unwrap_or_default(),
                    target_type: target_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.CreateTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTargetResponse {
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
        if !self.signing_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.CreateTargetResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if !self.signing_key.is_empty() {
            struct_ser.serialize_field("signingKey", &self.signing_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "signing_key",
            "signingKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            SigningKey,
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
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.CreateTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTargetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut signing_key__ = None;
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
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTargetResponse {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    signing_key: signing_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.CreateTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTargetRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.DeleteTargetRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTargetRequest {
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
            type Value = DeleteTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.DeleteTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTargetRequest, V::Error>
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
                Ok(DeleteTargetRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.DeleteTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTargetResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.DeleteTargetResponse", len)?;
        if let Some(v) = self.deletion_date.as_ref() {
            struct_ser.serialize_field("deletionDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTargetResponse {
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
            type Value = DeleteTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.DeleteTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTargetResponse, V::Error>
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
                Ok(DeleteTargetResponse {
                    deletion_date: deletion_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.DeleteTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.EventExecution", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                event_execution::Condition::Event(v) => {
                    struct_ser.serialize_field("event", v)?;
                }
                event_execution::Condition::Group(v) => {
                    struct_ser.serialize_field("group", v)?;
                }
                event_execution::Condition::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event",
            "group",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Event,
            Group,
            All,
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
                            "event" => Ok(GeneratedField::Event),
                            "group" => Ok(GeneratedField::Group),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.EventExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Event => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(event_execution::Condition::Event);
                        }
                        GeneratedField::Group => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(event_execution::Condition::Group);
                        }
                        GeneratedField::All => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(event_execution::Condition::All);
                        }
                    }
                }
                Ok(EventExecution {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.EventExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Execution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        if self.creation_date.is_some() {
            len += 1;
        }
        if self.change_date.is_some() {
            len += 1;
        }
        if !self.targets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.Execution", len)?;
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Execution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "condition",
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "targets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Condition,
            CreationDate,
            ChangeDate,
            Targets,
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
                            "condition" => Ok(GeneratedField::Condition),
                            "creationDate" | "creation_date" => Ok(GeneratedField::CreationDate),
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "targets" => Ok(GeneratedField::Targets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Execution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.Execution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Execution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut targets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map_.next_value()?;
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
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Execution {
                    condition: condition__,
                    creation_date: creation_date__,
                    change_date: change_date__,
                    targets: targets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.Execution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXECUTION_FIELD_NAME_UNSPECIFIED",
            Self::Id => "EXECUTION_FIELD_NAME_ID",
            Self::CreatedDate => "EXECUTION_FIELD_NAME_CREATED_DATE",
            Self::ChangedDate => "EXECUTION_FIELD_NAME_CHANGED_DATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXECUTION_FIELD_NAME_UNSPECIFIED",
            "EXECUTION_FIELD_NAME_ID",
            "EXECUTION_FIELD_NAME_CREATED_DATE",
            "EXECUTION_FIELD_NAME_CHANGED_DATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionFieldName;

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
                    "EXECUTION_FIELD_NAME_UNSPECIFIED" => Ok(ExecutionFieldName::Unspecified),
                    "EXECUTION_FIELD_NAME_ID" => Ok(ExecutionFieldName::Id),
                    "EXECUTION_FIELD_NAME_CREATED_DATE" => Ok(ExecutionFieldName::CreatedDate),
                    "EXECUTION_FIELD_NAME_CHANGED_DATE" => Ok(ExecutionFieldName::ChangedDate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionSearchFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ExecutionSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                execution_search_filter::Filter::InConditionsFilter(v) => {
                    struct_ser.serialize_field("inConditionsFilter", v)?;
                }
                execution_search_filter::Filter::ExecutionTypeFilter(v) => {
                    struct_ser.serialize_field("executionTypeFilter", v)?;
                }
                execution_search_filter::Filter::TargetFilter(v) => {
                    struct_ser.serialize_field("targetFilter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "in_conditions_filter",
            "inConditionsFilter",
            "execution_type_filter",
            "executionTypeFilter",
            "target_filter",
            "targetFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InConditionsFilter,
            ExecutionTypeFilter,
            TargetFilter,
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
                            "inConditionsFilter" | "in_conditions_filter" => Ok(GeneratedField::InConditionsFilter),
                            "executionTypeFilter" | "execution_type_filter" => Ok(GeneratedField::ExecutionTypeFilter),
                            "targetFilter" | "target_filter" => Ok(GeneratedField::TargetFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ExecutionSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutionSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InConditionsFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inConditionsFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::InConditionsFilter)
;
                        }
                        GeneratedField::ExecutionTypeFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionTypeFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::ExecutionTypeFilter)
;
                        }
                        GeneratedField::TargetFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(execution_search_filter::Filter::TargetFilter)
;
                        }
                    }
                }
                Ok(ExecutionSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ExecutionSearchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXECUTION_TYPE_UNSPECIFIED",
            Self::Request => "EXECUTION_TYPE_REQUEST",
            Self::Response => "EXECUTION_TYPE_RESPONSE",
            Self::Event => "EXECUTION_TYPE_EVENT",
            Self::Function => "EXECUTION_TYPE_FUNCTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXECUTION_TYPE_UNSPECIFIED",
            "EXECUTION_TYPE_REQUEST",
            "EXECUTION_TYPE_RESPONSE",
            "EXECUTION_TYPE_EVENT",
            "EXECUTION_TYPE_FUNCTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionType;

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
                    "EXECUTION_TYPE_UNSPECIFIED" => Ok(ExecutionType::Unspecified),
                    "EXECUTION_TYPE_REQUEST" => Ok(ExecutionType::Request),
                    "EXECUTION_TYPE_RESPONSE" => Ok(ExecutionType::Response),
                    "EXECUTION_TYPE_EVENT" => Ok(ExecutionType::Event),
                    "EXECUTION_TYPE_FUNCTION" => Ok(ExecutionType::Function),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionTypeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.execution_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ExecutionTypeFilter", len)?;
        if self.execution_type != 0 {
            let v = ExecutionType::try_from(self.execution_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.execution_type)))?;
            struct_ser.serialize_field("executionType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionTypeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "execution_type",
            "executionType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExecutionType,
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
                            "executionType" | "execution_type" => Ok(GeneratedField::ExecutionType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionTypeFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ExecutionTypeFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutionTypeFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut execution_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExecutionType => {
                            if execution_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionType"));
                            }
                            execution_type__ = Some(map_.next_value::<ExecutionType>()? as i32);
                        }
                    }
                }
                Ok(ExecutionTypeFilter {
                    execution_type: execution_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ExecutionTypeFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionExecution {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.FunctionExecution", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = FunctionExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.FunctionExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionExecution {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.FunctionExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTargetRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.GetTargetRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTargetRequest {
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
            type Value = GetTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.GetTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTargetRequest, V::Error>
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
                Ok(GetTargetRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.GetTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTargetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.GetTargetResponse", len)?;
        if let Some(v) = self.target.as_ref() {
            struct_ser.serialize_field("target", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTargetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
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
                            "target" => Ok(GeneratedField::Target),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.GetTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTargetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTargetResponse {
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.GetTargetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InConditionsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.conditions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.InConditionsFilter", len)?;
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InConditionsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conditions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Conditions,
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
                            "conditions" => Ok(GeneratedField::Conditions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InConditionsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.InConditionsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InConditionsFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conditions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InConditionsFilter {
                    conditions: conditions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.InConditionsFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InTargetIDsFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.InTargetIDsFilter", len)?;
        if !self.target_ids.is_empty() {
            struct_ser.serialize_field("targetIds", &self.target_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InTargetIDsFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_ids",
            "targetIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetIds,
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
                            "targetIds" | "target_ids" => Ok(GeneratedField::TargetIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InTargetIDsFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.InTargetIDsFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InTargetIDsFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetIds => {
                            if target_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIds"));
                            }
                            target_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InTargetIDsFilter {
                    target_ids: target_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.InTargetIDsFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionFunctionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionFunctionsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionFunctionsRequest {
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
            type Value = ListExecutionFunctionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionFunctionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionFunctionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListExecutionFunctionsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionFunctionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionFunctionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.functions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionFunctionsResponse", len)?;
        if !self.functions.is_empty() {
            struct_ser.serialize_field("functions", &self.functions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionFunctionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "functions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Functions,
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
                            "functions" => Ok(GeneratedField::Functions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionFunctionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionFunctionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionFunctionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut functions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Functions => {
                            if functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functions"));
                            }
                            functions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionFunctionsResponse {
                    functions: functions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionFunctionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionMethodsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionMethodsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionMethodsRequest {
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
            type Value = ListExecutionMethodsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionMethodsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionMethodsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListExecutionMethodsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionMethodsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionMethodsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.methods.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionMethodsResponse", len)?;
        if !self.methods.is_empty() {
            struct_ser.serialize_field("methods", &self.methods)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionMethodsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "methods",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Methods,
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
                            "methods" => Ok(GeneratedField::Methods),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionMethodsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionMethodsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionMethodsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut methods__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Methods => {
                            if methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methods"));
                            }
                            methods__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionMethodsResponse {
                    methods: methods__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionMethodsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionServicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionServicesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionServicesRequest {
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
            type Value = ListExecutionServicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionServicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionServicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListExecutionServicesRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionServicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionServicesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.services.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionServicesResponse", len)?;
        if !self.services.is_empty() {
            struct_ser.serialize_field("services", &self.services)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionServicesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "services",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Services,
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
                            "services" => Ok(GeneratedField::Services),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionServicesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionServicesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionServicesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut services__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Services => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("services"));
                            }
                            services__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionServicesResponse {
                    services: services__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionServicesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionsRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.sorting_column.as_ref() {
            let v = ExecutionFieldName::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionsRequest {
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
            type Value = ListExecutionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionsRequest, V::Error>
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
                            sorting_column__ = map_.next_value::<::std::option::Option<ExecutionFieldName>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionsRequest {
                    pagination: pagination__,
                    sorting_column: sorting_column__,
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExecutionsResponse {
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
        if !self.executions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListExecutionsResponse", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.executions.is_empty() {
            struct_ser.serialize_field("executions", &self.executions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExecutionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "executions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            Executions,
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
                            "executions" => Ok(GeneratedField::Executions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExecutionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListExecutionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExecutionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut executions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Executions => {
                            if executions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executions"));
                            }
                            executions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExecutionsResponse {
                    pagination: pagination__,
                    executions: executions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListExecutionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTargetsRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListTargetsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if let Some(v) = self.sorting_column.as_ref() {
            let v = TargetFieldName::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTargetsRequest {
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
            type Value = ListTargetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListTargetsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTargetsRequest, V::Error>
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
                            sorting_column__ = map_.next_value::<::std::option::Option<TargetFieldName>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTargetsRequest {
                    pagination: pagination__,
                    sorting_column: sorting_column__,
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListTargetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTargetsResponse {
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
        if !self.targets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ListTargetsResponse", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTargetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
            "targets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
            Targets,
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
                            "targets" => Ok(GeneratedField::Targets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTargetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ListTargetsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTargetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut targets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTargetsResponse {
                    pagination: pagination__,
                    targets: targets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ListTargetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestAsync {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.action.v2.RESTAsync", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestAsync {
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
            type Value = RestAsync;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.RESTAsync")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RestAsync, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RestAsync {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.RESTAsync", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestCall {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interrupt_on_error {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.RESTCall", len)?;
        if self.interrupt_on_error {
            struct_ser.serialize_field("interruptOnError", &self.interrupt_on_error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestCall {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interrupt_on_error",
            "interruptOnError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InterruptOnError,
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
                            "interruptOnError" | "interrupt_on_error" => Ok(GeneratedField::InterruptOnError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestCall;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.RESTCall")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RestCall, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interrupt_on_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InterruptOnError => {
                            if interrupt_on_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interruptOnError"));
                            }
                            interrupt_on_error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RestCall {
                    interrupt_on_error: interrupt_on_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.RESTCall", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestWebhook {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interrupt_on_error {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.RESTWebhook", len)?;
        if self.interrupt_on_error {
            struct_ser.serialize_field("interruptOnError", &self.interrupt_on_error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestWebhook {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interrupt_on_error",
            "interruptOnError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InterruptOnError,
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
                            "interruptOnError" | "interrupt_on_error" => Ok(GeneratedField::InterruptOnError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestWebhook;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.RESTWebhook")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RestWebhook, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interrupt_on_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InterruptOnError => {
                            if interrupt_on_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interruptOnError"));
                            }
                            interrupt_on_error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RestWebhook {
                    interrupt_on_error: interrupt_on_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.RESTWebhook", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.RequestExecution", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                request_execution::Condition::Method(v) => {
                    struct_ser.serialize_field("method", v)?;
                }
                request_execution::Condition::Service(v) => {
                    struct_ser.serialize_field("service", v)?;
                }
                request_execution::Condition::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method",
            "service",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Service,
            All,
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
                            "method" => Ok(GeneratedField::Method),
                            "service" => Ok(GeneratedField::Service),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.RequestExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(request_execution::Condition::Method);
                        }
                        GeneratedField::Service => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(request_execution::Condition::Service);
                        }
                        GeneratedField::All => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(request_execution::Condition::All);
                        }
                    }
                }
                Ok(RequestExecution {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.RequestExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseExecution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.ResponseExecution", len)?;
        if let Some(v) = self.condition.as_ref() {
            match v {
                response_execution::Condition::Method(v) => {
                    struct_ser.serialize_field("method", v)?;
                }
                response_execution::Condition::Service(v) => {
                    struct_ser.serialize_field("service", v)?;
                }
                response_execution::Condition::All(v) => {
                    struct_ser.serialize_field("all", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseExecution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method",
            "service",
            "all",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Service,
            All,
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
                            "method" => Ok(GeneratedField::Method),
                            "service" => Ok(GeneratedField::Service),
                            "all" => Ok(GeneratedField::All),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseExecution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.ResponseExecution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseExecution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(response_execution::Condition::Method);
                        }
                        GeneratedField::Service => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(response_execution::Condition::Service);
                        }
                        GeneratedField::All => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("all"));
                            }
                            condition__ = map_.next_value::<::std::option::Option<_>>()?.map(response_execution::Condition::All);
                        }
                    }
                }
                Ok(ResponseExecution {
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.ResponseExecution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetExecutionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.condition.is_some() {
            len += 1;
        }
        if !self.targets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.SetExecutionRequest", len)?;
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetExecutionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "condition",
            "targets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Condition,
            Targets,
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
                            "condition" => Ok(GeneratedField::Condition),
                            "targets" => Ok(GeneratedField::Targets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetExecutionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.SetExecutionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetExecutionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition__ = None;
                let mut targets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map_.next_value()?;
                        }
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetExecutionRequest {
                    condition: condition__,
                    targets: targets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.SetExecutionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetExecutionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.set_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.SetExecutionResponse", len)?;
        if let Some(v) = self.set_date.as_ref() {
            struct_ser.serialize_field("setDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetExecutionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "set_date",
            "setDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SetDate,
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
                            "setDate" | "set_date" => Ok(GeneratedField::SetDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetExecutionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.SetExecutionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetExecutionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut set_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SetDate => {
                            if set_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setDate"));
                            }
                            set_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetExecutionResponse {
                    set_date: set_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.SetExecutionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Target {
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
        if self.change_date.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if !self.endpoint.is_empty() {
            len += 1;
        }
        if !self.signing_key.is_empty() {
            len += 1;
        }
        if self.target_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.Target", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if !self.endpoint.is_empty() {
            struct_ser.serialize_field("endpoint", &self.endpoint)?;
        }
        if !self.signing_key.is_empty() {
            struct_ser.serialize_field("signingKey", &self.signing_key)?;
        }
        if let Some(v) = self.target_type.as_ref() {
            match v {
                target::TargetType::RestWebhook(v) => {
                    struct_ser.serialize_field("restWebhook", v)?;
                }
                target::TargetType::RestCall(v) => {
                    struct_ser.serialize_field("restCall", v)?;
                }
                target::TargetType::RestAsync(v) => {
                    struct_ser.serialize_field("restAsync", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Target {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "change_date",
            "changeDate",
            "name",
            "timeout",
            "endpoint",
            "signing_key",
            "signingKey",
            "rest_webhook",
            "restWebhook",
            "rest_call",
            "restCall",
            "rest_async",
            "restAsync",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            ChangeDate,
            Name,
            Timeout,
            Endpoint,
            SigningKey,
            RestWebhook,
            RestCall,
            RestAsync,
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
                            "changeDate" | "change_date" => Ok(GeneratedField::ChangeDate),
                            "name" => Ok(GeneratedField::Name),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            "restWebhook" | "rest_webhook" => Ok(GeneratedField::RestWebhook),
                            "restCall" | "rest_call" => Ok(GeneratedField::RestCall),
                            "restAsync" | "rest_async" => Ok(GeneratedField::RestAsync),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Target;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.Target")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Target, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut name__ = None;
                let mut timeout__ = None;
                let mut endpoint__ = None;
                let mut signing_key__ = None;
                let mut target_type__ = None;
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
                        GeneratedField::ChangeDate => {
                            if change_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            change_date__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RestWebhook => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restWebhook"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(target::TargetType::RestWebhook)
;
                        }
                        GeneratedField::RestCall => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restCall"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(target::TargetType::RestCall)
;
                        }
                        GeneratedField::RestAsync => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restAsync"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(target::TargetType::RestAsync)
;
                        }
                    }
                }
                Ok(Target {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    change_date: change_date__,
                    name: name__.unwrap_or_default(),
                    timeout: timeout__,
                    endpoint: endpoint__.unwrap_or_default(),
                    signing_key: signing_key__.unwrap_or_default(),
                    target_type: target_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.Target", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetFieldName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TARGET_FIELD_NAME_UNSPECIFIED",
            Self::Id => "TARGET_FIELD_NAME_ID",
            Self::CreatedDate => "TARGET_FIELD_NAME_CREATED_DATE",
            Self::ChangedDate => "TARGET_FIELD_NAME_CHANGED_DATE",
            Self::Name => "TARGET_FIELD_NAME_NAME",
            Self::TargetType => "TARGET_FIELD_NAME_TARGET_TYPE",
            Self::Url => "TARGET_FIELD_NAME_URL",
            Self::Timeout => "TARGET_FIELD_NAME_TIMEOUT",
            Self::InterruptOnError => "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TargetFieldName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TARGET_FIELD_NAME_UNSPECIFIED",
            "TARGET_FIELD_NAME_ID",
            "TARGET_FIELD_NAME_CREATED_DATE",
            "TARGET_FIELD_NAME_CHANGED_DATE",
            "TARGET_FIELD_NAME_NAME",
            "TARGET_FIELD_NAME_TARGET_TYPE",
            "TARGET_FIELD_NAME_URL",
            "TARGET_FIELD_NAME_TIMEOUT",
            "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetFieldName;

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
                    "TARGET_FIELD_NAME_UNSPECIFIED" => Ok(TargetFieldName::Unspecified),
                    "TARGET_FIELD_NAME_ID" => Ok(TargetFieldName::Id),
                    "TARGET_FIELD_NAME_CREATED_DATE" => Ok(TargetFieldName::CreatedDate),
                    "TARGET_FIELD_NAME_CHANGED_DATE" => Ok(TargetFieldName::ChangedDate),
                    "TARGET_FIELD_NAME_NAME" => Ok(TargetFieldName::Name),
                    "TARGET_FIELD_NAME_TARGET_TYPE" => Ok(TargetFieldName::TargetType),
                    "TARGET_FIELD_NAME_URL" => Ok(TargetFieldName::Url),
                    "TARGET_FIELD_NAME_TIMEOUT" => Ok(TargetFieldName::Timeout),
                    "TARGET_FIELD_NAME_INTERRUPT_ON_ERROR" => Ok(TargetFieldName::InterruptOnError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TargetFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.TargetFilter", len)?;
        if !self.target_id.is_empty() {
            struct_ser.serialize_field("targetId", &self.target_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_id",
            "targetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetId,
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
                            "targetId" | "target_id" => Ok(GeneratedField::TargetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.TargetFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetId => {
                            if target_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetId"));
                            }
                            target_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TargetFilter {
                    target_id: target_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.TargetFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetNameFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_name.is_empty() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.TargetNameFilter", len)?;
        if !self.target_name.is_empty() {
            struct_ser.serialize_field("targetName", &self.target_name)?;
        }
        if self.method != 0 {
            let v = super::super::filter::v2::TextFilterMethod::try_from(self.method)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetNameFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_name",
            "targetName",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetName,
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
                            "targetName" | "target_name" => Ok(GeneratedField::TargetName),
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
            type Value = TargetNameFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.TargetNameFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetNameFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_name__ = None;
                let mut method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetName => {
                            if target_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetName"));
                            }
                            target_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map_.next_value::<super::super::filter::v2::TextFilterMethod>()? as i32);
                        }
                    }
                }
                Ok(TargetNameFilter {
                    target_name: target_name__.unwrap_or_default(),
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.TargetNameFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TargetSearchFilter {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.TargetSearchFilter", len)?;
        if let Some(v) = self.filter.as_ref() {
            match v {
                target_search_filter::Filter::TargetNameFilter(v) => {
                    struct_ser.serialize_field("targetNameFilter", v)?;
                }
                target_search_filter::Filter::InTargetIdsFilter(v) => {
                    struct_ser.serialize_field("inTargetIdsFilter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TargetSearchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_name_filter",
            "targetNameFilter",
            "in_target_ids_filter",
            "inTargetIdsFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetNameFilter,
            InTargetIdsFilter,
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
                            "targetNameFilter" | "target_name_filter" => Ok(GeneratedField::TargetNameFilter),
                            "inTargetIdsFilter" | "in_target_ids_filter" => Ok(GeneratedField::InTargetIdsFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TargetSearchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.TargetSearchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TargetSearchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TargetNameFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetNameFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(target_search_filter::Filter::TargetNameFilter)
;
                        }
                        GeneratedField::InTargetIdsFilter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inTargetIdsFilter"));
                            }
                            filter__ = map_.next_value::<::std::option::Option<_>>()?.map(target_search_filter::Filter::InTargetIdsFilter)
;
                        }
                    }
                }
                Ok(TargetSearchFilter {
                    filter: filter__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.TargetSearchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTargetRequest {
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
        if self.name.is_some() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.endpoint.is_some() {
            len += 1;
        }
        if self.expiration_signing_key.is_some() {
            len += 1;
        }
        if self.target_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.UpdateTargetRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if let Some(v) = self.expiration_signing_key.as_ref() {
            struct_ser.serialize_field("expirationSigningKey", v)?;
        }
        if let Some(v) = self.target_type.as_ref() {
            match v {
                update_target_request::TargetType::RestWebhook(v) => {
                    struct_ser.serialize_field("restWebhook", v)?;
                }
                update_target_request::TargetType::RestCall(v) => {
                    struct_ser.serialize_field("restCall", v)?;
                }
                update_target_request::TargetType::RestAsync(v) => {
                    struct_ser.serialize_field("restAsync", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTargetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "timeout",
            "endpoint",
            "expiration_signing_key",
            "expirationSigningKey",
            "rest_webhook",
            "restWebhook",
            "rest_call",
            "restCall",
            "rest_async",
            "restAsync",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Timeout,
            Endpoint,
            ExpirationSigningKey,
            RestWebhook,
            RestCall,
            RestAsync,
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
                            "timeout" => Ok(GeneratedField::Timeout),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "expirationSigningKey" | "expiration_signing_key" => Ok(GeneratedField::ExpirationSigningKey),
                            "restWebhook" | "rest_webhook" => Ok(GeneratedField::RestWebhook),
                            "restCall" | "rest_call" => Ok(GeneratedField::RestCall),
                            "restAsync" | "rest_async" => Ok(GeneratedField::RestAsync),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTargetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.UpdateTargetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTargetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut timeout__ = None;
                let mut endpoint__ = None;
                let mut expiration_signing_key__ = None;
                let mut target_type__ = None;
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
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map_.next_value()?;
                        }
                        GeneratedField::ExpirationSigningKey => {
                            if expiration_signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationSigningKey"));
                            }
                            expiration_signing_key__ = map_.next_value()?;
                        }
                        GeneratedField::RestWebhook => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restWebhook"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(update_target_request::TargetType::RestWebhook)
;
                        }
                        GeneratedField::RestCall => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restCall"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(update_target_request::TargetType::RestCall)
;
                        }
                        GeneratedField::RestAsync => {
                            if target_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restAsync"));
                            }
                            target_type__ = map_.next_value::<::std::option::Option<_>>()?.map(update_target_request::TargetType::RestAsync)
;
                        }
                    }
                }
                Ok(UpdateTargetRequest {
                    id: id__.unwrap_or_default(),
                    name: name__,
                    timeout: timeout__,
                    endpoint: endpoint__,
                    expiration_signing_key: expiration_signing_key__,
                    target_type: target_type__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.UpdateTargetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTargetResponse {
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
        if self.signing_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.action.v2.UpdateTargetResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        if let Some(v) = self.signing_key.as_ref() {
            struct_ser.serialize_field("signingKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTargetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "change_date",
            "changeDate",
            "signing_key",
            "signingKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChangeDate,
            SigningKey,
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
                            "signingKey" | "signing_key" => Ok(GeneratedField::SigningKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTargetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.action.v2.UpdateTargetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTargetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut change_date__ = None;
                let mut signing_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChangeDate => {
                            if change_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeDate"));
                            }
                            change_date__ = map_.next_value()?;
                        }
                        GeneratedField::SigningKey => {
                            if signing_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signingKey"));
                            }
                            signing_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTargetResponse {
                    change_date: change_date__,
                    signing_key: signing_key__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.action.v2.UpdateTargetResponse", FIELDS, GeneratedVisitor)
    }
}
