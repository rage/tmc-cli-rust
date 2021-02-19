use std::io::{BufRead, BufReader, Read, Write};

/// Example use:
///
///let stdin = stdin();
///let mut lock = stdin.lock();
///
///let mut stdout = stdout();
///
///let mut io = IO::new(&mut stdout, &mut lock);
///
///let x = io.read_line();
///
///io.print(&x);

pub struct Io<'a> {
    output: &'a mut dyn Write,
    input: &'a mut dyn Read,
}

impl Io<'_> {
    pub fn new<'a>(output: &'a mut impl Write, input: &'a mut impl Read) -> Io<'a> {
        Io { output, input }
    }

    pub fn read_line(&mut self) -> String {
        let mut x = String::new();

        let mut reader = BufReader::new(&mut self.input);

        reader.read_line(&mut x).unwrap();
        x
    }

    pub fn print<S: Into<String>>(&mut self, output: S) {
        self.output.write_all(output.into().as_bytes()).expect("");
        self.output.flush().expect("Something went wrong");
    }

    pub fn println<S: Into<String>>(&mut self, output: S) {
        self.print(output);
        self.print("\n");
    }

    pub fn read_password(&mut self) -> String {
        rpassword::read_password().unwrap()
        //let mut reader = BufReader::new(&mut self.input);
        //read_password_with_reader(Some(&mut reader)).unwrap()
    }
}
