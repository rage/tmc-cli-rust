use std::io::{BufRead, BufReader, Read, Write};
use termcolor::{Color, ColorSpec, WriteColor};

pub enum PrintColor {
    Success,
    Normal,
    Failed,
}

pub struct Io<'a> {
    output: &'a mut dyn WriteColor,
    input: BufReader<&'a mut dyn Read>,
}

/// Example use:
/**
```no_run
use termcolor::{ColorChoice, StandardStream};
use tmc::{Io, PrintColor};

# fn f() -> anyhow::Result<()> {
let mut output = StandardStream::stderr(ColorChoice::Always);
let mut input = std::io::stdin();

let mut io = Io::new(
    &mut output,
    &mut input,
);

let x = io.read_line()?;

io.print(&x, PrintColor::Normal)?;
# Ok(()) }
```
*/
impl Io<'_> {
    pub fn new<'a>(output: &'a mut dyn WriteColor, input: &'a mut dyn Read) -> Io<'a> {
        let input = BufReader::new(input);
        Io { output, input }
    }

    pub fn read_line(&mut self) -> anyhow::Result<String> {
        let mut buf = String::new();
        self.input.read_line(&mut buf)?;
        Ok(buf)
    }

    pub fn print(&mut self, text_to_output: &str, print_color: PrintColor) -> anyhow::Result<()> {
        match print_color {
            PrintColor::Success => {
                let mut colorspec = ColorSpec::new();
                colorspec.set_fg(Some(Color::Green));
                colorspec.set_bold(true);
                self.output.set_color(&colorspec)?;

                self.output.write_all(text_to_output.as_bytes())?;

                colorspec.clear();
                self.output.set_color(&colorspec)?;
            }
            PrintColor::Normal => {
                self.output.write_all(text_to_output.as_bytes())?;
            }
            PrintColor::Failed => {
                let mut colorspec = ColorSpec::new();
                colorspec.set_fg(Some(Color::Red));
                colorspec.set_bold(true);
                self.output.set_color(&colorspec)?;

                self.output.write_all(text_to_output.as_bytes())?;

                colorspec.clear();
                self.output.set_color(&colorspec)?;
            }
        }
        Ok(())
    }

    pub fn println(&mut self, output: &str, print_color: PrintColor) -> anyhow::Result<()> {
        self.print(output, print_color)?;
        self.print("\n", PrintColor::Normal)?;
        Ok(())
    }

    pub fn read_password(&mut self) -> anyhow::Result<String> {
        rpassword::read_password().map_err(Into::into)
    }
}
