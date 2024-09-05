pub struct Options {
    pub force_action: bool,
    pub verbose: bool,
}

impl Options {
    pub fn new(args: Option<&Vec<String>>) -> Self {
        let mut instance = Options {
            force_action: false,
            verbose: false,
        };

        for arg in args.unwrap_or(&vec![]) {
            if arg == "-f" || arg == "--force" {
                instance.force_action = true
            }

            if arg == "-v" || arg == "--force" {
                instance.verbose = true
            }
        }

        return instance;
    }
}
