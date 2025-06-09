use pipewire::{
    spa::sys::spa_audio_info,
    sys::{pw_buffer, pw_stream},
};

#[repr(C)]
struct CallBackArgs {
    stream: *const pw_stream,
    buffer: *const pw_buffer,
    format: *const spa_audio_info,
}

type Callback = unsafe extern "C" fn(*const CallBackArgs);

unsafe extern "C" {
    fn intercept_audio(cb: Callback);
}

unsafe extern "C" fn rust_callback(value: *const CallBackArgs) {
    std::thread::sleep(std::time::Duration::from_millis(200));
    let len = unsafe {
        value
            .as_ref()
            .and_then(|x| x.format.as_ref())
            .map(|x| x.media_type)
    };

    println!("Rust: Received value from C: {}", len.unwrap_or(0));
}

#[cfg(target_os = "linux")]
pub fn create_audio_stream() -> Result<(), anyhow::Error> {
    unsafe {
        intercept_audio(rust_callback);
    }

    Ok(())
}
