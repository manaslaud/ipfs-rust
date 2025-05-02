use super::{Request, Response};
use libp2p::identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent};
use libp2p::kad::{store::MemoryStore, Behaviour as KademliaBehaviour, Event as KadEvent};
use libp2p::request_response::json::Behaviour as RequestResponseJsonBehaviour;
use libp2p::request_response::Event as RequestResponseEvent;
use libp2p::swarm::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "AgentEvent")]
pub struct AgentBehavior {
    pub kad: KademliaBehaviour<MemoryStore>,
    pub identify: IdentifyBehaviour,
    pub rr: RequestResponseJsonBehaviour<Request, Response>,
}

#[derive(Debug)]
pub enum AgentEvent {
    Kad(KadEvent),
    Identify(IdentifyEvent),
    RequestResponse(RequestResponseEvent<Request, Response>),
}

impl From<KadEvent> for AgentEvent {
    fn from(event: KadEvent) -> Self {
        AgentEvent::Kad(event)
    }
}

impl From<IdentifyEvent> for AgentEvent {
    fn from(event: IdentifyEvent) -> Self {
        AgentEvent::Identify(event)
    }
}

impl From<RequestResponseEvent<Request, Response>> for AgentEvent {
    fn from(event: RequestResponseEvent<Request, Response>) -> Self {
        AgentEvent::RequestResponse(event)
    }
}

impl AgentBehavior {
    pub fn new(
        kad: KademliaBehaviour<MemoryStore>,
        identify: IdentifyBehaviour,
        rr: RequestResponseJsonBehaviour<Request, Response>,
    ) -> Self {
        AgentBehavior { kad, identify, rr }
    }
}
