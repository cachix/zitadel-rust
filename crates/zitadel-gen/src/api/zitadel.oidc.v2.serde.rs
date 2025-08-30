// @generated
impl serde::Serialize for AuthRequest {
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
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.scope.is_empty() {
            len += 1;
        }
        if !self.redirect_uri.is_empty() {
            len += 1;
        }
        if !self.prompt.is_empty() {
            len += 1;
        }
        if !self.ui_locales.is_empty() {
            len += 1;
        }
        if self.login_hint.is_some() {
            len += 1;
        }
        if self.max_age.is_some() {
            len += 1;
        }
        if self.hint_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.AuthRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.creation_date.as_ref() {
            struct_ser.serialize_field("creationDate", v)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scope.is_empty() {
            struct_ser.serialize_field("scope", &self.scope)?;
        }
        if !self.redirect_uri.is_empty() {
            struct_ser.serialize_field("redirectUri", &self.redirect_uri)?;
        }
        if !self.prompt.is_empty() {
            let v = self.prompt.iter().cloned().map(|v| {
                Prompt::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("prompt", &v)?;
        }
        if !self.ui_locales.is_empty() {
            struct_ser.serialize_field("uiLocales", &self.ui_locales)?;
        }
        if let Some(v) = self.login_hint.as_ref() {
            struct_ser.serialize_field("loginHint", v)?;
        }
        if let Some(v) = self.max_age.as_ref() {
            struct_ser.serialize_field("maxAge", v)?;
        }
        if let Some(v) = self.hint_user_id.as_ref() {
            struct_ser.serialize_field("hintUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "creation_date",
            "creationDate",
            "client_id",
            "clientId",
            "scope",
            "redirect_uri",
            "redirectUri",
            "prompt",
            "ui_locales",
            "uiLocales",
            "login_hint",
            "loginHint",
            "max_age",
            "maxAge",
            "hint_user_id",
            "hintUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreationDate,
            ClientId,
            Scope,
            RedirectUri,
            Prompt,
            UiLocales,
            LoginHint,
            MaxAge,
            HintUserId,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "scope" => Ok(GeneratedField::Scope),
                            "redirectUri" | "redirect_uri" => Ok(GeneratedField::RedirectUri),
                            "prompt" => Ok(GeneratedField::Prompt),
                            "uiLocales" | "ui_locales" => Ok(GeneratedField::UiLocales),
                            "loginHint" | "login_hint" => Ok(GeneratedField::LoginHint),
                            "maxAge" | "max_age" => Ok(GeneratedField::MaxAge),
                            "hintUserId" | "hint_user_id" => Ok(GeneratedField::HintUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.AuthRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut creation_date__ = None;
                let mut client_id__ = None;
                let mut scope__ = None;
                let mut redirect_uri__ = None;
                let mut prompt__ = None;
                let mut ui_locales__ = None;
                let mut login_hint__ = None;
                let mut max_age__ = None;
                let mut hint_user_id__ = None;
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
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RedirectUri => {
                            if redirect_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectUri"));
                            }
                            redirect_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Prompt => {
                            if prompt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prompt"));
                            }
                            prompt__ = Some(map_.next_value::<Vec<Prompt>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::UiLocales => {
                            if ui_locales__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uiLocales"));
                            }
                            ui_locales__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginHint => {
                            if login_hint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginHint"));
                            }
                            login_hint__ = map_.next_value()?;
                        }
                        GeneratedField::MaxAge => {
                            if max_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAge"));
                            }
                            max_age__ = map_.next_value()?;
                        }
                        GeneratedField::HintUserId => {
                            if hint_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hintUserId"));
                            }
                            hint_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AuthRequest {
                    id: id__.unwrap_or_default(),
                    creation_date: creation_date__,
                    client_id: client_id__.unwrap_or_default(),
                    scope: scope__.unwrap_or_default(),
                    redirect_uri: redirect_uri__.unwrap_or_default(),
                    prompt: prompt__.unwrap_or_default(),
                    ui_locales: ui_locales__.unwrap_or_default(),
                    login_hint: login_hint__,
                    max_age: max_age__,
                    hint_user_id: hint_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.AuthRequest", FIELDS, GeneratedVisitor)
    }
}
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
        if self.error_uri.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.AuthorizationError", len)?;
        if self.error != 0 {
            let v = ErrorReason::try_from(self.error)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.error)))?;
            struct_ser.serialize_field("error", &v)?;
        }
        if let Some(v) = self.error_description.as_ref() {
            struct_ser.serialize_field("errorDescription", v)?;
        }
        if let Some(v) = self.error_uri.as_ref() {
            struct_ser.serialize_field("errorUri", v)?;
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
            "error_uri",
            "errorUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            ErrorDescription,
            ErrorUri,
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
                            "errorUri" | "error_uri" => Ok(GeneratedField::ErrorUri),
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
                formatter.write_str("struct zitadel.oidc.v2.AuthorizationError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                let mut error_description__ = None;
                let mut error_uri__ = None;
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
                        GeneratedField::ErrorUri => {
                            if error_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorUri"));
                            }
                            error_uri__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AuthorizationError {
                    error: error__.unwrap_or_default(),
                    error_description: error_description__,
                    error_uri: error_uri__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.AuthorizationError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizeOrDenyDeviceAuthorizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.device_authorization_id.is_empty() {
            len += 1;
        }
        if self.decision.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.AuthorizeOrDenyDeviceAuthorizationRequest", len)?;
        if !self.device_authorization_id.is_empty() {
            struct_ser.serialize_field("deviceAuthorizationId", &self.device_authorization_id)?;
        }
        if let Some(v) = self.decision.as_ref() {
            match v {
                authorize_or_deny_device_authorization_request::Decision::Session(v) => {
                    struct_ser.serialize_field("session", v)?;
                }
                authorize_or_deny_device_authorization_request::Decision::Deny(v) => {
                    struct_ser.serialize_field("deny", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizeOrDenyDeviceAuthorizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_authorization_id",
            "deviceAuthorizationId",
            "session",
            "deny",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceAuthorizationId,
            Session,
            Deny,
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
                            "deviceAuthorizationId" | "device_authorization_id" => Ok(GeneratedField::DeviceAuthorizationId),
                            "session" => Ok(GeneratedField::Session),
                            "deny" => Ok(GeneratedField::Deny),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizeOrDenyDeviceAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.AuthorizeOrDenyDeviceAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizeOrDenyDeviceAuthorizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_authorization_id__ = None;
                let mut decision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceAuthorizationId => {
                            if device_authorization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceAuthorizationId"));
                            }
                            device_authorization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Session => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("session"));
                            }
                            decision__ = map_.next_value::<::std::option::Option<_>>()?.map(authorize_or_deny_device_authorization_request::Decision::Session)
;
                        }
                        GeneratedField::Deny => {
                            if decision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deny"));
                            }
                            decision__ = map_.next_value::<::std::option::Option<_>>()?.map(authorize_or_deny_device_authorization_request::Decision::Deny)
;
                        }
                    }
                }
                Ok(AuthorizeOrDenyDeviceAuthorizationRequest {
                    device_authorization_id: device_authorization_id__.unwrap_or_default(),
                    decision: decision__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.AuthorizeOrDenyDeviceAuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizeOrDenyDeviceAuthorizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.oidc.v2.AuthorizeOrDenyDeviceAuthorizationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizeOrDenyDeviceAuthorizationResponse {
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
            type Value = AuthorizeOrDenyDeviceAuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.AuthorizeOrDenyDeviceAuthorizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizeOrDenyDeviceAuthorizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AuthorizeOrDenyDeviceAuthorizationResponse {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.AuthorizeOrDenyDeviceAuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCallbackRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.auth_request_id.is_empty() {
            len += 1;
        }
        if self.callback_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.CreateCallbackRequest", len)?;
        if !self.auth_request_id.is_empty() {
            struct_ser.serialize_field("authRequestId", &self.auth_request_id)?;
        }
        if let Some(v) = self.callback_kind.as_ref() {
            match v {
                create_callback_request::CallbackKind::Session(v) => {
                    struct_ser.serialize_field("session", v)?;
                }
                create_callback_request::CallbackKind::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCallbackRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_request_id",
            "authRequestId",
            "session",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthRequestId,
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
                            "authRequestId" | "auth_request_id" => Ok(GeneratedField::AuthRequestId),
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
            type Value = CreateCallbackRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.CreateCallbackRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCallbackRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_request_id__ = None;
                let mut callback_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthRequestId => {
                            if auth_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authRequestId"));
                            }
                            auth_request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Session => {
                            if callback_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("session"));
                            }
                            callback_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(create_callback_request::CallbackKind::Session)
;
                        }
                        GeneratedField::Error => {
                            if callback_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            callback_kind__ = map_.next_value::<::std::option::Option<_>>()?.map(create_callback_request::CallbackKind::Error)
;
                        }
                    }
                }
                Ok(CreateCallbackRequest {
                    auth_request_id: auth_request_id__.unwrap_or_default(),
                    callback_kind: callback_kind__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.CreateCallbackRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCallbackResponse {
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
        if !self.callback_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.CreateCallbackResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.callback_url.is_empty() {
            struct_ser.serialize_field("callbackUrl", &self.callback_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCallbackResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "callback_url",
            "callbackUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            CallbackUrl,
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
                            "callbackUrl" | "callback_url" => Ok(GeneratedField::CallbackUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCallbackResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.CreateCallbackResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCallbackResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut callback_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::CallbackUrl => {
                            if callback_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackUrl"));
                            }
                            callback_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateCallbackResponse {
                    details: details__,
                    callback_url: callback_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.CreateCallbackResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Deny {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.oidc.v2.Deny", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Deny {
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
            type Value = Deny;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.Deny")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Deny, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Deny {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.Deny", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceAuthorizationRequest {
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
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.scope.is_empty() {
            len += 1;
        }
        if !self.app_name.is_empty() {
            len += 1;
        }
        if !self.project_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.DeviceAuthorizationRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.scope.is_empty() {
            struct_ser.serialize_field("scope", &self.scope)?;
        }
        if !self.app_name.is_empty() {
            struct_ser.serialize_field("appName", &self.app_name)?;
        }
        if !self.project_name.is_empty() {
            struct_ser.serialize_field("projectName", &self.project_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceAuthorizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "client_id",
            "clientId",
            "scope",
            "app_name",
            "appName",
            "project_name",
            "projectName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ClientId,
            Scope,
            AppName,
            ProjectName,
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
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "scope" => Ok(GeneratedField::Scope),
                            "appName" | "app_name" => Ok(GeneratedField::AppName),
                            "projectName" | "project_name" => Ok(GeneratedField::ProjectName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceAuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.DeviceAuthorizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceAuthorizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut client_id__ = None;
                let mut scope__ = None;
                let mut app_name__ = None;
                let mut project_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appName"));
                            }
                            app_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectName => {
                            if project_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectName"));
                            }
                            project_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceAuthorizationRequest {
                    id: id__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    scope: scope__.unwrap_or_default(),
                    app_name: app_name__.unwrap_or_default(),
                    project_name: project_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.DeviceAuthorizationRequest", FIELDS, GeneratedVisitor)
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
            Self::InvalidRequest => "ERROR_REASON_INVALID_REQUEST",
            Self::UnauthorizedClient => "ERROR_REASON_UNAUTHORIZED_CLIENT",
            Self::AccessDenied => "ERROR_REASON_ACCESS_DENIED",
            Self::UnsupportedResponseType => "ERROR_REASON_UNSUPPORTED_RESPONSE_TYPE",
            Self::InvalidScope => "ERROR_REASON_INVALID_SCOPE",
            Self::ServerError => "ERROR_REASON_SERVER_ERROR",
            Self::TemporaryUnavailable => "ERROR_REASON_TEMPORARY_UNAVAILABLE",
            Self::InteractionRequired => "ERROR_REASON_INTERACTION_REQUIRED",
            Self::LoginRequired => "ERROR_REASON_LOGIN_REQUIRED",
            Self::AccountSelectionRequired => "ERROR_REASON_ACCOUNT_SELECTION_REQUIRED",
            Self::ConsentRequired => "ERROR_REASON_CONSENT_REQUIRED",
            Self::InvalidRequestUri => "ERROR_REASON_INVALID_REQUEST_URI",
            Self::InvalidRequestObject => "ERROR_REASON_INVALID_REQUEST_OBJECT",
            Self::RequestNotSupported => "ERROR_REASON_REQUEST_NOT_SUPPORTED",
            Self::RequestUriNotSupported => "ERROR_REASON_REQUEST_URI_NOT_SUPPORTED",
            Self::RegistrationNotSupported => "ERROR_REASON_REGISTRATION_NOT_SUPPORTED",
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
            "ERROR_REASON_INVALID_REQUEST",
            "ERROR_REASON_UNAUTHORIZED_CLIENT",
            "ERROR_REASON_ACCESS_DENIED",
            "ERROR_REASON_UNSUPPORTED_RESPONSE_TYPE",
            "ERROR_REASON_INVALID_SCOPE",
            "ERROR_REASON_SERVER_ERROR",
            "ERROR_REASON_TEMPORARY_UNAVAILABLE",
            "ERROR_REASON_INTERACTION_REQUIRED",
            "ERROR_REASON_LOGIN_REQUIRED",
            "ERROR_REASON_ACCOUNT_SELECTION_REQUIRED",
            "ERROR_REASON_CONSENT_REQUIRED",
            "ERROR_REASON_INVALID_REQUEST_URI",
            "ERROR_REASON_INVALID_REQUEST_OBJECT",
            "ERROR_REASON_REQUEST_NOT_SUPPORTED",
            "ERROR_REASON_REQUEST_URI_NOT_SUPPORTED",
            "ERROR_REASON_REGISTRATION_NOT_SUPPORTED",
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
                    "ERROR_REASON_INVALID_REQUEST" => Ok(ErrorReason::InvalidRequest),
                    "ERROR_REASON_UNAUTHORIZED_CLIENT" => Ok(ErrorReason::UnauthorizedClient),
                    "ERROR_REASON_ACCESS_DENIED" => Ok(ErrorReason::AccessDenied),
                    "ERROR_REASON_UNSUPPORTED_RESPONSE_TYPE" => Ok(ErrorReason::UnsupportedResponseType),
                    "ERROR_REASON_INVALID_SCOPE" => Ok(ErrorReason::InvalidScope),
                    "ERROR_REASON_SERVER_ERROR" => Ok(ErrorReason::ServerError),
                    "ERROR_REASON_TEMPORARY_UNAVAILABLE" => Ok(ErrorReason::TemporaryUnavailable),
                    "ERROR_REASON_INTERACTION_REQUIRED" => Ok(ErrorReason::InteractionRequired),
                    "ERROR_REASON_LOGIN_REQUIRED" => Ok(ErrorReason::LoginRequired),
                    "ERROR_REASON_ACCOUNT_SELECTION_REQUIRED" => Ok(ErrorReason::AccountSelectionRequired),
                    "ERROR_REASON_CONSENT_REQUIRED" => Ok(ErrorReason::ConsentRequired),
                    "ERROR_REASON_INVALID_REQUEST_URI" => Ok(ErrorReason::InvalidRequestUri),
                    "ERROR_REASON_INVALID_REQUEST_OBJECT" => Ok(ErrorReason::InvalidRequestObject),
                    "ERROR_REASON_REQUEST_NOT_SUPPORTED" => Ok(ErrorReason::RequestNotSupported),
                    "ERROR_REASON_REQUEST_URI_NOT_SUPPORTED" => Ok(ErrorReason::RequestUriNotSupported),
                    "ERROR_REASON_REGISTRATION_NOT_SUPPORTED" => Ok(ErrorReason::RegistrationNotSupported),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetAuthRequestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.auth_request_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.GetAuthRequestRequest", len)?;
        if !self.auth_request_id.is_empty() {
            struct_ser.serialize_field("authRequestId", &self.auth_request_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAuthRequestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_request_id",
            "authRequestId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthRequestId,
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
                            "authRequestId" | "auth_request_id" => Ok(GeneratedField::AuthRequestId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAuthRequestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.GetAuthRequestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAuthRequestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_request_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthRequestId => {
                            if auth_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authRequestId"));
                            }
                            auth_request_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAuthRequestRequest {
                    auth_request_id: auth_request_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.GetAuthRequestRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAuthRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.auth_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.GetAuthRequestResponse", len)?;
        if let Some(v) = self.auth_request.as_ref() {
            struct_ser.serialize_field("authRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAuthRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_request",
            "authRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthRequest,
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
                            "authRequest" | "auth_request" => Ok(GeneratedField::AuthRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAuthRequestResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.GetAuthRequestResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAuthRequestResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthRequest => {
                            if auth_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authRequest"));
                            }
                            auth_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAuthRequestResponse {
                    auth_request: auth_request__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.GetAuthRequestResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDeviceAuthorizationRequestRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.GetDeviceAuthorizationRequestRequest", len)?;
        if !self.user_code.is_empty() {
            struct_ser.serialize_field("userCode", &self.user_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDeviceAuthorizationRequestRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_code",
            "userCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserCode,
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
                            "userCode" | "user_code" => Ok(GeneratedField::UserCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDeviceAuthorizationRequestRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.GetDeviceAuthorizationRequestRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDeviceAuthorizationRequestRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserCode => {
                            if user_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userCode"));
                            }
                            user_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDeviceAuthorizationRequestRequest {
                    user_code: user_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.GetDeviceAuthorizationRequestRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDeviceAuthorizationRequestResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_authorization_request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.GetDeviceAuthorizationRequestResponse", len)?;
        if let Some(v) = self.device_authorization_request.as_ref() {
            struct_ser.serialize_field("deviceAuthorizationRequest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDeviceAuthorizationRequestResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_authorization_request",
            "deviceAuthorizationRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceAuthorizationRequest,
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
                            "deviceAuthorizationRequest" | "device_authorization_request" => Ok(GeneratedField::DeviceAuthorizationRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDeviceAuthorizationRequestResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.oidc.v2.GetDeviceAuthorizationRequestResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDeviceAuthorizationRequestResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_authorization_request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceAuthorizationRequest => {
                            if device_authorization_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceAuthorizationRequest"));
                            }
                            device_authorization_request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDeviceAuthorizationRequestResponse {
                    device_authorization_request: device_authorization_request__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.oidc.v2.GetDeviceAuthorizationRequestResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Prompt {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROMPT_UNSPECIFIED",
            Self::None => "PROMPT_NONE",
            Self::Login => "PROMPT_LOGIN",
            Self::Consent => "PROMPT_CONSENT",
            Self::SelectAccount => "PROMPT_SELECT_ACCOUNT",
            Self::Create => "PROMPT_CREATE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Prompt {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROMPT_UNSPECIFIED",
            "PROMPT_NONE",
            "PROMPT_LOGIN",
            "PROMPT_CONSENT",
            "PROMPT_SELECT_ACCOUNT",
            "PROMPT_CREATE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Prompt;

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
                    "PROMPT_UNSPECIFIED" => Ok(Prompt::Unspecified),
                    "PROMPT_NONE" => Ok(Prompt::None),
                    "PROMPT_LOGIN" => Ok(Prompt::Login),
                    "PROMPT_CONSENT" => Ok(Prompt::Consent),
                    "PROMPT_SELECT_ACCOUNT" => Ok(Prompt::SelectAccount),
                    "PROMPT_CREATE" => Ok(Prompt::Create),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("zitadel.oidc.v2.Session", len)?;
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
                formatter.write_str("struct zitadel.oidc.v2.Session")
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
        deserializer.deserialize_struct("zitadel.oidc.v2.Session", FIELDS, GeneratedVisitor)
    }
}
