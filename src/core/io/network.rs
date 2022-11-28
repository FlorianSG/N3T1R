use super::super::traits::IOHandler;

pub struct NetworkBackend {
	source_port: u16,
	destination_host: String,
	destination_port: u16,
}

impl NetworkBackend {
	pub fn new(source_port: u16, destination_host: &str, destination_port: u16) -> Self {
		return Self { source_port, destination_host: destination_host.to_owned(), destination_port };
	}
}

impl IOHandler for NetworkBackend {
	fn enable(&mut self) -> Result<(), String> {
		println!("Enabling network protocol from udp:{} to {}:{}", self.source_port, self.destination_host, self.destination_port);
		return Ok(());
	}

	fn disable(&mut self) {
		println!("Disabling network protocol");
	}

	fn send(&mut self, data: &[u8]) -> Result<(), String> {
		println!("Sending via network to '{}:{}': {:?}", self.destination_host, self.destination_port, data);
		return Ok(());
	}

	fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
		println!("Trying to receive via network from '{}:{}'...", self.destination_host, self.destination_port);
		return Ok(Some(format!("Hello back from '{}:{}'", self.destination_host, self.destination_port).as_bytes().to_vec()));
	}
}
