extern crate jack_bindings as jack;

use std::mem;
use std::ffi;

#[cfg(test)]
mod tests {
    use jack::JackOptions::JackNullOption;

    #[test]
    fn it_works() {
        /* open a client connection to the JACK server */
        let client_name = "hello jack rpi";
        let client = super::jack_client_open (client_name, JackNullOption).expect("Failed to open client");

        super::jack_client_close(client);
    }
}

pub fn jack_client_open(client_name : &str, options : jack::jack_options_t)
    -> Result<*mut jack::jack_client_t, jack::JackStatus> {
    unsafe {
        let mut status : jack::JackStatus = mem::uninitialized();
        let client_name = ffi::CString::new(client_name).unwrap();
        let client = jack::jack_client_open(client_name.as_ptr(), options, &mut status);

        if client.is_null() {
            return Err(status);
        }
        Ok(client)
    }
}

pub fn jack_client_close (client : *mut jack::jack_client_t) {
    unsafe {
        jack::jack_client_close(client);
    }
}
