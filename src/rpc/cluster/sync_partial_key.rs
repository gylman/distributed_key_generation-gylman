use std::sync::Arc;

use radius_sequencer_sdk::json_rpc::{types::RpcParameter, RpcError};
use serde::{Deserialize, Serialize};
use skde::key_generation::{verify_partial_key_validity, PartialKey, PartialKeyProof};
use tracing::info;

use crate::{
    state::AppState,
    types::{Address, KeyGeneratorModel, PartialKeyListModel},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncPartialKey {
    pub address: Address,
    pub key_id: u64,
    pub partial_key: PartialKey,
    pub partial_key_proof: PartialKeyProof,
}

impl SyncPartialKey {
    pub const METHOD_NAME: &'static str = "sync_partial_key";

    pub async fn handler(parameter: RpcParameter, context: Arc<AppState>) -> Result<(), RpcError> {
        let parameter = parameter.parse::<Self>()?;
        let is_key_generator_in_cluster = !KeyGeneratorModel::get(&parameter.address).is_err();

        if is_key_generator_in_cluster {
            info!(
                "sync_partial_key - address: {:?} / partial_key: {:?}",
                parameter.address, parameter.partial_key
            );

            let is_valid = verify_partial_key_validity(
                context.skde_params(),
                parameter.partial_key.clone(),
                parameter.partial_key_proof,
            );

            // TODO:
            if !is_valid {
                return Ok(());
            }

            let mut partial_key_list = PartialKeyListModel::get_mut_or_default(parameter.key_id)?;
            partial_key_list.insert(parameter.address, parameter.partial_key);
            partial_key_list.update()?;
        }

        Ok(())
    }
}
