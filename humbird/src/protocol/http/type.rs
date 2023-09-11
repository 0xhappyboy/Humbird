use super::{request::Request, response::Response};

/// http request process
pub type HttpRequestProcess = fn(Request, Response) -> Response;
