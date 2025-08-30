// @generated
impl serde::Serialize for AddCustomOrgIamPolicyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.org_id.is_empty() {
            len += 1;
        }
        if self.user_login_must_be_domain {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.AddCustomOrgIAMPolicyRequest", len)?;
        if !self.org_id.is_empty() {
            struct_ser.serialize_field("orgId", &self.org_id)?;
        }
        if self.user_login_must_be_domain {
            struct_ser.serialize_field("userLoginMustBeDomain", &self.user_login_must_be_domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddCustomOrgIamPolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_id",
            "orgId",
            "user_login_must_be_domain",
            "userLoginMustBeDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgId,
            UserLoginMustBeDomain,
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
                            "userLoginMustBeDomain" | "user_login_must_be_domain" => Ok(GeneratedField::UserLoginMustBeDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddCustomOrgIamPolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.AddCustomOrgIAMPolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddCustomOrgIamPolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org_id__ = None;
                let mut user_login_must_be_domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserLoginMustBeDomain => {
                            if user_login_must_be_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userLoginMustBeDomain"));
                            }
                            user_login_must_be_domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddCustomOrgIamPolicyRequest {
                    org_id: org_id__.unwrap_or_default(),
                    user_login_must_be_domain: user_login_must_be_domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.AddCustomOrgIAMPolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataApiApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_id.is_empty() {
            len += 1;
        }
        if self.app.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataAPIApplication", len)?;
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("appId", &self.app_id)?;
        }
        if let Some(v) = self.app.as_ref() {
            struct_ser.serialize_field("app", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataApiApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app_id",
            "appId",
            "app",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppId,
            App,
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
                            "appId" | "app_id" => Ok(GeneratedField::AppId),
                            "app" => Ok(GeneratedField::App),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataApiApplication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataAPIApplication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataApiApplication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_id__ = None;
                let mut app__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appId"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::App => {
                            if app__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app"));
                            }
                            app__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataApiApplication {
                    app_id: app_id__.unwrap_or_default(),
                    app: app__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataAPIApplication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.action_id.is_empty() {
            len += 1;
        }
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataAction", len)?;
        if !self.action_id.is_empty() {
            struct_ser.serialize_field("actionId", &self.action_id)?;
        }
        if let Some(v) = self.action.as_ref() {
            struct_ser.serialize_field("action", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action_id",
            "actionId",
            "action",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActionId,
            Action,
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
                            "actionId" | "action_id" => Ok(GeneratedField::ActionId),
                            "action" => Ok(GeneratedField::Action),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_id__ = None;
                let mut action__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActionId => {
                            if action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionId"));
                            }
                            action_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataAction {
                    action_id: action_id__.unwrap_or_default(),
                    action: action__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataAppKey {
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
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.app_id.is_empty() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataAppKey", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("appId", &self.app_id)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if self.r#type != 0 {
            let v = super::super::authn::v1::KeyType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        if !self.public_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataAppKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "project_id",
            "projectId",
            "app_id",
            "appId",
            "client_id",
            "clientId",
            "type",
            "expiration_date",
            "expirationDate",
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ProjectId,
            AppId,
            ClientId,
            Type,
            ExpirationDate,
            PublicKey,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "appId" | "app_id" => Ok(GeneratedField::AppId),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "type" => Ok(GeneratedField::Type),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataAppKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataAppKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataAppKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut project_id__ = None;
                let mut app_id__ = None;
                let mut client_id__ = None;
                let mut r#type__ = None;
                let mut expiration_date__ = None;
                let mut public_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appId"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<super::super::authn::v1::KeyType>()? as i32);
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DataAppKey {
                    id: id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    app_id: app_id__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    expiration_date: expiration_date__,
                    public_key: public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataAppKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataHumanUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataHumanUser", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if self.state != 0 {
            let v = super::super::user::v1::UserState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataHumanUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user",
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            User,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "user" => Ok(GeneratedField::User),
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
            type Value = DataHumanUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataHumanUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataHumanUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user__ = None;
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<super::super::user::v1::UserState>()? as i32);
                        }
                    }
                }
                Ok(DataHumanUser {
                    user_id: user_id__.unwrap_or_default(),
                    user: user__,
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataHumanUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataJwtidp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.idp_id.is_empty() {
            len += 1;
        }
        if self.idp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataJWTIDP", len)?;
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if let Some(v) = self.idp.as_ref() {
            struct_ser.serialize_field("idp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataJwtidp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_id",
            "idpId",
            "idp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpId,
            Idp,
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
                            "idpId" | "idp_id" => Ok(GeneratedField::IdpId),
                            "idp" => Ok(GeneratedField::Idp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataJwtidp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataJWTIDP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataJwtidp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_id__ = None;
                let mut idp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Idp => {
                            if idp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idp"));
                            }
                            idp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataJwtidp {
                    idp_id: idp_id__.unwrap_or_default(),
                    idp: idp__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataJWTIDP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataMachineKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataMachineKey", len)?;
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.r#type != 0 {
            let v = super::super::authn::v1::KeyType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        if !self.public_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataMachineKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_id",
            "keyId",
            "user_id",
            "userId",
            "type",
            "expiration_date",
            "expirationDate",
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyId,
            UserId,
            Type,
            ExpirationDate,
            PublicKey,
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
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "type" => Ok(GeneratedField::Type),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataMachineKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataMachineKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataMachineKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_id__ = None;
                let mut user_id__ = None;
                let mut r#type__ = None;
                let mut expiration_date__ = None;
                let mut public_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<super::super::authn::v1::KeyType>()? as i32);
                        }
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DataMachineKey {
                    key_id: key_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    expiration_date: expiration_date__,
                    public_key: public_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataMachineKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataMachineUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataMachineUser", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if self.state != 0 {
            let v = super::super::user::v1::UserState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataMachineUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user",
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            User,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "user" => Ok(GeneratedField::User),
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
            type Value = DataMachineUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataMachineUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataMachineUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user__ = None;
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<super::super::user::v1::UserState>()? as i32);
                        }
                    }
                }
                Ok(DataMachineUser {
                    user_id: user_id__.unwrap_or_default(),
                    user: user__,
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataMachineUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataOidcApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_id.is_empty() {
            len += 1;
        }
        if self.app.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataOIDCApplication", len)?;
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("appId", &self.app_id)?;
        }
        if let Some(v) = self.app.as_ref() {
            struct_ser.serialize_field("app", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataOidcApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app_id",
            "appId",
            "app",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppId,
            App,
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
                            "appId" | "app_id" => Ok(GeneratedField::AppId),
                            "app" => Ok(GeneratedField::App),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataOidcApplication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataOIDCApplication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataOidcApplication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_id__ = None;
                let mut app__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appId"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::App => {
                            if app__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app"));
                            }
                            app__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataOidcApplication {
                    app_id: app_id__.unwrap_or_default(),
                    app: app__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataOIDCApplication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataOidcidp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.idp_id.is_empty() {
            len += 1;
        }
        if self.idp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataOIDCIDP", len)?;
        if !self.idp_id.is_empty() {
            struct_ser.serialize_field("idpId", &self.idp_id)?;
        }
        if let Some(v) = self.idp.as_ref() {
            struct_ser.serialize_field("idp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataOidcidp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idp_id",
            "idpId",
            "idp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdpId,
            Idp,
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
                            "idpId" | "idp_id" => Ok(GeneratedField::IdpId),
                            "idp" => Ok(GeneratedField::Idp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataOidcidp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataOIDCIDP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataOidcidp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idp_id__ = None;
                let mut idp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IdpId => {
                            if idp_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idpId"));
                            }
                            idp_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Idp => {
                            if idp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idp"));
                            }
                            idp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataOidcidp {
                    idp_id: idp_id__.unwrap_or_default(),
                    idp: idp__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataOIDCIDP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataOrg {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.org_id.is_empty() {
            len += 1;
        }
        if self.org.is_some() {
            len += 1;
        }
        if self.iam_policy.is_some() {
            len += 1;
        }
        if self.label_policy.is_some() {
            len += 1;
        }
        if self.lockout_policy.is_some() {
            len += 1;
        }
        if self.login_policy.is_some() {
            len += 1;
        }
        if self.password_complexity_policy.is_some() {
            len += 1;
        }
        if self.privacy_policy.is_some() {
            len += 1;
        }
        if !self.projects.is_empty() {
            len += 1;
        }
        if !self.project_roles.is_empty() {
            len += 1;
        }
        if !self.api_apps.is_empty() {
            len += 1;
        }
        if !self.oidc_apps.is_empty() {
            len += 1;
        }
        if !self.human_users.is_empty() {
            len += 1;
        }
        if !self.machine_users.is_empty() {
            len += 1;
        }
        if !self.trigger_actions.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if !self.project_grants.is_empty() {
            len += 1;
        }
        if !self.user_grants.is_empty() {
            len += 1;
        }
        if !self.org_members.is_empty() {
            len += 1;
        }
        if !self.project_members.is_empty() {
            len += 1;
        }
        if !self.project_grant_members.is_empty() {
            len += 1;
        }
        if !self.user_metadata.is_empty() {
            len += 1;
        }
        if !self.login_texts.is_empty() {
            len += 1;
        }
        if !self.init_messages.is_empty() {
            len += 1;
        }
        if !self.password_reset_messages.is_empty() {
            len += 1;
        }
        if !self.verify_email_messages.is_empty() {
            len += 1;
        }
        if !self.verify_phone_messages.is_empty() {
            len += 1;
        }
        if !self.domain_claimed_messages.is_empty() {
            len += 1;
        }
        if !self.passwordless_registration_messages.is_empty() {
            len += 1;
        }
        if !self.oidc_idps.is_empty() {
            len += 1;
        }
        if !self.jwt_idps.is_empty() {
            len += 1;
        }
        if !self.second_factors.is_empty() {
            len += 1;
        }
        if !self.multi_factors.is_empty() {
            len += 1;
        }
        if !self.idps.is_empty() {
            len += 1;
        }
        if !self.user_links.is_empty() {
            len += 1;
        }
        if !self.domains.is_empty() {
            len += 1;
        }
        if !self.app_keys.is_empty() {
            len += 1;
        }
        if !self.machine_keys.is_empty() {
            len += 1;
        }
        if !self.invite_user_messages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataOrg", len)?;
        if !self.org_id.is_empty() {
            struct_ser.serialize_field("orgId", &self.org_id)?;
        }
        if let Some(v) = self.org.as_ref() {
            struct_ser.serialize_field("org", v)?;
        }
        if let Some(v) = self.iam_policy.as_ref() {
            struct_ser.serialize_field("iamPolicy", v)?;
        }
        if let Some(v) = self.label_policy.as_ref() {
            struct_ser.serialize_field("labelPolicy", v)?;
        }
        if let Some(v) = self.lockout_policy.as_ref() {
            struct_ser.serialize_field("lockoutPolicy", v)?;
        }
        if let Some(v) = self.login_policy.as_ref() {
            struct_ser.serialize_field("loginPolicy", v)?;
        }
        if let Some(v) = self.password_complexity_policy.as_ref() {
            struct_ser.serialize_field("passwordComplexityPolicy", v)?;
        }
        if let Some(v) = self.privacy_policy.as_ref() {
            struct_ser.serialize_field("privacyPolicy", v)?;
        }
        if !self.projects.is_empty() {
            struct_ser.serialize_field("projects", &self.projects)?;
        }
        if !self.project_roles.is_empty() {
            struct_ser.serialize_field("projectRoles", &self.project_roles)?;
        }
        if !self.api_apps.is_empty() {
            struct_ser.serialize_field("apiApps", &self.api_apps)?;
        }
        if !self.oidc_apps.is_empty() {
            struct_ser.serialize_field("oidcApps", &self.oidc_apps)?;
        }
        if !self.human_users.is_empty() {
            struct_ser.serialize_field("humanUsers", &self.human_users)?;
        }
        if !self.machine_users.is_empty() {
            struct_ser.serialize_field("machineUsers", &self.machine_users)?;
        }
        if !self.trigger_actions.is_empty() {
            struct_ser.serialize_field("triggerActions", &self.trigger_actions)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.project_grants.is_empty() {
            struct_ser.serialize_field("projectGrants", &self.project_grants)?;
        }
        if !self.user_grants.is_empty() {
            struct_ser.serialize_field("userGrants", &self.user_grants)?;
        }
        if !self.org_members.is_empty() {
            struct_ser.serialize_field("orgMembers", &self.org_members)?;
        }
        if !self.project_members.is_empty() {
            struct_ser.serialize_field("projectMembers", &self.project_members)?;
        }
        if !self.project_grant_members.is_empty() {
            struct_ser.serialize_field("projectGrantMembers", &self.project_grant_members)?;
        }
        if !self.user_metadata.is_empty() {
            struct_ser.serialize_field("userMetadata", &self.user_metadata)?;
        }
        if !self.login_texts.is_empty() {
            struct_ser.serialize_field("loginTexts", &self.login_texts)?;
        }
        if !self.init_messages.is_empty() {
            struct_ser.serialize_field("initMessages", &self.init_messages)?;
        }
        if !self.password_reset_messages.is_empty() {
            struct_ser.serialize_field("passwordResetMessages", &self.password_reset_messages)?;
        }
        if !self.verify_email_messages.is_empty() {
            struct_ser.serialize_field("verifyEmailMessages", &self.verify_email_messages)?;
        }
        if !self.verify_phone_messages.is_empty() {
            struct_ser.serialize_field("verifyPhoneMessages", &self.verify_phone_messages)?;
        }
        if !self.domain_claimed_messages.is_empty() {
            struct_ser.serialize_field("domainClaimedMessages", &self.domain_claimed_messages)?;
        }
        if !self.passwordless_registration_messages.is_empty() {
            struct_ser.serialize_field("passwordlessRegistrationMessages", &self.passwordless_registration_messages)?;
        }
        if !self.oidc_idps.is_empty() {
            struct_ser.serialize_field("oidcIdps", &self.oidc_idps)?;
        }
        if !self.jwt_idps.is_empty() {
            struct_ser.serialize_field("jwtIdps", &self.jwt_idps)?;
        }
        if !self.second_factors.is_empty() {
            struct_ser.serialize_field("secondFactors", &self.second_factors)?;
        }
        if !self.multi_factors.is_empty() {
            struct_ser.serialize_field("multiFactors", &self.multi_factors)?;
        }
        if !self.idps.is_empty() {
            struct_ser.serialize_field("idps", &self.idps)?;
        }
        if !self.user_links.is_empty() {
            struct_ser.serialize_field("userLinks", &self.user_links)?;
        }
        if !self.domains.is_empty() {
            struct_ser.serialize_field("domains", &self.domains)?;
        }
        if !self.app_keys.is_empty() {
            struct_ser.serialize_field("appKeys", &self.app_keys)?;
        }
        if !self.machine_keys.is_empty() {
            struct_ser.serialize_field("machineKeys", &self.machine_keys)?;
        }
        if !self.invite_user_messages.is_empty() {
            struct_ser.serialize_field("inviteUserMessages", &self.invite_user_messages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataOrg {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "org_id",
            "orgId",
            "org",
            "iam_policy",
            "iamPolicy",
            "label_policy",
            "labelPolicy",
            "lockout_policy",
            "lockoutPolicy",
            "login_policy",
            "loginPolicy",
            "password_complexity_policy",
            "passwordComplexityPolicy",
            "privacy_policy",
            "privacyPolicy",
            "projects",
            "project_roles",
            "projectRoles",
            "api_apps",
            "apiApps",
            "oidc_apps",
            "oidcApps",
            "human_users",
            "humanUsers",
            "machine_users",
            "machineUsers",
            "trigger_actions",
            "triggerActions",
            "actions",
            "project_grants",
            "projectGrants",
            "user_grants",
            "userGrants",
            "org_members",
            "orgMembers",
            "project_members",
            "projectMembers",
            "project_grant_members",
            "projectGrantMembers",
            "user_metadata",
            "userMetadata",
            "login_texts",
            "loginTexts",
            "init_messages",
            "initMessages",
            "password_reset_messages",
            "passwordResetMessages",
            "verify_email_messages",
            "verifyEmailMessages",
            "verify_phone_messages",
            "verifyPhoneMessages",
            "domain_claimed_messages",
            "domainClaimedMessages",
            "passwordless_registration_messages",
            "passwordlessRegistrationMessages",
            "oidc_idps",
            "oidcIdps",
            "jwt_idps",
            "jwtIdps",
            "second_factors",
            "secondFactors",
            "multi_factors",
            "multiFactors",
            "idps",
            "user_links",
            "userLinks",
            "domains",
            "app_keys",
            "appKeys",
            "machine_keys",
            "machineKeys",
            "invite_user_messages",
            "inviteUserMessages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrgId,
            Org,
            IamPolicy,
            LabelPolicy,
            LockoutPolicy,
            LoginPolicy,
            PasswordComplexityPolicy,
            PrivacyPolicy,
            Projects,
            ProjectRoles,
            ApiApps,
            OidcApps,
            HumanUsers,
            MachineUsers,
            TriggerActions,
            Actions,
            ProjectGrants,
            UserGrants,
            OrgMembers,
            ProjectMembers,
            ProjectGrantMembers,
            UserMetadata,
            LoginTexts,
            InitMessages,
            PasswordResetMessages,
            VerifyEmailMessages,
            VerifyPhoneMessages,
            DomainClaimedMessages,
            PasswordlessRegistrationMessages,
            OidcIdps,
            JwtIdps,
            SecondFactors,
            MultiFactors,
            Idps,
            UserLinks,
            Domains,
            AppKeys,
            MachineKeys,
            InviteUserMessages,
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
                            "org" => Ok(GeneratedField::Org),
                            "iamPolicy" | "iam_policy" => Ok(GeneratedField::IamPolicy),
                            "labelPolicy" | "label_policy" => Ok(GeneratedField::LabelPolicy),
                            "lockoutPolicy" | "lockout_policy" => Ok(GeneratedField::LockoutPolicy),
                            "loginPolicy" | "login_policy" => Ok(GeneratedField::LoginPolicy),
                            "passwordComplexityPolicy" | "password_complexity_policy" => Ok(GeneratedField::PasswordComplexityPolicy),
                            "privacyPolicy" | "privacy_policy" => Ok(GeneratedField::PrivacyPolicy),
                            "projects" => Ok(GeneratedField::Projects),
                            "projectRoles" | "project_roles" => Ok(GeneratedField::ProjectRoles),
                            "apiApps" | "api_apps" => Ok(GeneratedField::ApiApps),
                            "oidcApps" | "oidc_apps" => Ok(GeneratedField::OidcApps),
                            "humanUsers" | "human_users" => Ok(GeneratedField::HumanUsers),
                            "machineUsers" | "machine_users" => Ok(GeneratedField::MachineUsers),
                            "triggerActions" | "trigger_actions" => Ok(GeneratedField::TriggerActions),
                            "actions" => Ok(GeneratedField::Actions),
                            "projectGrants" | "project_grants" => Ok(GeneratedField::ProjectGrants),
                            "userGrants" | "user_grants" => Ok(GeneratedField::UserGrants),
                            "orgMembers" | "org_members" => Ok(GeneratedField::OrgMembers),
                            "projectMembers" | "project_members" => Ok(GeneratedField::ProjectMembers),
                            "projectGrantMembers" | "project_grant_members" => Ok(GeneratedField::ProjectGrantMembers),
                            "userMetadata" | "user_metadata" => Ok(GeneratedField::UserMetadata),
                            "loginTexts" | "login_texts" => Ok(GeneratedField::LoginTexts),
                            "initMessages" | "init_messages" => Ok(GeneratedField::InitMessages),
                            "passwordResetMessages" | "password_reset_messages" => Ok(GeneratedField::PasswordResetMessages),
                            "verifyEmailMessages" | "verify_email_messages" => Ok(GeneratedField::VerifyEmailMessages),
                            "verifyPhoneMessages" | "verify_phone_messages" => Ok(GeneratedField::VerifyPhoneMessages),
                            "domainClaimedMessages" | "domain_claimed_messages" => Ok(GeneratedField::DomainClaimedMessages),
                            "passwordlessRegistrationMessages" | "passwordless_registration_messages" => Ok(GeneratedField::PasswordlessRegistrationMessages),
                            "oidcIdps" | "oidc_idps" => Ok(GeneratedField::OidcIdps),
                            "jwtIdps" | "jwt_idps" => Ok(GeneratedField::JwtIdps),
                            "secondFactors" | "second_factors" => Ok(GeneratedField::SecondFactors),
                            "multiFactors" | "multi_factors" => Ok(GeneratedField::MultiFactors),
                            "idps" => Ok(GeneratedField::Idps),
                            "userLinks" | "user_links" => Ok(GeneratedField::UserLinks),
                            "domains" => Ok(GeneratedField::Domains),
                            "appKeys" | "app_keys" => Ok(GeneratedField::AppKeys),
                            "machineKeys" | "machine_keys" => Ok(GeneratedField::MachineKeys),
                            "inviteUserMessages" | "invite_user_messages" => Ok(GeneratedField::InviteUserMessages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataOrg;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataOrg")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataOrg, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut org_id__ = None;
                let mut org__ = None;
                let mut iam_policy__ = None;
                let mut label_policy__ = None;
                let mut lockout_policy__ = None;
                let mut login_policy__ = None;
                let mut password_complexity_policy__ = None;
                let mut privacy_policy__ = None;
                let mut projects__ = None;
                let mut project_roles__ = None;
                let mut api_apps__ = None;
                let mut oidc_apps__ = None;
                let mut human_users__ = None;
                let mut machine_users__ = None;
                let mut trigger_actions__ = None;
                let mut actions__ = None;
                let mut project_grants__ = None;
                let mut user_grants__ = None;
                let mut org_members__ = None;
                let mut project_members__ = None;
                let mut project_grant_members__ = None;
                let mut user_metadata__ = None;
                let mut login_texts__ = None;
                let mut init_messages__ = None;
                let mut password_reset_messages__ = None;
                let mut verify_email_messages__ = None;
                let mut verify_phone_messages__ = None;
                let mut domain_claimed_messages__ = None;
                let mut passwordless_registration_messages__ = None;
                let mut oidc_idps__ = None;
                let mut jwt_idps__ = None;
                let mut second_factors__ = None;
                let mut multi_factors__ = None;
                let mut idps__ = None;
                let mut user_links__ = None;
                let mut domains__ = None;
                let mut app_keys__ = None;
                let mut machine_keys__ = None;
                let mut invite_user_messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrgId => {
                            if org_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgId"));
                            }
                            org_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Org => {
                            if org__.is_some() {
                                return Err(serde::de::Error::duplicate_field("org"));
                            }
                            org__ = map_.next_value()?;
                        }
                        GeneratedField::IamPolicy => {
                            if iam_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("iamPolicy"));
                            }
                            iam_policy__ = map_.next_value()?;
                        }
                        GeneratedField::LabelPolicy => {
                            if label_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labelPolicy"));
                            }
                            label_policy__ = map_.next_value()?;
                        }
                        GeneratedField::LockoutPolicy => {
                            if lockout_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lockoutPolicy"));
                            }
                            lockout_policy__ = map_.next_value()?;
                        }
                        GeneratedField::LoginPolicy => {
                            if login_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginPolicy"));
                            }
                            login_policy__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordComplexityPolicy => {
                            if password_complexity_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordComplexityPolicy"));
                            }
                            password_complexity_policy__ = map_.next_value()?;
                        }
                        GeneratedField::PrivacyPolicy => {
                            if privacy_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privacyPolicy"));
                            }
                            privacy_policy__ = map_.next_value()?;
                        }
                        GeneratedField::Projects => {
                            if projects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projects"));
                            }
                            projects__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectRoles => {
                            if project_roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoles"));
                            }
                            project_roles__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApiApps => {
                            if api_apps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiApps"));
                            }
                            api_apps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OidcApps => {
                            if oidc_apps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcApps"));
                            }
                            oidc_apps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HumanUsers => {
                            if human_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("humanUsers"));
                            }
                            human_users__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MachineUsers => {
                            if machine_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machineUsers"));
                            }
                            machine_users__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TriggerActions => {
                            if trigger_actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggerActions"));
                            }
                            trigger_actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectGrants => {
                            if project_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrants"));
                            }
                            project_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserGrants => {
                            if user_grants__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGrants"));
                            }
                            user_grants__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrgMembers => {
                            if org_members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgMembers"));
                            }
                            org_members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectMembers => {
                            if project_members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectMembers"));
                            }
                            project_members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectGrantMembers => {
                            if project_grant_members__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrantMembers"));
                            }
                            project_grant_members__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserMetadata => {
                            if user_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userMetadata"));
                            }
                            user_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginTexts => {
                            if login_texts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginTexts"));
                            }
                            login_texts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitMessages => {
                            if init_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initMessages"));
                            }
                            init_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordResetMessages => {
                            if password_reset_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordResetMessages"));
                            }
                            password_reset_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VerifyEmailMessages => {
                            if verify_email_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyEmailMessages"));
                            }
                            verify_email_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VerifyPhoneMessages => {
                            if verify_phone_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyPhoneMessages"));
                            }
                            verify_phone_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DomainClaimedMessages => {
                            if domain_claimed_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainClaimedMessages"));
                            }
                            domain_claimed_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordlessRegistrationMessages => {
                            if passwordless_registration_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordlessRegistrationMessages"));
                            }
                            passwordless_registration_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OidcIdps => {
                            if oidc_idps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oidcIdps"));
                            }
                            oidc_idps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JwtIdps => {
                            if jwt_idps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jwtIdps"));
                            }
                            jwt_idps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SecondFactors => {
                            if second_factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secondFactors"));
                            }
                            second_factors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MultiFactors => {
                            if multi_factors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiFactors"));
                            }
                            multi_factors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Idps => {
                            if idps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idps"));
                            }
                            idps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserLinks => {
                            if user_links__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userLinks"));
                            }
                            user_links__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domains => {
                            if domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domains"));
                            }
                            domains__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppKeys => {
                            if app_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appKeys"));
                            }
                            app_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MachineKeys => {
                            if machine_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machineKeys"));
                            }
                            machine_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InviteUserMessages => {
                            if invite_user_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inviteUserMessages"));
                            }
                            invite_user_messages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DataOrg {
                    org_id: org_id__.unwrap_or_default(),
                    org: org__,
                    iam_policy: iam_policy__,
                    label_policy: label_policy__,
                    lockout_policy: lockout_policy__,
                    login_policy: login_policy__,
                    password_complexity_policy: password_complexity_policy__,
                    privacy_policy: privacy_policy__,
                    projects: projects__.unwrap_or_default(),
                    project_roles: project_roles__.unwrap_or_default(),
                    api_apps: api_apps__.unwrap_or_default(),
                    oidc_apps: oidc_apps__.unwrap_or_default(),
                    human_users: human_users__.unwrap_or_default(),
                    machine_users: machine_users__.unwrap_or_default(),
                    trigger_actions: trigger_actions__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    project_grants: project_grants__.unwrap_or_default(),
                    user_grants: user_grants__.unwrap_or_default(),
                    org_members: org_members__.unwrap_or_default(),
                    project_members: project_members__.unwrap_or_default(),
                    project_grant_members: project_grant_members__.unwrap_or_default(),
                    user_metadata: user_metadata__.unwrap_or_default(),
                    login_texts: login_texts__.unwrap_or_default(),
                    init_messages: init_messages__.unwrap_or_default(),
                    password_reset_messages: password_reset_messages__.unwrap_or_default(),
                    verify_email_messages: verify_email_messages__.unwrap_or_default(),
                    verify_phone_messages: verify_phone_messages__.unwrap_or_default(),
                    domain_claimed_messages: domain_claimed_messages__.unwrap_or_default(),
                    passwordless_registration_messages: passwordless_registration_messages__.unwrap_or_default(),
                    oidc_idps: oidc_idps__.unwrap_or_default(),
                    jwt_idps: jwt_idps__.unwrap_or_default(),
                    second_factors: second_factors__.unwrap_or_default(),
                    multi_factors: multi_factors__.unwrap_or_default(),
                    idps: idps__.unwrap_or_default(),
                    user_links: user_links__.unwrap_or_default(),
                    domains: domains__.unwrap_or_default(),
                    app_keys: app_keys__.unwrap_or_default(),
                    machine_keys: machine_keys__.unwrap_or_default(),
                    invite_user_messages: invite_user_messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataOrg", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataProject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if self.project.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataProject", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.project.as_ref() {
            struct_ser.serialize_field("project", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataProject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "project",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            Project,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "project" => Ok(GeneratedField::Project),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataProject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataProject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataProject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut project__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Project => {
                            if project__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            project__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataProject {
                    project_id: project_id__.unwrap_or_default(),
                    project: project__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataProject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataProjectGrant {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.grant_id.is_empty() {
            len += 1;
        }
        if self.project_grant.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.DataProjectGrant", len)?;
        if !self.grant_id.is_empty() {
            struct_ser.serialize_field("grantId", &self.grant_id)?;
        }
        if let Some(v) = self.project_grant.as_ref() {
            struct_ser.serialize_field("projectGrant", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataProjectGrant {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grant_id",
            "grantId",
            "project_grant",
            "projectGrant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrantId,
            ProjectGrant,
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
                            "grantId" | "grant_id" => Ok(GeneratedField::GrantId),
                            "projectGrant" | "project_grant" => Ok(GeneratedField::ProjectGrant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataProjectGrant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.DataProjectGrant")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataProjectGrant, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grant_id__ = None;
                let mut project_grant__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::GrantId => {
                            if grant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grantId"));
                            }
                            grant_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectGrant => {
                            if project_grant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectGrant"));
                            }
                            project_grant__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataProjectGrant {
                    grant_id: grant_id__.unwrap_or_default(),
                    project_grant: project_grant__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.DataProjectGrant", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportHumanUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.profile.is_some() {
            len += 1;
        }
        if self.email.is_some() {
            len += 1;
        }
        if self.phone.is_some() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        if self.hashed_password.is_some() {
            len += 1;
        }
        if self.password_change_required {
            len += 1;
        }
        if self.request_passwordless_registration {
            len += 1;
        }
        if !self.otp_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.ExportHumanUser", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.phone.as_ref() {
            struct_ser.serialize_field("phone", v)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if let Some(v) = self.hashed_password.as_ref() {
            struct_ser.serialize_field("hashedPassword", v)?;
        }
        if self.password_change_required {
            struct_ser.serialize_field("passwordChangeRequired", &self.password_change_required)?;
        }
        if self.request_passwordless_registration {
            struct_ser.serialize_field("requestPasswordlessRegistration", &self.request_passwordless_registration)?;
        }
        if !self.otp_code.is_empty() {
            struct_ser.serialize_field("otpCode", &self.otp_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportHumanUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name",
            "userName",
            "profile",
            "email",
            "phone",
            "password",
            "hashed_password",
            "hashedPassword",
            "password_change_required",
            "passwordChangeRequired",
            "request_passwordless_registration",
            "requestPasswordlessRegistration",
            "otp_code",
            "otpCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserName,
            Profile,
            Email,
            Phone,
            Password,
            HashedPassword,
            PasswordChangeRequired,
            RequestPasswordlessRegistration,
            OtpCode,
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "profile" => Ok(GeneratedField::Profile),
                            "email" => Ok(GeneratedField::Email),
                            "phone" => Ok(GeneratedField::Phone),
                            "password" => Ok(GeneratedField::Password),
                            "hashedPassword" | "hashed_password" => Ok(GeneratedField::HashedPassword),
                            "passwordChangeRequired" | "password_change_required" => Ok(GeneratedField::PasswordChangeRequired),
                            "requestPasswordlessRegistration" | "request_passwordless_registration" => Ok(GeneratedField::RequestPasswordlessRegistration),
                            "otpCode" | "otp_code" => Ok(GeneratedField::OtpCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportHumanUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.ExportHumanUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportHumanUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_name__ = None;
                let mut profile__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                let mut password__ = None;
                let mut hashed_password__ = None;
                let mut password_change_required__ = None;
                let mut request_passwordless_registration__ = None;
                let mut otp_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Profile => {
                            if profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            profile__ = map_.next_value()?;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = map_.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HashedPassword => {
                            if hashed_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashedPassword"));
                            }
                            hashed_password__ = map_.next_value()?;
                        }
                        GeneratedField::PasswordChangeRequired => {
                            if password_change_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChangeRequired"));
                            }
                            password_change_required__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestPasswordlessRegistration => {
                            if request_passwordless_registration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestPasswordlessRegistration"));
                            }
                            request_passwordless_registration__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OtpCode => {
                            if otp_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otpCode"));
                            }
                            otp_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExportHumanUser {
                    user_name: user_name__.unwrap_or_default(),
                    profile: profile__,
                    email: email__,
                    phone: phone__,
                    password: password__.unwrap_or_default(),
                    hashed_password: hashed_password__,
                    password_change_required: password_change_required__.unwrap_or_default(),
                    request_passwordless_registration: request_passwordless_registration__.unwrap_or_default(),
                    otp_code: otp_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.ExportHumanUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for export_human_user::Email {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if self.is_email_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.ExportHumanUser.Email", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.is_email_verified {
            struct_ser.serialize_field("isEmailVerified", &self.is_email_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for export_human_user::Email {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "is_email_verified",
            "isEmailVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            IsEmailVerified,
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
                            "email" => Ok(GeneratedField::Email),
                            "isEmailVerified" | "is_email_verified" => Ok(GeneratedField::IsEmailVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = export_human_user::Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.ExportHumanUser.Email")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<export_human_user::Email, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut is_email_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsEmailVerified => {
                            if is_email_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEmailVerified"));
                            }
                            is_email_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(export_human_user::Email {
                    email: email__.unwrap_or_default(),
                    is_email_verified: is_email_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.ExportHumanUser.Email", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for export_human_user::HashedPassword {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.algorithm.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.ExportHumanUser.HashedPassword", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.algorithm.is_empty() {
            struct_ser.serialize_field("algorithm", &self.algorithm)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for export_human_user::HashedPassword {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
            "algorithm",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
            Algorithm,
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
                            "value" => Ok(GeneratedField::Value),
                            "algorithm" => Ok(GeneratedField::Algorithm),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = export_human_user::HashedPassword;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.ExportHumanUser.HashedPassword")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<export_human_user::HashedPassword, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                let mut algorithm__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Algorithm => {
                            if algorithm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("algorithm"));
                            }
                            algorithm__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(export_human_user::HashedPassword {
                    value: value__.unwrap_or_default(),
                    algorithm: algorithm__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.ExportHumanUser.HashedPassword", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for export_human_user::Phone {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone.is_empty() {
            len += 1;
        }
        if self.is_phone_verified {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.ExportHumanUser.Phone", len)?;
        if !self.phone.is_empty() {
            struct_ser.serialize_field("phone", &self.phone)?;
        }
        if self.is_phone_verified {
            struct_ser.serialize_field("isPhoneVerified", &self.is_phone_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for export_human_user::Phone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phone",
            "is_phone_verified",
            "isPhoneVerified",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phone,
            IsPhoneVerified,
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
                            "phone" => Ok(GeneratedField::Phone),
                            "isPhoneVerified" | "is_phone_verified" => Ok(GeneratedField::IsPhoneVerified),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = export_human_user::Phone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.ExportHumanUser.Phone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<export_human_user::Phone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone__ = None;
                let mut is_phone_verified__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPhoneVerified => {
                            if is_phone_verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPhoneVerified"));
                            }
                            is_phone_verified__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(export_human_user::Phone {
                    phone: phone__.unwrap_or_default(),
                    is_phone_verified: is_phone_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.ExportHumanUser.Phone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for export_human_user::Profile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.first_name.is_empty() {
            len += 1;
        }
        if !self.last_name.is_empty() {
            len += 1;
        }
        if !self.nick_name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.preferred_language.is_empty() {
            len += 1;
        }
        if self.gender != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.ExportHumanUser.Profile", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if !self.nick_name.is_empty() {
            struct_ser.serialize_field("nickName", &self.nick_name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.preferred_language.is_empty() {
            struct_ser.serialize_field("preferredLanguage", &self.preferred_language)?;
        }
        if self.gender != 0 {
            let v = super::super::user::v1::Gender::try_from(self.gender)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.gender)))?;
            struct_ser.serialize_field("gender", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for export_human_user::Profile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "first_name",
            "firstName",
            "last_name",
            "lastName",
            "nick_name",
            "nickName",
            "display_name",
            "displayName",
            "preferred_language",
            "preferredLanguage",
            "gender",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstName,
            LastName,
            NickName,
            DisplayName,
            PreferredLanguage,
            Gender,
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
                            "firstName" | "first_name" => Ok(GeneratedField::FirstName),
                            "lastName" | "last_name" => Ok(GeneratedField::LastName),
                            "nickName" | "nick_name" => Ok(GeneratedField::NickName),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "preferredLanguage" | "preferred_language" => Ok(GeneratedField::PreferredLanguage),
                            "gender" => Ok(GeneratedField::Gender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = export_human_user::Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.ExportHumanUser.Profile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<export_human_user::Profile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut nick_name__ = None;
                let mut display_name__ = None;
                let mut preferred_language__ = None;
                let mut gender__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FirstName => {
                            if first_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstName"));
                            }
                            first_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastName => {
                            if last_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastName"));
                            }
                            last_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NickName => {
                            if nick_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickName"));
                            }
                            nick_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PreferredLanguage => {
                            if preferred_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguage"));
                            }
                            preferred_language__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gender => {
                            if gender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            gender__ = Some(map_.next_value::<super::super::user::v1::Gender>()? as i32);
                        }
                    }
                }
                Ok(export_human_user::Profile {
                    first_name: first_name__.unwrap_or_default(),
                    last_name: last_name__.unwrap_or_default(),
                    nick_name: nick_name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    preferred_language: preferred_language__.unwrap_or_default(),
                    gender: gender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.ExportHumanUser.Profile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlowType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FLOW_TYPE_UNSPECIFIED",
            Self::ExternalAuthentication => "FLOW_TYPE_EXTERNAL_AUTHENTICATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FlowType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FLOW_TYPE_UNSPECIFIED",
            "FLOW_TYPE_EXTERNAL_AUTHENTICATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FlowType;

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
                    "FLOW_TYPE_UNSPECIFIED" => Ok(FlowType::Unspecified),
                    "FLOW_TYPE_EXTERNAL_AUTHENTICATION" => Ok(FlowType::ExternalAuthentication),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ImportDataOrg {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.orgs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.ImportDataOrg", len)?;
        if !self.orgs.is_empty() {
            struct_ser.serialize_field("orgs", &self.orgs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportDataOrg {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "orgs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Orgs,
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
                            "orgs" => Ok(GeneratedField::Orgs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportDataOrg;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.ImportDataOrg")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportDataOrg, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut orgs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Orgs => {
                            if orgs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orgs"));
                            }
                            orgs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportDataOrg {
                    orgs: orgs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.ImportDataOrg", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetTriggerActionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.flow_type != 0 {
            len += 1;
        }
        if self.trigger_type != 0 {
            len += 1;
        }
        if !self.action_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.v1.v1.SetTriggerActionsRequest", len)?;
        if self.flow_type != 0 {
            let v = FlowType::try_from(self.flow_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.flow_type)))?;
            struct_ser.serialize_field("flowType", &v)?;
        }
        if self.trigger_type != 0 {
            let v = TriggerType::try_from(self.trigger_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.trigger_type)))?;
            struct_ser.serialize_field("triggerType", &v)?;
        }
        if !self.action_ids.is_empty() {
            struct_ser.serialize_field("actionIds", &self.action_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetTriggerActionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "flow_type",
            "flowType",
            "trigger_type",
            "triggerType",
            "action_ids",
            "actionIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FlowType,
            TriggerType,
            ActionIds,
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
                            "flowType" | "flow_type" => Ok(GeneratedField::FlowType),
                            "triggerType" | "trigger_type" => Ok(GeneratedField::TriggerType),
                            "actionIds" | "action_ids" => Ok(GeneratedField::ActionIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetTriggerActionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.v1.v1.SetTriggerActionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetTriggerActionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut flow_type__ = None;
                let mut trigger_type__ = None;
                let mut action_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FlowType => {
                            if flow_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flowType"));
                            }
                            flow_type__ = Some(map_.next_value::<FlowType>()? as i32);
                        }
                        GeneratedField::TriggerType => {
                            if trigger_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggerType"));
                            }
                            trigger_type__ = Some(map_.next_value::<TriggerType>()? as i32);
                        }
                        GeneratedField::ActionIds => {
                            if action_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionIds"));
                            }
                            action_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetTriggerActionsRequest {
                    flow_type: flow_type__.unwrap_or_default(),
                    trigger_type: trigger_type__.unwrap_or_default(),
                    action_ids: action_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.v1.v1.SetTriggerActionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TriggerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TRIGGER_TYPE_UNSPECIFIED",
            Self::PostAuthentication => "TRIGGER_TYPE_POST_AUTHENTICATION",
            Self::PreCreation => "TRIGGER_TYPE_PRE_CREATION",
            Self::PostCreation => "TRIGGER_TYPE_POST_CREATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TriggerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TRIGGER_TYPE_UNSPECIFIED",
            "TRIGGER_TYPE_POST_AUTHENTICATION",
            "TRIGGER_TYPE_PRE_CREATION",
            "TRIGGER_TYPE_POST_CREATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TriggerType;

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
                    "TRIGGER_TYPE_UNSPECIFIED" => Ok(TriggerType::Unspecified),
                    "TRIGGER_TYPE_POST_AUTHENTICATION" => Ok(TriggerType::PostAuthentication),
                    "TRIGGER_TYPE_PRE_CREATION" => Ok(TriggerType::PreCreation),
                    "TRIGGER_TYPE_POST_CREATION" => Ok(TriggerType::PostCreation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
