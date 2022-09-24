pub mod components {
    use tabled::Tabled;

    #[derive(Tabled)]
    pub struct Process<'a> {
        pub pid: String,
        pub user: &'a str,
        pub command: &'a str,
        pub cpu: f32,
        pub mem: f32,
        pub virt_mem: f32,
    }
}
