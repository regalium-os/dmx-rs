// @generated
impl serde::Serialize for DeleteNodeRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.DeleteNodeRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteNodeRequest {
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
            type Value = DeleteNodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.DeleteNodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteNodeRequest, V::Error>
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
                Ok(DeleteNodeRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.DeleteNodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoverNodesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.protocols.is_empty() {
            len += 1;
        }
        if self.probe_unidentified_ports {
            len += 1;
        }
        if self.wait.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.DiscoverNodesRequest", len)?;
        if !self.protocols.is_empty() {
            let v = self.protocols.iter().cloned().map(|v| {
                Protocol::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("protocols", &v)?;
        }
        if self.probe_unidentified_ports {
            struct_ser.serialize_field("probeUnidentifiedPorts", &self.probe_unidentified_ports)?;
        }
        if let Some(v) = self.wait.as_ref() {
            struct_ser.serialize_field("wait", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoverNodesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocols",
            "probe_unidentified_ports",
            "probeUnidentifiedPorts",
            "wait",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protocols,
            ProbeUnidentifiedPorts,
            Wait,
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
                            "protocols" => Ok(GeneratedField::Protocols),
                            "probeUnidentifiedPorts" | "probe_unidentified_ports" => Ok(GeneratedField::ProbeUnidentifiedPorts),
                            "wait" => Ok(GeneratedField::Wait),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoverNodesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.DiscoverNodesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoverNodesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocols__ = None;
                let mut probe_unidentified_ports__ = None;
                let mut wait__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Protocols => {
                            if protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocols"));
                            }
                            protocols__ = Some(map_.next_value::<Vec<Protocol>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::ProbeUnidentifiedPorts => {
                            if probe_unidentified_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("probeUnidentifiedPorts"));
                            }
                            probe_unidentified_ports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Wait => {
                            if wait__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wait"));
                            }
                            wait__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DiscoverNodesRequest {
                    protocols: protocols__.unwrap_or_default(),
                    probe_unidentified_ports: probe_unidentified_ports__.unwrap_or_default(),
                    wait: wait__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.DiscoverNodesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoverNodesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nodes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.DiscoverNodesResponse", len)?;
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoverNodesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nodes,
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
                            "nodes" => Ok(GeneratedField::Nodes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoverNodesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.DiscoverNodesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoverNodesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nodes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nodes => {
                            if nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DiscoverNodesResponse {
                    nodes: nodes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.DiscoverNodesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetNodeRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.GetNodeRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetNodeRequest {
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
            type Value = GetNodeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.GetNodeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetNodeRequest, V::Error>
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
                Ok(GetNodeRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.GetNodeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNodesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.ListNodesRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNodesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrderBy,
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
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListNodesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.ListNodesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNodesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListNodesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.ListNodesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNodesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nodes.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.ListNodesResponse", len)?;
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNodesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodes",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nodes,
            NextPageToken,
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
                            "nodes" => Ok(GeneratedField::Nodes),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListNodesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.ListNodesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNodesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nodes__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nodes => {
                            if nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListNodesResponse {
                    nodes: nodes__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.ListNodesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Node {
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
        if self.protocol != 0 {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.mac_address.is_empty() {
            len += 1;
        }
        if !self.cid.is_empty() {
            len += 1;
        }
        if !self.serial_number.is_empty() {
            len += 1;
        }
        if !self.firmware_version.is_empty() {
            len += 1;
        }
        if !self.ports.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.discover_time.is_some() {
            len += 1;
        }
        if self.last_seen_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.Node", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.protocol != 0 {
            let v = Protocol::try_from(self.protocol)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.mac_address.is_empty() {
            struct_ser.serialize_field("macAddress", &self.mac_address)?;
        }
        if !self.cid.is_empty() {
            struct_ser.serialize_field("cid", &self.cid)?;
        }
        if !self.serial_number.is_empty() {
            struct_ser.serialize_field("serialNumber", &self.serial_number)?;
        }
        if !self.firmware_version.is_empty() {
            struct_ser.serialize_field("firmwareVersion", &self.firmware_version)?;
        }
        if !self.ports.is_empty() {
            struct_ser.serialize_field("ports", &self.ports)?;
        }
        if self.state != 0 {
            let v = NodeState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.discover_time.as_ref() {
            struct_ser.serialize_field("discoverTime", v)?;
        }
        if let Some(v) = self.last_seen_time.as_ref() {
            struct_ser.serialize_field("lastSeenTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Node {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "protocol",
            "display_name",
            "displayName",
            "description",
            "address",
            "mac_address",
            "macAddress",
            "cid",
            "serial_number",
            "serialNumber",
            "firmware_version",
            "firmwareVersion",
            "ports",
            "state",
            "discover_time",
            "discoverTime",
            "last_seen_time",
            "lastSeenTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Protocol,
            DisplayName,
            Description,
            Address,
            MacAddress,
            Cid,
            SerialNumber,
            FirmwareVersion,
            Ports,
            State,
            DiscoverTime,
            LastSeenTime,
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
                            "protocol" => Ok(GeneratedField::Protocol),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "address" => Ok(GeneratedField::Address),
                            "macAddress" | "mac_address" => Ok(GeneratedField::MacAddress),
                            "cid" => Ok(GeneratedField::Cid),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "firmwareVersion" | "firmware_version" => Ok(GeneratedField::FirmwareVersion),
                            "ports" => Ok(GeneratedField::Ports),
                            "state" => Ok(GeneratedField::State),
                            "discoverTime" | "discover_time" => Ok(GeneratedField::DiscoverTime),
                            "lastSeenTime" | "last_seen_time" => Ok(GeneratedField::LastSeenTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.Node")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Node, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut protocol__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut address__ = None;
                let mut mac_address__ = None;
                let mut cid__ = None;
                let mut serial_number__ = None;
                let mut firmware_version__ = None;
                let mut ports__ = None;
                let mut state__ = None;
                let mut discover_time__ = None;
                let mut last_seen_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value::<Protocol>()? as i32);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MacAddress => {
                            if mac_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("macAddress"));
                            }
                            mac_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cid => {
                            if cid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cid"));
                            }
                            cid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirmwareVersion => {
                            if firmware_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firmwareVersion"));
                            }
                            firmware_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ports => {
                            if ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ports"));
                            }
                            ports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<NodeState>()? as i32);
                        }
                        GeneratedField::DiscoverTime => {
                            if discover_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("discoverTime"));
                            }
                            discover_time__ = map_.next_value()?;
                        }
                        GeneratedField::LastSeenTime => {
                            if last_seen_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastSeenTime"));
                            }
                            last_seen_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Node {
                    name: name__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    mac_address: mac_address__.unwrap_or_default(),
                    cid: cid__.unwrap_or_default(),
                    serial_number: serial_number__.unwrap_or_default(),
                    firmware_version: firmware_version__.unwrap_or_default(),
                    ports: ports__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    discover_time: discover_time__,
                    last_seen_time: last_seen_time__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.Node", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "NODE_STATE_UNSPECIFIED",
            Self::Reachable => "NODE_STATE_REACHABLE",
            Self::Unreachable => "NODE_STATE_UNREACHABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NodeState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NODE_STATE_UNSPECIFIED",
            "NODE_STATE_REACHABLE",
            "NODE_STATE_UNREACHABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeState;

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
                    "NODE_STATE_UNSPECIFIED" => Ok(NodeState::Unspecified),
                    "NODE_STATE_REACHABLE" => Ok(NodeState::Reachable),
                    "NODE_STATE_UNREACHABLE" => Ok(NodeState::Unreachable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Port {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index != 0 {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        if self.universe_number != 0 {
            len += 1;
        }
        if self.merging {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.node.v1.Port", len)?;
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if self.direction != 0 {
            let v = PortDirection::try_from(self.direction)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        if self.universe_number != 0 {
            struct_ser.serialize_field("universeNumber", &self.universe_number)?;
        }
        if self.merging {
            struct_ser.serialize_field("merging", &self.merging)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Port {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index",
            "direction",
            "universe_number",
            "universeNumber",
            "merging",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            Direction,
            UniverseNumber,
            Merging,
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
                            "index" => Ok(GeneratedField::Index),
                            "direction" => Ok(GeneratedField::Direction),
                            "universeNumber" | "universe_number" => Ok(GeneratedField::UniverseNumber),
                            "merging" => Ok(GeneratedField::Merging),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Port;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.node.v1.Port")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Port, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut direction__ = None;
                let mut universe_number__ = None;
                let mut merging__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map_.next_value::<PortDirection>()? as i32);
                        }
                        GeneratedField::UniverseNumber => {
                            if universe_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("universeNumber"));
                            }
                            universe_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Merging => {
                            if merging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merging"));
                            }
                            merging__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Port {
                    index: index__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                    universe_number: universe_number__.unwrap_or_default(),
                    merging: merging__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.node.v1.Port", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PortDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PORT_DIRECTION_UNSPECIFIED",
            Self::Output => "PORT_DIRECTION_OUTPUT",
            Self::Input => "PORT_DIRECTION_INPUT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PortDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PORT_DIRECTION_UNSPECIFIED",
            "PORT_DIRECTION_OUTPUT",
            "PORT_DIRECTION_INPUT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PortDirection;

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
                    "PORT_DIRECTION_UNSPECIFIED" => Ok(PortDirection::Unspecified),
                    "PORT_DIRECTION_OUTPUT" => Ok(PortDirection::Output),
                    "PORT_DIRECTION_INPUT" => Ok(PortDirection::Input),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Protocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROTOCOL_UNSPECIFIED",
            Self::Artnet => "PROTOCOL_ARTNET",
            Self::Sacn => "PROTOCOL_SACN",
            Self::Serial => "PROTOCOL_SERIAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Protocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROTOCOL_UNSPECIFIED",
            "PROTOCOL_ARTNET",
            "PROTOCOL_SACN",
            "PROTOCOL_SERIAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Protocol;

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
                    "PROTOCOL_UNSPECIFIED" => Ok(Protocol::Unspecified),
                    "PROTOCOL_ARTNET" => Ok(Protocol::Artnet),
                    "PROTOCOL_SACN" => Ok(Protocol::Sacn),
                    "PROTOCOL_SERIAL" => Ok(Protocol::Serial),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
