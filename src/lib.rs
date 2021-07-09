#![feature(rustc_private)]

extern crate libc;

use std::io::{Write, Read};

fn my_mkfifo(path: &str) {
    let mkfifo_res = unsafe {
        libc::mkfifo(path.as_ptr() as _, libc::S_IREAD | libc::S_IWRITE)
    };
    let errno = unsafe { *libc::__error() };
    dbg!(errno, mkfifo_res);
    if mkfifo_res == -1 {
        let buf: [u8; 128] = [0u8; 128];
        unsafe {
            libc::strerror_r(errno, buf.as_ptr() as _, 128);
        }
        let err_msg = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
        dbg!(err_msg);
        panic!("system call failed");
    }
}

const PATH: &str = "/Users/bytedance/rust_playground/pipe_files/my_pipe_0";

extern "C" {
    fn rand() -> i32;
}

// fn random() -> String {
//     format!("/Users/bytedance/rust_playground/pipe_files/my_pipe_{}_{}", unsafe{ rand() }, unsafe{ rand() })
// }

fn to_cstr(path: &str) -> String {
    format!("{}\0", path)
}

#[test]
fn sender_process() {

    // let path = random();
    //
    // println!("random path: {}", &path);
    if std::path::Path::new(PATH).exists() {
        std::fs::remove_file(PATH).unwrap();
    }

    my_mkfifo(&to_cstr(PATH));
    let mut pipe = std::fs::OpenOptions::new().write(true).open(PATH).unwrap();
    let msg = b"hello\n";
    pipe.write_all(msg).unwrap();
}

#[test]
fn receiver_process() {
    let mut pipe = std::fs::File::open(PATH).unwrap();
    let mut buf = String::new();
    pipe.read_to_string(&mut buf);
    println!("read: {}", buf);
}


