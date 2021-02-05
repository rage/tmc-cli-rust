use std::io::{Write, BufRead};

// Example use:

//let stdin = stdin();
//let mut lock = stdin.lock();

//let mut stdout = stdout();

//let mut io = IO::new(&mut stdout, &mut lock);

//let x = io.read_line();

//io.print(&x);



pub struct IO <'a>{
    output: &'a mut dyn Write,
    input: &'a mut dyn BufRead, 
}

impl IO<'_> {

    pub fn new<'a>(output: &'a mut impl Write, input: &'a mut impl BufRead)-> IO<'a> {
        IO{
            output,
            input
        }
    }


    pub fn read_line(self: &mut Self) -> String{
        let mut x = String::new();

        self.input.read_line(&mut x).expect("moi");

        x
    }

    pub fn print(self: &mut Self, output: &String) {
        self.output.write(output.as_bytes()).expect("");
    }
}
