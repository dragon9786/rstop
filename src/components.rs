pub mod components {
    use tabled::Tabled;

    #[repr(C)]
    #[derive(Tabled)]
    pub struct Process<'a> {
        pub pid: String,
        pub command: &'a str,
        pub run_time_in_minutes: u64,
        pub mem_usage: f32,
        pub virt_mem_usage: f32,
    }
}
