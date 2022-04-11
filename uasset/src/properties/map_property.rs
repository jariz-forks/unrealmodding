use std::{collections::HashMap, hash::Hash, io::{Cursor}};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{impl_property_data_trait, {Asset, unreal_types::{FName, Guid}}};
use crate::error::Error;
use crate::properties::{PropertyDataTrait, PropertyTrait};
use crate::unreal_types::ToFName;

use super::{Property};

#[derive(PartialEq, Eq)]
pub struct MapProperty {
    pub name: FName,
    pub property_guid: Option<Guid>,
    pub duplication_index: i32,
    pub key_type: FName,
    pub value_type: FName,
    pub value: HashMap<Property, Property>,
    pub keys_to_remove: Option<Vec<Property>>,
}
impl_property_data_trait!(MapProperty);

impl Hash for MapProperty {
    //todo: probably do something with map
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.property_guid.hash(state);
        self.key_type.hash(state);
        self.value_type.hash(state);
    }
}

impl MapProperty {
    fn map_type_to_class(asset: &mut Asset, type_name: FName, name: FName, length: i64, include_header: bool, is_key: bool) -> Result<Property, Error> {
        match type_name.content.as_str() {
            "StructProperty" => {
                let struct_type = match is_key {
                    true => asset.map_key_override.get(&name.content).map(|s| s.to_owned()),
                    false => asset.map_value_override.get(&name.content).map(|s| s.to_owned())
                }.unwrap_or(String::from("Generic"));

                Property::from_type(asset, &FName::new(struct_type.to_string(), 0), name, false, 1, 0, 0)
            }
            _ => {
                Property::from_type(asset, &type_name, name, include_header, length, 0, 0)
            }
        }
    }

    pub fn new(asset: &mut Asset, name: FName, include_header: bool, duplication_index: i32) -> Result<Self, Error> {
        let mut type_1 = None;
        let mut type_2 = None;
        let mut property_guid = None;

        if include_header {
            type_1 = Some(asset.read_fname()?);
            type_2 = Some(asset.read_fname()?);
            property_guid = asset.read_property_guid()?;
        }

        let num_keys_to_remove = asset.cursor.read_i32::<LittleEndian>()?;
        let mut keys_to_remove = None;

        let type_1 = type_1.ok_or(Error::invalid_file("No type1".to_string()))?;
        let type_2 = type_2.ok_or(Error::invalid_file("No type2".to_string()))?;

        for _ in 0..num_keys_to_remove as usize {
            let mut vec = Vec::with_capacity(num_keys_to_remove as usize);
            vec.push(MapProperty::map_type_to_class(asset, type_1.clone(), name.clone(), 0, false, true)?);
            keys_to_remove = Some(vec);
        }

        let num_entries = asset.cursor.read_i32::<LittleEndian>()?;
        let mut values = HashMap::new();

        for _ in 0..num_entries {
            let key = MapProperty::map_type_to_class(asset, type_1.clone(), name.clone(), 0, false, true)?;
            let value = MapProperty::map_type_to_class(asset, type_2.clone(), name.clone(), 0, false, false)?;

            values.insert(key, value);
        }

        Ok(MapProperty {
            name,
            property_guid: property_guid,
            duplication_index,
            key_type: type_1,
            value_type: type_2,
            value: values,
            keys_to_remove,
        })
    }
}

impl PropertyTrait for MapProperty {
    fn write(&self, asset: &Asset, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<usize, Error> {
        if include_header {
            if let Some(key) = self.value.keys().next() {
                asset.write_fname(cursor, &key.to_fname())?;
                let value = self.value.values().next().unwrap();
                asset.write_fname(cursor, &value.to_fname())?;
            } else {
                asset.write_fname(cursor, &self.key_type)?;
                asset.write_fname(cursor, &self.value_type)?
            }
            asset.write_property_guid(cursor, &self.property_guid)?;
        }

        let begin = cursor.position();
        cursor.write_i32::<LittleEndian>(match self.keys_to_remove {
            Some(ref e) => e.len(),
            None => 0
        } as i32)?;

        if let Some(ref keys_to_remove) = self.keys_to_remove {
            for key in keys_to_remove {
                key.write(asset, cursor, false)?;
            }
        }

        cursor.write_i32::<LittleEndian>(self.value.len() as i32)?;

        for (key, value) in &self.value {
            key.write(asset, cursor, false)?;
            value.write(asset, cursor, false)?;
        }

        Ok((cursor.position() - begin) as usize)
    }
}
