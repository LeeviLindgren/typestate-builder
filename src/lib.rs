pub mod error;
use std::marker::PhantomData;

#[derive(Debug, Default, Clone)]
pub enum HTTPMethod {
    #[default]
    Get,
    Post,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Request {
    url: String,
    method: HTTPMethod,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

// ======= States ======
#[derive(Clone, Default)]
pub struct NoUrl;
#[derive(Clone, Default)]
pub struct Url(String);

#[derive(Clone, Default)]
pub struct NoMethod;
#[derive(Clone, Default)]
pub struct Method(HTTPMethod);

#[derive(Clone, Default)]
pub struct Sealed;
#[derive(Clone, Default)]
pub struct NotSealed;

// ======= Builder ======
#[derive(Default, Clone)]
pub struct RequestBuilder<U, M, S> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
    _marker_seal: PhantomData<S>,
}

/// Constructor
///
/// New method is available only for `NoUrl` and `NoMethod` states.
impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

/// Build
///
/// Building is only possible when the url and method are set, and
/// the builder is sealed.
impl RequestBuilder<Url, Method, Sealed> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method.0,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// Seal the builder
impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn seal(self) -> RequestBuilder<U, M, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            _marker_seal: PhantomData,
        }
    }
}

/// Setter methods
///
/// Setter methods are only available when in `NotSealed` state.
impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            _marker_seal: PhantomData,
        }
    }
    pub fn method(self, method: HTTPMethod) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method),
            headers: self.headers,
            body: self.body,
            _marker_seal: PhantomData,
        }
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        let _ = self.body.insert(body.into());
        self
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}
