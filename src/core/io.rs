use super::traits::IOHandler;

pub mod serial;
pub mod rendezvous;
pub mod network;

pub enum IOBackend {
	Disabled,
	Serial(serial::SerialBackend),
	RendezVous(rendezvous::RendezVousBackend),
	Network(network::NetworkBackend),
}

impl IOBackend {
	pub fn new() -> Self {
		return Self::Disabled;
	}

	pub fn new_serial(port_name: &str) -> Self {
		return Self::Serial(serial::SerialBackend::new(port_name));
	}

	pub fn new_rendezvous(room_name: &str) -> Self {
		return Self::RendezVous(rendezvous::RendezVousBackend::new(room_name));
	}

	pub fn new_network(source_port: u16, destination_host: &str, destination_port: u16) -> Self {
		return Self::Network(network::NetworkBackend::new(source_port, destination_host, destination_port));
	}
}

impl IOHandler for IOBackend {
	fn enable(&mut self) -> Result<(), String> {
		return match self {
			Self::Disabled => { Ok(()) },
			Self::Serial(backend) => { backend.enable() },
			Self::RendezVous(backend) => { backend.enable() },
			Self::Network(backend) => { backend.enable() },
		};
	}

	fn disable(&mut self) {
		match self {
			Self::Disabled => {},
			Self::Serial(backend) => { backend.disable() },
			Self::RendezVous(backend) => { backend.disable() },
			Self::Network(backend) => { backend.disable() },
		};
	}

	fn send(&mut self, data: &[u8]) -> Result<(), String> {
		return match self {
			Self::Disabled => { Ok(()) },
			Self::Serial(backend) => { backend.send(data) },
			Self::RendezVous(backend) => { backend.send(data) },
			Self::Network(backend) => { backend.send(data) },
		};
	}

	fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
		return match self {
			Self::Disabled => { Ok(None) },
			Self::Serial(backend) => { backend.receive() },
			Self::RendezVous(backend) => { backend.receive() },
			Self::Network(backend) => { backend.receive() },
		};
	}
}