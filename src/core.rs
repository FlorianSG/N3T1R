use std::collections::HashMap;
use std::time::Duration;

mod io;
use self::io::IOBackend;
use self::io::serial::SerialBackend;

// To-Do: find a more "realistic" value
pub const MAXIMUM_DATA_LEN: usize = 255;
pub const SERIAL_RX_TIMEOUT: Duration = Duration::from_millis(10);

pub mod traits {
	pub trait IOHandler {
		fn enable(&mut self) -> Result<(), String> {
			unimplemented!();
		}

		fn disable(&mut self) {
			unimplemented!();
		}

		fn send(&mut self, _data: &[u8]) -> Result<(), String> {
			unimplemented!();
		}

		fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
			unimplemented!();
		}
	}
}

use self::traits::IOHandler;

pub struct IRCommunicationHandler {
	backend: IOBackend,
}

impl IRCommunicationHandler {
	pub fn new() -> Self {
		return Self { backend: IOBackend::new() };
	}

	pub fn get_available_serial_ports() -> Result<HashMap<String, String>, String> {
		return SerialBackend::get_available_serial_ports();
	}

	pub fn get_available_rooms() -> Result<Vec<String>, String> {
		//return Ok(Vec::new());
		return Ok(vec!["Room A".to_string(), "Room B".to_string(), "Room C".to_string()]);
	}

	pub fn select_serial_backend(&mut self, port_name: &str) {
		self.backend = IOBackend::new_serial(port_name);
	}

	pub fn select_rendezvous_backend(&mut self, room_name: &str) {
		self.backend = IOBackend::new_rendezvous(room_name);
	}

	pub fn select_network_backend(&mut self, source_port: u16, destination_host: &str, destination_port: u16) {
		self.backend = IOBackend::new_network(source_port, destination_host, destination_port);
	}
}

impl IOHandler for IRCommunicationHandler {
	fn enable(&mut self) -> Result<(), String> {
		return self.backend.enable();
	}

	fn disable(&mut self) {
		self.backend.disable();
	}

	fn send(&mut self, data: &[u8]) -> Result<(), String> {
		println!("[core.rs]::IRCommunicationHandler.send({:?})", data);
		return self.backend.send(data);
	}

	fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
		let ret = self.backend.receive();
		if ret.is_ok() {
			if let Some(data) = ret.as_ref().unwrap() {
				println!("[core.rs]::IRCommunicationHandler.receive() -> {:?}", data);
			}
		}
		return ret;
	}
}