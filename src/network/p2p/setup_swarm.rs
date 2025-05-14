use libp2p::identity;
use libp2p::request_response;
use libp2p::PeerId;
use crate::network::p2p::transport::build_transport;
use super::{Request, Response};
 use libp2p::kad::{
    RoutingUpdate,
    Config as KadConfig,
    Behaviour as KadBehavior,
    Event as KadEvent,
    store::MemoryStore as KadInMemory,
};
use libp2p::request_response::json::Behaviour as RequestResponseJsonBehaviour;
use libp2p::request_response::Config as reqResConfig;
use libp2p::request_response::Event as RequestResponseEvent;
use libp2p::identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent, Config as IdentifyConfig};
use std::time::Duration;
use libp2p::request_response::ProtocolSupport::Full;

const _PROTOCOL_VERSION : String = String::from("/manaslibp2p/protocol/1.0.0");
const _PROTOCOL_NAME : String = String::from("/manaslibp2p/protocol");
// const _AGENT_VERSION : &str = "/manaslibp2p/agent/1.0.0";

pub fn setup_swarm() -> Result<,std::io::Error>{
    let id_keys = identity::Keypair::generate_ed25519();
    let node_public_key = id_keys.public();
    let peer_id = PeerId::from(node_public_key);
    let transport = build_transport(id_keys);
    match transport {
        Ok(res) => res,
        Err(err) => return std::io::Error(err)
    }
    
    //setting up behaviours

    //kad behaviour
    let kad_config = KadConfig::new(_PROTOCOL_VERSION);
    let kad_memory = KadInMemory::new(local_peer_id);
    let kad = KadBehavior::with_config(local_peer_id, kad_memory, kad_config);

    //identify behaviour
    let identify_config = IdentifyConfig::new(_PROTOCOL_VERSION,node_public_key);
    let identify_behaviour =  IdentifyBehaviour::new(identify_config);

    //request-response behaviour
    let req_res_config = reqResConfig::default()
        .with_max_concurrent_streams(32)
        .with_request_timeout(Duration::from_secs(60));
    
    let protocols = vec![
        (_PROTOCOL_NAME.to_string(), Full),
    ];
    
    let req_res_behaviour = RequestResponseJsonBehaviour::<Request, Response>::new(
        protocols,
        req_res_config
    );

}

// use std::fmt::Error;
// use std::time::Duration;
// use libp2p::{
//     Multiaddr,
//     identity,
//     PeerId,
//     StreamProtocol,
//     SwarmBuilder,
//     tcp::Config as TcpConfig,
//     yamux::Config as YamuxConfig
// };
// use libp2p::kad::{
//     RoutingUpdate,
//     Config as KadConfig,
//     Behaviour as KadBehavior,
//     Event as KadEvent,
//     store::MemoryStore as KadInMemory,
// };
// use libp2p::identify::{
//     Config as IdentifyConfig,
//     Behaviour as IdentifyBehavior,
//     Event as IdentifyEvent
// };
// use libp2p::noise::Config as NoiceConfig;
// use crate::constants::constants::_STREAMPROTOCOLNAME;

// struct returnTypeOfBuildSwarm {
//     local_peer_id: PeerId,

// }
// async fn build_swarm() -> Result<returnTypeOfBuildSwarm, Error> {
//     //generating the idenity pair
//     let local_key = identity::Keypair::generate_ed25519();
//     let res = returnTypeOfBuildSwarm {
//         local_peer_id: PeerId::from(local_key.public()),
//     };
//     Ok(res)
// }
// let mut swarm = SwarmBuilder::with_existing_identity(local_key.clone()).with_tokio().with_tcp(TcpConfig::default(),  NoiceConfig::new,
// YamuxConfig::default)?.with_behaviour( |key| {
//     let local_peer_id = PeerIPeerIdd::from(key.clone().public());
//     println!("LocalPeerID: {local_peer_id}");
//     let mut kad_config = KadConfig::default();
//     kad_config.set_protocol_names(vec![StreamProtocol::new(_STREAMPROTOCOLNAME)]);
//     let kad_memory = KadInMemory::new(local_peer_id);
//     let kad = KadBehavior::with_config(local_peer_id, kad_memory, kad_config);
//     let identity_config = IdentifyConfig::new(
//         _STREAMPROTOCOLNAME.to_string(),
//         key.clone().public()
//     )
//     .with_push_listen_addr_updates(true)
//     .with_interval(Duration::from_secs(30));
//     let rr_config = RequestResponseConfig::default();
//     let rr_protocol = StreamProtocol::new("/agent/message/1.0.0");
//     let rr_behavior = RequestResponseBehavior::<GreeRequest, GreetResponse>::new([(rr_protocol, RequestResponseProtocolSupport::Full)], rr_config);

//     let identify = IdentifyBehavior::new(identity_config);
//     AgentBehavior::new(kad, identify, rr_behavior)

// });
