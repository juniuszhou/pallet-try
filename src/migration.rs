use super::*;
use rstd::collections::btree_map::BTreeMap;

// Triggered when runtime upgrade
pub fn on_runtime_upgrade<T: Trait>() {
    match StorageVersion::get() {
        // Nothing if already upgraded
        Releases::V2_0_0 => (),
        // Start the upgrade process
        Releases::V1_0_0 => upgrade_v1_to_v2::<T>(),
    }
}

fn upgrade_v1_to_v2<T: Trait>() {
    StorageVersion::put(Releases::V2_0_0);
}
