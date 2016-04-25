struct Option {
    required: bool,
    nb_params: u8,
    names: Vec<String>,
    description: String 
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
