use crate::{client::Client, config::TmcCliConfig, Io};
use mockito::ServerGuard;
use std::io::Cursor;
use tempfile::NamedTempFile;
use termcolor::NoColor;
use tmc_langs::tmc::{
    oauth2::{basic::BasicTokenType, AccessToken, EmptyExtraTokenFields},
    Token,
};

pub fn logging() {
    let _ = flexi_logger::Logger::try_with_env()
        .unwrap()
        .log_to_stdout()
        .start();
}

pub fn input_output() -> (Cursor<Vec<u8>>, NoColor<Vec<u8>>) {
    let input = Cursor::new(Vec::<u8>::new());
    let output = NoColor::new(Vec::<u8>::new());
    (input, output)
}

pub struct TestSetup<'a> {
    pub io: Io<'a>,
    pub client: Client,
}

pub fn setup<'a>(
    input: &'a mut Cursor<Vec<u8>>,
    output: &'a mut NoColor<Vec<u8>>,
    server: &'a ServerGuard,
) -> TestSetup<'a> {
    let config_file = NamedTempFile::new().unwrap();
    let _config = TmcCliConfig::load(config_file.path().to_path_buf()).unwrap();
    TestSetup {
        io: Io::new(output, input),
        client: Client::new(server.url().parse().unwrap(), false).unwrap(),
    }
}

pub fn tmc_token() -> Token {
    Token::new(
        AccessToken::new("test".to_string()),
        BasicTokenType::Bearer,
        EmptyExtraTokenFields {},
    )
}
