#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("archp_simulator/cxx/cpu.hpp");

        type CPU;

        fn create_cpu() -> UniquePtr<CPU>;

        fn got_finish(&self) -> bool;
        fn time(&self) -> u64;
        fn increase_time(&self, add: u64);
        fn flip_clk(&self);
        fn set_rst(&self, rst: bool);
        fn posedge_clk(&self) -> bool;
        fn eval(&self);
    }
}
