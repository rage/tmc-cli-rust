use super::util::Client;
use crate::io::{Io, PrintColor};

pub fn logout(io: &mut Io, client: &mut Client) -> anyhow::Result<()> {
    client.logout()?;
    io.println("Logged out successfully.", PrintColor::Success)?;
    Ok(())
}

#[cfg(test)]
mod tests {
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
