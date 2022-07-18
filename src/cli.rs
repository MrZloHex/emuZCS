use clap::{load_yaml, App};

pub struct CliSet {
    rom_filename: String,
    verbosity:    bool,
    manuality:    bool,
    cpu_only:     bool,
}

impl CliSet {
    pub fn new() -> Self {
        Self {
            rom_filename: String::new(),
            verbosity:    false,
            manuality:    false,
            cpu_only:     false,
        }
    }

    pub fn get_cli_settings() -> Self {
        let mut cli_set = Self::new();

        let yaml = load_yaml!("cli.yaml");
        let matches = App::from(yaml).get_matches();

        cli_set.rom_filename = matches.value_of("rom").unwrap().to_string();
        cli_set.verbosity    = matches.is_present("verbose");
        cli_set.manuality    = matches.is_present("manual");
        cli_set.cpu_only     = matches.is_present("cpu-only");

        cli_set
    }

    pub fn get_rom_fname(&self) -> String {
        self.rom_filename.clone()
    }

    pub fn get_verbosity(&self) -> bool {
        self.verbosity
    }

    pub fn get_manuality(&self) -> bool {
        self.manuality
    }

    pub fn get_cpu_only(&self) -> bool {
        self.cpu_only
    }
}
