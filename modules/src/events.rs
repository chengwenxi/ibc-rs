use std::collections::HashMap;

use anomaly::BoxError;
use serde_derive::{Deserialize, Serialize};

use crate::ics02_client::events as ClientEvents;
use crate::ics02_client::events::NewBlock;
use crate::ics03_connection::events as ConnectionEvents;
use crate::ics04_channel::events as ChannelEvents;
use crate::Height;
use prost::alloc::fmt::Formatter;
use std::fmt;

/// Events types
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IbcEventType {
    CreateClient,
    UpdateClient,
    SendPacket,
    WriteAck,
}

impl IbcEventType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            IbcEventType::CreateClient => "create_client",
            IbcEventType::UpdateClient => "update_client",
            IbcEventType::SendPacket => "send_packet",
            IbcEventType::WriteAck => "write_acknowledgement",
        }
    }
}

/// Events created by the IBC component of a chain, destined for a relayer.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IbcEvent {
    NewBlock(NewBlock),

    CreateClient(ClientEvents::CreateClient),
    UpdateClient(ClientEvents::UpdateClient),
    UpgradeClient(ClientEvents::UpgradeClient),
    ClientMisbehaviour(ClientEvents::ClientMisbehaviour),

    OpenInitConnection(ConnectionEvents::OpenInit),
    OpenTryConnection(ConnectionEvents::OpenTry),
    OpenAckConnection(ConnectionEvents::OpenAck),
    OpenConfirmConnection(ConnectionEvents::OpenConfirm),

    OpenInitChannel(ChannelEvents::OpenInit),
    OpenTryChannel(ChannelEvents::OpenTry),
    OpenAckChannel(ChannelEvents::OpenAck),
    OpenConfirmChannel(ChannelEvents::OpenConfirm),
    CloseInitChannel(ChannelEvents::CloseInit),
    CloseConfirmChannel(ChannelEvents::CloseConfirm),

    SendPacket(ChannelEvents::SendPacket),
    ReceivePacket(ChannelEvents::ReceivePacket),
    WriteAcknowledgement(ChannelEvents::WriteAcknowledgement),
    AcknowledgePacket(ChannelEvents::AcknowledgePacket),
    TimeoutPacket(ChannelEvents::TimeoutPacket),
    TimeoutOnClosePacket(ChannelEvents::TimeoutOnClosePacket),

    Empty(String),      // Special event, signifying empty response
    ChainError(String), // Special event, signifying an error on CheckTx or DeliverTx
}

/// For use in debug messages
pub struct VecIbcEvents(pub Vec<IbcEvent>);
impl fmt::Display for VecIbcEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "events:")?;
        for v in &self.0 {
            writeln!(f, "\t{}", v)?;
        }
        Ok(())
    }
}

impl fmt::Display for IbcEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IbcEvent::NewBlock(ev) => write!(f, "NewBlock({})", ev.height),

            IbcEvent::CreateClient(ev) => write!(f, "CreateClientEv({})", ev),
            IbcEvent::UpdateClient(ev) => write!(f, "UpdateClientEv({})", ev),
            IbcEvent::UpgradeClient(ev) => write!(f, "UpgradeClientEv({:?})", ev),
            IbcEvent::ClientMisbehaviour(ev) => write!(f, "ClientMisbehaviourEv({:?})", ev),

            IbcEvent::OpenInitConnection(ev) => write!(f, "OpenInitConnectionEv({:?})", ev),
            IbcEvent::OpenTryConnection(ev) => write!(f, "OpenTryConnectionEv({:?})", ev),
            IbcEvent::OpenAckConnection(ev) => write!(f, "OpenAckConnectionEv({:?})", ev),
            IbcEvent::OpenConfirmConnection(ev) => write!(f, "OpenConfirmConnectionEv({:?})", ev),

            IbcEvent::OpenInitChannel(ev) => write!(f, "OpenInitChannelEv({:?})", ev),
            IbcEvent::OpenTryChannel(ev) => write!(f, "OpenTryChannelEv({:?})", ev),
            IbcEvent::OpenAckChannel(ev) => write!(f, "OpenAckChannelEv({:?})", ev),
            IbcEvent::OpenConfirmChannel(ev) => write!(f, "OpenConfirmChannelEv({:?})", ev),
            IbcEvent::CloseInitChannel(ev) => write!(f, "CloseInitChannelEv({})", ev),
            IbcEvent::CloseConfirmChannel(ev) => write!(f, "CloseConfirmChannelEv({:?})", ev),

            IbcEvent::SendPacket(ev) => write!(f, "SendPacketEv({})", ev),
            IbcEvent::ReceivePacket(ev) => write!(f, "ReceivePacketEv({})", ev),
            IbcEvent::WriteAcknowledgement(ev) => write!(f, "WriteAcknowledgementEv({})", ev),
            IbcEvent::AcknowledgePacket(ev) => write!(f, "AcknowledgePacketEv({})", ev),
            IbcEvent::TimeoutPacket(ev) => write!(f, "TimeoutPacketEv({})", ev),
            IbcEvent::TimeoutOnClosePacket(ev) => write!(f, "TimeoutOnClosePacketEv({})", ev),

            IbcEvent::Empty(ev) => write!(f, "EmptyEv({})", ev),
            IbcEvent::ChainError(ev) => write!(f, "ChainErrorEv({})", ev),
        }
    }
}

// This is tendermint specific
pub fn from_tx_response_event(height: Height, event: &tendermint::abci::Event) -> Option<IbcEvent> {
    // Return the first hit we find
    if let Some(mut client_res) = ClientEvents::try_from_tx(event) {
        client_res.set_height(height);
        Some(client_res)
    } else if let Some(mut conn_res) = ConnectionEvents::try_from_tx(event) {
        conn_res.set_height(height);
        Some(conn_res)
    } else if let Some(mut chan_res) = ChannelEvents::try_from_tx(event) {
        chan_res.set_height(height);
        Some(chan_res)
    } else {
        None
    }
}

impl IbcEvent {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn height(&self) -> Height {
        match self {
            IbcEvent::NewBlock(bl) => bl.height(),
            IbcEvent::CreateClient(ev) => ev.height(),
            IbcEvent::UpdateClient(ev) => ev.height(),
            IbcEvent::ClientMisbehaviour(ev) => ev.height(),
            IbcEvent::OpenInitConnection(ev) => ev.height(),
            IbcEvent::OpenTryConnection(ev) => ev.height(),
            IbcEvent::OpenAckConnection(ev) => ev.height(),
            IbcEvent::OpenConfirmConnection(ev) => ev.height(),
            IbcEvent::OpenInitChannel(ev) => ev.height(),
            IbcEvent::OpenTryChannel(ev) => ev.height(),
            IbcEvent::OpenAckChannel(ev) => ev.height(),
            IbcEvent::OpenConfirmChannel(ev) => ev.height(),
            IbcEvent::CloseInitChannel(ev) => ev.height(),
            IbcEvent::CloseConfirmChannel(ev) => ev.height(),
            IbcEvent::SendPacket(ev) => ev.height(),
            IbcEvent::ReceivePacket(ev) => ev.height(),
            IbcEvent::WriteAcknowledgement(ev) => ev.height(),
            IbcEvent::AcknowledgePacket(ev) => ev.height(),
            IbcEvent::TimeoutPacket(ev) => ev.height(),
            _ => unimplemented!(),
        }
    }

    pub fn set_height(&mut self, height: Height) {
        match self {
            IbcEvent::NewBlock(ev) => ev.set_height(height),
            IbcEvent::CreateClient(ev) => ev.set_height(height),
            IbcEvent::UpdateClient(ev) => ev.set_height(height),
            IbcEvent::UpgradeClient(ev) => ev.set_height(height),
            IbcEvent::ClientMisbehaviour(ev) => ev.set_height(height),
            IbcEvent::OpenInitConnection(ev) => ev.set_height(height),
            IbcEvent::OpenTryConnection(ev) => ev.set_height(height),
            IbcEvent::OpenAckConnection(ev) => ev.set_height(height),
            IbcEvent::OpenConfirmConnection(ev) => ev.set_height(height),
            IbcEvent::OpenInitChannel(ev) => ev.set_height(height),
            IbcEvent::OpenTryChannel(ev) => ev.set_height(height),
            IbcEvent::OpenAckChannel(ev) => ev.set_height(height),
            IbcEvent::OpenConfirmChannel(ev) => ev.set_height(height),
            IbcEvent::CloseInitChannel(ev) => ev.set_height(height),
            IbcEvent::CloseConfirmChannel(ev) => ev.set_height(height),
            IbcEvent::SendPacket(ev) => ev.set_height(height),
            IbcEvent::ReceivePacket(ev) => ev.set_height(height),
            IbcEvent::WriteAcknowledgement(ev) => ev.set_height(height),
            IbcEvent::AcknowledgePacket(ev) => ev.set_height(height),
            IbcEvent::TimeoutPacket(ev) => ev.set_height(height),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawObject {
    pub height: Height,
    pub action: String,
    pub idx: usize,
    pub events: HashMap<String, Vec<String>>,
}

impl RawObject {
    pub fn new(
        height: Height,
        action: String,
        idx: usize,
        events: HashMap<String, Vec<String>>,
    ) -> RawObject {
        RawObject {
            height,
            action,
            idx,
            events,
        }
    }
}

pub fn extract_events<S: ::std::hash::BuildHasher>(
    events: &HashMap<String, Vec<String>, S>,
    action_string: &str,
) -> Result<(), BoxError> {
    if let Some(message_action) = events.get("message.action") {
        if message_action.contains(&action_string.to_owned()) {
            return Ok(());
        }
        return Err("Missing action string".into());
    }
    Err("Incorrect Event Type".into())
}

#[macro_export]
macro_rules! make_event {
    ($a:ident, $b:literal) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        pub struct $a {
            pub data: ::std::collections::HashMap<String, Vec<String>>,
        }
        impl ::std::convert::TryFrom<$crate::events::RawObject> for $a {
            type Error = ::anomaly::BoxError;

            fn try_from(result: $crate::events::RawObject) -> Result<Self, Self::Error> {
                match $crate::events::extract_events(&result.events, $b) {
                    Ok(()) => Ok($a {
                        data: result.events.clone(),
                    }),
                    Err(e) => Err(e),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! attribute {
    ($a:ident, $b:literal) => {
        $a.events.get($b).ok_or($b)?[$a.idx].parse()?
    };
}

#[macro_export]
macro_rules! some_attribute {
    ($a:ident, $b:literal) => {
        $a.events.get($b).ok_or($b)?[$a.idx].parse().ok()
    };
}
