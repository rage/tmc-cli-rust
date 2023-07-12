use super::util::Client;
use crate::io::{Io, PrintColor};

/// Lists available courses from clients organization
pub fn list_courses(io: &mut Io, client: &mut Client) -> anyhow::Result<()> {
    let mut course_list = client.list_courses()?;
    course_list.sort_unstable_by(|l, r| l.name.cmp(&r.name));
    io.println("", PrintColor::Normal)?;
    for course in course_list {
        io.println(&course.name, PrintColor::Normal)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Mock, Server, ServerGuard};
    use std::io::Cursor;
    use termcolor::NoColor;

    fn logging() {
        let _ = flexi_logger::Logger::try_with_env()
            .unwrap()
            .log_to_stdout()
            .start();
    }

    fn mock_server() -> (ServerGuard, Vec<Mock>) {
        let mut server = Server::new();
        let mut mocks = Vec::new();
        mocks.push(
            server
                .mock("GET", "/api/v8/core/org/test/courses")
                .match_query(Matcher::Any)
                .with_body(
                    serde_json::json!([
                        {
                            "id": 1,
                            "name": "name",
                            "title": "title",
                            "details_url": "example.com",
                            "unlock_url": "example.com",
                            "reviews_url": "example.com",
                            "comet_url": "example.com",
                            "spyware_urls": ["example.com"],
                        },
                        {
                            "id": 1,
                            "name": "mooc-tutustumiskurssi",
                            "title": "title",
                            "details_url": "example.com",
                            "unlock_url": "example.com",
                            "reviews_url": "example.com",
                            "comet_url": "example.com",
                            "spyware_urls": ["example.com"],
                        },
                    ])
                    .to_string(),
                )
                .create(),
        );
        mocks.push(
            server
                .mock("GET", "/api/v8/courses/1/exercises")
                .match_query(Matcher::Any)
                .with_body(
                    serde_json::json!([
                        {
                            "id": 1,
                            "available_points": [],
                            "awarded_points": [],
                            "name": "part01-01_example_exercise",
                            "disabled": false,
                            "unlocked": true,
                        },
                        {
                            "id": 2,
                            "available_points": [],
                            "awarded_points": [],
                            "name": "part02-03_example_valid",
                            "disabled": false,
                            "unlocked": true,
                        },
                    ])
                    .to_string(),
                )
                .create(),
        );
        (server, mocks)
    }

    #[test]
    fn list_courses_with_client_test() {
        logging();
        let (server, _mocks) = mock_server();

        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let mut client = Client::new(server.url().parse().unwrap(), false).unwrap();
        list_courses(&mut io, &mut client).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert!(output[0].eq(""), "first line should be empty");
        assert!(
            output[1].eq("mooc-tutustumiskurssi"),
            "Expected 'mooc-tutustumiskurssi', got {}",
            output[1]
        );
        assert!(output[2].eq("name"), "Expected 'name', got '{}'", output[2]);
    }
}
