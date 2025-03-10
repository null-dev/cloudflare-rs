use super::Key;

use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Lists a namespace's keys.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/subresources/keys/methods/list/>
#[derive(Debug)]
pub struct ListNamespaceKeys<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: ListNamespaceKeysParams,
}

impl EndpointSpec for ListNamespaceKeys<'_> {
    type JsonResponse = Vec<Key>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/keys",
            self.account_identifier, self.namespace_identifier
        )
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespaceKeysParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub prefix: Option<String>,
}
