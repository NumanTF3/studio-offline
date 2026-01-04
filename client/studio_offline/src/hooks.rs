use std::{ffi::CStr, fs, ptr};

pub type FromComponentsFn = extern "C" fn(
    res: *mut u128,
    schema: usize,
    host: usize,
    path: usize,
    query: usize,
    fragment: usize,
);
pub type TrustCheckFn = extern "C" fn(str1: *const i8, a2: i8, a3: i8) -> *mut u64;
pub type HttpRequestNotTrustedFn = extern "C" fn(a1: *mut usize, a2: usize) -> *mut i8;

pub static mut ORIGINAL: Option<FromComponentsFn> = None;
pub static mut OG_TC: Option<TrustCheckFn> = None;
pub static mut ORIGINAL_HTTP_NT: Option<HttpRequestNotTrustedFn> = None;

/// Reads IP and port from files
fn read_ip_port() -> (String, String) {
    let ip = fs::read_to_string("../../Settings/ip.txt")
        .unwrap_or_else(|_| "127.0.0.1".to_string())
        .trim()
        .to_string();

    let port = fs::read_to_string("../../Settings/clientport.txt")
        .unwrap_or_else(|_| "80".to_string())
        .trim()
        .to_string();

    (ip, port)
}

pub extern "C" fn hook_test(
    res: *mut u128,
    schema: usize,
    host: usize,
    path: usize,
    query: usize,
    fragment: usize,
) {
    unsafe {
        let (ip, port) = read_ip_port();
        let host_str = format!("{}:{}", ip, port);
        let scheme_str = "http";

        // Allocate host C string
        let host_bytes = host_str.clone() + "\0";
        let host_ptr = host_bytes.as_ptr() as usize;
        *(host as *mut usize) = host_ptr;
        *(host as *mut usize).add(1) = host_bytes.len() - 1;

        // Allocate scheme C string
        let scheme_bytes = scheme_str.to_string() + "\0";
        let scheme_ptr = scheme_bytes.as_ptr() as usize;
        *(schema as *mut usize) = scheme_ptr;
        *(schema as *mut usize).add(1) = scheme_bytes.len() - 1;

        if let Some(orig) = ORIGINAL {
            orig(res, schema, host, path, query, fragment);
        }
    }
}

pub extern "C" fn trustcheck_hook(str1: *const i8, a2: i8, a3: i8) -> *mut u64 {
    unsafe {
        let url = CStr::from_ptr(str1).to_string_lossy();
        let (ip, port) = read_ip_port();
        let replacement = format!("http://{}:{}", ip, port) + "\0";

        if url.contains("http://localhost") && a3 == 0 {
            if let Some(orig) = OG_TC {
                return orig(replacement.as_ptr() as *const i8, a2, a3);
            }
        }

        if let Some(orig) = OG_TC {
            return orig(str1, a2, a3);
        }

        ptr::null_mut()
    }
}

pub extern "C" fn nottrusted_hook(_a1: *mut usize, _a2: usize) -> *mut i8 {
    b"1\0".as_ptr() as *mut i8
}
