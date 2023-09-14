use super::method::Method;

pub struct Request {
    path: String,
    query_str: Option<String>,
    method: Method
}