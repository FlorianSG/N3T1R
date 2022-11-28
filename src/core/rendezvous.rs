use std::env;
use std::fs;
use std::path;
use std::process;


fn get_rendezvous_folder() -> path::PathBuf {
	const RDV_DIR: &str = "ntir11-rdv";
	return env::temp_dir().join(RDV_DIR);
}

pub struct RendezVous {
	rendezvous_directory: path::PathBuf,
	advertisement_filename: path::PathBuf,
}

impl RendezVous {
	fn advertise(&self, local_udp_port: u16) -> std::io::Result<()> {
		if !self.rendezvous_directory.exists() {
			fs::create_dir(&self.rendezvous_directory) ?;
		}

		fs::write(&self.advertisement_filename, local_udp_port.to_string()) ?;
		return Ok(());
	}

	fn look_for_peer_advertisement(&self) -> Option<u16> {
		if let Ok(entries) = fs::read_dir(&self.rendezvous_directory) {	// For each entry in the directory...
			for entry in entries {										//
				if let Ok(entry) = entry {								//
					if let Ok(file_type) = entry.file_type() {			//
						let filename = entry.path();
						
						if file_type.is_file() && filename != self.advertisement_filename {
							if let Ok(content) = fs::read(&filename) {
								let _ = fs::remove_file(&filename);

								return match String::from_utf8_lossy(&content).parse() {
									Ok(udp_port) => Some(udp_port),
									Err(_) => None,
								}
							}
						}
					}
				}
			}
		}
		return None;
	}

	pub fn new(local_udp_port: u16) -> std::io::Result<RendezVous> {
		let rendezvous_directory = get_rendezvous_folder();
		let advertisement_filename = rendezvous_directory.join(process::id().to_string());
		let rendezvous = RendezVous { rendezvous_directory, advertisement_filename };

		rendezvous.advertise(local_udp_port) ?;
		return Ok(rendezvous);
	}

	pub fn try_to_meet(&self) -> Option<u16> {
		if let Some(peer_udp_port) = self.look_for_peer_advertisement() {
			return Some(peer_udp_port);
		}

		return None;
	}

	pub fn cleanup(&self) {
		if self.advertisement_filename.exists() {
			let _ = fs::remove_file(&self.advertisement_filename);
		}
	}
}

impl Drop for RendezVous {
	fn drop(&mut self) {
	   self.cleanup();
	}
}