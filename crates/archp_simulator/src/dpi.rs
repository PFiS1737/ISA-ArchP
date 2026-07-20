#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("archp_simulator/cxx/dpi/Memory.hpp");
        include!("archp_simulator/cxx/dpi/PixelDisplay.hpp");
        include!("archp_simulator/cxx/dpi/Program.hpp");

        type Memory;
        fn init(self: &Memory, size_in_bytes: usize);

        type PixelDisplay;
        fn init(self: &PixelDisplay, w: u32, h: u32, scale: u32) -> bool;
        fn destroy(self: &PixelDisplay);
        fn handle_event(self: &PixelDisplay) -> bool;

        type Program;
        fn open(self: &Program, file_name: &str) -> Result<()>;
    }
}

unsafe extern "C" {
    pub static mem: ffi::Memory;
    pub static pd: ffi::PixelDisplay;
    pub static program: ffi::Program;
}

impl Drop for ffi::PixelDisplay {
    fn drop(&mut self) {
        self.destroy();
    }
}
