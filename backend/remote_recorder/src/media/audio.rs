use std::os::raw::c_int;

type Callback = extern "C" fn(c_int);

unsafe extern "C" {
    fn intercept_audio(cb: Callback);
}

extern "C" fn rust_callback(value: c_int) {
    println!("Rust: Received value from C: {}", value);
}

#[cfg(target_os = "linux")]
pub fn create_audio_stream() -> Result<(), anyhow::Error> {
    unsafe {
        intercept_audio(rust_callback);
    }

    Ok(())
}
