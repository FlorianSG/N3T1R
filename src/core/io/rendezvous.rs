use super::super::traits::IOHandler;

pub struct RendezVousBackend {
	room_name: String,
}

impl RendezVousBackend {
	pub fn new(room_name: &str) -> Self {
		return Self { room_name: room_name.to_owned() };
	}
}

impl IOHandler for RendezVousBackend {
	fn enable(&mut self) -> Result<(), String> {
		println!("Enabling rendez-vous protocol on room '{}'", self.room_name);
		return Ok(());
	}

	fn disable(&mut self) {
		println!("Disabling rendez-vous protocol");
	}

	fn send(&mut self, data: &[u8]) -> Result<(), String> {
		println!("Sending via rendez-vous on room '{}': {:?}", self.room_name, data);
		return Ok(());
	}

	fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
		println!("Trying to receive via rendez-vous on room '{}'...", self.room_name);
		return Ok(Some(format!("Hello back from '{}'", self.room_name).as_bytes().to_vec()));
	}
}
