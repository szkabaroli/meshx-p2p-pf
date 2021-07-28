# Pathfinder for MeshX P2P Sharing

This is a pathfinder project for integrating P2P resource sharing in MeshX.

## Overview

MeshX built using a [hexagonal arhitecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)) with ports and adapters. Ports are just interfaces (or in trait-s in rust) while the adapters are implementing these ports.

For P2P networking we are using the awesome [libp2p](https://github.com/libp2p/rust-libp2p) crate.

## Specification

### IP2PSharingPort

Primary adapter. Used to get notifications from the underlying P2P network in the domain layer on resource sharing events.

- `fn on_inbound_object(from_peer_id, object)` - Incoming event when a resource shared with the local peer.
  
- `fn on_inbound_invoke(from_peer_id, flow_name, flow_input)` - Called when the remote peer sends a flow trigger to the local peer.

### IP2PSharingStore
Secondary adapter. Used to register what objects are available for which participant in the network. 

- `fn add_outbound_object(to_peer_id, object)` - Basicly shares a local object with a remote one. The remote peer should recive a notification using _`inbound_object(...)`_ event.

- `fn remove_outbound_object(to_peer_id, object_name)` - Remove an object from the outbound sharing list which effectively means to object is no longer shared.

- `fn list_outbound_objects(to_peer_id) returns object[]` - Lists all the resources shared with _`to_peer_id`_.

- `fn outbound_invoke(to_peer_id, flow_name, flow_input)` - Invoke a remote flow by it's name.

### TestDomain

This is the place where the main business logic resides, but for this pathfinder project we are just testing the the P2P Sharing features here. We are using the ports here to interact with the outer layer.