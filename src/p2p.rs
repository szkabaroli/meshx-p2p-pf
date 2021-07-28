// Kademlia
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaEvent};
// Mdns
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
// Core
use futures::StreamExt;
use libp2p::swarm::NetworkBehaviourEventProcess;
use libp2p::{development_transport, identity};
use libp2p::{NetworkBehaviour, PeerId, Swarm};
use std::error::Error;

use crate::ports::IP2PSharingPort;

// Swarm Type
use libp2p::core::either::EitherOutput;
use libp2p::kad::handler::{KademliaHandlerEvent, KademliaHandlerIn, KademliaHandlerProto};
use libp2p::kad::QueryId;
use libp2p::swarm::protocols_handler::DummyProtocolsHandler;
use libp2p::swarm::{ExpandedSwarm, IntoProtocolsHandler, IntoProtocolsHandlerSelect, SwarmEvent};

pub type SwarmType = ExpandedSwarm<
    MyBehaviour,
    EitherOutput<KademliaHandlerIn<QueryId>, void::Void>,
    EitherOutput<KademliaHandlerEvent<QueryId>, void::Void>,
    IntoProtocolsHandlerSelect<KademliaHandlerProto<QueryId>, DummyProtocolsHandler>,
>;

// TODO: key-value protocol
// We create a custom network behaviour that combines Kademlia and mDNS.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    #[behaviour(ignore)]
    #[allow(dead_code)]
    test: u32,

    kademlia: Kademlia<MemoryStore>,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(peers) => {
                for (peer, addr) in peers {
                    log::info!("discovered using mdns {} {} {}", peer, addr, self.test);

                    self.kademlia.add_address(&peer, addr);
                }
            }
            MdnsEvent::Expired(expired) => {
                for (peer, addr) in expired {
                    log::info!("expired {} {}", peer, addr);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for MyBehaviour {
    fn inject_event(&mut self, message: KademliaEvent) {
        match message {
            KademliaEvent::OutboundQueryCompleted { .. } => {}
            _ => {}
        }
    }
}

pub struct P2PSharingAdapter {
    //pub swarm: SwarmType,
}

impl IP2PSharingPort for P2PSharingAdapter {}

impl P2PSharingAdapter {
    pub async fn new() -> Self {
        P2PSharingAdapter {}
    }

    pub async fn run(
        &mut self,
        domain: Box<dyn IP2PSharingPort>,
        local_key: identity::Keypair,
        local_peer_id: PeerId,
    ) -> Result<(), Box<dyn Error>> {
        // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol.
        let transport = development_transport(local_key).await?;

        // Create a swarm to manage peers and events.
        let mut swarm = {
            // Create a Kademlia behaviour.
            let store = MemoryStore::new(local_peer_id);
            let kademlia = Kademlia::new(local_peer_id, store);
            let mdns = Mdns::new(MdnsConfig::default()).await?;
            let behaviour = MyBehaviour {
                kademlia,
                mdns,
                test: 0
            };
            Swarm::new(transport, behaviour, local_peer_id)
        };

        if let Some(port) = std::env::args().nth(1) {
            // Listen on all interfaces and whatever port the OS assigns.
            let _listen = swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;
        }

        loop {
            // Poll the swarm
            match swarm.select_next_some().await {
                // NOTE: Initially, I was under the impression that I would be able to intercept
                // events from the ClientBehavior here. Yet, the below info! call is never reached.
                // This remains the case in libp2p example code that I have experimented with.
                SwarmEvent::Behaviour(e) => log::info!("idk: {:?}", e),
    
                _ => log::debug!("Some other event; this is handled specifically in the actual codebase, but for this question, all I really care about is the above behavior event."),
            };
        }

        /*loop {
            self.swarm.next_event().await;
        }*/
    }
}
