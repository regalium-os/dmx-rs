// @generated
impl serde::Serialize for BlackoutUniverseRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.BlackoutUniverseRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlackoutUniverseRequest {
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
            type Value = BlackoutUniverseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.BlackoutUniverseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BlackoutUniverseRequest, V::Error>
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
                Ok(BlackoutUniverseRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.BlackoutUniverseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUniverseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.universe.is_some() {
            len += 1;
        }
        if !self.universe_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.CreateUniverseRequest", len)?;
        if let Some(v) = self.universe.as_ref() {
            struct_ser.serialize_field("universe", v)?;
        }
        if !self.universe_id.is_empty() {
            struct_ser.serialize_field("universeId", &self.universe_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUniverseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "universe",
            "universe_id",
            "universeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Universe,
            UniverseId,
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
                            "universe" => Ok(GeneratedField::Universe),
                            "universeId" | "universe_id" => Ok(GeneratedField::UniverseId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUniverseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.CreateUniverseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUniverseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut universe__ = None;
                let mut universe_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Universe => {
                            if universe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("universe"));
                            }
                            universe__ = map_.next_value()?;
                        }
                        GeneratedField::UniverseId => {
                            if universe_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("universeId"));
                            }
                            universe_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateUniverseRequest {
                    universe: universe__,
                    universe_id: universe_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.CreateUniverseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUniverseRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.DeleteUniverseRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUniverseRequest {
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
            type Value = DeleteUniverseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.DeleteUniverseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUniverseRequest, V::Error>
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
                Ok(DeleteUniverseRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.DeleteUniverseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLevelsRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.GetLevelsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLevelsRequest {
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
            type Value = GetLevelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.GetLevelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLevelsRequest, V::Error>
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
                Ok(GetLevelsRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.GetLevelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUniverseRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.GetUniverseRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUniverseRequest {
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
            type Value = GetUniverseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.GetUniverseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUniverseRequest, V::Error>
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
                Ok(GetUniverseRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.GetUniverseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Input {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.merge_mode != 0 {
            len += 1;
        }
        if self.max_sources != 0 {
            len += 1;
        }
        if !self.sources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.Input", len)?;
        if self.merge_mode != 0 {
            let v = MergeMode::try_from(self.merge_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.merge_mode)))?;
            struct_ser.serialize_field("mergeMode", &v)?;
        }
        if self.max_sources != 0 {
            struct_ser.serialize_field("maxSources", &self.max_sources)?;
        }
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Input {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "merge_mode",
            "mergeMode",
            "max_sources",
            "maxSources",
            "sources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MergeMode,
            MaxSources,
            Sources,
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
                            "mergeMode" | "merge_mode" => Ok(GeneratedField::MergeMode),
                            "maxSources" | "max_sources" => Ok(GeneratedField::MaxSources),
                            "sources" => Ok(GeneratedField::Sources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Input;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.Input")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Input, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut merge_mode__ = None;
                let mut max_sources__ = None;
                let mut sources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MergeMode => {
                            if merge_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergeMode"));
                            }
                            merge_mode__ = Some(map_.next_value::<MergeMode>()? as i32);
                        }
                        GeneratedField::MaxSources => {
                            if max_sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSources"));
                            }
                            max_sources__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Input {
                    merge_mode: merge_mode__.unwrap_or_default(),
                    max_sources: max_sources__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.Input", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Levels {
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
        if !self.slots.is_empty() {
            len += 1;
        }
        if self.generation != 0 {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.Levels", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.slots.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("slots", pbjson::private::base64::encode(&self.slots).as_str())?;
        }
        if self.generation != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("generation", ToString::to_string(&self.generation).as_str())?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Levels {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "slots",
            "generation",
            "update_time",
            "updateTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Slots,
            Generation,
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
                            "slots" => Ok(GeneratedField::Slots),
                            "generation" => Ok(GeneratedField::Generation),
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
            type Value = Levels;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.Levels")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Levels, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut slots__ = None;
                let mut generation__ = None;
                let mut update_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Slots => {
                            if slots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slots"));
                            }
                            slots__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Generation => {
                            if generation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generation"));
                            }
                            generation__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Levels {
                    name: name__.unwrap_or_default(),
                    slots: slots__.unwrap_or_default(),
                    generation: generation__.unwrap_or_default(),
                    update_time: update_time__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.Levels", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUniversesRequest {
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
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.ListUniversesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListUniversesRequest {
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
            type Value = ListUniversesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.ListUniversesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUniversesRequest, V::Error>
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
                Ok(ListUniversesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.ListUniversesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUniversesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.universes.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.ListUniversesResponse", len)?;
        if !self.universes.is_empty() {
            struct_ser.serialize_field("universes", &self.universes)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUniversesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "universes",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Universes,
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
                            "universes" => Ok(GeneratedField::Universes),
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
            type Value = ListUniversesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.ListUniversesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUniversesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut universes__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Universes => {
                            if universes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("universes"));
                            }
                            universes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUniversesResponse {
                    universes: universes__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.ListUniversesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MergeMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MERGE_MODE_UNSPECIFIED",
            Self::Htp => "MERGE_MODE_HTP",
            Self::Ltp => "MERGE_MODE_LTP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MergeMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MERGE_MODE_UNSPECIFIED",
            "MERGE_MODE_HTP",
            "MERGE_MODE_LTP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MergeMode;

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
                    "MERGE_MODE_UNSPECIFIED" => Ok(MergeMode::Unspecified),
                    "MERGE_MODE_HTP" => Ok(MergeMode::Htp),
                    "MERGE_MODE_LTP" => Ok(MergeMode::Ltp),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Output {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interval.is_some() {
            len += 1;
        }
        if self.keep_alive.is_some() {
            len += 1;
        }
        if self.priority.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.Output", len)?;
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if let Some(v) = self.keep_alive.as_ref() {
            struct_ser.serialize_field("keepAlive", v)?;
        }
        if let Some(v) = self.priority.as_ref() {
            struct_ser.serialize_field("priority", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Output {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interval",
            "keep_alive",
            "keepAlive",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interval,
            KeepAlive,
            Priority,
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
                            "interval" => Ok(GeneratedField::Interval),
                            "keepAlive" | "keep_alive" => Ok(GeneratedField::KeepAlive),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Output;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.Output")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Output, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interval__ = None;
                let mut keep_alive__ = None;
                let mut priority__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map_.next_value()?;
                        }
                        GeneratedField::KeepAlive => {
                            if keep_alive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keepAlive"));
                            }
                            keep_alive__ = map_.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Output {
                    interval: interval__,
                    keep_alive: keep_alive__,
                    priority: priority__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.Output", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Source {
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
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.merged {
            len += 1;
        }
        if self.last_seen_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.Source", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if self.merged {
            struct_ser.serialize_field("merged", &self.merged)?;
        }
        if let Some(v) = self.last_seen_time.as_ref() {
            struct_ser.serialize_field("lastSeenTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "display_name",
            "displayName",
            "priority",
            "merged",
            "last_seen_time",
            "lastSeenTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            DisplayName,
            Priority,
            Merged,
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
                            "id" => Ok(GeneratedField::Id),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "priority" => Ok(GeneratedField::Priority),
                            "merged" => Ok(GeneratedField::Merged),
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
            type Value = Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.Source")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Source, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut display_name__ = None;
                let mut priority__ = None;
                let mut merged__ = None;
                let mut last_seen_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Merged => {
                            if merged__.is_some() {
                                return Err(serde::de::Error::duplicate_field("merged"));
                            }
                            merged__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastSeenTime => {
                            if last_seen_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastSeenTime"));
                            }
                            last_seen_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Source {
                    id: id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    merged: merged__.unwrap_or_default(),
                    last_seen_time: last_seen_time__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.Source", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamLevelsRequest {
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
        if self.interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.StreamLevelsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamLevelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Interval,
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
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamLevelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.StreamLevelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamLevelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut interval__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamLevelsRequest {
                    name: name__.unwrap_or_default(),
                    interval: interval__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.StreamLevelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamLevelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.levels.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.StreamLevelsResponse", len)?;
        if let Some(v) = self.levels.as_ref() {
            struct_ser.serialize_field("levels", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamLevelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "levels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Levels,
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
                            "levels" => Ok(GeneratedField::Levels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamLevelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.StreamLevelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamLevelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut levels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Levels => {
                            if levels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("levels"));
                            }
                            levels__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StreamLevelsResponse {
                    levels: levels__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.StreamLevelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Universe {
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
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        if self.number != 0 {
            len += 1;
        }
        if !self.node.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.slot_count != 0 {
            len += 1;
        }
        if self.create_time.is_some() {
            len += 1;
        }
        if self.role.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.Universe", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.protocol != 0 {
            let v = super::super::node::v1::Protocol::try_from(self.protocol)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        if self.number != 0 {
            struct_ser.serialize_field("number", &self.number)?;
        }
        if !self.node.is_empty() {
            struct_ser.serialize_field("node", &self.node)?;
        }
        if self.state != 0 {
            let v = UniverseState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if self.slot_count != 0 {
            struct_ser.serialize_field("slotCount", &self.slot_count)?;
        }
        if let Some(v) = self.create_time.as_ref() {
            struct_ser.serialize_field("createTime", v)?;
        }
        if let Some(v) = self.role.as_ref() {
            match v {
                universe::Role::Output(v) => {
                    struct_ser.serialize_field("output", v)?;
                }
                universe::Role::Input(v) => {
                    struct_ser.serialize_field("input", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Universe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "display_name",
            "displayName",
            "protocol",
            "number",
            "node",
            "state",
            "slot_count",
            "slotCount",
            "create_time",
            "createTime",
            "output",
            "input",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DisplayName,
            Protocol,
            Number,
            Node,
            State,
            SlotCount,
            CreateTime,
            Output,
            Input,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "number" => Ok(GeneratedField::Number),
                            "node" => Ok(GeneratedField::Node),
                            "state" => Ok(GeneratedField::State),
                            "slotCount" | "slot_count" => Ok(GeneratedField::SlotCount),
                            "createTime" | "create_time" => Ok(GeneratedField::CreateTime),
                            "output" => Ok(GeneratedField::Output),
                            "input" => Ok(GeneratedField::Input),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Universe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.Universe")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Universe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut display_name__ = None;
                let mut protocol__ = None;
                let mut number__ = None;
                let mut node__ = None;
                let mut state__ = None;
                let mut slot_count__ = None;
                let mut create_time__ = None;
                let mut role__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value::<super::super::node::v1::Protocol>()? as i32);
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<UniverseState>()? as i32);
                        }
                        GeneratedField::SlotCount => {
                            if slot_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slotCount"));
                            }
                            slot_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreateTime => {
                            if create_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createTime"));
                            }
                            create_time__ = map_.next_value()?;
                        }
                        GeneratedField::Output => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            role__ = map_.next_value::<::std::option::Option<_>>()?.map(universe::Role::Output)
;
                        }
                        GeneratedField::Input => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            role__ = map_.next_value::<::std::option::Option<_>>()?.map(universe::Role::Input)
;
                        }
                    }
                }
                Ok(Universe {
                    name: name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    number: number__.unwrap_or_default(),
                    node: node__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    slot_count: slot_count__.unwrap_or_default(),
                    create_time: create_time__,
                    role: role__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.Universe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UniverseState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNIVERSE_STATE_UNSPECIFIED",
            Self::Live => "UNIVERSE_STATE_LIVE",
            Self::Idle => "UNIVERSE_STATE_IDLE",
            Self::Failing => "UNIVERSE_STATE_FAILING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UniverseState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNIVERSE_STATE_UNSPECIFIED",
            "UNIVERSE_STATE_LIVE",
            "UNIVERSE_STATE_IDLE",
            "UNIVERSE_STATE_FAILING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UniverseState;

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
                    "UNIVERSE_STATE_UNSPECIFIED" => Ok(UniverseState::Unspecified),
                    "UNIVERSE_STATE_LIVE" => Ok(UniverseState::Live),
                    "UNIVERSE_STATE_IDLE" => Ok(UniverseState::Idle),
                    "UNIVERSE_STATE_FAILING" => Ok(UniverseState::Failing),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateLevelsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.levels.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.UpdateLevelsRequest", len)?;
        if let Some(v) = self.levels.as_ref() {
            struct_ser.serialize_field("levels", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateLevelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "levels",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Levels,
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
                            "levels" => Ok(GeneratedField::Levels),
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
            type Value = UpdateLevelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.UpdateLevelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateLevelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut levels__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Levels => {
                            if levels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("levels"));
                            }
                            levels__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateLevelsRequest {
                    levels: levels__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.UpdateLevelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUniverseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.universe.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.UpdateUniverseRequest", len)?;
        if let Some(v) = self.universe.as_ref() {
            struct_ser.serialize_field("universe", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUniverseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "universe",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Universe,
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
                            "universe" => Ok(GeneratedField::Universe),
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
            type Value = UpdateUniverseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.UpdateUniverseRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUniverseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut universe__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Universe => {
                            if universe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("universe"));
                            }
                            universe__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUniverseRequest {
                    universe: universe__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.UpdateUniverseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteLevelsRequest {
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
        if self.start_address != 0 {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dmx.universe.v1.WriteLevelsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.start_address != 0 {
            struct_ser.serialize_field("startAddress", &self.start_address)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteLevelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "start_address",
            "startAddress",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            StartAddress,
            Values,
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
                            "startAddress" | "start_address" => Ok(GeneratedField::StartAddress),
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteLevelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dmx.universe.v1.WriteLevelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteLevelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut start_address__ = None;
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartAddress => {
                            if start_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startAddress"));
                            }
                            start_address__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(WriteLevelsRequest {
                    name: name__.unwrap_or_default(),
                    start_address: start_address__.unwrap_or_default(),
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dmx.universe.v1.WriteLevelsRequest", FIELDS, GeneratedVisitor)
    }
}
