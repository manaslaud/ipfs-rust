use std::io;
use std::time::Duration;
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::Boxed;
use libp2p::core::{transport::MemoryTransport, upgrade, Transport};
use libp2p::{noise, PeerId};
use libp2p::{identity, yamux};

pub(crate) type TTransport = Boxed<(PeerId, StreamMuxerBox)>;

pub fn build_transport(id_keys: identity::Keypair) -> io::Result<TTransport> {
    let noise = noise::Config::new(&id_keys).unwrap();
    let transport = MemoryTransport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise)
        .multiplex(yamux::Config::default())
        .timeout(Duration::from_secs(20))
        .boxed();
    
    Ok(transport)
}