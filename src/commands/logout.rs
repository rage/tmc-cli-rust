use super::util::Client;
use crate::io::{Io, PrintColor};

pub fn logout(io: &mut dyn Io, client: &mut dyn Client) -> anyhow::Result<()> {
    client.logout()?;
    io.println("Logged out successfully.", PrintColor::Success)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{super::util::*, *};
    use std::slice::Iter;

    pub struct IoTest<'a> {
        list: &'a mut Vec<String>,
        input: &'a mut Iter<'a, &'a str>,
    }

    impl IoTest<'_> {
        pub fn buffer_length(&mut self) -> usize {
            self.list.len()
        }

        pub fn buffer_get(&mut self, index: usize) -> String {
            self.list[index].to_string()
        }
    }

    impl Io for IoTest<'_> {
        fn read_line(&mut self) -> anyhow::Result<String> {
            let res = match self.input.next() {
                Some(string) => string,
                None => "",
            };
            Ok(res.to_string())
        }

        fn print(&mut self, output: &str, _font_color: PrintColor) -> anyhow::Result<()> {
            print!("{}", output);
            self.list.push(output.to_string());
            Ok(())
        }

        fn println(&mut self, output: &str, _font_color: PrintColor) -> anyhow::Result<()> {
            println!("{}", output);
            self.list.push(output.to_string());
            Ok(())
        }

        fn read_password(&mut self) -> anyhow::Result<String> {
            self.read_line()
        }
    }

    /*
    #[test]
    fn logout_when_logged_in_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock = MockClient::new();
        mock.expect_load_login().returning(|| Ok(()));
        mock.expect_logout().returning(|| ());

        logout(&mut io, &mut mock);

        assert_eq!(1, io.buffer_length());
        if io.buffer_length() == 1 {
            assert!(io
                .buffer_get(0)
                .to_string()
                .eq(&"Logged out successfully.".to_string()));
        }
    } */
}
