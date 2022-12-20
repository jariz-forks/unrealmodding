use crate::{
    cast,
    error::Error,
    impl_property_data_trait, optional_guid, optional_guid_write,
    properties::{str_property::StrProperty, Property, PropertyDataTrait, PropertyTrait},
    reader::asset_reader::AssetReader,
    unreal_types::{FName, Guid},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovieSceneEvalTemplatePtrProperty {
    pub name: FName,
    pub property_guid: Option<Guid>,
    pub duplication_index: i32,
    pub value: Vec<Property>,
}
impl_property_data_trait!(MovieSceneEvalTemplatePtrProperty);

impl MovieSceneEvalTemplatePtrProperty {
    pub fn new<Reader: AssetReader>(
        asset: &mut Reader,
        name: FName,
        include_header: bool,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);

        let mut value: Vec<Property> = Vec::new();

        let type_name_fname = asset.add_fname("TypeName");
        let type_name = StrProperty::new(asset, type_name_fname, include_header, 0)?;

        if type_name.value.is_some() {
            value.push(type_name.into());
            while let Some(data) = Property::new(asset, Some(&name), true)? {
                value.push(data);
            }
        }

        Ok(MovieSceneEvalTemplatePtrProperty {
            name,
            property_guid,
            duplication_index,
            value,
        })
    }
}

impl PropertyTrait for MovieSceneEvalTemplatePtrProperty {
    fn write<Writer: crate::reader::asset_writer::AssetWriter>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);

        let begin = asset.position();

        let mut had_typename = false;

        for property in &self.value {
            if property.get_name().content == "TypeName" {
                let str_property: &StrProperty = cast!(Property, StrProperty, property)
                    .ok_or_else(|| {
                        Error::no_data("TypeName property is not StrProperty".to_string())
                    })?;
                had_typename = str_property.value.is_some();
                asset.write_string(&str_property.value)?;
            } else {
                Property::write(property, asset, true)?;
            }
        }

        if had_typename {
            let none_fname = asset.add_fname("None");
            asset.write_fname(&none_fname)?;
        }

        Ok((asset.position() - begin) as usize)
    }
}
