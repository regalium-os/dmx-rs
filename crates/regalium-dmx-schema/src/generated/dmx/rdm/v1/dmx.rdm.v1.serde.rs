// @generated
impl serde::Serialize for DeleteDeviceRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.DeleteDeviceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDeviceRequest {
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
            type Value = DeleteDeviceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.DeleteDeviceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteDeviceRequest, V::Error>
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
                Ok(DeleteDeviceRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.DeleteDeviceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Device {
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
        if !self.rdm_uid.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.manufacturer_id != 0 {
            len += 1;
        }
        if !self.manufacturer.is_empty() {
            len += 1;
        }
        if self.model_id != 0 {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if self.product_category != 0 {
            len += 1;
        }
        if self.software_version_id != 0 {
            len += 1;
        }
        if !self.software_version.is_empty() {
            len += 1;
        }
        if self.start_address != 0 {
            len += 1;
        }
        if self.footprint != 0 {
            len += 1;
        }
        if self.personality != 0 {
            len += 1;
        }
        if self.personality_count != 0 {
            len += 1;
        }
        if self.sub_device_count != 0 {
            len += 1;
        }
        if self.sensor_count != 0 {
            len += 1;
        }
        if self.identifying {
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
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.Device", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.rdm_uid.is_empty() {
            struct_ser.serialize_field("rdmUid", &self.rdm_uid)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.manufacturer_id != 0 {
            struct_ser.serialize_field("manufacturerId", &self.manufacturer_id)?;
        }
        if !self.manufacturer.is_empty() {
            struct_ser.serialize_field("manufacturer", &self.manufacturer)?;
        }
        if self.model_id != 0 {
            struct_ser.serialize_field("modelId", &self.model_id)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if self.product_category != 0 {
            struct_ser.serialize_field("productCategory", &self.product_category)?;
        }
        if self.software_version_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("softwareVersionId", ToString::to_string(&self.software_version_id).as_str())?;
        }
        if !self.software_version.is_empty() {
            struct_ser.serialize_field("softwareVersion", &self.software_version)?;
        }
        if self.start_address != 0 {
            struct_ser.serialize_field("startAddress", &self.start_address)?;
        }
        if self.footprint != 0 {
            struct_ser.serialize_field("footprint", &self.footprint)?;
        }
        if self.personality != 0 {
            struct_ser.serialize_field("personality", &self.personality)?;
        }
        if self.personality_count != 0 {
            struct_ser.serialize_field("personalityCount", &self.personality_count)?;
        }
        if self.sub_device_count != 0 {
            struct_ser.serialize_field("subDeviceCount", &self.sub_device_count)?;
        }
        if self.sensor_count != 0 {
            struct_ser.serialize_field("sensorCount", &self.sensor_count)?;
        }
        if self.identifying {
            struct_ser.serialize_field("identifying", &self.identifying)?;
        }
        if self.state != 0 {
            let v = DeviceState::try_from(self.state)
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
impl<'de> serde::Deserialize<'de> for Device {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "rdm_uid",
            "rdmUid",
            "display_name",
            "displayName",
            "manufacturer_id",
            "manufacturerId",
            "manufacturer",
            "model_id",
            "modelId",
            "model",
            "product_category",
            "productCategory",
            "software_version_id",
            "softwareVersionId",
            "software_version",
            "softwareVersion",
            "start_address",
            "startAddress",
            "footprint",
            "personality",
            "personality_count",
            "personalityCount",
            "sub_device_count",
            "subDeviceCount",
            "sensor_count",
            "sensorCount",
            "identifying",
            "state",
            "discover_time",
            "discoverTime",
            "last_seen_time",
            "lastSeenTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RdmUid,
            DisplayName,
            ManufacturerId,
            Manufacturer,
            ModelId,
            Model,
            ProductCategory,
            SoftwareVersionId,
            SoftwareVersion,
            StartAddress,
            Footprint,
            Personality,
            PersonalityCount,
            SubDeviceCount,
            SensorCount,
            Identifying,
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
                            "rdmUid" | "rdm_uid" => Ok(GeneratedField::RdmUid),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "manufacturerId" | "manufacturer_id" => Ok(GeneratedField::ManufacturerId),
                            "manufacturer" => Ok(GeneratedField::Manufacturer),
                            "modelId" | "model_id" => Ok(GeneratedField::ModelId),
                            "model" => Ok(GeneratedField::Model),
                            "productCategory" | "product_category" => Ok(GeneratedField::ProductCategory),
                            "softwareVersionId" | "software_version_id" => Ok(GeneratedField::SoftwareVersionId),
                            "softwareVersion" | "software_version" => Ok(GeneratedField::SoftwareVersion),
                            "startAddress" | "start_address" => Ok(GeneratedField::StartAddress),
                            "footprint" => Ok(GeneratedField::Footprint),
                            "personality" => Ok(GeneratedField::Personality),
                            "personalityCount" | "personality_count" => Ok(GeneratedField::PersonalityCount),
                            "subDeviceCount" | "sub_device_count" => Ok(GeneratedField::SubDeviceCount),
                            "sensorCount" | "sensor_count" => Ok(GeneratedField::SensorCount),
                            "identifying" => Ok(GeneratedField::Identifying),
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
            type Value = Device;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.Device")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Device, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut rdm_uid__ = None;
                let mut display_name__ = None;
                let mut manufacturer_id__ = None;
                let mut manufacturer__ = None;
                let mut model_id__ = None;
                let mut model__ = None;
                let mut product_category__ = None;
                let mut software_version_id__ = None;
                let mut software_version__ = None;
                let mut start_address__ = None;
                let mut footprint__ = None;
                let mut personality__ = None;
                let mut personality_count__ = None;
                let mut sub_device_count__ = None;
                let mut sensor_count__ = None;
                let mut identifying__ = None;
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
                        GeneratedField::RdmUid => {
                            if rdm_uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rdmUid"));
                            }
                            rdm_uid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ManufacturerId => {
                            if manufacturer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturerId"));
                            }
                            manufacturer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Manufacturer => {
                            if manufacturer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            manufacturer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelId => {
                            if model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelId"));
                            }
                            model_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProductCategory => {
                            if product_category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("productCategory"));
                            }
                            product_category__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SoftwareVersionId => {
                            if software_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("softwareVersionId"));
                            }
                            software_version_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SoftwareVersion => {
                            if software_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("softwareVersion"));
                            }
                            software_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartAddress => {
                            if start_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startAddress"));
                            }
                            start_address__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Footprint => {
                            if footprint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("footprint"));
                            }
                            footprint__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Personality => {
                            if personality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("personality"));
                            }
                            personality__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PersonalityCount => {
                            if personality_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("personalityCount"));
                            }
                            personality_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SubDeviceCount => {
                            if sub_device_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDeviceCount"));
                            }
                            sub_device_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SensorCount => {
                            if sensor_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sensorCount"));
                            }
                            sensor_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Identifying => {
                            if identifying__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifying"));
                            }
                            identifying__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<DeviceState>()? as i32);
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
                Ok(Device {
                    name: name__.unwrap_or_default(),
                    rdm_uid: rdm_uid__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    manufacturer_id: manufacturer_id__.unwrap_or_default(),
                    manufacturer: manufacturer__.unwrap_or_default(),
                    model_id: model_id__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    product_category: product_category__.unwrap_or_default(),
                    software_version_id: software_version_id__.unwrap_or_default(),
                    software_version: software_version__.unwrap_or_default(),
                    start_address: start_address__.unwrap_or_default(),
                    footprint: footprint__.unwrap_or_default(),
                    personality: personality__.unwrap_or_default(),
                    personality_count: personality_count__.unwrap_or_default(),
                    sub_device_count: sub_device_count__.unwrap_or_default(),
                    sensor_count: sensor_count__.unwrap_or_default(),
                    identifying: identifying__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    discover_time: discover_time__,
                    last_seen_time: last_seen_time__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.Device", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DEVICE_STATE_UNSPECIFIED",
            Self::Responding => "DEVICE_STATE_RESPONDING",
            Self::Silent => "DEVICE_STATE_SILENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DeviceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEVICE_STATE_UNSPECIFIED",
            "DEVICE_STATE_RESPONDING",
            "DEVICE_STATE_SILENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceState;

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
                    "DEVICE_STATE_UNSPECIFIED" => Ok(DeviceState::Unspecified),
                    "DEVICE_STATE_RESPONDING" => Ok(DeviceState::Responding),
                    "DEVICE_STATE_SILENT" => Ok(DeviceState::Silent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoverDevicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.DiscoverDevicesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoverDevicesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
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
                            "parent" => Ok(GeneratedField::Parent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoverDevicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.DiscoverDevicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoverDevicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DiscoverDevicesRequest {
                    parent: parent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.DiscoverDevicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoverDevicesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.devices.is_empty() {
            len += 1;
        }
        if self.incomplete {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.DiscoverDevicesResponse", len)?;
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        if self.incomplete {
            struct_ser.serialize_field("incomplete", &self.incomplete)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoverDevicesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "devices",
            "incomplete",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Devices,
            Incomplete,
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
                            "devices" => Ok(GeneratedField::Devices),
                            "incomplete" => Ok(GeneratedField::Incomplete),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoverDevicesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.DiscoverDevicesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DiscoverDevicesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut devices__ = None;
                let mut incomplete__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Incomplete => {
                            if incomplete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("incomplete"));
                            }
                            incomplete__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DiscoverDevicesResponse {
                    devices: devices__.unwrap_or_default(),
                    incomplete: incomplete__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.DiscoverDevicesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDeviceRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.GetDeviceRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDeviceRequest {
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
            type Value = GetDeviceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.GetDeviceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDeviceRequest, V::Error>
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
                Ok(GetDeviceRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.GetDeviceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetParameterRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.GetParameterRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetParameterRequest {
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
            type Value = GetParameterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.GetParameterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetParameterRequest, V::Error>
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
                Ok(GetParameterRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.GetParameterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDevicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
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
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.ListDevicesRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
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
impl<'de> serde::Deserialize<'de> for ListDevicesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
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
            Parent,
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
                            "parent" => Ok(GeneratedField::Parent),
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
            type Value = ListDevicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.ListDevicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDevicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
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
                Ok(ListDevicesRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.ListDevicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDevicesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.devices.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.ListDevicesResponse", len)?;
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDevicesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "devices",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Devices,
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
                            "devices" => Ok(GeneratedField::Devices),
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
            type Value = ListDevicesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.ListDevicesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDevicesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut devices__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDevicesResponse {
                    devices: devices__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.ListDevicesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListParametersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.ListParametersRequest", len)?;
        if !self.parent.is_empty() {
            struct_ser.serialize_field("parent", &self.parent)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListParametersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parent,
            PageSize,
            PageToken,
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
                            "parent" => Ok(GeneratedField::Parent),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListParametersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.ListParametersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListParametersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parent => {
                            if parent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent"));
                            }
                            parent__ = Some(map_.next_value()?);
                        }
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
                    }
                }
                Ok(ListParametersRequest {
                    parent: parent__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.ListParametersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListParametersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.ListParametersResponse", len)?;
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListParametersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameters",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parameters,
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
                            "parameters" => Ok(GeneratedField::Parameters),
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
            type Value = ListParametersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.ListParametersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListParametersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameters__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListParametersResponse {
                    parameters: parameters__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.ListParametersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Parameter {
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
        if self.pid != 0 {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.Parameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.pid != 0 {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Parameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "pid",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Pid,
            Data,
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
                            "pid" => Ok(GeneratedField::Pid),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Parameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.Parameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Parameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut pid__ = None;
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Parameter {
                    name: name__.unwrap_or_default(),
                    pid: pid__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.Parameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDeviceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.UpdateDeviceRequest", len)?;
        if let Some(v) = self.device.as_ref() {
            struct_ser.serialize_field("device", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDeviceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Device,
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
                            "device" => Ok(GeneratedField::Device),
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
            type Value = UpdateDeviceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.UpdateDeviceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateDeviceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Device => {
                            if device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            device__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateDeviceRequest {
                    device: device__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.UpdateDeviceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateParameterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parameter.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.rdm.v1.UpdateParameterRequest", len)?;
        if let Some(v) = self.parameter.as_ref() {
            struct_ser.serialize_field("parameter", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateParameterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parameter",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parameter,
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
                            "parameter" => Ok(GeneratedField::Parameter),
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
            type Value = UpdateParameterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.rdm.v1.UpdateParameterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateParameterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameter__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Parameter => {
                            if parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            parameter__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateParameterRequest {
                    parameter: parameter__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.rdm.v1.UpdateParameterRequest", FIELDS, GeneratedVisitor)
    }
}
