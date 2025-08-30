// @generated
impl serde::Serialize for AuthorizationError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error != 0 {
            len += 1;
        }
        if self.error_description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.AuthorizationError", len)?;
        if self.error != 0 {
            let v = ErrorReason::try_from(self.error)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.error)))?;
            struct_ser.serialize_field("error", &v)?;
        }
        if let Some(v) = self.error_description.as_ref() {
            struct_ser.serialize_field("errorDescription", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "error_description",
            "errorDescription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            ErrorDescription,
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
                            "error" => Ok(GeneratedField::Error),
                            "errorDescription" | "error_description" => Ok(GeneratedField::ErrorDescription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.AuthorizationError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut error_description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map_.next_value::<ErrorReason>()? as i32);
                        }
                        GeneratedField::ErrorDescription => {
                            if error_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorDescription"));
                            }
                            error_description__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AuthorizationError {
                    error: error__.unwrap_or_default(),
                    error_description: error_description__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.AuthorizationError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResponseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saml_request_id.is_empty() {
            len += 1;
        }
        if self.response_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.CreateResponseRequest", len)?;
        if !self.saml_request_id.is_empty() {
            struct_ser.serialize_field("samlRequestId", &self.saml_request_id)?;
        }
        if let Some(v) = self.response_kind.as_ref() {
            match v {
                create_response_request::ResponseKind::Session(v) => {
                    struct_ser.serialize_field("session", v)?;
                }
                create_response_request::ResponseKind::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResponseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saml_request_id",
            "samlRequestId",
            "session",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SamlRequestId,
            Session,
            Error,
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
                            "samlRequestId" | "saml_request_id" => Ok(GeneratedField::SamlRequestId),
                            "session" => Ok(GeneratedField::Session),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResponseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.CreateResponseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResponseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saml_request_id__ = None;
                let mut response_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SamlRequestId => {
                            if saml_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlRequestId"));
                            }
                            saml_request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Session => {
                            if response_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("session"));
                            }
                            response_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(create_response_request::ResponseKind::Session)
;
                        }
                        GeneratedField::Error => {
                            if response_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            response_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(create_response_request::ResponseKind::Error)
;
                        }
                    }
                }
                Ok(CreateResponseRequest {
                    saml_request_id: saml_request_id__.unwrap_or_default(),
                    response_kind: response_kind__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.CreateResponseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResponseResponse {
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
        if !self.url.is_empty() {
            len += 1;
        }
        if self.binding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.CreateResponseResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if let Some(v) = self.binding.as_ref() {
            match v {
                create_response_response::Binding::Redirect(v) => {
                    struct_ser.serialize_field("redirect", v)?;
                }
                create_response_response::Binding::Post(v) => {
                    struct_ser.serialize_field("post", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResponseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "url",
            "redirect",
            "post",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Url,
            Redirect,
            Post,
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
                            "url" => Ok(GeneratedField::Url),
                            "redirect" => Ok(GeneratedField::Redirect),
                            "post" => Ok(GeneratedField::Post),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResponseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.CreateResponseResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResponseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut url__ = None;
                let mut binding__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Redirect => {
                            if binding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirect"));
                            }
                            binding__ = map_.next_value::<::std::option::Option<_>>()?.map(create_response_response::Binding::Redirect)
;
                        }
                        GeneratedField::Post => {
                            if binding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("post"));
                            }
                            binding__ = map_.next_value::<::std::option::Option<_>>()?.map(create_response_response::Binding::Post)
;
                        }
                    }
                }
                Ok(CreateResponseResponse {
                    details: details__,
                    url: url__.unwrap_or_default(),
                    binding: binding__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.CreateResponseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorReason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ERROR_REASON_UNSPECIFIED",
            Self::VersionMissmatch => "ERROR_REASON_VERSION_MISSMATCH",
            Self::AuthNFailed => "ERROR_REASON_AUTH_N_FAILED",
            Self::InvalidAttrNameOrValue => "ERROR_REASON_INVALID_ATTR_NAME_OR_VALUE",
            Self::InvalidNameidPolicy => "ERROR_REASON_INVALID_NAMEID_POLICY",
            Self::RequestDenied => "ERROR_REASON_REQUEST_DENIED",
            Self::RequestUnsupported => "ERROR_REASON_REQUEST_UNSUPPORTED",
            Self::UnsupportedBinding => "ERROR_REASON_UNSUPPORTED_BINDING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorReason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ERROR_REASON_UNSPECIFIED",
            "ERROR_REASON_VERSION_MISSMATCH",
            "ERROR_REASON_AUTH_N_FAILED",
            "ERROR_REASON_INVALID_ATTR_NAME_OR_VALUE",
            "ERROR_REASON_INVALID_NAMEID_POLICY",
            "ERROR_REASON_REQUEST_DENIED",
            "ERROR_REASON_REQUEST_UNSUPPORTED",
            "ERROR_REASON_UNSUPPORTED_BINDING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorReason;

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
                    "ERROR_REASON_UNSPECIFIED" => Ok(ErrorReason::Unspecified),
                    "ERROR_REASON_VERSION_MISSMATCH" => Ok(ErrorReason::VersionMissmatch),
                    "ERROR_REASON_AUTH_N_FAILED" => Ok(ErrorReason::AuthNFailed),
                    "ERROR_REASON_INVALID_ATTR_NAME_OR_VALUE" => Ok(ErrorReason::InvalidAttrNameOrValue),
                    "ERROR_REASON_INVALID_NAMEID_POLICY" => Ok(ErrorReason::InvalidNameidPolicy),
                    "ERROR_REASON_REQUEST_DENIED" => Ok(ErrorReason::RequestDenied),
                    "ERROR_REASON_REQUEST_UNSUPPORTED" => Ok(ErrorReason::RequestUnsupported),
                    "ERROR_REASON_UNSUPPORTED_BINDING" => Ok(ErrorReason::UnsupportedBinding),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetSamlRequestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saml_request_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.GetSAMLRequestRequest", len)?;
        if !self.saml_request_id.is_empty() {
            struct_ser.serialize_field("samlRequestId", &self.saml_request_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSamlRequestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saml_request_id",
            "samlRequestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SamlRequestId,
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
                            "samlRequestId" | "saml_request_id" => Ok(GeneratedField::SamlRequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSamlRequestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.GetSAMLRequestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSamlRequestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saml_request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SamlRequestId => {
                            if saml_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlRequestId"));
                            }
                            saml_request_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSamlRequestRequest {
                    saml_request_id: saml_request_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.GetSAMLRequestRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSamlRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.saml_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.GetSAMLRequestResponse", len)?;
        if let Some(v) = self.saml_request.as_ref() {
            struct_ser.serialize_field("samlRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSamlRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saml_request",
            "samlRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SamlRequest,
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
                            "samlRequest" | "saml_request" => Ok(GeneratedField::SamlRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSamlRequestResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.GetSAMLRequestResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSamlRequestResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saml_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SamlRequest => {
                            if saml_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlRequest"));
                            }
                            saml_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSamlRequestResponse {
                    saml_request: saml_request__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.GetSAMLRequestResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PostResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relay_state.is_empty() {
            len += 1;
        }
        if !self.saml_response.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.PostResponse", len)?;
        if !self.relay_state.is_empty() {
            struct_ser.serialize_field("relayState", &self.relay_state)?;
        }
        if !self.saml_response.is_empty() {
            struct_ser.serialize_field("samlResponse", &self.saml_response)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relay_state",
            "relayState",
            "saml_response",
            "samlResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelayState,
            SamlResponse,
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
                            "relayState" | "relay_state" => Ok(GeneratedField::RelayState),
                            "samlResponse" | "saml_response" => Ok(GeneratedField::SamlResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.PostResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PostResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relay_state__ = None;
                let mut saml_response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RelayState => {
                            if relay_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayState"));
                            }
                            relay_state__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SamlResponse => {
                            if saml_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samlResponse"));
                            }
                            saml_response__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PostResponse {
                    relay_state: relay_state__.unwrap_or_default(),
                    saml_response: saml_response__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.PostResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedirectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.saml.v2.RedirectResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedirectResponse {
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
            type Value = RedirectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.RedirectResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RedirectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RedirectResponse {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.RedirectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SamlRequest {
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
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.assertion_consumer_service.is_empty() {
            len += 1;
        }
        if !self.relay_state.is_empty() {
            len += 1;
        }
        if !self.binding.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.SAMLRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.assertion_consumer_service.is_empty() {
            struct_ser.serialize_field("assertionConsumerService", &self.assertion_consumer_service)?;
        }
        if !self.relay_state.is_empty() {
            struct_ser.serialize_field("relayState", &self.relay_state)?;
        }
        if !self.binding.is_empty() {
            struct_ser.serialize_field("binding", &self.binding)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SamlRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "issuer",
            "assertion_consumer_service",
            "assertionConsumerService",
            "relay_state",
            "relayState",
            "binding",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            Issuer,
            AssertionConsumerService,
            RelayState,
            Binding,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "assertionConsumerService" | "assertion_consumer_service" => Ok(GeneratedField::AssertionConsumerService),
                            "relayState" | "relay_state" => Ok(GeneratedField::RelayState),
                            "binding" => Ok(GeneratedField::Binding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SamlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.SAMLRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SamlRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut issuer__ = None;
                let mut assertion_consumer_service__ = None;
                let mut relay_state__ = None;
                let mut binding__ = None;
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
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssertionConsumerService => {
                            if assertion_consumer_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertionConsumerService"));
                            }
                            assertion_consumer_service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RelayState => {
                            if relay_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relayState"));
                            }
                            relay_state__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Binding => {
                            if binding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binding"));
                            }
                            binding__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SamlRequest {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    issuer: issuer__.unwrap_or_default(),
                    assertion_consumer_service: assertion_consumer_service__.unwrap_or_default(),
                    relay_state: relay_state__.unwrap_or_default(),
                    binding: binding__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.SAMLRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Session {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_id.is_empty() {
            len += 1;
        }
        if !self.session_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.saml.v2.Session", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        if !self.session_token.is_empty() {
            struct_ser.serialize_field("sessionToken", &self.session_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Session {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
            "session_token",
            "sessionToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
            SessionToken,
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
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            "sessionToken" | "session_token" => Ok(GeneratedField::SessionToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Session;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.saml.v2.Session")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Session, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                let mut session_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SessionToken => {
                            if session_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionToken"));
                            }
                            session_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Session {
                    session_id: session_id__.unwrap_or_default(),
                    session_token: session_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.saml.v2.Session", FIELDS, GeneratedVisitor)
    }
}
