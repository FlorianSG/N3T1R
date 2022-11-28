#![allow(dead_code)]

mod core;
use crate::core::IRCommunicationHandler;
use crate::core::traits::IOHandler;

fn main() {
	println!("=== NTiR-11 § Test App ===");

	println!("{:?}\n", IRCommunicationHandler::get_available_serial_ports().unwrap());
	
	println!("{:?}\n", IRCommunicationHandler::get_available_serial_ports().unwrap()["COM7"].as_bytes());

	let mut handler = IRCommunicationHandler::new();
	handler.select_serial_backend("COM7");
	handler.enable().expect("Ooops");

	loop {
		if let Some(data) = handler.receive().expect("Receive crashed") {
			println!("Received: {:?}", data);
			//handler.send("Bonjour, le, monde !".as_bytes()).expect("Send crashed");
		}
		//std::thread::sleep(std::time::Duration::from_secs(1));
	}

	//handler.disable();
}
