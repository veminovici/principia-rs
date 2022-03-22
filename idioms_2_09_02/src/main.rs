// Accepting Strings
// https://rust-unofficial.github.io/patterns/idioms/ffi/accepting-strings.html

// CStr

// pub mod unsafe_module {
//     #[no_mangle]
//     pub unsafe extern "C" fn mylib_log(msg: *const libc::c_char, level: libc::c_int) {
//         let level: crate::LogLevel = match level {};

//         let msg_str: &str = match std::ffi::CStr::from_ptr(msg).to_str() {
//             Ok(s) => s,
//             Err(e) => {
//                 crate::log_err("FFI string conversion failed");
//                 returnl
//             }
//         };

//         crate::log(msg_str, level);
//     }
// }

fn main() {
    println!("Hello, world!");
}
