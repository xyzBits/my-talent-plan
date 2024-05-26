fn main() {
    /// Registers a custom panic hook, replacing the previously registered hook.
    ///
    /// The panic hook is invoked when a thread panics, but before the
    /// panic runtime is invoked. As such, the hook will run with both the
    /// aborting and unwinding runtimes.
    ///
    /// The default hook, which is registered at startup, prints a message
    /// to standard error and generates a backtrace of requested. This
    /// behavior can be customized using the set_hook function. The current
    /// hook can be retrieved while reinstating the default hook with the
    /// take_hook function.
    std::panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
    }));

    panic_anyway(0);
}


fn panic_anyway(age: u32) {
    if age <= 0 {
        panic!("Are you kidding me?")
    }
}