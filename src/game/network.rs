use crate::game::event::GameEvent;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}

#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub id: u64,
    pub event: GameEvent,
    pub delay: f32,
    pub from_server: bool,
}

pub struct NetworkSimulator {
    pub connection_state: ConnectionState,
    pub messages: VecDeque<NetworkMessage>,
    pub latency_ms: f32,
    pub packet_loss: f32,
    pub message_counter: u64,
    pub server_events: Vec<GameEvent>,
    pub connect_timer: f32,
    pub connect_duration: f32,
    pub disconnect_reason: Option<String>,
}

impl Default for NetworkSimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkSimulator {
    pub fn new() -> Self {
        Self {
            connection_state: ConnectionState::Disconnected,
            messages: VecDeque::new(),
            latency_ms: 100.0, // Simulated 100ms latency
            packet_loss: 0.01, // 1% packet loss
            message_counter: 0,
            server_events: Vec::new(),
            connect_timer: 0.0,
            connect_duration: 2.0, // 2 second connection time
            disconnect_reason: None,
        }
    }

    pub fn connect(&mut self) {
        if self.connection_state == ConnectionState::Disconnected {
            self.connection_state = ConnectionState::Connecting;
            self.connect_timer = 0.0;
            log::info!("Connecting to server...");
        }
    }

    pub fn disconnect(&mut self, reason: &str) {
        self.connection_state = ConnectionState::Disconnecting;
        self.disconnect_reason = Some(reason.to_string());
        log::info!("Disconnecting: {}", reason);
    }

    pub fn update(&mut self, dt: f32) {
        match self.connection_state {
            ConnectionState::Connecting => {
                self.connect_timer += dt;
                if self.connect_timer >= self.connect_duration {
                    self.connection_state = ConnectionState::Connected;
                    log::info!("Connected to server!");
                    // Send welcome event
                    self.server_events.push(GameEvent::AreaTransition {
                        from: "title".to_string(),
                        to: "mac_anu".to_string(),
                    });
                }
            }
            ConnectionState::Disconnecting => {
                self.connection_state = ConnectionState::Disconnected;
                self.messages.clear();
                log::info!("Disconnected from server");
            }
            _ => {}
        }

        // Process message delays
        let mut ready_messages = Vec::new();
        self.messages.retain(|msg| {
            if msg.delay <= 0.0 {
                ready_messages.push(msg.clone());
                false
            } else {
                true
            }
        });

        // Deliver ready messages
        for msg in ready_messages {
            if rand::random::<f32>() > self.packet_loss && msg.from_server {
                self.server_events.push(msg.event);
            }
            // Client messages would be handled here
        }
    }

    pub fn send_event(&mut self, event: GameEvent) {
        if self.connection_state != ConnectionState::Connected {
            log::warn!("Cannot send event: not connected");
            return;
        }

        self.message_counter += 1;
        let delay = self.latency_ms / 1000.0;

        self.messages.push_back(NetworkMessage {
            id: self.message_counter,
            event,
            delay,
            from_server: false,
        });
    }

    pub fn receive_server_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.server_events)
    }

    pub fn is_connected(&self) -> bool {
        self.connection_state == ConnectionState::Connected
    }

    pub fn is_connecting(&self) -> bool {
        self.connection_state == ConnectionState::Connecting
    }

    pub fn connection_progress(&self) -> f32 {
        if self.connection_state == ConnectionState::Connecting {
            (self.connect_timer / self.connect_duration).min(1.0)
        } else {
            1.0
        }
    }

    pub fn set_latency(&mut self, ms: f32) {
        self.latency_ms = ms.clamp(0.0, 5000.0);
    }

    pub fn set_packet_loss(&mut self, loss: f32) {
        self.packet_loss = loss.clamp(0.0, 1.0);
    }

    pub fn simulate_disconnect(&mut self) {
        self.disconnect("Connection lost");
        self.connection_state = ConnectionState::Disconnected;
        self.messages.clear();
        log::warn!("Simulated network disconnect");
    }

    pub fn message_queue_size(&self) -> usize {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let net = NetworkSimulator::new();
        assert_eq!(net.connection_state, ConnectionState::Disconnected);
        assert!((net.latency_ms - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_connect() {
        let mut net = NetworkSimulator::new();
        net.connect();
        assert_eq!(net.connection_state, ConnectionState::Connecting);
    }

    #[test]
    fn test_connection_progress() {
        let mut net = NetworkSimulator::new();
        net.connect();
        net.update(1.0);
        assert!((net.connection_progress() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_full_connection() {
        let mut net = NetworkSimulator::new();
        net.connect();
        net.update(2.5);
        assert!(net.is_connected());
    }

    #[test]
    fn test_disconnect() {
        let mut net = NetworkSimulator::new();
        net.connect();
        net.update(2.5);
        assert!(net.is_connected());

        net.disconnect("User quit");
        net.update(0.1);
        assert_eq!(net.connection_state, ConnectionState::Disconnected);
    }

    #[test]
    fn test_send_event() {
        let mut net = NetworkSimulator::new();
        net.connect();
        net.update(2.5);

        net.send_event(GameEvent::PlayerDamaged {
            amount: 10,
            source: "Goblin".to_string(),
        });
        assert_eq!(net.message_queue_size(), 1);
    }

    #[test]
    fn test_send_event_not_connected() {
        let mut net = NetworkSimulator::new();
        net.send_event(GameEvent::LevelUp { new_level: 2 });
        assert_eq!(net.message_queue_size(), 0);
    }

    #[test]
    fn test_receive_server_events() {
        let mut net = NetworkSimulator::new();
        net.connect();
        net.update(2.5);

        let events = net.receive_server_events();
        assert_eq!(events.len(), 1); // Welcome event
        assert!(matches!(events[0], GameEvent::AreaTransition { .. }));
    }

    #[test]
    fn test_latency_config() {
        let mut net = NetworkSimulator::new();
        net.set_latency(250.0);
        assert!((net.latency_ms - 250.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_packet_loss_config() {
        let mut net = NetworkSimulator::new();
        net.set_packet_loss(0.5);
        assert!((net.packet_loss - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_simulate_disconnect() {
        let mut net = NetworkSimulator::new();
        net.connect();
        net.update(2.5);
        net.simulate_disconnect();
        assert_eq!(net.connection_state, ConnectionState::Disconnected);
        assert_eq!(net.disconnect_reason, Some("Connection lost".to_string()));
    }
}
