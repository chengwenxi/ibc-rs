use ibc::ics24_host::identifier::ChainId;
use ibc_relayer::chain::runtime::ChainRuntime;
use ibc_relayer::chain::CosmosSdkChain;
use ibc_relayer::{chain::handle::ChainHandle, config::Config};

use crate::error::{Error, Kind};

#[derive(Clone, Debug)]
/// Pair of chain handles that are used by most CLIs.
pub struct ChainHandlePair {
    /// Source chain handle
    pub src: Box<dyn ChainHandle>,
    /// Destination chain handle
    pub dst: Box<dyn ChainHandle>,
}

impl ChainHandlePair {
    /// Spawn the source and destination chain runtime from the configuration and chain identifiers,
    /// and return the pair of associated handles.
    pub fn spawn(
        config: &Config,
        src_chain_id: &ChainId,
        dst_chain_id: &ChainId,
    ) -> Result<Self, Error> {
        let src = spawn_chain_runtime(config, src_chain_id)?;
        let dst = spawn_chain_runtime(config, dst_chain_id)?;

        Ok(ChainHandlePair { src, dst })
    }
}

/// Spawns a chain runtime from the configuration and given a chain identifier.
/// Returns the corresponding handle if successful.
pub fn spawn_chain_runtime(
    config: &Config,
    chain_id: &ChainId,
) -> Result<Box<dyn ChainHandle>, Error> {
    let chain_config = config
        .find_chain(chain_id)
        .cloned()
        .ok_or_else(|| format!("missing chain for id ({}) in configuration file", chain_id))
        .map_err(|e| Kind::Config.context(e))?;

    let chain_res =
        ChainRuntime::<CosmosSdkChain>::spawn(chain_config).map_err(|e| Kind::Runtime.context(e));

    let handle = chain_res.map(|(handle, _)| handle)?;

    Ok(handle)
}
