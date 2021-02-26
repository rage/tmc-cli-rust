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
///

pub struct IoProduction<'a> {
    output: &'a mut dyn Write,
    input: &'a mut dyn Read,
}

pub trait Io {
    fn read_line(&mut self) -> String;
    fn print(&mut self, output: String);
    fn println(&mut self, output: String);
    fn read_password(&mut self) -> String;
}

impl IoProduction<'_> {
    pub fn new<'a>(output: &'a mut impl Write, input: &'a mut impl Read) -> IoProduction<'a> {
        IoProduction { output, input }
    }
}

impl Io for IoProduction<'_> {
    fn read_line(&mut self) -> String {
        let mut x = String::new();

        let mut reader = BufReader::new(&mut self.input);

        reader.read_line(&mut x).unwrap();
        x
    }

    fn print(&mut self, output: String) {
        self.output.write_all(output.as_bytes()).expect("");
        self.output.flush().expect("Something went wrong");
    }

    fn println(&mut self, output: String) {
        self.print(output);
        self.print("\n".to_string());
    }

    fn read_password(&mut self) -> String {
        rpassword::read_password().unwrap()
        //let mut reader = BufReader::new(&mut self.input);
        //read_password_with_reader(Some(&mut reader)).unwrap()
    }
}
