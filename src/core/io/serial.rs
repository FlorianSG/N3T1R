use std::collections::HashMap;
use std::time::Instant;

use serialport;

use super::super::traits::IOHandler;
use super::super::MAXIMUM_DATA_LEN;
use super::super::SERIAL_RX_TIMEOUT;

pub struct SerialBackend {
	port_name: String,
	serial_port: Option<Box<dyn serialport::SerialPort>>,
}

impl SerialBackend {
	pub fn new(port_name: &str) -> Self {
		return Self { port_name: port_name.to_owned(), serial_port: None };
	}

	pub fn get_available_serial_ports() -> Result<HashMap<String, String>, String> {
		match serialport::available_ports() {
			Ok(ports) => {
				return Ok(HashMap::from_iter(ports.into_iter().map(|port| {
					let name: String = port.port_name;
					let description: String = match port.port_type {
						serialport::SerialPortType::UsbPort(info) => {
							match info.product {
								Some(string) => { string },
								None => { String::from("") },
							}
						},
						_ => { String::from("") },
					};
					return (name, description);
				})));
			},
			Err(e) => {
				return Err(e.to_string());
			},
		}
	}

	fn do_send(&mut self, data: &[u8]) -> Result<(), String> {
		let serial_port = self.serial_port.as_mut().unwrap();
		
		let mut buffer: Vec<u8> = Vec::with_capacity(data.len() + 1);
		buffer.push(data.len() as u8);
		buffer.extend(data);

		match serial_port.write_all(&buffer) {
			Ok(()) => {
				match serial_port.flush() {
					Ok(()) => {
						return Ok(());
					},
					Err(e) => {
						return Err(format!("Fail on flush: {}", e.to_string()));
					},
				}
			},
			Err(e) => {
				return Err(format!("Fail on write_all: {}", e.to_string()));
			},
		}
	}

	fn do_receive(&mut self) -> Result<Option<Vec<u8>>, String> {
		let serial_port = self.serial_port.as_mut().unwrap();
		let mut buffer: Vec<u8> = Vec::with_capacity(MAXIMUM_DATA_LEN);
		let mut expected_len: usize = 0;
		let mut last_received_ts: Instant = Instant::now();

		loop {
			match serial_port.bytes_to_read() {
				Ok(available_bytes) => {
					if available_bytes > 0 {
						// Receive size header
						if expected_len == 0 {
							let mut len_buf: Vec<u8> = vec![0];
							
							if let Err(e) = serial_port.read(&mut len_buf) {
								return Err(format!("Fail on read: {}", e.to_string()));
							}
							else {
								expected_len = len_buf[0] as usize;
							}
						}
						// Receive actual data
						else {
							let start = buffer.len();
							buffer.extend(vec![0; available_bytes as usize]);
							let stop = buffer.len();
							
							if let Err(e) = serial_port.read(&mut buffer[start..stop]) {
								return Err(format!("Fail on read: {}", e.to_string()));
							}

							last_received_ts = Instant::now();

							if buffer.len() >= expected_len {
								break;
							}
						}
					}
					else if expected_len == 0 || last_received_ts.elapsed() > SERIAL_RX_TIMEOUT {
						break;
					}
				},
				Err(e) => {
					return Err(format!("Fail on bytes_to_read: {}", e.to_string()));
				},
			}
		}

		if buffer.len() > 0 && buffer.len() == expected_len {
			return Ok(Some(buffer));
		}

		return Ok(None);
	}
}

impl IOHandler for SerialBackend {
	fn enable(&mut self) -> Result<(), String> {
		if self.serial_port.is_none() {
			println!("~~~NTiR-11~~~: Enabling serial port '{}' with SERIAL_RX_TIMEOUT set to '{:?}'", self.port_name, SERIAL_RX_TIMEOUT);

			match serialport::new(&self.port_name, 115200).open() {
				Ok(serial_port) => {
					self.serial_port = Some(serial_port);
				},
				Err(e) => {
					return Err(e.to_string());
				},
			}
		}
		return Ok(());
	}

	fn disable(&mut self) {
		if self.serial_port.is_some() {
			println!("~~~NTiR-11~~~: Disabling serial port");

			self.serial_port = None;
		}
	}

	fn send(&mut self, data: &[u8]) -> Result<(), String> {
		if self.serial_port.is_some() {
			return self.do_send(data);
		}
		return Ok(());
	}

	fn receive(&mut self) -> Result<Option<Vec<u8>>, String> {
		if self.serial_port.is_some() {
			return self.do_receive();
		}
		return Ok(None);
	}
}
