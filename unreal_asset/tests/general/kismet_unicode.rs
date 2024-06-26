use std::io::Cursor;

use unreal_asset::{engine_version::EngineVersion, Asset, Error};

#[allow(clippy::duplicate_mod)]
#[path = "../shared.rs"]
mod shared;

macro_rules! assets_folder {
    () => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/assets/general/npc_onop/"
        )
    };
}

const TEST_ASSETS: [(&[u8], &[u8]); 1] = [(
    include_bytes!(concat!(assets_folder!(), "NPC_Onop_IO_Bech.uasset")),
    include_bytes!(concat!(assets_folder!(), "NPC_Onop_IO_Bech.uexp")),
)];

#[test]
fn kismet_unicode() -> Result<(), Error> {
    for (test_asset, asset_bulk) in TEST_ASSETS {
        let mut asset = Asset::new(
            Cursor::new(test_asset),
            Some(Cursor::new(asset_bulk)),
            EngineVersion::VER_UE4_25,
            None,
        )?;
        shared::verify_binary_equality(test_asset, Some(asset_bulk), &mut asset)?;
    }

    Ok(())
}
