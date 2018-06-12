use std::fs::File;
use std::io::Read;

use super::rocket;
use super::rocket_contrib;
use super::rocket_contrib::Template;
use rocket::http::Method::*;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};

fn test_query_file<T>(path: &str, file: T, status: Status)
where
    T: Into<Option<&'static str>>,
{
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get(path).dispatch();
    assert_eq!(response.status(), status);

    let body_data = response.body().and_then(|body| body.into_bytes());
    if let Some(filename) = file.into() {
        let expected_data = read_file(filename);
        assert!(body_data.map_or(false, |s| s == expected_data));
    }
}

fn read_file(path: &str) -> Vec<u8> {
    let mut fp = File::open(&path).expect(&format!("Can not open {}", path));
    let mut file_content = vec![];

    fp.read_to_end(&mut file_content)
        .expect(&format!("Reading {} failed.", path));
    file_content
}

/// test for static file reading
#[test]
fn test_index_html() {
    test_query_file("/", "templates/index.html", Status::Ok);
    test_query_file("/?v=1", "templates/index.html", Status::Ok);
    test_query_file(
        "/?this=should&be=ignored",
        "templates/index.html",
        Status::Ok,
    );
}

#[test]
fn test_invalid_path() {
    test_query_file("/error_not_exist", None, Status::NotFound);
    test_query_file("/error/not/exist", None, Status::NotFound);
    test_query_file("/error/not/exist?a=b&c=d", None, Status::NotFound);
}

/// test for login page
fn client() -> Client {
    let rocket = rocket::ignite().mount("/", routes![super::login]);
    Client::new(rocket).unwrap()
}

#[test]
fn test_login() {
    let client = client();
    let mut r = client.get("/login").dispatch();
    assert_eq!(r.body_string(), None);
}

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => {{
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    }};
}

// test for template generation
//#[ignore]
//fn test_404() {
//    // Check that the error catcher works.
//    dispatch!(
//        Get,
//        "/hello/",
//        |client: &Client, mut response: LocalResponse| {
//            let mut map = ::std::collections::HashMap::new();
//            map.insert("path", "/hello/");
//
//            let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
//            assert_eq!(response.status(), Status::NotFound);
//            assert_eq!(response.body_string(), Some(expected));
//        }
//    );
//}

//#[test]
//fn test_name() {
//    // Check that the /hello/<name> route works.
//    dispatch!(Get, "/hello/Jack", |client: &Client, mut response: LocalResponse| {
//        let context = super::TemplateContext1 {
//            path: "Jack".into(),
//            items: vec!["One".into(), "Two".into(), "Three".into()]
//        };
//
//        let expected = Template::show(client.rocket(), "index", &context).unwrap();
//        assert_eq!(response.status(), Status::Ok);
//        assert_eq!(response.body_string(), Some(expected));
//    });
//}
