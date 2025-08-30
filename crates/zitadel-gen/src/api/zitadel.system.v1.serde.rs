// @generated
impl serde::Serialize for AddDomainRequest {
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
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddDomainRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddDomainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Domain,
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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddDomainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddDomainRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddDomainResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddDomainResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddDomainResponse {
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
            type Value = AddDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddDomainResponse, V::Error>
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
                Ok(AddDomainResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddInstanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if !self.first_org_name.is_empty() {
            len += 1;
        }
        if !self.custom_domain.is_empty() {
            len += 1;
        }
        if !self.owner_user_name.is_empty() {
            len += 1;
        }
        if self.owner_email.is_some() {
            len += 1;
        }
        if self.owner_profile.is_some() {
            len += 1;
        }
        if self.owner_password.is_some() {
            len += 1;
        }
        if !self.default_language.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddInstanceRequest", len)?;
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        if !self.first_org_name.is_empty() {
            struct_ser.serialize_field("firstOrgName", &self.first_org_name)?;
        }
        if !self.custom_domain.is_empty() {
            struct_ser.serialize_field("customDomain", &self.custom_domain)?;
        }
        if !self.owner_user_name.is_empty() {
            struct_ser.serialize_field("ownerUserName", &self.owner_user_name)?;
        }
        if let Some(v) = self.owner_email.as_ref() {
            struct_ser.serialize_field("ownerEmail", v)?;
        }
        if let Some(v) = self.owner_profile.as_ref() {
            struct_ser.serialize_field("ownerProfile", v)?;
        }
        if let Some(v) = self.owner_password.as_ref() {
            struct_ser.serialize_field("ownerPassword", v)?;
        }
        if !self.default_language.is_empty() {
            struct_ser.serialize_field("defaultLanguage", &self.default_language)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddInstanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_name",
            "instanceName",
            "first_org_name",
            "firstOrgName",
            "custom_domain",
            "customDomain",
            "owner_user_name",
            "ownerUserName",
            "owner_email",
            "ownerEmail",
            "owner_profile",
            "ownerProfile",
            "owner_password",
            "ownerPassword",
            "default_language",
            "defaultLanguage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceName,
            FirstOrgName,
            CustomDomain,
            OwnerUserName,
            OwnerEmail,
            OwnerProfile,
            OwnerPassword,
            DefaultLanguage,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "firstOrgName" | "first_org_name" => Ok(GeneratedField::FirstOrgName),
                            "customDomain" | "custom_domain" => Ok(GeneratedField::CustomDomain),
                            "ownerUserName" | "owner_user_name" => Ok(GeneratedField::OwnerUserName),
                            "ownerEmail" | "owner_email" => Ok(GeneratedField::OwnerEmail),
                            "ownerProfile" | "owner_profile" => Ok(GeneratedField::OwnerProfile),
                            "ownerPassword" | "owner_password" => Ok(GeneratedField::OwnerPassword),
                            "defaultLanguage" | "default_language" => Ok(GeneratedField::DefaultLanguage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddInstanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddInstanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddInstanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_name__ = None;
                let mut first_org_name__ = None;
                let mut custom_domain__ = None;
                let mut owner_user_name__ = None;
                let mut owner_email__ = None;
                let mut owner_profile__ = None;
                let mut owner_password__ = None;
                let mut default_language__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstOrgName => {
                            if first_org_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstOrgName"));
                            }
                            first_org_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomDomain => {
                            if custom_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customDomain"));
                            }
                            custom_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OwnerUserName => {
                            if owner_user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerUserName"));
                            }
                            owner_user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OwnerEmail => {
                            if owner_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerEmail"));
                            }
                            owner_email__ = map_.next_value()?;
                        }
                        GeneratedField::OwnerProfile => {
                            if owner_profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerProfile"));
                            }
                            owner_profile__ = map_.next_value()?;
                        }
                        GeneratedField::OwnerPassword => {
                            if owner_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerPassword"));
                            }
                            owner_password__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultLanguage => {
                            if default_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLanguage"));
                            }
                            default_language__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddInstanceRequest {
                    instance_name: instance_name__.unwrap_or_default(),
                    first_org_name: first_org_name__.unwrap_or_default(),
                    custom_domain: custom_domain__.unwrap_or_default(),
                    owner_user_name: owner_user_name__.unwrap_or_default(),
                    owner_email: owner_email__,
                    owner_profile: owner_profile__,
                    owner_password: owner_password__,
                    default_language: default_language__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddInstanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for add_instance_request::Email {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddInstanceRequest.Email", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.is_email_verified {
            struct_ser.serialize_field("isEmailVerified", &self.is_email_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for add_instance_request::Email {
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
            type Value = add_instance_request::Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddInstanceRequest.Email")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<add_instance_request::Email, V::Error>
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
                Ok(add_instance_request::Email {
                    email: email__.unwrap_or_default(),
                    is_email_verified: is_email_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddInstanceRequest.Email", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for add_instance_request::Password {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.password.is_empty() {
            len += 1;
        }
        if self.password_change_required {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddInstanceRequest.Password", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if self.password_change_required {
            struct_ser.serialize_field("passwordChangeRequired", &self.password_change_required)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for add_instance_request::Password {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
            "password_change_required",
            "passwordChangeRequired",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
            PasswordChangeRequired,
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
                            "password" => Ok(GeneratedField::Password),
                            "passwordChangeRequired" | "password_change_required" => Ok(GeneratedField::PasswordChangeRequired),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = add_instance_request::Password;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddInstanceRequest.Password")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<add_instance_request::Password, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                let mut password_change_required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordChangeRequired => {
                            if password_change_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChangeRequired"));
                            }
                            password_change_required__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(add_instance_request::Password {
                    password: password__.unwrap_or_default(),
                    password_change_required: password_change_required__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddInstanceRequest.Password", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for add_instance_request::Profile {
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
        if !self.preferred_language.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddInstanceRequest.Profile", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if !self.preferred_language.is_empty() {
            struct_ser.serialize_field("preferredLanguage", &self.preferred_language)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for add_instance_request::Profile {
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
            "preferred_language",
            "preferredLanguage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstName,
            LastName,
            PreferredLanguage,
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
                            "preferredLanguage" | "preferred_language" => Ok(GeneratedField::PreferredLanguage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = add_instance_request::Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddInstanceRequest.Profile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<add_instance_request::Profile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut preferred_language__ = None;
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
                        GeneratedField::PreferredLanguage => {
                            if preferred_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguage"));
                            }
                            preferred_language__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(add_instance_request::Profile {
                    first_name: first_name__.unwrap_or_default(),
                    last_name: last_name__.unwrap_or_default(),
                    preferred_language: preferred_language__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddInstanceRequest.Profile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddInstanceResponse {
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
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddInstanceResponse", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddInstanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
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
                            "instanceId" | "instance_id" => Ok(GeneratedField::InstanceId),
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
            type Value = AddInstanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddInstanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddInstanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddInstanceResponse {
                    instance_id: instance_id__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddInstanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddQuotaRequest {
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
        if self.unit != 0 {
            len += 1;
        }
        if self.from.is_some() {
            len += 1;
        }
        if self.reset_interval.is_some() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        if self.limit {
            len += 1;
        }
        if !self.notifications.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddQuotaRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if self.unit != 0 {
            let v = super::super::quota::v1::Unit::try_from(self.unit)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        if let Some(v) = self.from.as_ref() {
            struct_ser.serialize_field("from", v)?;
        }
        if let Some(v) = self.reset_interval.as_ref() {
            struct_ser.serialize_field("resetInterval", v)?;
        }
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&self.amount).as_str())?;
        }
        if self.limit {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if !self.notifications.is_empty() {
            struct_ser.serialize_field("notifications", &self.notifications)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddQuotaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "unit",
            "from",
            "reset_interval",
            "resetInterval",
            "amount",
            "limit",
            "notifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Unit,
            From,
            ResetInterval,
            Amount,
            Limit,
            Notifications,
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
                            "unit" => Ok(GeneratedField::Unit),
                            "from" => Ok(GeneratedField::From),
                            "resetInterval" | "reset_interval" => Ok(GeneratedField::ResetInterval),
                            "amount" => Ok(GeneratedField::Amount),
                            "limit" => Ok(GeneratedField::Limit),
                            "notifications" => Ok(GeneratedField::Notifications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddQuotaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddQuotaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddQuotaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut unit__ = None;
                let mut from__ = None;
                let mut reset_interval__ = None;
                let mut amount__ = None;
                let mut limit__ = None;
                let mut notifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value::<super::super::quota::v1::Unit>()? as i32);
                        }
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = map_.next_value()?;
                        }
                        GeneratedField::ResetInterval => {
                            if reset_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetInterval"));
                            }
                            reset_interval__ = map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notifications => {
                            if notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notifications"));
                            }
                            notifications__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddQuotaRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    from: from__,
                    reset_interval: reset_interval__,
                    amount: amount__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    notifications: notifications__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddQuotaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddQuotaResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.AddQuotaResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddQuotaResponse {
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
            type Value = AddQuotaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.AddQuotaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddQuotaResponse, V::Error>
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
                Ok(AddQuotaResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.AddQuotaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkSetLimitsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.limits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.BulkSetLimitsRequest", len)?;
        if !self.limits.is_empty() {
            struct_ser.serialize_field("limits", &self.limits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkSetLimitsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limits,
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
                            "limits" => Ok(GeneratedField::Limits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkSetLimitsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.BulkSetLimitsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BulkSetLimitsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limits => {
                            if limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limits"));
                            }
                            limits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BulkSetLimitsRequest {
                    limits: limits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.BulkSetLimitsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BulkSetLimitsResponse {
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
        if !self.target_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.BulkSetLimitsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.target_details.is_empty() {
            struct_ser.serialize_field("targetDetails", &self.target_details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BulkSetLimitsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "target_details",
            "targetDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            TargetDetails,
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
                            "targetDetails" | "target_details" => Ok(GeneratedField::TargetDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BulkSetLimitsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.BulkSetLimitsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BulkSetLimitsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut target_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::TargetDetails => {
                            if target_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetDetails"));
                            }
                            target_details__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BulkSetLimitsResponse {
                    details: details__,
                    target_details: target_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.BulkSetLimitsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangeSubscriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.subscription_name.is_empty() {
            len += 1;
        }
        if self.request_limit != 0 {
            len += 1;
        }
        if self.action_mins_limit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ChangeSubscriptionRequest", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.subscription_name.is_empty() {
            struct_ser.serialize_field("subscriptionName", &self.subscription_name)?;
        }
        if self.request_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("requestLimit", ToString::to_string(&self.request_limit).as_str())?;
        }
        if self.action_mins_limit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("actionMinsLimit", ToString::to_string(&self.action_mins_limit).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChangeSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "subscription_name",
            "subscriptionName",
            "request_limit",
            "requestLimit",
            "action_mins_limit",
            "actionMinsLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            SubscriptionName,
            RequestLimit,
            ActionMinsLimit,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "subscriptionName" | "subscription_name" => Ok(GeneratedField::SubscriptionName),
                            "requestLimit" | "request_limit" => Ok(GeneratedField::RequestLimit),
                            "actionMinsLimit" | "action_mins_limit" => Ok(GeneratedField::ActionMinsLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChangeSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ChangeSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChangeSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut subscription_name__ = None;
                let mut request_limit__ = None;
                let mut action_mins_limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubscriptionName => {
                            if subscription_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptionName"));
                            }
                            subscription_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestLimit => {
                            if request_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestLimit"));
                            }
                            request_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ActionMinsLimit => {
                            if action_mins_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionMinsLimit"));
                            }
                            action_mins_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ChangeSubscriptionRequest {
                    domain: domain__.unwrap_or_default(),
                    subscription_name: subscription_name__.unwrap_or_default(),
                    request_limit: request_limit__.unwrap_or_default(),
                    action_mins_limit: action_mins_limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ChangeSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangeSubscriptionResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ChangeSubscriptionResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChangeSubscriptionResponse {
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
            type Value = ChangeSubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ChangeSubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChangeSubscriptionResponse, V::Error>
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
                Ok(ChangeSubscriptionResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ChangeSubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClearViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.database.is_empty() {
            len += 1;
        }
        if !self.view_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ClearViewRequest", len)?;
        if !self.database.is_empty() {
            struct_ser.serialize_field("database", &self.database)?;
        }
        if !self.view_name.is_empty() {
            struct_ser.serialize_field("viewName", &self.view_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClearViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "database",
            "view_name",
            "viewName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Database,
            ViewName,
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
                            "database" => Ok(GeneratedField::Database),
                            "viewName" | "view_name" => Ok(GeneratedField::ViewName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClearViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ClearViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClearViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut database__ = None;
                let mut view_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Database => {
                            if database__.is_some() {
                                return Err(serde::de::Error::duplicate_field("database"));
                            }
                            database__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ViewName => {
                            if view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewName"));
                            }
                            view_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClearViewRequest {
                    database: database__.unwrap_or_default(),
                    view_name: view_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ClearViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClearViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.system.v1.ClearViewResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClearViewResponse {
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
            type Value = ClearViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ClearViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClearViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ClearViewResponse {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ClearViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateInstanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if !self.first_org_name.is_empty() {
            len += 1;
        }
        if !self.custom_domain.is_empty() {
            len += 1;
        }
        if !self.default_language.is_empty() {
            len += 1;
        }
        if self.owner.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest", len)?;
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        if !self.first_org_name.is_empty() {
            struct_ser.serialize_field("firstOrgName", &self.first_org_name)?;
        }
        if !self.custom_domain.is_empty() {
            struct_ser.serialize_field("customDomain", &self.custom_domain)?;
        }
        if !self.default_language.is_empty() {
            struct_ser.serialize_field("defaultLanguage", &self.default_language)?;
        }
        if let Some(v) = self.owner.as_ref() {
            match v {
                create_instance_request::Owner::Human(v) => {
                    struct_ser.serialize_field("human", v)?;
                }
                create_instance_request::Owner::Machine(v) => {
                    struct_ser.serialize_field("machine", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateInstanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_name",
            "instanceName",
            "first_org_name",
            "firstOrgName",
            "custom_domain",
            "customDomain",
            "default_language",
            "defaultLanguage",
            "human",
            "machine",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceName,
            FirstOrgName,
            CustomDomain,
            DefaultLanguage,
            Human,
            Machine,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "firstOrgName" | "first_org_name" => Ok(GeneratedField::FirstOrgName),
                            "customDomain" | "custom_domain" => Ok(GeneratedField::CustomDomain),
                            "defaultLanguage" | "default_language" => Ok(GeneratedField::DefaultLanguage),
                            "human" => Ok(GeneratedField::Human),
                            "machine" => Ok(GeneratedField::Machine),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateInstanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateInstanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_name__ = None;
                let mut first_org_name__ = None;
                let mut custom_domain__ = None;
                let mut default_language__ = None;
                let mut owner__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstOrgName => {
                            if first_org_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstOrgName"));
                            }
                            first_org_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomDomain => {
                            if custom_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customDomain"));
                            }
                            custom_domain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DefaultLanguage => {
                            if default_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLanguage"));
                            }
                            default_language__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Human => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("human"));
                            }
                            owner__ = map_.next_value::<::std::option::Option<_>>()?.map(create_instance_request::Owner::Human)
;
                        }
                        GeneratedField::Machine => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machine"));
                            }
                            owner__ = map_.next_value::<::std::option::Option<_>>()?.map(create_instance_request::Owner::Machine)
;
                        }
                    }
                }
                Ok(CreateInstanceRequest {
                    instance_name: instance_name__.unwrap_or_default(),
                    first_org_name: first_org_name__.unwrap_or_default(),
                    custom_domain: custom_domain__.unwrap_or_default(),
                    default_language: default_language__.unwrap_or_default(),
                    owner: owner__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::Email {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.Email", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if self.is_email_verified {
            struct_ser.serialize_field("isEmailVerified", &self.is_email_verified)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::Email {
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
            type Value = create_instance_request::Email;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.Email")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::Email, V::Error>
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
                Ok(create_instance_request::Email {
                    email: email__.unwrap_or_default(),
                    is_email_verified: is_email_verified__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.Email", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::Human {
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
        if self.email.is_some() {
            len += 1;
        }
        if self.profile.is_some() {
            len += 1;
        }
        if self.password.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.Human", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.email.as_ref() {
            struct_ser.serialize_field("email", v)?;
        }
        if let Some(v) = self.profile.as_ref() {
            struct_ser.serialize_field("profile", v)?;
        }
        if let Some(v) = self.password.as_ref() {
            struct_ser.serialize_field("password", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::Human {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name",
            "userName",
            "email",
            "profile",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserName,
            Email,
            Profile,
            Password,
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
                            "email" => Ok(GeneratedField::Email),
                            "profile" => Ok(GeneratedField::Profile),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_instance_request::Human;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.Human")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::Human, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_name__ = None;
                let mut email__ = None;
                let mut profile__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = map_.next_value()?;
                        }
                        GeneratedField::Profile => {
                            if profile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            profile__ = map_.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = map_.next_value()?;
                        }
                    }
                }
                Ok(create_instance_request::Human {
                    user_name: user_name__.unwrap_or_default(),
                    email: email__,
                    profile: profile__,
                    password: password__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.Human", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::Machine {
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
        if !self.name.is_empty() {
            len += 1;
        }
        if self.personal_access_token.is_some() {
            len += 1;
        }
        if self.machine_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.Machine", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.personal_access_token.as_ref() {
            struct_ser.serialize_field("personalAccessToken", v)?;
        }
        if let Some(v) = self.machine_key.as_ref() {
            struct_ser.serialize_field("machineKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::Machine {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name",
            "userName",
            "name",
            "personal_access_token",
            "personalAccessToken",
            "machine_key",
            "machineKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserName,
            Name,
            PersonalAccessToken,
            MachineKey,
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
                            "name" => Ok(GeneratedField::Name),
                            "personalAccessToken" | "personal_access_token" => Ok(GeneratedField::PersonalAccessToken),
                            "machineKey" | "machine_key" => Ok(GeneratedField::MachineKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_instance_request::Machine;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.Machine")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::Machine, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_name__ = None;
                let mut name__ = None;
                let mut personal_access_token__ = None;
                let mut machine_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PersonalAccessToken => {
                            if personal_access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("personalAccessToken"));
                            }
                            personal_access_token__ = map_.next_value()?;
                        }
                        GeneratedField::MachineKey => {
                            if machine_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machineKey"));
                            }
                            machine_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(create_instance_request::Machine {
                    user_name: user_name__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    personal_access_token: personal_access_token__,
                    machine_key: machine_key__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.Machine", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::MachineKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.expiration_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.MachineKey", len)?;
        if self.r#type != 0 {
            let v = super::super::authn::v1::KeyType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::MachineKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "expiration_date",
            "expirationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            ExpirationDate,
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
                            "type" => Ok(GeneratedField::Type),
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_instance_request::MachineKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.MachineKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::MachineKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut expiration_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                    }
                }
                Ok(create_instance_request::MachineKey {
                    r#type: r#type__.unwrap_or_default(),
                    expiration_date: expiration_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.MachineKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::Password {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.password.is_empty() {
            len += 1;
        }
        if self.password_change_required {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.Password", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        if self.password_change_required {
            struct_ser.serialize_field("passwordChangeRequired", &self.password_change_required)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::Password {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
            "password_change_required",
            "passwordChangeRequired",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
            PasswordChangeRequired,
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
                            "password" => Ok(GeneratedField::Password),
                            "passwordChangeRequired" | "password_change_required" => Ok(GeneratedField::PasswordChangeRequired),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_instance_request::Password;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.Password")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::Password, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                let mut password_change_required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PasswordChangeRequired => {
                            if password_change_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passwordChangeRequired"));
                            }
                            password_change_required__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(create_instance_request::Password {
                    password: password__.unwrap_or_default(),
                    password_change_required: password_change_required__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.Password", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::PersonalAccessToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expiration_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.PersonalAccessToken", len)?;
        if let Some(v) = self.expiration_date.as_ref() {
            struct_ser.serialize_field("expirationDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::PersonalAccessToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expiration_date",
            "expirationDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpirationDate,
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
                            "expirationDate" | "expiration_date" => Ok(GeneratedField::ExpirationDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_instance_request::PersonalAccessToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.PersonalAccessToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::PersonalAccessToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expiration_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExpirationDate => {
                            if expiration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationDate"));
                            }
                            expiration_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(create_instance_request::PersonalAccessToken {
                    expiration_date: expiration_date__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.PersonalAccessToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_instance_request::Profile {
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
        if !self.preferred_language.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceRequest.Profile", len)?;
        if !self.first_name.is_empty() {
            struct_ser.serialize_field("firstName", &self.first_name)?;
        }
        if !self.last_name.is_empty() {
            struct_ser.serialize_field("lastName", &self.last_name)?;
        }
        if !self.preferred_language.is_empty() {
            struct_ser.serialize_field("preferredLanguage", &self.preferred_language)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_instance_request::Profile {
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
            "preferred_language",
            "preferredLanguage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstName,
            LastName,
            PreferredLanguage,
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
                            "preferredLanguage" | "preferred_language" => Ok(GeneratedField::PreferredLanguage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_instance_request::Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceRequest.Profile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_instance_request::Profile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_name__ = None;
                let mut last_name__ = None;
                let mut preferred_language__ = None;
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
                        GeneratedField::PreferredLanguage => {
                            if preferred_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferredLanguage"));
                            }
                            preferred_language__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(create_instance_request::Profile {
                    first_name: first_name__.unwrap_or_default(),
                    last_name: last_name__.unwrap_or_default(),
                    preferred_language: preferred_language__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceRequest.Profile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateInstanceResponse {
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
        if self.details.is_some() {
            len += 1;
        }
        if !self.pat.is_empty() {
            len += 1;
        }
        if !self.machine_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.CreateInstanceResponse", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.pat.is_empty() {
            struct_ser.serialize_field("pat", &self.pat)?;
        }
        if !self.machine_key.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("machineKey", pbjson::private::base64::encode(&self.machine_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateInstanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "details",
            "pat",
            "machine_key",
            "machineKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Details,
            Pat,
            MachineKey,
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
                            "details" => Ok(GeneratedField::Details),
                            "pat" => Ok(GeneratedField::Pat),
                            "machineKey" | "machine_key" => Ok(GeneratedField::MachineKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateInstanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.CreateInstanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateInstanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut details__ = None;
                let mut pat__ = None;
                let mut machine_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Pat => {
                            if pat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pat"));
                            }
                            pat__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MachineKey => {
                            if machine_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machineKey"));
                            }
                            machine_key__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreateInstanceResponse {
                    instance_id: instance_id__.unwrap_or_default(),
                    details: details__,
                    pat: pat__.unwrap_or_default(),
                    machine_key: machine_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.CreateInstanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExistsDomainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ExistsDomainRequest", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExistsDomainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExistsDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ExistsDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExistsDomainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExistsDomainRequest {
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ExistsDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExistsDomainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.exists {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ExistsDomainResponse", len)?;
        if self.exists {
            struct_ser.serialize_field("exists", &self.exists)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExistsDomainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exists",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Exists,
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
                            "exists" => Ok(GeneratedField::Exists),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExistsDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ExistsDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExistsDomainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exists__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Exists => {
                            if exists__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exists"));
                            }
                            exists__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExistsDomainResponse {
                    exists: exists__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ExistsDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FailedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.database.is_empty() {
            len += 1;
        }
        if !self.view_name.is_empty() {
            len += 1;
        }
        if self.failed_sequence != 0 {
            len += 1;
        }
        if self.failure_count != 0 {
            len += 1;
        }
        if !self.error_message.is_empty() {
            len += 1;
        }
        if self.last_failed.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.FailedEvent", len)?;
        if !self.database.is_empty() {
            struct_ser.serialize_field("database", &self.database)?;
        }
        if !self.view_name.is_empty() {
            struct_ser.serialize_field("viewName", &self.view_name)?;
        }
        if self.failed_sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("failedSequence", ToString::to_string(&self.failed_sequence).as_str())?;
        }
        if self.failure_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("failureCount", ToString::to_string(&self.failure_count).as_str())?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        if let Some(v) = self.last_failed.as_ref() {
            struct_ser.serialize_field("lastFailed", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FailedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "database",
            "view_name",
            "viewName",
            "failed_sequence",
            "failedSequence",
            "failure_count",
            "failureCount",
            "error_message",
            "errorMessage",
            "last_failed",
            "lastFailed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Database,
            ViewName,
            FailedSequence,
            FailureCount,
            ErrorMessage,
            LastFailed,
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
                            "database" => Ok(GeneratedField::Database),
                            "viewName" | "view_name" => Ok(GeneratedField::ViewName),
                            "failedSequence" | "failed_sequence" => Ok(GeneratedField::FailedSequence),
                            "failureCount" | "failure_count" => Ok(GeneratedField::FailureCount),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "lastFailed" | "last_failed" => Ok(GeneratedField::LastFailed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FailedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.FailedEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FailedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut database__ = None;
                let mut view_name__ = None;
                let mut failed_sequence__ = None;
                let mut failure_count__ = None;
                let mut error_message__ = None;
                let mut last_failed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Database => {
                            if database__.is_some() {
                                return Err(serde::de::Error::duplicate_field("database"));
                            }
                            database__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ViewName => {
                            if view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewName"));
                            }
                            view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FailedSequence => {
                            if failed_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedSequence"));
                            }
                            failed_sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureCount => {
                            if failure_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureCount"));
                            }
                            failure_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastFailed => {
                            if last_failed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastFailed"));
                            }
                            last_failed__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FailedEvent {
                    database: database__.unwrap_or_default(),
                    view_name: view_name__.unwrap_or_default(),
                    failed_sequence: failed_sequence__.unwrap_or_default(),
                    failure_count: failure_count__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                    last_failed: last_failed__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.FailedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInstanceRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.GetInstanceRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInstanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetInstanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.GetInstanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInstanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetInstanceRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.GetInstanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetInstanceResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.GetInstanceResponse", len)?;
        if let Some(v) = self.instance.as_ref() {
            struct_ser.serialize_field("instance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetInstanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetInstanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.GetInstanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetInstanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetInstanceResponse {
                    instance: instance__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.GetInstanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUsageRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.GetUsageRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUsageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUsageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.GetUsageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUsageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUsageRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.GetUsageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthzRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.system.v1.HealthzRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthzRequest {
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
            type Value = HealthzRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.HealthzRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthzRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HealthzRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.HealthzRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthzResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.system.v1.HealthzResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthzResponse {
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
            type Value = HealthzResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.HealthzResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HealthzResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HealthzResponse {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.HealthzResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDomainsRequest {
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
        if self.query.is_some() {
            len += 1;
        }
        if self.sorting_column != 0 {
            len += 1;
        }
        if !self.queries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListDomainsRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if self.sorting_column != 0 {
            let v = super::super::instance::v1::DomainFieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDomainsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "query",
            "sorting_column",
            "sortingColumn",
            "queries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Query,
            SortingColumn,
            Queries,
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
                            "query" => Ok(GeneratedField::Query),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "queries" => Ok(GeneratedField::Queries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDomainsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListDomainsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDomainsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut query__ = None;
                let mut sorting_column__ = None;
                let mut queries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<super::super::instance::v1::DomainFieldName>()? as i32);
                        }
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDomainsRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    query: query__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListDomainsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDomainsResponse {
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
        if self.sorting_column != 0 {
            len += 1;
        }
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListDomainsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.sorting_column != 0 {
            let v = super::super::instance::v1::DomainFieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDomainsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "sorting_column",
            "sortingColumn",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SortingColumn,
            Result,
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
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDomainsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListDomainsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDomainsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut sorting_column__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<super::super::instance::v1::DomainFieldName>()? as i32);
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDomainsResponse {
                    details: details__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListDomainsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFailedEventsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.system.v1.ListFailedEventsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFailedEventsRequest {
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
            type Value = ListFailedEventsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListFailedEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFailedEventsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListFailedEventsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListFailedEventsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFailedEventsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListFailedEventsResponse", len)?;
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFailedEventsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFailedEventsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListFailedEventsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFailedEventsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFailedEventsResponse {
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListFailedEventsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListIamMembersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query.is_some() {
            len += 1;
        }
        if !self.instance_id.is_empty() {
            len += 1;
        }
        if !self.queries.is_empty() {
            len += 1;
        }
        if self.sorting_column != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListIAMMembersRequest", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        if self.sorting_column != 0 {
            let v = super::super::member::v1::MemberFieldColumnName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListIamMembersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "instance_id",
            "instanceId",
            "queries",
            "sorting_column",
            "sortingColumn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            InstanceId,
            Queries,
            SortingColumn,
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
                            "query" => Ok(GeneratedField::Query),
                            "instanceId" | "instance_id" => Ok(GeneratedField::InstanceId),
                            "queries" => Ok(GeneratedField::Queries),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListIamMembersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListIAMMembersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListIamMembersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut instance_id__ = None;
                let mut queries__ = None;
                let mut sorting_column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<super::super::member::v1::MemberFieldColumnName>()? as i32);
                        }
                    }
                }
                Ok(ListIamMembersRequest {
                    query: query__,
                    instance_id: instance_id__.unwrap_or_default(),
                    queries: queries__.unwrap_or_default(),
                    sorting_column: sorting_column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListIAMMembersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListIamMembersResponse {
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
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListIAMMembersResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListIamMembersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListIamMembersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListIAMMembersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListIamMembersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListIamMembersResponse {
                    details: details__,
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListIAMMembersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListInstancesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query.is_some() {
            len += 1;
        }
        if self.sorting_column != 0 {
            len += 1;
        }
        if !self.queries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListInstancesRequest", len)?;
        if let Some(v) = self.query.as_ref() {
            struct_ser.serialize_field("query", v)?;
        }
        if self.sorting_column != 0 {
            let v = super::super::instance::v1::FieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListInstancesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "sorting_column",
            "sortingColumn",
            "queries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            SortingColumn,
            Queries,
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
                            "query" => Ok(GeneratedField::Query),
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "queries" => Ok(GeneratedField::Queries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListInstancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListInstancesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListInstancesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut sorting_column__ = None;
                let mut queries__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<super::super::instance::v1::FieldName>()? as i32);
                        }
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListInstancesRequest {
                    query: query__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListInstancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListInstancesResponse {
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
        if self.sorting_column != 0 {
            len += 1;
        }
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListInstancesResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        if self.sorting_column != 0 {
            let v = super::super::instance::v1::FieldName::try_from(self.sorting_column)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.sorting_column)))?;
            struct_ser.serialize_field("sortingColumn", &v)?;
        }
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListInstancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "details",
            "sorting_column",
            "sortingColumn",
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Details,
            SortingColumn,
            Result,
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
                            "sortingColumn" | "sorting_column" => Ok(GeneratedField::SortingColumn),
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListInstancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListInstancesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListInstancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                let mut sorting_column__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = map_.next_value()?;
                        }
                        GeneratedField::SortingColumn => {
                            if sorting_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sortingColumn"));
                            }
                            sorting_column__ = Some(map_.next_value::<super::super::instance::v1::FieldName>()? as i32);
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListInstancesResponse {
                    details: details__,
                    sorting_column: sorting_column__.unwrap_or_default(),
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListInstancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.system.v1.ListViewsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsRequest {
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
            type Value = ListViewsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListViewsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListViewsRequest {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListViewsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ListViewsResponse", len)?;
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListViewsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ListViewsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListViewsResponse {
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ListViewsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveDomainRequest {
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
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveDomainRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDomainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Domain,
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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveDomainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveDomainRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveDomainResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveDomainResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDomainResponse {
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
            type Value = RemoveDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveDomainResponse, V::Error>
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
                Ok(RemoveDomainResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveFailedEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.database.is_empty() {
            len += 1;
        }
        if !self.view_name.is_empty() {
            len += 1;
        }
        if self.failed_sequence != 0 {
            len += 1;
        }
        if !self.instance_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveFailedEventRequest", len)?;
        if !self.database.is_empty() {
            struct_ser.serialize_field("database", &self.database)?;
        }
        if !self.view_name.is_empty() {
            struct_ser.serialize_field("viewName", &self.view_name)?;
        }
        if self.failed_sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("failedSequence", ToString::to_string(&self.failed_sequence).as_str())?;
        }
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveFailedEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "database",
            "view_name",
            "viewName",
            "failed_sequence",
            "failedSequence",
            "instance_id",
            "instanceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Database,
            ViewName,
            FailedSequence,
            InstanceId,
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
                            "database" => Ok(GeneratedField::Database),
                            "viewName" | "view_name" => Ok(GeneratedField::ViewName),
                            "failedSequence" | "failed_sequence" => Ok(GeneratedField::FailedSequence),
                            "instanceId" | "instance_id" => Ok(GeneratedField::InstanceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveFailedEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveFailedEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveFailedEventRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut database__ = None;
                let mut view_name__ = None;
                let mut failed_sequence__ = None;
                let mut instance_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Database => {
                            if database__.is_some() {
                                return Err(serde::de::Error::duplicate_field("database"));
                            }
                            database__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ViewName => {
                            if view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewName"));
                            }
                            view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FailedSequence => {
                            if failed_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedSequence"));
                            }
                            failed_sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveFailedEventRequest {
                    database: database__.unwrap_or_default(),
                    view_name: view_name__.unwrap_or_default(),
                    failed_sequence: failed_sequence__.unwrap_or_default(),
                    instance_id: instance_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveFailedEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveFailedEventResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveFailedEventResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveFailedEventResponse {
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
            type Value = RemoveFailedEventResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveFailedEventResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveFailedEventResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveFailedEventResponse {
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveFailedEventResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveInstanceRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveInstanceRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveInstanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveInstanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveInstanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveInstanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveInstanceRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveInstanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveInstanceResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveInstanceResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveInstanceResponse {
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
            type Value = RemoveInstanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveInstanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveInstanceResponse, V::Error>
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
                Ok(RemoveInstanceResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveInstanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveQuotaRequest {
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
        if self.unit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveQuotaRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if self.unit != 0 {
            let v = super::super::quota::v1::Unit::try_from(self.unit)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveQuotaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Unit,
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
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveQuotaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveQuotaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveQuotaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value::<super::super::quota::v1::Unit>()? as i32);
                        }
                    }
                }
                Ok(RemoveQuotaRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveQuotaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveQuotaResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.RemoveQuotaResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveQuotaResponse {
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
            type Value = RemoveQuotaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.RemoveQuotaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveQuotaResponse, V::Error>
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
                Ok(RemoveQuotaResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.RemoveQuotaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetLimitsRequest {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ResetLimitsRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetLimitsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResetLimitsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ResetLimitsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetLimitsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResetLimitsRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ResetLimitsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResetLimitsResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.ResetLimitsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResetLimitsResponse {
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
            type Value = ResetLimitsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.ResetLimitsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResetLimitsResponse, V::Error>
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
                Ok(ResetLimitsResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.ResetLimitsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetInstanceFeatureRequest {
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
        if self.feature_id != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetInstanceFeatureRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if self.feature_id != 0 {
            let v = super::super::feature::v1::InstanceFeature::try_from(self.feature_id)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.feature_id)))?;
            struct_ser.serialize_field("featureId", &v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                set_instance_feature_request::Value::Bool(v) => {
                    struct_ser.serialize_field("bool", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetInstanceFeatureRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "feature_id",
            "featureId",
            "bool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            FeatureId,
            Bool,
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
                            "featureId" | "feature_id" => Ok(GeneratedField::FeatureId),
                            "bool" => Ok(GeneratedField::Bool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetInstanceFeatureRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetInstanceFeatureRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetInstanceFeatureRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut feature_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FeatureId => {
                            if feature_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("featureId"));
                            }
                            feature_id__ = Some(map_.next_value::<super::super::feature::v1::InstanceFeature>()? as i32);
                        }
                        GeneratedField::Bool => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(set_instance_feature_request::Value::Bool);
                        }
                    }
                }
                Ok(SetInstanceFeatureRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    feature_id: feature_id__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetInstanceFeatureRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetInstanceFeatureResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetInstanceFeatureResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetInstanceFeatureResponse {
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
            type Value = SetInstanceFeatureResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetInstanceFeatureResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetInstanceFeatureResponse, V::Error>
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
                Ok(SetInstanceFeatureResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetInstanceFeatureResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetLimitsRequest {
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
        if self.audit_log_retention.is_some() {
            len += 1;
        }
        if self.block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetLimitsRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if let Some(v) = self.audit_log_retention.as_ref() {
            struct_ser.serialize_field("auditLogRetention", v)?;
        }
        if let Some(v) = self.block.as_ref() {
            struct_ser.serialize_field("block", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetLimitsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "audit_log_retention",
            "auditLogRetention",
            "block",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            AuditLogRetention,
            Block,
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
                            "auditLogRetention" | "audit_log_retention" => Ok(GeneratedField::AuditLogRetention),
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetLimitsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetLimitsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetLimitsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut audit_log_retention__ = None;
                let mut block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuditLogRetention => {
                            if audit_log_retention__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auditLogRetention"));
                            }
                            audit_log_retention__ = map_.next_value()?;
                        }
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetLimitsRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    audit_log_retention: audit_log_retention__,
                    block: block__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetLimitsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetLimitsResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetLimitsResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetLimitsResponse {
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
            type Value = SetLimitsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetLimitsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetLimitsResponse, V::Error>
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
                Ok(SetLimitsResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetLimitsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPrimaryDomainRequest {
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
        if !self.domain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetPrimaryDomainRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPrimaryDomainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Domain,
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
                            "domain" => Ok(GeneratedField::Domain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPrimaryDomainRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetPrimaryDomainRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPrimaryDomainRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut domain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetPrimaryDomainRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetPrimaryDomainRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPrimaryDomainResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetPrimaryDomainResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPrimaryDomainResponse {
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
            type Value = SetPrimaryDomainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetPrimaryDomainResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetPrimaryDomainResponse, V::Error>
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
                Ok(SetPrimaryDomainResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetPrimaryDomainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetQuotaRequest {
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
        if self.unit != 0 {
            len += 1;
        }
        if self.from.is_some() {
            len += 1;
        }
        if self.reset_interval.is_some() {
            len += 1;
        }
        if self.amount != 0 {
            len += 1;
        }
        if self.limit {
            len += 1;
        }
        if !self.notifications.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetQuotaRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if self.unit != 0 {
            let v = super::super::quota::v1::Unit::try_from(self.unit)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        if let Some(v) = self.from.as_ref() {
            struct_ser.serialize_field("from", v)?;
        }
        if let Some(v) = self.reset_interval.as_ref() {
            struct_ser.serialize_field("resetInterval", v)?;
        }
        if self.amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("amount", ToString::to_string(&self.amount).as_str())?;
        }
        if self.limit {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if !self.notifications.is_empty() {
            struct_ser.serialize_field("notifications", &self.notifications)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetQuotaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "unit",
            "from",
            "reset_interval",
            "resetInterval",
            "amount",
            "limit",
            "notifications",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            Unit,
            From,
            ResetInterval,
            Amount,
            Limit,
            Notifications,
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
                            "unit" => Ok(GeneratedField::Unit),
                            "from" => Ok(GeneratedField::From),
                            "resetInterval" | "reset_interval" => Ok(GeneratedField::ResetInterval),
                            "amount" => Ok(GeneratedField::Amount),
                            "limit" => Ok(GeneratedField::Limit),
                            "notifications" => Ok(GeneratedField::Notifications),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetQuotaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetQuotaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetQuotaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut unit__ = None;
                let mut from__ = None;
                let mut reset_interval__ = None;
                let mut amount__ = None;
                let mut limit__ = None;
                let mut notifications__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value::<super::super::quota::v1::Unit>()? as i32);
                        }
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = map_.next_value()?;
                        }
                        GeneratedField::ResetInterval => {
                            if reset_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resetInterval"));
                            }
                            reset_interval__ = map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notifications => {
                            if notifications__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notifications"));
                            }
                            notifications__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetQuotaRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    from: from__,
                    reset_interval: reset_interval__,
                    amount: amount__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    notifications: notifications__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetQuotaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetQuotaResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.SetQuotaResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetQuotaResponse {
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
            type Value = SetQuotaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.SetQuotaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetQuotaResponse, V::Error>
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
                Ok(SetQuotaResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.SetQuotaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateInstanceRequest {
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
        if !self.instance_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.UpdateInstanceRequest", len)?;
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateInstanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_id",
            "instanceId",
            "instance_name",
            "instanceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceId,
            InstanceName,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateInstanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.UpdateInstanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateInstanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_id__ = None;
                let mut instance_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateInstanceRequest {
                    instance_id: instance_id__.unwrap_or_default(),
                    instance_name: instance_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.UpdateInstanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateInstanceResponse {
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
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.UpdateInstanceResponse", len)?;
        if let Some(v) = self.details.as_ref() {
            struct_ser.serialize_field("details", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateInstanceResponse {
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
            type Value = UpdateInstanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.UpdateInstanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateInstanceResponse, V::Error>
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
                Ok(UpdateInstanceResponse {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.UpdateInstanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for View {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.database.is_empty() {
            len += 1;
        }
        if !self.view_name.is_empty() {
            len += 1;
        }
        if self.processed_sequence != 0 {
            len += 1;
        }
        if self.event_timestamp.is_some() {
            len += 1;
        }
        if self.last_successful_spooler_run.is_some() {
            len += 1;
        }
        if !self.instance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zitadel.system.v1.View", len)?;
        if !self.database.is_empty() {
            struct_ser.serialize_field("database", &self.database)?;
        }
        if !self.view_name.is_empty() {
            struct_ser.serialize_field("viewName", &self.view_name)?;
        }
        if self.processed_sequence != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("processedSequence", ToString::to_string(&self.processed_sequence).as_str())?;
        }
        if let Some(v) = self.event_timestamp.as_ref() {
            struct_ser.serialize_field("eventTimestamp", v)?;
        }
        if let Some(v) = self.last_successful_spooler_run.as_ref() {
            struct_ser.serialize_field("lastSuccessfulSpoolerRun", v)?;
        }
        if !self.instance.is_empty() {
            struct_ser.serialize_field("instance", &self.instance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for View {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "database",
            "view_name",
            "viewName",
            "processed_sequence",
            "processedSequence",
            "event_timestamp",
            "eventTimestamp",
            "last_successful_spooler_run",
            "lastSuccessfulSpoolerRun",
            "instance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Database,
            ViewName,
            ProcessedSequence,
            EventTimestamp,
            LastSuccessfulSpoolerRun,
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
                            "database" => Ok(GeneratedField::Database),
                            "viewName" | "view_name" => Ok(GeneratedField::ViewName),
                            "processedSequence" | "processed_sequence" => Ok(GeneratedField::ProcessedSequence),
                            "eventTimestamp" | "event_timestamp" => Ok(GeneratedField::EventTimestamp),
                            "lastSuccessfulSpoolerRun" | "last_successful_spooler_run" => Ok(GeneratedField::LastSuccessfulSpoolerRun),
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
            type Value = View;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zitadel.system.v1.View")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<View, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut database__ = None;
                let mut view_name__ = None;
                let mut processed_sequence__ = None;
                let mut event_timestamp__ = None;
                let mut last_successful_spooler_run__ = None;
                let mut instance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Database => {
                            if database__.is_some() {
                                return Err(serde::de::Error::duplicate_field("database"));
                            }
                            database__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ViewName => {
                            if view_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewName"));
                            }
                            view_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProcessedSequence => {
                            if processed_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processedSequence"));
                            }
                            processed_sequence__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventTimestamp => {
                            if event_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventTimestamp"));
                            }
                            event_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::LastSuccessfulSpoolerRun => {
                            if last_successful_spooler_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastSuccessfulSpoolerRun"));
                            }
                            last_successful_spooler_run__ = map_.next_value()?;
                        }
                        GeneratedField::Instance => {
                            if instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            instance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(View {
                    database: database__.unwrap_or_default(),
                    view_name: view_name__.unwrap_or_default(),
                    processed_sequence: processed_sequence__.unwrap_or_default(),
                    event_timestamp: event_timestamp__,
                    last_successful_spooler_run: last_successful_spooler_run__,
                    instance: instance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zitadel.system.v1.View", FIELDS, GeneratedVisitor)
    }
}
