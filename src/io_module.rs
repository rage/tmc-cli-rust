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
    buffer: &'a mut Buffer,
    output: &'a mut dyn Write,
    input: BufReader<&'a mut dyn Read>,
    test_mode: bool,
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
        buffer: &'a mut Buffer,
        output: &'a mut impl Write,
        input: &'a mut dyn Read,
        test_mode: bool,
    ) -> IoProduction<'a> {
        let reader = BufReader::new(input);
        IoProduction {
            bufferwriter,
            buffer,
            output,
            input: reader,
            test_mode,
        }
    }
}

impl Io for IoProduction<'_> {
    fn read_line(&mut self) -> String {
        let mut x = String::new();

        self.input.read_line(&mut x).unwrap();
        x
    }

    fn print(&mut self, text_to_output: &str, font_color: PrintColor) {
        match self.test_mode {
            true => {
                self.output.write_all(text_to_output.as_bytes()).expect("");
                self.output.flush().expect("Something went wrong");
            }
            false => match font_color {
                PrintColor::Success => {
                    let mut colorspec = ColorSpec::new();
                    colorspec.set_fg(Some(Color::Green));
                    colorspec.set_bold(true);
                    self.buffer.set_color(&colorspec).unwrap();

                    self.buffer.write_all(text_to_output.as_bytes()).expect("");
                    self.bufferwriter.print(self.buffer).unwrap();
                    self.buffer.clear();

                    colorspec.clear();
                    self.buffer.set_color(&colorspec).unwrap();
                }
                PrintColor::Normal => {
                    self.buffer.write_all(text_to_output.as_bytes()).expect("");
                    self.bufferwriter.print(self.buffer).unwrap();
                    self.buffer.clear();
                }
                PrintColor::Failed => {
                    let mut colorspec = ColorSpec::new();
                    colorspec.set_fg(Some(Color::Red));
                    colorspec.set_bold(true);
                    self.buffer.set_color(&colorspec).unwrap();

                    self.buffer.write_all(text_to_output.as_bytes()).expect("");
                    self.bufferwriter.print(self.buffer).unwrap();
                    self.buffer.clear();

                    colorspec.clear();
                    self.buffer.set_color(&colorspec).unwrap();
                }
            },
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
