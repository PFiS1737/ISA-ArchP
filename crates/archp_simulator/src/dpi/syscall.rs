use super::SYSTEM;

#[unsafe(no_mangle)]
extern "C" fn system_call() {
    SYSTEM.get().unwrap().system_call();
}
