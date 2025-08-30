// @generated
impl serde::Serialize for AddedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.projection_sleep.is_some() {
            len += 1;
        }
        if self.blob.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.AddedEvent", len)?;
        if let Some(v) = self.projection_sleep.as_ref() {
            struct_ser.serialize_field("projectionSleep", v)?;
        }
        if let Some(v) = self.blob.as_ref() {
            struct_ser.serialize_field("blob", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projection_sleep",
            "projectionSleep",
            "blob",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectionSleep,
            Blob,
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
                            "projectionSleep" | "projection_sleep" => Ok(GeneratedField::ProjectionSleep),
                            "blob" => Ok(GeneratedField::Blob),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.AddedEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projection_sleep__ = None;
                let mut blob__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectionSleep => {
                            if projection_sleep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectionSleep"));
                            }
                            projection_sleep__ = map_.next_value()?;
                        }
                        GeneratedField::Blob => {
                            if blob__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blob"));
                            }
                            blob__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddedEvent {
                    projection_sleep: projection_sleep__,
                    blob: blob__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.AddedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.projection_sleep.is_some() {
            len += 1;
        }
        if self.blob.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.ChangedEvent", len)?;
        if let Some(v) = self.projection_sleep.as_ref() {
            struct_ser.serialize_field("projectionSleep", v)?;
        }
        if let Some(v) = self.blob.as_ref() {
            struct_ser.serialize_field("blob", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChangedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projection_sleep",
            "projectionSleep",
            "blob",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectionSleep,
            Blob,
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
                            "projectionSleep" | "projection_sleep" => Ok(GeneratedField::ProjectionSleep),
                            "blob" => Ok(GeneratedField::Blob),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChangedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.ChangedEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChangedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projection_sleep__ = None;
                let mut blob__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectionSleep => {
                            if projection_sleep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectionSleep"));
                            }
                            projection_sleep__ = map_.next_value()?;
                        }
                        GeneratedField::Blob => {
                            if blob__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blob"));
                            }
                            blob__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ChangedEvent {
                    projection_sleep: projection_sleep__,
                    blob: blob__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.ChangedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDebugEventsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.instance.is_some() {
            len += 1;
        }
        if !self.aggregate_id.is_empty() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.CreateDebugEventsRequest", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        if !self.aggregate_id.is_empty() {
            struct_ser.serialize_field("aggregateId", &self.aggregate_id)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDebugEventsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
            "aggregate_id",
            "aggregateId",
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Instance,
            AggregateId,
            Events,
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
                            "aggregateId" | "aggregate_id" => Ok(GeneratedField::AggregateId),
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDebugEventsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.CreateDebugEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDebugEventsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                let mut aggregate_id__ = None;
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                        GeneratedField::AggregateId => {
                            if aggregate_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregateId"));
                            }
                            aggregate_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateDebugEventsRequest {
                    instance: instance__,
                    aggregate_id: aggregate_id__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.CreateDebugEventsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDebugEventsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.CreateDebugEventsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDebugEventsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CreateDebugEventsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.CreateDebugEventsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDebugEventsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateDebugEventsResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.CreateDebugEventsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Event {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.Event", len)?;
        if let Some(v) = self.event.as_ref() {
            match v {
                event::Event::Add(v) => {
                    struct_ser.serialize_field("add", v)?;
                }
                event::Event::Change(v) => {
                    struct_ser.serialize_field("change", v)?;
                }
                event::Event::Remove(v) => {
                    struct_ser.serialize_field("remove", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Event {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "add",
            "change",
            "remove",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Add,
            Change,
            Remove,
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
                            "add" => Ok(GeneratedField::Add),
                            "change" => Ok(GeneratedField::Change),
                            "remove" => Ok(GeneratedField::Remove),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.Event")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Event, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Add => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("add"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(event::Event::Add)
;
                        }
                        GeneratedField::Change => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("change"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(event::Event::Change)
;
                        }
                        GeneratedField::Remove => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remove"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(event::Event::Remove)
;
                        }
                    }
                }
                Ok(Event {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.Event", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDebugEventsStateByIdRequest {
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
        if self.trigger_bulk {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.GetDebugEventsStateByIdRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.trigger_bulk {
            struct_ser.serialize_field("triggerBulk", &self.trigger_bulk)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDebugEventsStateByIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "trigger_bulk",
            "triggerBulk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TriggerBulk,
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
                            "triggerBulk" | "trigger_bulk" => Ok(GeneratedField::TriggerBulk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDebugEventsStateByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.GetDebugEventsStateByIdRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDebugEventsStateByIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut trigger_bulk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TriggerBulk => {
                            if trigger_bulk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggerBulk"));
                            }
                            trigger_bulk__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDebugEventsStateByIdRequest {
                    id: id__.unwrap_or_default(),
                    trigger_bulk: trigger_bulk__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.GetDebugEventsStateByIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDebugEventsStateByIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.GetDebugEventsStateByIdResponse", len)?;
        if let Some(v) = self.state.as_ref() {
            struct_ser.serialize_field("state", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDebugEventsStateByIdResponse {
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
            type Value = GetDebugEventsStateByIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.GetDebugEventsStateByIdResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDebugEventsStateByIdResponse, V::Error>
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
                            state__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDebugEventsStateByIdResponse {
                    state: state__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.GetDebugEventsStateByIdResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDebugEventsStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trigger_bulk {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.ListDebugEventsStatesRequest", len)?;
        if self.trigger_bulk {
            struct_ser.serialize_field("triggerBulk", &self.trigger_bulk)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDebugEventsStatesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trigger_bulk",
            "triggerBulk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TriggerBulk,
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
                            "triggerBulk" | "trigger_bulk" => Ok(GeneratedField::TriggerBulk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDebugEventsStatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.ListDebugEventsStatesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDebugEventsStatesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trigger_bulk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TriggerBulk => {
                            if trigger_bulk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggerBulk"));
                            }
                            trigger_bulk__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDebugEventsStatesRequest {
                    trigger_bulk: trigger_bulk__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.ListDebugEventsStatesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDebugEventsStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.states.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.ListDebugEventsStatesResponse", len)?;
        if !self.states.is_empty() {
            struct_ser.serialize_field("states", &self.states)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDebugEventsStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "states",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            States,
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
                            "states" => Ok(GeneratedField::States),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDebugEventsStatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.ListDebugEventsStatesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDebugEventsStatesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut states__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::States => {
                            if states__.is_some() {
                                return Err(serde::de::Error::duplicate_field("states"));
                            }
                            states__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDebugEventsStatesResponse {
                    states: states__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.ListDebugEventsStatesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemovedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.projection_sleep.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.RemovedEvent", len)?;
        if let Some(v) = self.projection_sleep.as_ref() {
            struct_ser.serialize_field("projectionSleep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemovedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projection_sleep",
            "projectionSleep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectionSleep,
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
                            "projectionSleep" | "projection_sleep" => Ok(GeneratedField::ProjectionSleep),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemovedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.RemovedEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemovedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projection_sleep__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectionSleep => {
                            if projection_sleep__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectionSleep"));
                            }
                            projection_sleep__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RemovedEvent {
                    projection_sleep: projection_sleep__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.RemovedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        if !self.blob.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.resources.debug_events.v3alpha.State", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.blob.is_empty() {
            struct_ser.serialize_field("blob", &self.blob)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "blob",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Blob,
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
                            "details" => Ok(GeneratedField::Details),
                            "blob" => Ok(GeneratedField::Blob),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.resources.debug_events.v3alpha.State")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<State, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut blob__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Blob => {
                            if blob__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blob"));
                            }
                            blob__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(State {
                    details: details__,
                    blob: blob__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.resources.debug_events.v3alpha.State", FIELDS, GeneratedVisitor)
    }
}
