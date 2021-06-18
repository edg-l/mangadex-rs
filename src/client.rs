use crate::{
    api::auth::{Login, Logout, RefreshToken},
    errors::{Errors, Result},
    schema::auth::{AuthTokens, RefreshTokenResponse},
    Endpoint, FromResponse, UrlSerdeQS,
};
use reqwest::Url;
use serde::de::DeserializeOwned;

#[cfg(not(target_arch = "wasm32"))]
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "-rs",
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// The client used to talk to the api.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: Url,
    tokens: Option<AuthTokens>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new("https://api.mangadex.org/").expect("Error creating default API client")
    }
}

impl Client {
    /// Create a new client.
    pub fn new(base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder();

        #[cfg(not(target_arch = "wasm32"))]
        let client = client.user_agent(APP_USER_AGENT);

        let client = client.build()?;

        Ok(Self {
            http: client,
            base_url: Url::parse(base_url)?,
            tokens: None,
        })
    }

    pub(crate) async fn send_request<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        <<E as Endpoint>::Response as FromResponse>::Response: DeserializeOwned,
    {
        let mut endpoint_url = self.base_url.join(&endpoint.path())?;
        if let Some(query) = endpoint.query() {
            endpoint_url = endpoint_url.query_qs(query);
        }

        let mut req = self.http.request(endpoint.method(), endpoint_url);
        if let Some(body) = endpoint.body() {
            req = req.json(body);
        }

        if let Some(multipart) = endpoint.multipart() {
            req = req.multipart(multipart);
        }

        if let Some(tokens) = self.get_tokens() {
            req = req.bearer_auth(&tokens.session);
        } else if endpoint.require_auth() {
            return Err(Errors::MissingTokens);
        }

        let res = req.send().await?;
        let res = res
            .json::<<E::Response as FromResponse>::Response>()
            .await?;

        Ok(FromResponse::from_response(res))
    }

    /// Login
    ///
    /// * `username` - Should be between [1, 64] characters.
    /// * `password` - Should be between [8, 1024] characters.
    pub async fn login(&mut self, username: &str, password: &str) -> Result<&AuthTokens> {
        let tokens = Login { username, password }.send(self).await?.tokens;

        self.set_tokens(Some(tokens));
        Ok(self.get_tokens().unwrap())
    }

    /// Get the tokens used for authentication
    pub fn get_tokens(&self) -> Option<&AuthTokens> {
        self.tokens.as_ref()
    }

    /// Set the tokens used for authentication.
    pub fn set_tokens(&mut self, tokens: Option<AuthTokens>) {
        self.tokens = tokens;
    }

    /// Logout
    pub async fn logout(&mut self) -> Result<()> {
        Logout.send(self).await?;

        self.set_tokens(None);
        Ok(())
    }

    /// Refresh token endpoint
    pub async fn refresh_tokens(&mut self) -> Result<RefreshTokenResponse> {
        let refresh_token = &self.get_tokens().ok_or(Errors::MissingTokens)?.refresh;
        let res = RefreshToken { refresh_token }.send(self).await?;

        self.set_tokens(Some(res.tokens.clone()));
        Ok(res)
    }

    /// Ping the api server
    pub async fn ping(&self) -> Result<()> {
        let endpoint = self.base_url.join("/ping")?;

        let res = self.http.get(endpoint).send().await?;
        if res.text().await? == "pong" {
            Ok(())
        } else {
            Err(Errors::PingError)
        }
    }
}

/// Helper macro to quickly implement the `Endpoint` trait,
/// and optionally a `send()` method for the input struct.
///
/// The first argument is the endpoint, the second the input data, and the third the response.
/// These arguments are seperated by commas.
///
/// The endpoint is specified by the HTTP method, followed by the path. To get a dymanic path
/// based on the input structure, surround the path with parenthesis:
/// ```
/// POST ("/account/activate/{}", id)
/// ```
/// The format is the same as the `format!()` macro, except `id` will be substituted by `self.id`,
/// where `self` represents an instance of the second parameter.
///
/// The input structure is preceded by an attribute-like structure. Inside it, it is possible to
/// specify either `query`, `body`, or `no_data`. In the first case, the input structure will be
/// serialized as the query parameter, in the second as a json body, and in the third no data will
/// be sent with the request. If an `auth` tag is also included, the request will not be made if
/// the user is not authenticated. Some examples of valid tags are:
/// ```
/// #[query] QueryReq
/// #[body] BodyReq
/// #[query auth] QueryReq
/// #[no_data] QueryStruct
/// ```
/// The input structure itself should implement `serde::Serialize` if it is used as a body or query.
/// By strategically using `#[serde(skip)]` and `#[serde(flatten)]`, this is powerfull enough for
/// our use case.
///
/// The final argument is the output type, tagged similarly to the input, to modify the behaviour
/// of the generated `send()` method. Specifically:
/// - \<no tag\>: `send()` will simply return `Result<Output>`
/// - `flatten_result`: If `Output = Result<T>`, the return type will be simplified to `Result<T>`
/// - `discard_result`: If `Output = Result<T>`, discard `T`, and return `Result<()>`
/// - `no_send`: Do not implement a `send()` function
///
/// Example:
/// ```
/// impl_endpoint! {
///     GET "/path/to/endpoint",
///     #[query] StructWithData<'_>,
///     #[flatten_result] Result<ResponseType>
/// }
/// ```
macro_rules! impl_endpoint {
    {
        $method:ident $path:tt,
        #[$payload:ident $($auth:ident)?] $typ:ty,
        $(#[$out_res:ident])? $out:ty
    } => {

        impl $crate::common::Endpoint for $typ {
            type Response = $out;

            fn method(&self) -> reqwest::Method {
                reqwest::Method::$method
            }

            impl_endpoint! { @path $path }
            impl_endpoint! { @payload $payload }
            $(impl_endpoint! { @$auth })?
        }

        impl_endpoint! { @send $(:$out_res)?, $typ, $out }
    };
    { @path ($path:expr, $($arg:ident),+) } => {
        fn path(&self) -> std::borrow::Cow<str> {
            std::borrow::Cow::Owned(format!($path, $(self.$arg),+))
        }
    };
    { @path $path:expr } => {
        fn path(&self) -> std::borrow::Cow<str> {
            std::borrow::Cow::Borrowed($path)
        }
    };
    { @payload query } => {
        type Query = Self;
        type Body = ();
        fn query(&self) -> Option<&Self::Query> {
            Some(&self)
        }
    };
    { @payload body } => {
        type Query = ();
        type Body = Self;
        fn body(&self) -> Option<&Self::Body> {
            Some(&self)
        }
    };
    { @payload no_data } => {
        type Query = ();
        type Body = ();
    };
    { @auth } => {
        fn require_auth(&self) -> bool {
            true
        }
    };
    { @send, $typ:ty, $out:ty } => {
        impl $typ {
            pub async fn send(&self, client: &$crate::Client) -> $crate::Result<$out> {
                client.send_request(self).await
            }
        }
    };
    { @send:flatten_result, $typ:ty, $out:ty } => {
        impl $typ {
            pub async fn send(&self, client: &$crate::Client) -> $out {
                client.send_request(self).await?
            }
        }
    };
    { @send:discard_result, $typ:ty, $out:ty } => {
        impl $typ {
            pub async fn send(&self, client: &$crate::Client) -> $crate::Result<()> {
                client.send_request(self).await??;
                Ok(())
            }
        }
    };
    { @send:no_send, $typ:ty, $out:ty } => { };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        Client::default();
    }

    #[tokio::test]
    async fn ping_server() {
        let client = Client::default();
        client.ping().await.unwrap();
    }
}
