struct Reader<'a> {
    stream: &'a str,
    pos: i32,
    col: i32,
    line: i32
}

impl<'a> Reader<'a> {

    /// This function instanciate a new Input Reader
    ///
    /// # Examples
    ///
    /// ```
    /// use args_parser::input;
    ///
    /// let read = input::Reader::new("hello");
    /// assert_eq!("hello", read.stream);
    /// ```
    fn new(input: &'a str) -> Reader {
        Reader {
            pos: 0,
            col: 0,
            line: 1,
            stream: input
        }
    }
}


