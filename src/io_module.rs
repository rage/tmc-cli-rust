use std::io::{BufRead, Write};

// Example use:

//let stdin = stdin();
//let mut lock = stdin.lock();

//let mut stdout = stdout();

//let mut io = IO::new(&mut stdout, &mut lock);

//let x = io.read_line();

//io.print(&x);

pub struct IO<'a> {
    output: &'a mut dyn Write,
    input: &'a mut dyn BufRead,
}

impl IO<'_> {
    pub fn new<'a>(output: &'a mut impl Write, input: &'a mut impl BufRead) -> IO<'a> {
        IO { output, input }
    }

    pub fn read_line(&mut self) -> String {
        let mut x = String::new();

        self.input.read_line(&mut x).expect("moi");
        x
    }

    pub fn print<S: Into<String>>(&mut self, output: S) {
        self.output.write(output.into().as_bytes()).expect("");
        self.output.flush().expect("Something went wrong");
    }

    pub fn println<S: Into<String>>(&mut self, output: S) {
        self.print(output);
        self.print("\n");
    }
}
