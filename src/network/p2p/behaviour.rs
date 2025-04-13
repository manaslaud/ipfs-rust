use libp2p::kad::Event as KadEvent;
use libp2p::kad::{store::MemoryStore, Behaviour,};
use libp2p::identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent};
use libp2p::swarm::NetworkBehaviour;
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "AgentEvent")]
pub struct AgentBehavior {
    pub kad: Behaviour<MemoryStore>,
    pub identify: IdentifyBehaviour,
}
#[derive(Debug)]
pub enum AgentEvent {
    Kad(KadEvent),
    Identify(IdentifyEvent),
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