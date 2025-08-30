// @generated
impl serde::Serialize for ActivateWebKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.ActivateWebKeyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivateWebKeyRequest {
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
            type Value = ActivateWebKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.ActivateWebKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivateWebKeyRequest, V::Error>
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
                Ok(ActivateWebKeyRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.ActivateWebKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ActivateWebKeyResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.ActivateWebKeyResponse", len)?;
        if let Some(v) = self.change_date.as_ref() {
            struct_ser.serialize_field("changeDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActivateWebKeyResponse {
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
            type Value = ActivateWebKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.ActivateWebKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ActivateWebKeyResponse, V::Error>
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
                Ok(ActivateWebKeyResponse {
                    change_date: change_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.ActivateWebKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.CreateWebKeyRequest", len)?;
        if let Some(v) = self.key.as_ref() {
            match v {
                create_web_key_request::Key::Rsa(v) => {
                    struct_ser.serialize_field("rsa", v)?;
                }
                create_web_key_request::Key::Ecdsa(v) => {
                    struct_ser.serialize_field("ecdsa", v)?;
                }
                create_web_key_request::Key::Ed25519(v) => {
                    struct_ser.serialize_field("ed25519", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rsa",
            "ecdsa",
            "ed25519",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rsa,
            Ecdsa,
            Ed25519,
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
                            "rsa" => Ok(GeneratedField::Rsa),
                            "ecdsa" => Ok(GeneratedField::Ecdsa),
                            "ed25519" => Ok(GeneratedField::Ed25519),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.CreateWebKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rsa => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rsa"));
                            }
                            key__ = map_.next_value::<::std::option::Option<_>>()?.map(create_web_key_request::Key::Rsa)
;
                        }
                        GeneratedField::Ecdsa => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsa"));
                            }
                            key__ = map_.next_value::<::std::option::Option<_>>()?.map(create_web_key_request::Key::Ecdsa)
;
                        }
                        GeneratedField::Ed25519 => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ed25519"));
                            }
                            key__ = map_.next_value::<::std::option::Option<_>>()?.map(create_web_key_request::Key::Ed25519)
;
                        }
                    }
                }
                Ok(CreateWebKeyRequest {
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.CreateWebKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebKeyResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.CreateWebKeyResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebKeyResponse {
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
            type Value = CreateWebKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.CreateWebKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebKeyResponse, V::Error>
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
                Ok(CreateWebKeyResponse {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.CreateWebKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWebKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.DeleteWebKeyRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWebKeyRequest {
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
            type Value = DeleteWebKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.DeleteWebKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWebKeyRequest, V::Error>
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
                Ok(DeleteWebKeyRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.DeleteWebKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteWebKeyResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.DeleteWebKeyResponse", len)?;
        if let Some(v) = self.deletion_date.as_ref() {
            struct_ser.serialize_field("deletionDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteWebKeyResponse {
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
            type Value = DeleteWebKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.DeleteWebKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteWebKeyResponse, V::Error>
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
                Ok(DeleteWebKeyResponse {
                    deletion_date: deletion_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.DeleteWebKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Ecdsa {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.curve != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.ECDSA", len)?;
        if self.curve != 0 {
            let v = EcdsaCurve::try_from(self.curve)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.curve)))?;
            struct_ser.serialize_field("curve", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ecdsa {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "curve",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Curve,
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
                            "curve" => Ok(GeneratedField::Curve),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ecdsa;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.ECDSA")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Ecdsa, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut curve__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Curve => {
                            if curve__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curve"));
                            }
                            curve__ = Some(map_.next_value::<EcdsaCurve>()? as i32);
                        }
                    }
                }
                Ok(Ecdsa {
                    curve: curve__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.ECDSA", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EcdsaCurve {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ECDSA_CURVE_UNSPECIFIED",
            Self::P256 => "ECDSA_CURVE_P256",
            Self::P384 => "ECDSA_CURVE_P384",
            Self::P512 => "ECDSA_CURVE_P512",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EcdsaCurve {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ECDSA_CURVE_UNSPECIFIED",
            "ECDSA_CURVE_P256",
            "ECDSA_CURVE_P384",
            "ECDSA_CURVE_P512",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EcdsaCurve;

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
                    "ECDSA_CURVE_UNSPECIFIED" => Ok(EcdsaCurve::Unspecified),
                    "ECDSA_CURVE_P256" => Ok(EcdsaCurve::P256),
                    "ECDSA_CURVE_P384" => Ok(EcdsaCurve::P384),
                    "ECDSA_CURVE_P512" => Ok(EcdsaCurve::P512),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Ed25519 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.ED25519", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ed25519 {
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
            type Value = Ed25519;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.ED25519")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Ed25519, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Ed25519 {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.ED25519", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.ListWebKeysRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebKeysRequest {
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
            type Value = ListWebKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.ListWebKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListWebKeysRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.ListWebKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.web_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.ListWebKeysResponse", len)?;
        if !self.web_keys.is_empty() {
            struct_ser.serialize_field("webKeys", &self.web_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "web_keys",
            "webKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebKeys,
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
                            "webKeys" | "web_keys" => Ok(GeneratedField::WebKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWebKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.ListWebKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut web_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebKeys => {
                            if web_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webKeys"));
                            }
                            web_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListWebKeysResponse {
                    web_keys: web_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.ListWebKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rsa {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bits != 0 {
            len += 1;
        }
        if self.hasher != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.RSA", len)?;
        if self.bits != 0 {
            let v = RsaBits::try_from(self.bits)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.bits)))?;
            struct_ser.serialize_field("bits", &v)?;
        }
        if self.hasher != 0 {
            let v = RsaHasher::try_from(self.hasher)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.hasher)))?;
            struct_ser.serialize_field("hasher", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rsa {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bits",
            "hasher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bits,
            Hasher,
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
                            "bits" => Ok(GeneratedField::Bits),
                            "hasher" => Ok(GeneratedField::Hasher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rsa;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.RSA")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rsa, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bits__ = None;
                let mut hasher__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bits => {
                            if bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bits"));
                            }
                            bits__ = Some(map_.next_value::<RsaBits>()? as i32);
                        }
                        GeneratedField::Hasher => {
                            if hasher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hasher"));
                            }
                            hasher__ = Some(map_.next_value::<RsaHasher>()? as i32);
                        }
                    }
                }
                Ok(Rsa {
                    bits: bits__.unwrap_or_default(),
                    hasher: hasher__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.RSA", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RsaBits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RSA_BITS_UNSPECIFIED",
            Self::RsaBits2048 => "RSA_BITS_2048",
            Self::RsaBits3072 => "RSA_BITS_3072",
            Self::RsaBits4096 => "RSA_BITS_4096",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RsaBits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RSA_BITS_UNSPECIFIED",
            "RSA_BITS_2048",
            "RSA_BITS_3072",
            "RSA_BITS_4096",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RsaBits;

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
                    "RSA_BITS_UNSPECIFIED" => Ok(RsaBits::Unspecified),
                    "RSA_BITS_2048" => Ok(RsaBits::RsaBits2048),
                    "RSA_BITS_3072" => Ok(RsaBits::RsaBits3072),
                    "RSA_BITS_4096" => Ok(RsaBits::RsaBits4096),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RsaHasher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RSA_HASHER_UNSPECIFIED",
            Self::Sha256 => "RSA_HASHER_SHA256",
            Self::Sha384 => "RSA_HASHER_SHA384",
            Self::Sha512 => "RSA_HASHER_SHA512",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RsaHasher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RSA_HASHER_UNSPECIFIED",
            "RSA_HASHER_SHA256",
            "RSA_HASHER_SHA384",
            "RSA_HASHER_SHA512",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RsaHasher;

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
                    "RSA_HASHER_UNSPECIFIED" => Ok(RsaHasher::Unspecified),
                    "RSA_HASHER_SHA256" => Ok(RsaHasher::Sha256),
                    "RSA_HASHER_SHA384" => Ok(RsaHasher::Sha384),
                    "RSA_HASHER_SHA512" => Ok(RsaHasher::Sha512),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
            Self::Initial => "STATE_INITIAL",
            Self::Active => "STATE_ACTIVE",
            Self::Inactive => "STATE_INACTIVE",
            Self::Removed => "STATE_REMOVED",
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
            "STATE_INITIAL",
            "STATE_ACTIVE",
            "STATE_INACTIVE",
            "STATE_REMOVED",
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
                    "STATE_INITIAL" => Ok(State::Initial),
                    "STATE_ACTIVE" => Ok(State::Active),
                    "STATE_INACTIVE" => Ok(State::Inactive),
                    "STATE_REMOVED" => Ok(State::Removed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebKey {
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
        if self.state != 0 {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.webkey.v2beta.WebKey", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
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
        if let Some(v) = self.key.as_ref() {
            match v {
                web_key::Key::Rsa(v) => {
                    struct_ser.serialize_field("rsa", v)?;
                }
                web_key::Key::Ecdsa(v) => {
                    struct_ser.serialize_field("ecdsa", v)?;
                }
                web_key::Key::Ed25519(v) => {
                    struct_ser.serialize_field("ed25519", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebKey {
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
            "state",
            "rsa",
            "ecdsa",
            "ed25519",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            ChangeDate,
            State,
            Rsa,
            Ecdsa,
            Ed25519,
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
                            "state" => Ok(GeneratedField::State),
                            "rsa" => Ok(GeneratedField::Rsa),
                            "ecdsa" => Ok(GeneratedField::Ecdsa),
                            "ed25519" => Ok(GeneratedField::Ed25519),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.webkey.v2beta.WebKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut change_date__ = None;
                let mut state__ = None;
                let mut key__ = None;
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
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<State>()? as i32);
                        }
                        GeneratedField::Rsa => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rsa"));
                            }
                            key__ = map_.next_value::<::std::option::Option<_>>()?.map(web_key::Key::Rsa)
;
                        }
                        GeneratedField::Ecdsa => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsa"));
                            }
                            key__ = map_.next_value::<::std::option::Option<_>>()?.map(web_key::Key::Ecdsa)
;
                        }
                        GeneratedField::Ed25519 => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ed25519"));
                            }
                            key__ = map_.next_value::<::std::option::Option<_>>()?.map(web_key::Key::Ed25519)
;
                        }
                    }
                }
                Ok(WebKey {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    change_date: change_date__,
                    state: state__.unwrap_or_default(),
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.webkey.v2beta.WebKey", FIELDS, GeneratedVisitor)
    }
}
