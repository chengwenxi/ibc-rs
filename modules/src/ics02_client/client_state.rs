use core::marker::{Send, Sync};
use std::convert::TryFrom;

use prost_types::Any;
use serde::Serialize;
use tendermint_proto::Protobuf;

use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::error::{Error, Kind};

use crate::ics07_tendermint::client_state;
use crate::ics24_host::identifier::ChainId;
#[cfg(any(test, feature = "mocks"))]
use crate::mock::client_state::MockClientState;
use crate::Height;

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";
pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";

#[dyn_clonable::clonable]
pub trait ClientState: Clone + std::fmt::Debug + Send + Sync {
    /// Return the chain identifier which this client is serving (i.e., the client is verifying
    /// consensus states from this chain).
    fn chain_id(&self) -> ChainId;

    /// Type of client associated with this state (eg. Tendermint)
    fn client_type(&self) -> ClientType;

    /// Latest height of consensus state
    fn latest_height(&self) -> Height;

    /// Freeze status of the client
    fn is_frozen(&self) -> bool;

    /// Wrap into an `AnyClientState`
    fn wrap_any(self) -> AnyClientState;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum AnyClientState {
    Tendermint(client_state::ClientState),

    #[cfg(any(test, feature = "mocks"))]
    Mock(MockClientState),
}

impl AnyClientState {
    pub fn latest_height(&self) -> Height {
        match self {
            Self::Tendermint(tm_state) => tm_state.latest_height(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(mock_state) => mock_state.latest_height(),
        }
    }

    pub fn client_type(&self) -> ClientType {
        match self {
            Self::Tendermint(state) => state.client_type(),

            #[cfg(any(test, feature = "mocks"))]
            Self::Mock(state) => state.client_type(),
        }
    }
}

impl Protobuf<Any> for AnyClientState {}

impl TryFrom<Any> for AnyClientState {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        match raw.type_url.as_str() {
            "" => Err(Kind::EmptyClientStateResponse.into()),

            TENDERMINT_CLIENT_STATE_TYPE_URL => Ok(AnyClientState::Tendermint(
                client_state::ClientState::decode_vec(&raw.value)
                    .map_err(|e| Kind::InvalidRawClientState.context(e))?,
            )),

            #[cfg(any(test, feature = "mocks"))]
            MOCK_CLIENT_STATE_TYPE_URL => Ok(AnyClientState::Mock(
                MockClientState::decode_vec(&raw.value)
                    .map_err(|e| Kind::InvalidRawClientState.context(e))?,
            )),

            _ => Err(Kind::UnknownClientStateType(raw.type_url).into()),
        }
    }
}

impl From<AnyClientState> for Any {
    fn from(value: AnyClientState) -> Self {
        match value {
            AnyClientState::Tendermint(value) => Any {
                type_url: TENDERMINT_CLIENT_STATE_TYPE_URL.to_string(),
                value: value.encode_vec().unwrap(),
            },
            #[cfg(any(test, feature = "mocks"))]
            AnyClientState::Mock(value) => Any {
                type_url: MOCK_CLIENT_STATE_TYPE_URL.to_string(),
                value: value.encode_vec().unwrap(),
            },
        }
    }
}

impl ClientState for AnyClientState {
    fn chain_id(&self) -> ChainId {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.chain_id(),

            #[cfg(any(test, feature = "mocks"))]
            AnyClientState::Mock(mock_state) => mock_state.chain_id(),
        }
    }

    fn client_type(&self) -> ClientType {
        self.client_type()
    }

    fn latest_height(&self) -> Height {
        self.latest_height()
    }

    fn is_frozen(&self) -> bool {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.is_frozen(),

            #[cfg(any(test, feature = "mocks"))]
            AnyClientState::Mock(mock_state) => mock_state.is_frozen(),
        }
    }

    fn wrap_any(self) -> AnyClientState {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use prost_types::Any;

    use crate::ics02_client::client_state::AnyClientState;
    use crate::ics07_tendermint::client_state::test_util::get_dummy_tendermint_client_state;
    use crate::ics07_tendermint::header::test_util::get_dummy_tendermint_header;

    #[test]
    fn any_client_state_serialization() {
        let tm_client_state = get_dummy_tendermint_client_state(get_dummy_tendermint_header());

        let raw: Any = tm_client_state.clone().into();
        let tm_client_state_back = AnyClientState::try_from(raw).unwrap();
        assert_eq!(tm_client_state, tm_client_state_back);
    }
}
