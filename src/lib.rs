extern crate libc;
extern crate jack_bindings as jack;

use std::mem;
use std::ffi;

#[cfg(test)]
mod tests {

    use super::*;
    use jack::JackOptions::JackNullOption;
    use std::ptr;

    //    jack_port_t *input_port;
    //    jack_port_t *output_port;
    //    jack_client_t *client;
    //
    //    /**
    //     * The process callback for this JACK application is called in a
    //     * special realtime thread once for each audio cycle.
    //     *
    //     * This client does nothing more than copy data from its input
    //     * port to its output port. It will exit when stopped by
    //     * the user (e.g. using Ctrl-C on a unix-ish operating system)
    //     */
    //    int
    //    process (jack_nframes_t nframes, void *arg)
    //    {
    //        jack_default_audio_sample_t *in, *out;
    //
    //        in = jack_port_get_buffer (input_port, nframes);
    //        out = jack_port_get_buffer (output_port, nframes);
    //        memcpy (out, in,
    //        sizeof (jack_default_audio_sample_t) * nframes);
    //
    //        return 0;
    //    }

    unsafe extern "C" fn process(nframes: jack::jack_nframes_t, arg: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {
        let mut _input_port : jack::jack_port_t = mem::uninitialized();
        let mut _output_port : jack::jack_port_t = mem::uninitialized();
        let mut in_buffer = jack::jack_port_get_buffer(&mut _input_port, nframes) as *mut ::libc::c_void;
        let mut out_buffer = jack::jack_port_get_buffer(&mut _output_port, nframes) as *mut ::libc::c_void;
        let buffer_size = nframes as usize * std::mem::size_of::<jack::jack_default_audio_sample_t>();
        ::libc::memcpy(out_buffer, in_buffer, buffer_size);
        0
    }

    #[test]
    fn it_works() {
        // open a client connection to the JACK server
        let client = super::jack_client_open("hello jack rpi", JackNullOption).expect("Failed to open client");

        println!("Opened jack client");

        // tell the JACK server to call `process()' whenever
        // there is work to be done.
        super::jack_set_process_callback(client, Some(process), ptr::null_mut());

        super::jack_client_close(client);
    }
}

pub fn jack_client_open(client_name: &str, options: jack::jack_options_t)
                        -> Result<*mut jack::jack_client_t, jack::JackStatus> {
    unsafe {
        let mut status: jack::JackStatus = mem::uninitialized();
        let client_name = ffi::CString::new(client_name).unwrap();
        let client = jack::jack_client_open(client_name.as_ptr(), options, &mut status);

        if client.is_null() {
            return Err(status);
        }
        Ok(client)
    }
}

pub fn jack_client_close(client: *mut jack::jack_client_t) {
    unsafe {
        jack::jack_client_close(client);
    }
}

pub fn jack_set_process_callback(client: *mut jack::jack_client_t,
                                 process_callback: jack::JackProcessCallback,
                                 arg: *mut ::std::os::raw::c_void) -> Result<(), ::std::os::raw::c_int> {
    unsafe {
        let result = jack::jack_set_process_callback(client, process_callback, arg);
        match result {
            0 => { Ok(()) }
            _ => { Err(result) }
        }
    }
}
