// @generated
impl serde::Serialize for GetPolicyRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.server.v1.GetPolicyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPolicyRequest {
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
            type Value = GetPolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.server.v1.GetPolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPolicyRequest, V::Error>
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
                Ok(GetPolicyRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.server.v1.GetPolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServerStatusRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.server.v1.GetServerStatusRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServerStatusRequest {
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
            type Value = GetServerStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.server.v1.GetServerStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetServerStatusRequest, V::Error>
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
                Ok(GetServerStatusRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.server.v1.GetServerStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Policy {
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
        if self.blackout {
            len += 1;
        }
        if self.hold {
            len += 1;
        }
        if self.master != 0 {
            len += 1;
        }
        if self.mcp_writes_allowed {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.server.v1.Policy", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.blackout {
            struct_ser.serialize_field("blackout", &self.blackout)?;
        }
        if self.hold {
            struct_ser.serialize_field("hold", &self.hold)?;
        }
        if self.master != 0 {
            struct_ser.serialize_field("master", &self.master)?;
        }
        if self.mcp_writes_allowed {
            struct_ser.serialize_field("mcpWritesAllowed", &self.mcp_writes_allowed)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "blackout",
            "hold",
            "master",
            "mcp_writes_allowed",
            "mcpWritesAllowed",
            "update_time",
            "updateTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Blackout,
            Hold,
            Master,
            McpWritesAllowed,
            UpdateTime,
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
                            "blackout" => Ok(GeneratedField::Blackout),
                            "hold" => Ok(GeneratedField::Hold),
                            "master" => Ok(GeneratedField::Master),
                            "mcpWritesAllowed" | "mcp_writes_allowed" => Ok(GeneratedField::McpWritesAllowed),
                            "updateTime" | "update_time" => Ok(GeneratedField::UpdateTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.server.v1.Policy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut blackout__ = None;
                let mut hold__ = None;
                let mut master__ = None;
                let mut mcp_writes_allowed__ = None;
                let mut update_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Blackout => {
                            if blackout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blackout"));
                            }
                            blackout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Hold => {
                            if hold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hold"));
                            }
                            hold__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Master => {
                            if master__.is_some() {
                                return Err(serde::de::Error::duplicate_field("master"));
                            }
                            master__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::McpWritesAllowed => {
                            if mcp_writes_allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mcpWritesAllowed"));
                            }
                            mcp_writes_allowed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Policy {
                    name: name__.unwrap_or_default(),
                    blackout: blackout__.unwrap_or_default(),
                    hold: hold__.unwrap_or_default(),
                    master: master__.unwrap_or_default(),
                    mcp_writes_allowed: mcp_writes_allowed__.unwrap_or_default(),
                    update_time: update_time__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.server.v1.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerStatus {
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
        if !self.version.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.output_count != 0 {
            len += 1;
        }
        if self.input_count != 0 {
            len += 1;
        }
        if self.node_count != 0 {
            len += 1;
        }
        if self.device_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.server.v1.ServerStatus", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if self.output_count != 0 {
            struct_ser.serialize_field("outputCount", &self.output_count)?;
        }
        if self.input_count != 0 {
            struct_ser.serialize_field("inputCount", &self.input_count)?;
        }
        if self.node_count != 0 {
            struct_ser.serialize_field("nodeCount", &self.node_count)?;
        }
        if self.device_count != 0 {
            struct_ser.serialize_field("deviceCount", &self.device_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "start_time",
            "startTime",
            "output_count",
            "outputCount",
            "input_count",
            "inputCount",
            "node_count",
            "nodeCount",
            "device_count",
            "deviceCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            StartTime,
            OutputCount,
            InputCount,
            NodeCount,
            DeviceCount,
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
                            "version" => Ok(GeneratedField::Version),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "outputCount" | "output_count" => Ok(GeneratedField::OutputCount),
                            "inputCount" | "input_count" => Ok(GeneratedField::InputCount),
                            "nodeCount" | "node_count" => Ok(GeneratedField::NodeCount),
                            "deviceCount" | "device_count" => Ok(GeneratedField::DeviceCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.server.v1.ServerStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServerStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut start_time__ = None;
                let mut output_count__ = None;
                let mut input_count__ = None;
                let mut node_count__ = None;
                let mut device_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::OutputCount => {
                            if output_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputCount"));
                            }
                            output_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InputCount => {
                            if input_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputCount"));
                            }
                            input_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NodeCount => {
                            if node_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeCount"));
                            }
                            node_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceCount => {
                            if device_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceCount"));
                            }
                            device_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ServerStatus {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    start_time: start_time__,
                    output_count: output_count__.unwrap_or_default(),
                    input_count: input_count__.unwrap_or_default(),
                    node_count: node_count__.unwrap_or_default(),
                    device_count: device_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.server.v1.ServerStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePolicyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.server.v1.UpdatePolicyRequest", len)?;
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policy,
            UpdateMask,
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
                            "policy" => Ok(GeneratedField::Policy),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.server.v1.UpdatePolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePolicyRequest {
                    policy: policy__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.server.v1.UpdatePolicyRequest", FIELDS, GeneratedVisitor)
    }
}
