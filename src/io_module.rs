use std::io::{BufRead, BufReader, Read, Write};

use termcolor::{Buffer, BufferWriter, Color, ColorSpec, WriteColor};

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

pub enum PrintColor {
    Success,
    Normal,
    Failed,
}

pub struct IoProduction<'a> {
    bufferwriter: &'a mut BufferWriter,
    output: &'a mut Buffer,
    input: BufReader<&'a mut dyn Read>,
}

pub trait Io {
    fn read_line(&mut self) -> String;
    fn print(&mut self, output: &str, font_color: PrintColor);
    fn println(&mut self, output: &str, font_color: PrintColor);
    fn read_password(&mut self) -> String;
}

impl IoProduction<'_> {
    pub fn new<'a>(
        bufferwriter: &'a mut BufferWriter,
        output: &'a mut Buffer,
        input: &'a mut dyn Read,
    ) -> IoProduction<'a> {
        let reader = BufReader::new(input);
        IoProduction {
            bufferwriter,
            output,
            input: reader,
        }
    }
}

impl Io for IoProduction<'_> {
    fn read_line(&mut self) -> String {
        let mut x = String::new();

        self.input.read_line(&mut x).unwrap();
        x
    }

    fn print(&mut self, output: &str, font_color: PrintColor) {
        match font_color {
            PrintColor::Success => {
                let mut colorspec = ColorSpec::new();
                colorspec.set_fg(Some(Color::Green));
                colorspec.set_bold(true);
                self.output.set_color(&colorspec).unwrap();

                self.output.write_all(output.as_bytes()).expect("");
                self.bufferwriter.print(&self.output).unwrap();
                self.output.clear();

                colorspec.clear();
                self.output.set_color(&colorspec).unwrap();
            }
            PrintColor::Normal => {
                self.output.write_all(output.as_bytes()).expect("");
                self.bufferwriter.print(&self.output).unwrap();
                self.output.clear();
            }
            PrintColor::Failed => {
                let mut colorspec = ColorSpec::new();
                colorspec.set_fg(Some(Color::Red));
                colorspec.set_bold(true);
                self.output.set_color(&colorspec).unwrap();

                self.output.write_all(output.as_bytes()).expect("");
                self.bufferwriter.print(&self.output).unwrap();
                self.output.clear();

                colorspec.clear();
                self.output.set_color(&colorspec).unwrap();
            }
        }
    }

    fn println(&mut self, output: &str, font_color: PrintColor) {
        self.print(output, font_color);
        self.print("\n", PrintColor::Normal);
    }

    fn read_password(&mut self) -> String {
        rpassword::read_password().unwrap()
        //let mut reader = BufReader::new(&mut self.input);
        //read_password_with_reader(Some(&mut reader)).unwrap()
    }
}
