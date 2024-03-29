use super::server::Handler;
use crate::http::{Response, Request, StatusCode, Method};
use std::fs;
use std::path::PathBuf;
use std::io::Error;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // fs::canonicalize(path).ok()
        //     .filter(|path| path.starts_with(&self.public_path))
        //     .and_then(|path| fs::read_to_string(path).ok())
        //     .or_else(|| {
        //         println!("Directory Traversal Attack Attempted: {}", file_path);
        //         None
        //     })

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                // "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                // "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
