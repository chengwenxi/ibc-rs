use abscissa_core::{Command, Options, Runnable};

use ibc::ics24_host::identifier::{ChainId, ChannelId, PortId};
use ibc_relayer::link::LinkParameters;
use ibc_relayer::relay::{channel_relay, relay_on_new_link};

use crate::cli_utils::ChainHandlePair;
use crate::conclude::Output;
use crate::prelude::*;

#[derive(Clone, Command, Debug, Options)]
pub struct StartCmd {
    #[options(free, required, help = "identifier of the source chain")]
    src_chain_id: ChainId,

    #[options(free, required, help = "identifier of the destination chain")]
    dst_chain_id: ChainId,

    #[options(help = "identifier of the source port", short = "p")]
    src_port_id: Option<PortId>,

    #[options(help = "identifier of the source channel", short = "c")]
    src_channel_id: Option<ChannelId>,
}

impl Runnable for StartCmd {
    fn run(&self) {
        let config = app_config();

        let chains = match ChainHandlePair::spawn(&config, &self.src_chain_id, &self.dst_chain_id) {
            Ok(chains) => chains,
            Err(e) => return Output::error(format!("{}", e)).exit(),
        };

        match (&self.src_port_id, &self.src_channel_id) {
            (Some(src_port_id), Some(src_channel_id)) => {
                match channel_relay(
                    chains.src,
                    chains.dst,
                    LinkParameters {
                        src_port_id: src_port_id.clone(),
                        src_channel_id: src_channel_id.clone(),
                    },
                ) {
                    Ok(()) => Output::success(()).exit(),
                    Err(e) => Output::error(e.to_string()).exit(),
                }
            }
            (None, None) => {
                // Relay for a single channel, first on the first connection between the two chains
                let relay_path = config.first_matching_path(&self.src_chain_id, &self.dst_chain_id);

                match relay_path {
                    Some((connection, path)) => {
                        info!("Start relayer on {:?}", self);

                        match relay_on_new_link(
                            chains.src,
                            chains.dst,
                            connection.delay,
                            path.ordering,
                            path.clone(),
                        ) {
                            Ok(()) => Output::success(()).exit(),
                            Err(e) => Output::error(e.to_string()).exit(),
                        }
                    }
                    None => {
                        Output::error(format!("No connections configured for {:?}", self)).exit()
                    }
                }
            }
            _ => Output::error(format!(
                "Invalid parameters, either both port and channel must be specified or none: {:?}",
                self
            ))
            .exit(),
        }
    }
}
