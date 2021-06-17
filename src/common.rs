use std::borrow::Cow;

use isolanguage_1::LanguageCode;
use reqwest::{Method, Response};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use crate::{
    errors::{ApiErrors, Errors},
    Client, Result,
};

pub type LocalizedString = std::collections::HashMap<LanguageCode, String>;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: ResourceType,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiData<T> {
    pub data: T,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiObject<A, T = ResourceType> {
    pub id: Uuid,
    pub r#type: T,
    pub attributes: A,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct NoData;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Results<T> {
    pub results: Vec<T>,
    pub limit: i32,
    pub offset: i32,
    pub total: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl PaginationQuery {
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Manga,
    Chapter,
    CoverArt,
    Author,
    Artist,
    ScanlationGroup,
    Tag,
    User,
    CustomList,
}

pub(crate) trait UrlSerdeQS {
    fn query_qs<T: Serialize>(self, query: &T) -> Self;
}

impl UrlSerdeQS for url::Url {
    fn query_qs<T: Serialize>(mut self, query: &T) -> Self {
        self.set_query(Some(
            &serde_qs::to_string(query).expect("Failed to encode query string"),
        ));
        self
    }
}

pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Deserialize)]
#[serde(tag = "result", remote = "std::result::Result")]
enum ApiResultDef<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "error")]
    Err(E),
}

#[derive(Deserialize)]
#[serde(bound = "T: DeserializeOwned, E: DeserializeOwned")]
pub(crate) struct ApiResult<T, E = ApiErrors>(
    #[serde(with = "ApiResultDef")] std::result::Result<T, E>,
);

impl<T, E> ApiResult<T, E> {
    fn into_result(self) -> Result<T, E> {
        self.0
    }
}

/// So this trait isn't ideal, but it's the only practical way I could figure out on stable Rust,
// without specialization. With that in mind, please don't expose it to the public API.
pub(crate) trait FromResponse: Sized {
    type Response;

    fn from_response(res: Self::Response) -> Self;
}

impl<T> FromResponse for Result<T, Errors> {
    type Response = ApiResult<T, ApiErrors>;

    fn from_response(value: Self::Response) -> Self {
        value.into_result().map_err(|e| e.into())
    }
}

impl<T> FromResponse for Results<Result<T, Errors>> {
    type Response = Results<ApiResult<T, ApiErrors>>;

    fn from_response(value: Self::Response) -> Self {
        Results {
            results: value
                .results
                .into_iter()
                .map(|r| r.into_result().map_err(|e| e.into()))
                .collect(),
            offset: value.offset,
            limit: value.limit,
            total: value.total,
        }
    }
}

impl<T> FromResponse for Vec<Result<T, Errors>> {
    type Response = Vec<ApiResult<T, ApiErrors>>;

    fn from_response(value: Self::Response) -> Self {
        value
            .into_iter()
            .map(|r| r.into_result().map_err(|e| e.into()))
            .collect()
    }
}

pub(crate) trait Endpoint {
    type Query: Serialize;
    type Body: Serialize;
    type Response: FromResponse;

    fn path(&self) -> Cow<str>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn require_auth(&self) -> bool {
        false
    }

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }
}

impl Client {
    pub(crate) async fn send_request<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        <<E as Endpoint>::Response as FromResponse>::Response: DeserializeOwned,
    {
        let mut endpoint_url = self.base_url.join(&endpoint.path())?;
        if let Some(query) = endpoint.query() {
            endpoint_url = endpoint_url.query_qs(query);
        }

        let mut res = self.http.request(endpoint.method(), endpoint_url);
        if let Some(body) = endpoint.body() {
            res = res.json(body);
        }

        if endpoint.require_auth() {
            let tokens = self.require_tokens()?;
            res = res.bearer_auth(&tokens.session);
        }

        let res = res
            .send()
            .await?
            .json::<<E::Response as FromResponse>::Response>()
            .await?;

        Ok(FromResponse::from_response(res))
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

// TODO: Remove these metods
impl Client {
    /// Deserialize ApiResult<T> then convert to Result<T>
    pub async fn json_api_result<T>(res: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(res.json::<ApiResult<T, ApiErrors>>().await?.into_result()?)
    }

    /// Deserialize as Results<ApiResult<T>> then convert to Results<Result<T>>
    pub async fn json_api_results<T>(res: Response) -> Result<Results<Result<T>>>
    where
        T: DeserializeOwned,
    {
        let res = res.json::<Results<ApiResult<T, ApiErrors>>>().await?;
        Ok(Results {
            results: res
                .results
                .into_iter()
                .map(|r| r.into_result().map_err(|e| e.into()))
                .collect(),
            offset: res.offset,
            limit: res.limit,
            total: res.total,
        })
    }

    /// Deserialize as Vec<ApiResult<T>> then convert to Vec<Result<T>>
    pub async fn json_api_result_vec<T>(res: Response) -> Result<Vec<Result<T>>>
    where
        T: DeserializeOwned,
    {
        let res = res.json::<Vec<ApiResult<T, ApiErrors>>>().await?;
        Ok(res
            .into_iter()
            .map(|r| r.into_result().map_err(|e| e.into()))
            .collect())
    }
}
