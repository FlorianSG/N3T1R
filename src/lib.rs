use std::ffi::{CStr, CString};
use std::slice;

use libc::{c_char, size_t};

mod core;
use crate::core::IRCommunicationHandler;
use crate::core::traits::IOHandler;
use crate::core::MAXIMUM_DATA_LEN;

fn into_ptr<T>(instance: T) -> *mut T {
	return Box::into_raw(Box::new(instance));
}

fn from_ptr<'a, T>(pointer: *mut T) -> Option<&'a mut T> {
	return unsafe { pointer.as_mut() };
}

fn from_c_str<'a>(ptr: *const c_char) -> Option<&'a str> {
	if ptr.is_null() {
		return None;
	}
	else {
		let c_str = unsafe { CStr::from_ptr(ptr) };
		if let Ok(r_str) = c_str.to_str() {
			return Some(r_str);
		}
		else {
			return None;
		}
	}
}

fn into_c_str(string: &str) -> *mut c_char {
	let c_str = CString::new(string).unwrap();
	return c_str.into_raw();
}

// === Public API ===

// String FFI
#[no_mangle]
pub extern "C" fn n3t1r_str_free(str_ptr: *mut c_char) {
	if str_ptr.is_null() {
		panic!("n3t1r_str_free: null str_ptr");
	}
	else {
		unsafe { drop(CString::from_raw(str_ptr)) };
	}
}

// Result<(), String> FFI:
#[no_mangle]
pub extern "C" fn n3t1r_result_is_error(result_ptr: *mut Result<(), String>) -> bool {
	if let Some(result) = from_ptr(result_ptr) {
		return result.is_err();
	}
	else {
		panic!("n3t1r_result_is_error: null result_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_result_get_error_message(result_ptr: *mut Result<(), String>) -> *mut c_char {
	if let Some(result) = from_ptr(result_ptr) {
		return into_c_str(match result {
			Err(error) => { error },
			Ok(()) => { "" },
		});
	}
	else {
		panic!("n3t1r_result_get_error_message: null result_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_result_free(result_ptr: *mut Result<(), String>) {
	if let None = from_ptr(result_ptr) {
		panic!("n3t1r_result_free: null result_ptr");
	}
}

// Vec<String> FFI
#[no_mangle]
pub extern "C" fn n3t1r_vec_string_new() -> *mut Vec<String> {
	return into_ptr(Vec::<String>::new());
}

#[no_mangle]
pub extern "C" fn n3t1r_vec_string_free(vector_ptr: *mut Vec<String>) {
	if let None = from_ptr(vector_ptr) {
		panic!("n3t1r_vec_string_free: null vector_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_vec_string_len(vector_ptr: *mut Vec<String>) -> size_t {
	if let Some(vector) = from_ptr(vector_ptr) {
		return vector.len() as size_t;
	}
	else {
		panic!("n3t1r_vec_string_len: null vector_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_vec_string_get(vector_ptr: *mut Vec<String>, index: size_t) -> *mut c_char {
	if let Some(vector) = from_ptr(vector_ptr) {
		if index < vector.len() {
			return into_c_str(&vector[index]);
		}
		else {
			panic!("n3t1r_vec_string_get: index out of range ({}), vector length is {}", index, vector.len());
		}
	}
	else {
		panic!("n3t1r_vec_string_get: null vector_ptr");
	}
}

// IRCommunicationHandler static FFI
#[no_mangle]
pub extern "C" fn n3t1r_get_available_serial_ports(names_vector_ptr: *mut Vec<String>, descriptions_vector_ptr: *mut Vec<String>) -> *mut Result<(), String> {
	if let Some(names_vector) = from_ptr(names_vector_ptr) {
		match IRCommunicationHandler::get_available_serial_ports() {
			Err(error) => {
				return into_ptr(Err(error));
			},
			Ok(serial_ports) => {
				names_vector.extend(serial_ports.clone().into_keys());
				
				if let Some(descriptions_vector) = from_ptr(descriptions_vector_ptr) {
					for name in names_vector {
						descriptions_vector.push(serial_ports[name].clone());
					}
				}

				return into_ptr(Ok(()));
			},
		}
	}
	else {
		panic!("n3t1r_get_available_serial_ports: null names_vector_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_get_available_rooms(vector_ptr: *mut Vec<String>) -> *mut Result<(), String> {
	if let Some(vector) = from_ptr(vector_ptr) {
		match IRCommunicationHandler::get_available_rooms() {
			Err(error) => {
				return into_ptr(Err(error));
			},
			Ok(rooms) => {
				vector.extend(rooms.iter().cloned());
				return into_ptr(Ok(()));
			},
		}
	}
	else {
		panic!("n3t1r_get_available_rooms: null vector_ptr");
	}
}

// IRCommunicationHandler instance FFI
#[no_mangle]
pub extern "C" fn n3t1r_irch_new() -> *mut IRCommunicationHandler {
	return into_ptr(IRCommunicationHandler::new());
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_free(instance_ptr: *mut IRCommunicationHandler) {
	if let None = from_ptr(instance_ptr) {
		panic!("n3t1r_irch_free: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_select_serial_backend(instance_ptr: *mut IRCommunicationHandler, port_name_ptr: *const c_char) {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		if let Some(port_name) = from_c_str(port_name_ptr) {
			communication_handler.select_serial_backend(port_name);
		}
		else {
			panic!("n3t1r_irch_select_serial_backend: null port_name_ptr");
		}
	}
	else {
		panic!("n3t1r_irch_select_serial_backend: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_select_rendezvous_backend(instance_ptr: *mut IRCommunicationHandler, room_name_ptr: *const c_char) {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		if let Some(room_name) = from_c_str(room_name_ptr) {
			communication_handler.select_rendezvous_backend(room_name);
		}
		else {
			panic!("n3t1r_irch_select_rendezvous_backend: null room_name_ptr");
		}
	}
	else {
		panic!("n3t1r_irch_select_rendezvous_backend: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_select_network_backend(instance_ptr: *mut IRCommunicationHandler, source_port: u16, destination_host_ptr: *const c_char, destination_port: u16) {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		if let Some(destination_host) = from_c_str(destination_host_ptr) {
			communication_handler.select_network_backend(source_port, destination_host, destination_port);
		}
		else {
			panic!("n3t1r_irch_select_network_backend: null destination_host_ptr");
		}
	}
	else {
		panic!("n3t1r_irch_select_network_backend: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_enable(instance_ptr: *mut IRCommunicationHandler) -> *mut Result<(), String> {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		return into_ptr(communication_handler.enable());
	}
	else {
		panic!("n3t1r_irch_enable: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_disable(instance_ptr: *mut IRCommunicationHandler) {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		communication_handler.disable();
	}
	else {
		panic!("n3t1r_irch_disable: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_send(instance_ptr: *mut IRCommunicationHandler, data_ptr: *const u8, data_len: size_t) -> *mut Result<(), String> {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		if data_ptr.is_null() {
			panic!("n3t1r_irch_send: null data_ptr");
		}

		let data = unsafe { slice::from_raw_parts(data_ptr, data_len as usize) };		
		return into_ptr(communication_handler.send(data));
	}
	else {
		panic!("n3t1r_irch_send: null instance_ptr");
	}
}

#[no_mangle]
pub extern "C" fn n3t1r_irch_receive(instance_ptr: *mut IRCommunicationHandler, data_ptr: *mut u8, data_len_ptr: *mut size_t) -> *mut Result<(), String> {
	if let Some(communication_handler) = from_ptr(instance_ptr) {
		if data_ptr.is_null() {
			panic!("n3t1r_irch_receive: null data_ptr");
		}
		if data_len_ptr.is_null() {
			panic!("n3t1r_irch_receive: null data_len_ptr");
		}

		let data_len = unsafe { &mut *data_len_ptr };
		let max_data_len = *data_len;

		if max_data_len < MAXIMUM_DATA_LEN {
			panic!("n3t1r_irch_receive: max_data_len too low ({}), should be at least {}", max_data_len, MAXIMUM_DATA_LEN);
		}

		let data_target = unsafe { slice::from_raw_parts_mut(data_ptr, max_data_len as usize) };
		match communication_handler.receive() {
			Err(error) => {
				*data_len = 0;
				return into_ptr(Err(error));
			},
			Ok(result) => {
				if let Some(data) = result {
					data_target[..data.len()].clone_from_slice(&data);
					*data_len = data.len() as size_t;
				}
				else {
					*data_len = 0;
				}
				return into_ptr(Ok(()));
			}
		}
	}
	else {
		panic!("n3t1r_irch_receive: null instance_ptr");
	}
}
