use tokio::handler::Event;
use tokio::network::NetworkEvent;

use super::{NodeHandler, ExternalMessage, NodeTimeout};

impl NodeHandler
{
    /// TODO
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Network(network) => self.handle_network_event(network),
            Event::Timeout(timeout) => self.handle_timeout(timeout),
            Event::Api(api) => self.handle_api_event(api),
        }
    }

    fn handle_network_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::PeerConnected(peer, connect) => self.handle_connected(peer, connect),
            NetworkEvent::PeerDisconnected(peer) => self.handle_disconnected(peer),
            NetworkEvent::MessageReceived(peer, raw) => self.handle_message(peer, raw),
        }
    }

    fn handle_api_event(&mut self, event: ExternalMessage) {
        match event {
            ExternalMessage::Transaction(tx) => {
                self.handle_incoming_tx(tx);
            }
            ExternalMessage::PeerAdd(address) => {
                info!("Send Connect message to {}", address);
                self.connect(&address);
            }
        }
    }

    fn handle_timeout(&mut self, timeout: NodeTimeout) {
        match timeout {
            NodeTimeout::Round(height, round) => self.handle_round_timeout(height, round),
            NodeTimeout::Request(data, peer) => self.handle_request_timeout(data, peer),
            NodeTimeout::Status(height) => self.handle_status_timeout(height),
            NodeTimeout::PeerExchange => self.handle_peer_exchange_timeout(),
            NodeTimeout::UpdateApiState => self.handle_update_api_state_timeout(),
            NodeTimeout::Propose(height, round) => self.handle_propose_timeout(height, round),
        }
    }
}