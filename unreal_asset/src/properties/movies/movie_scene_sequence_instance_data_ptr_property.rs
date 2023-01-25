use byteorder::LittleEndian;

use crate::{
    error::Error,
    impl_property_data_trait, optional_guid, optional_guid_write,
    properties::PropertyTrait,
    reader::{asset_reader::AssetReader, asset_writer::AssetWriter},
    types::{FName, Guid, PackageIndex},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovieSceneSequenceInstanceDataPtrProperty {
    pub name: FName,
    pub property_guid: Option<Guid>,
    pub duplication_index: i32,
    pub value: PackageIndex,
}
impl_property_data_trait!(MovieSceneSequenceInstanceDataPtrProperty);

impl MovieSceneSequenceInstanceDataPtrProperty {
    pub fn new<Reader: AssetReader>(
        asset: &mut Reader,
        name: FName,
        include_header: bool,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);

        let value = PackageIndex::new(asset.read_i32::<LittleEndian>()?);

        Ok(MovieSceneSequenceInstanceDataPtrProperty {
            name,
            property_guid,
            duplication_index,
            value,
        })
    }
}

impl PropertyTrait for MovieSceneSequenceInstanceDataPtrProperty {
    fn write<Writer: AssetWriter>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);

        let begin = asset.position();
        asset.write_i32::<LittleEndian>(self.value.index)?;

        Ok((asset.position() - begin) as usize)
    }
}