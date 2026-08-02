use std::ffi::{CStr, c_void};

use crate::system::{
    devices::{FrameBuffer, Ram},
    register::Regs,
};

pub fn print_int(regs: Regs) {
    print!("{}", regs[10] as i32);
}

pub fn read_int(mut regs: Regs) {
    let mut n: i32 = 0;
    unsafe {
        libc::scanf(c"%d".as_ptr(), &mut n);
    }
    regs[10] = n as u32;
}

pub fn print_string(regs: Regs, ram: &Ram) {
    let addr = regs[10] as usize;
    let data = ram.data.read().unwrap();

    print!(
        "{}",
        CStr::from_bytes_until_nul(&data[addr..])
            .unwrap()
            .to_str()
            .unwrap()
    );
}

pub fn print_char(regs: Regs) {
    print!("{}", regs[10] as u8 as char)
}

pub fn read_char(mut regs: Regs) {
    let c = unsafe { libc::getchar() };
    regs[10] = c as u32;
}

pub fn random_int(mut regs: Regs) {
    let n: i32 = rand::random();
    regs[10] = n as u32;
}

pub fn random_int_range(mut regs: Regs) {
    let n = rand::random_range(0..regs[11] as i32);
    regs[10] = n as u32;
}

pub fn read(mut regs: Regs, ram: &Ram) {
    let fd = regs[10] as i32;
    let addr = regs[11] as usize;
    let count = regs[12] as usize;

    let mut buf = vec![0u8; count];
    let res = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut c_void, count) };

    let mut data = ram.data.write().unwrap();
    data[addr..addr + count].copy_from_slice(&buf);

    regs[10] = res as u32;
}

pub fn write(mut regs: Regs, ram: &Ram) {
    let fd = regs[10] as i32;
    let addr = regs[11] as usize;
    let count = regs[12] as usize;

    let data = ram.data.read().unwrap();

    let res = unsafe {
        libc::write(
            fd,
            data[addr..addr + count].as_ptr() as *const c_void,
            count,
        )
    };

    regs[10] = res as u32;
}

pub fn set_pixel(regs: Regs, framebuffer: &FrameBuffer) {
    let x = regs[10] as usize;
    let y = regs[11] as usize;
    let color = regs[12];

    framebuffer.set_pixel(x, y, color);
}
