mod domain;
mod p2p;
mod ports;

use domain::TestDomain;
use futures::prelude::*;
use libp2p::identity::Keypair;
use libp2p::PeerId;
use p2p::P2PSharingAdapter;
use std::error::Error;

use std::cell::RefCell;
use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random key for ourselves.
    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    let p2p_adapter = P2PSharingAdapter::new().await;
    let p2p_adapter = Rc::new(RefCell::new(p2p_adapter));
    let domain = Box::new(TestDomain::new(p2p_adapter.clone()));

    p2p_adapter
        .borrow_mut()
        .run(domain, keypair, peer_id)
        .await?;

    Ok(())
}
