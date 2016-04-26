struct Option {
    required: bool,         // Is this option required
    nb_params: u8,          // How many parameters are following this option ?
    names: Vec<String>,     // Vector of name corresponding to this option
    description: String     // Short description of this option (to generate help)
}

struct Arguments {
    args: Vec<Option>
}

impl Arguments {
    fn new() -> Arguments {
        Arguments { args: Vec::new() }
    }

    fn register_option(&mut self, opt: Option) -> () {
        self.args.push(opt);
    }

    fn parse_args(&self) -> Result<Arguments, &str> {
        let current_args = std::env::args();
        let parsed_args = Arguments::new();

        //Ok(parsed_args);
        std::result::Result::Err("You should not do this.");
    }
}

fn main() {
    let mut parser = Arguments::new();
    let opt = Option{required: true,
                     nb_params: 2,
                     names: vec![String::from("--hello"), String::from("-h")],
                     description: String::from("Un argument pour dire bonjour")
    };

    parser.register_option(opt);
}
