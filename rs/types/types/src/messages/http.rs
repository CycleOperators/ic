// DFN-467: clippy complains about the code generated by derive(Arbitrary)
#![cfg_attr(test, allow(clippy::unit_arg))]
//! HTTP requests that the Internet Computer is prepared to handle.

use super::{query::QuerySource, Blob};
use crate::{
    crypto::SignedBytesWithoutDomainSeparator,
    messages::{
        message_id::hash_of_map, MessageId, Query, ReadState, SignedIngressContent, UserSignature,
    },
    Height, Time, UserId,
};
use ic_base_types::{CanisterId, CanisterIdError, NodeId, PrincipalId};
use ic_crypto_tree_hash::{MixedHashTree, Path};
use maplit::btreemap;
#[cfg(test)]
use proptest_derive::Arbitrary;
use serde::{ser::SerializeTuple, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    convert::TryFrom,
    error::Error,
    fmt,
};
use strum_macros::AsRefStr;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum CallOrQuery {
    Call,
    Query,
}

pub(crate) fn representation_independent_hash_call_or_query(
    request_type: CallOrQuery,
    canister_id: Vec<u8>,
    method_name: &str,
    arg: Vec<u8>,
    ingress_expiry: u64,
    sender: Vec<u8>,
    nonce: Option<&[u8]>,
) -> [u8; 32] {
    use RawHttpRequestVal::*;
    let mut map = btreemap! {
        "request_type".to_string() => match request_type {
            CallOrQuery::Call => String("call".to_string()),
            CallOrQuery::Query => String("query".to_string()),
        },
        "canister_id".to_string() => Bytes(canister_id),
        "method_name".to_string() => String(method_name.to_string()),
        "arg".to_string() => Bytes(arg),
        "ingress_expiry".to_string() => U64(ingress_expiry),
        "sender".to_string() => Bytes(sender),
    };
    if let Some(some_nonce) = nonce {
        map.insert("nonce".to_string(), Bytes(some_nonce.to_vec()));
    }
    hash_of_map(&map)
}

pub(crate) fn representation_independent_hash_read_state(
    ingress_expiry: u64,
    paths: &[Path],
    sender: Vec<u8>,
    nonce: Option<&[u8]>,
) -> [u8; 32] {
    use RawHttpRequestVal::*;
    let mut map = btreemap! {
        "request_type".to_string() => String("read_state".to_string()),
        "ingress_expiry".to_string() => U64(ingress_expiry),
        "paths".to_string() => Array(paths
                .iter()
                .map(|p| {
                    RawHttpRequestVal::Array(
                        p.iter()
                            .map(|b| RawHttpRequestVal::Bytes(b.clone().into_vec()))
                            .collect(),
                    )
                })
                .collect()),
        "sender".to_string() => Bytes(sender),
    };
    if let Some(some_nonce) = nonce {
        map.insert("nonce".to_string(), Bytes(some_nonce.to_vec()));
    }
    hash_of_map(&map)
}

/// Describes the fields of a canister update call as defined in
/// `<https://internetcomputer.org/docs/current/references/ic-interface-spec#http-call>`.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct HttpCanisterUpdate {
    pub canister_id: Blob,
    pub method_name: String,
    pub arg: Blob,
    pub sender: Blob,
    /// Indicates when the message should expire.  Represented as nanoseconds
    /// since UNIX epoch.
    pub ingress_expiry: u64,
    // Do not include omitted fields in MessageId calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Blob>,
}

impl HttpCanisterUpdate {
    /// Returns the representation-independent hash.
    pub fn representation_independent_hash(&self) -> [u8; 32] {
        representation_independent_hash_call_or_query(
            CallOrQuery::Call,
            self.canister_id.0.clone(),
            &self.method_name,
            self.arg.0.clone(),
            self.ingress_expiry,
            self.sender.0.clone(),
            self.nonce.as_ref().map(|x| x.0.as_slice()),
        )
    }

    pub fn id(&self) -> MessageId {
        MessageId::from(self.representation_independent_hash())
    }
}

/// Describes the contents of a /api/v2/canister/_/call request.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "request_type")]
pub enum HttpCallContent {
    Call {
        #[serde(flatten)]
        update: HttpCanisterUpdate,
    },
}

impl HttpCallContent {
    /// Returns the representation-independent hash.
    pub fn representation_independent_hash(&self) -> [u8; 32] {
        let Self::Call { update } = self;
        update.representation_independent_hash()
    }

    pub fn ingress_expiry(&self) -> u64 {
        match self {
            Self::Call { update } => update.ingress_expiry,
        }
    }
}

/// Describes the fields of a canister query call (a query from a user to a canister) as
/// defined in `<https://internetcomputer.org/docs/current/references/ic-interface-spec#http-query>`.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpUserQuery {
    pub canister_id: Blob,
    pub method_name: String,
    pub arg: Blob,
    pub sender: Blob,
    /// Indicates when the message should expire.  Represented as nanoseconds
    /// since UNIX epoch.
    pub ingress_expiry: u64,
    // Do not include omitted fields in MessageId calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Blob>,
}

/// Describes the contents of a /api/v2/canister/_/query request.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "request_type")]
pub enum HttpQueryContent {
    Query {
        #[serde(flatten)]
        query: HttpUserQuery,
    },
}

impl HttpQueryContent {
    pub fn representation_independent_hash(&self) -> [u8; 32] {
        match self {
            Self::Query { query } => query.representation_independent_hash(),
        }
    }

    pub fn id(&self) -> MessageId {
        MessageId::from(self.representation_independent_hash())
    }
}

/// A `read_state` request as defined in `<https://internetcomputer.org/docs/current/references/ic-interface-spec#http-read-state>`.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpReadState {
    pub sender: Blob,
    // A list of paths, where a path is itself a sequence of labels.
    pub paths: Vec<Path>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Blob>,
    pub ingress_expiry: u64,
}

/// Describes the contents of a /api/v2/canister/_/read_state request.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "request_type")]
pub enum HttpReadStateContent {
    ReadState {
        #[serde(flatten)]
        read_state: HttpReadState,
    },
}

impl HttpReadStateContent {
    pub fn representation_independent_hash(&self) -> [u8; 32] {
        match self {
            Self::ReadState { read_state } => read_state.representation_independent_hash(),
        }
    }

    pub fn id(&self) -> MessageId {
        MessageId::from(self.representation_independent_hash())
    }
}

impl HttpUserQuery {
    /// Returns the representation-independent hash.
    pub fn representation_independent_hash(&self) -> [u8; 32] {
        representation_independent_hash_call_or_query(
            CallOrQuery::Query,
            self.canister_id.0.clone(),
            &self.method_name,
            self.arg.0.clone(),
            self.ingress_expiry,
            self.sender.0.clone(),
            self.nonce.as_ref().map(|x| x.0.as_slice()),
        )
    }
}

impl HttpReadState {
    /// Returns the representation-independent hash.
    pub fn representation_independent_hash(&self) -> [u8; 32] {
        representation_independent_hash_read_state(
            self.ingress_expiry,
            self.paths.as_slice(),
            self.sender.0.clone(),
            self.nonce.as_ref().map(|x| x.0.as_slice()),
        )
    }
}

/// A request envelope as defined in
/// `<https://internetcomputer.org/docs/current/references/ic-interface-spec#authentication>`.
///
/// The content is either [`HttpCallContent`], [`HttpQueryContent`] or
/// [`HttpReadStateContent`].
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct HttpRequestEnvelope<C> {
    pub content: C,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_pubkey: Option<Blob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_sig: Option<Blob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_delegation: Option<Vec<SignedDelegation>>,
}

/// A strongly-typed version of [`HttpRequestEnvelope`].
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
pub struct HttpRequest<C> {
    content: C,
    auth: Authentication,
}

/// The authentication associated with an HTTP request.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
pub enum Authentication {
    Authenticated(UserSignature),
    Anonymous,
}

/// Common attributes that all HTTP request contents should have.
pub trait HttpRequestContent {
    fn id(&self) -> MessageId;

    fn sender(&self) -> UserId;

    fn ingress_expiry(&self) -> u64;

    fn nonce(&self) -> Option<Vec<u8>>;
}

/// A trait implemented by HTTP requests that contain a `canister_id`.
pub trait HasCanisterId {
    fn canister_id(&self) -> CanisterId;
}

impl<C: HttpRequestContent> HttpRequest<C> {
    pub fn id(&self) -> MessageId {
        self.content.id()
    }

    pub fn sender(&self) -> UserId {
        self.content.sender()
    }

    pub fn ingress_expiry(&self) -> u64 {
        self.content.ingress_expiry()
    }

    pub fn nonce(&self) -> Option<Vec<u8>> {
        self.content.nonce()
    }
}

impl<C> HttpRequest<C> {
    pub fn content(&self) -> &C {
        &self.content
    }

    pub fn take_content(self) -> C {
        self.content
    }

    pub fn authentication(&self) -> &Authentication {
        &self.auth
    }
}

impl HttpRequestContent for Query {
    fn id(&self) -> MessageId {
        self.id()
    }

    fn sender(&self) -> UserId {
        match self.source {
            QuerySource::User { user_id, .. } => user_id,
            QuerySource::Anonymous => UserId::from(PrincipalId::default()),
        }
    }

    fn ingress_expiry(&self) -> u64 {
        match self.source {
            QuerySource::User { ingress_expiry, .. } => ingress_expiry,
            QuerySource::Anonymous => 0,
        }
    }

    fn nonce(&self) -> Option<Vec<u8>> {
        match &self.source {
            QuerySource::User { nonce, .. } => nonce.clone(),
            QuerySource::Anonymous => None,
        }
    }
}

impl HttpRequestContent for ReadState {
    fn id(&self) -> MessageId {
        self.id()
    }

    fn sender(&self) -> UserId {
        self.source
    }

    fn ingress_expiry(&self) -> u64 {
        self.ingress_expiry
    }

    fn nonce(&self) -> Option<Vec<u8>> {
        self.nonce.clone()
    }
}

impl TryFrom<HttpRequestEnvelope<HttpQueryContent>> for HttpRequest<Query> {
    type Error = HttpRequestError;

    fn try_from(envelope: HttpRequestEnvelope<HttpQueryContent>) -> Result<Self, Self::Error> {
        let auth = to_authentication(&envelope)?;
        match envelope.content {
            HttpQueryContent::Query { query } => Ok(HttpRequest {
                content: Query::try_from(query)?,
                auth,
            }),
        }
    }
}

impl TryFrom<HttpRequestEnvelope<HttpReadStateContent>> for HttpRequest<ReadState> {
    type Error = HttpRequestError;

    fn try_from(envelope: HttpRequestEnvelope<HttpReadStateContent>) -> Result<Self, Self::Error> {
        let auth = to_authentication(&envelope)?;
        match envelope.content {
            HttpReadStateContent::ReadState { read_state } => Ok(HttpRequest {
                content: ReadState::try_from(read_state)?,
                auth,
            }),
        }
    }
}

impl TryFrom<HttpRequestEnvelope<HttpCallContent>> for HttpRequest<SignedIngressContent> {
    type Error = HttpRequestError;

    fn try_from(envelope: HttpRequestEnvelope<HttpCallContent>) -> Result<Self, Self::Error> {
        let auth = to_authentication(&envelope)?;
        match envelope.content {
            HttpCallContent::Call { update } => Ok(HttpRequest {
                content: SignedIngressContent::try_from(update)?,
                auth,
            }),
        }
    }
}

/// Errors returned by `HttpHandler` when processing ingress messages.
#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum HttpRequestError {
    InvalidMessageId(String),
    InvalidIngressExpiry(String),
    InvalidDelegationExpiry(String),
    InvalidPrincipalId(String),
    MissingPubkeyOrSignature(String),
    InvalidEncoding(String),
}

impl From<serde_cbor::Error> for HttpRequestError {
    fn from(err: serde_cbor::Error) -> Self {
        HttpRequestError::InvalidEncoding(format!("{}", err))
    }
}

impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpRequestError::InvalidMessageId(msg) => write!(f, "invalid message ID: {}", msg),
            HttpRequestError::InvalidIngressExpiry(msg) => write!(f, "{}", msg),
            HttpRequestError::InvalidDelegationExpiry(msg) => write!(f, "{}", msg),
            HttpRequestError::InvalidPrincipalId(msg) => write!(f, "invalid princial id: {}", msg),
            HttpRequestError::MissingPubkeyOrSignature(msg) => {
                write!(f, "missing pubkey or signature: {}", msg)
            }
            HttpRequestError::InvalidEncoding(err) => write!(f, "Invalid CBOR encoding: {}", err),
        }
    }
}

impl Error for HttpRequestError {}

impl From<CanisterIdError> for HttpRequestError {
    fn from(err: CanisterIdError) -> Self {
        Self::InvalidPrincipalId(format!("Converting to canister id failed with {}", err))
    }
}

/// Describes a delegation map as defined in
/// `<https://internetcomputer.org/docs/current/references/ic-interface-spec#certification-delegation>`.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Delegation {
    pubkey: Blob,
    expiration: Time,
    targets: Option<Vec<Blob>>,
}

impl Delegation {
    pub fn new(pubkey: Vec<u8>, expiration: Time) -> Self {
        Self {
            pubkey: Blob(pubkey),
            expiration,
            targets: None,
        }
    }

    pub fn new_with_targets(pubkey: Vec<u8>, expiration: Time, targets: Vec<CanisterId>) -> Self {
        Self {
            pubkey: Blob(pubkey),
            expiration,
            targets: Some(targets.iter().map(|c| Blob(c.get().to_vec())).collect()),
        }
    }

    pub fn pubkey(&self) -> &Vec<u8> {
        &self.pubkey.0
    }

    pub fn expiration(&self) -> Time {
        self.expiration
    }

    pub fn targets(&self) -> Result<Option<BTreeSet<CanisterId>>, String> {
        match &self.targets {
            None => Ok(None),
            Some(targets) => {
                let mut target_canister_ids = BTreeSet::new();
                for target in targets {
                    target_canister_ids.insert(CanisterId::unchecked_from_principal(
                        PrincipalId::try_from(target.0.as_slice())
                            .map_err(|e| format!("Error parsing canister ID: {}", e))?,
                    ));
                }
                Ok(Some(target_canister_ids))
            }
        }
    }

    pub fn number_of_targets(&self) -> Option<usize> {
        self.targets.as_ref().map(Vec::len)
    }
}

impl SignedBytesWithoutDomainSeparator for Delegation {
    fn as_signed_bytes_without_domain_separator(&self) -> Vec<u8> {
        use RawHttpRequestVal::*;

        let mut map = btreemap! {
            "pubkey" => Bytes(self.pubkey.0.clone()),
            "expiration" => U64(self.expiration.as_nanos_since_unix_epoch()),
        };
        if let Some(targets) = &self.targets {
            map.insert(
                "targets",
                Array(targets.iter().map(|t| Bytes(t.0.clone())).collect()),
            );
        }

        hash_of_map(&map).to_vec()
    }
}

/// Describes a delegation as defined in
/// `<https://internetcomputer.org/docs/current/references/ic-interface-spec#certification-delegation>`.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SignedDelegation {
    delegation: Delegation,
    signature: Blob,
}

impl SignedDelegation {
    pub fn new(delegation: Delegation, signature: Vec<u8>) -> Self {
        Self {
            delegation,
            signature: Blob(signature),
        }
    }

    pub fn delegation(&self) -> &Delegation {
        &self.delegation
    }

    pub fn take_delegation(self) -> Delegation {
        self.delegation
    }

    pub fn signature(&self) -> &Blob {
        &self.signature
    }
}

/// The different types of values supported in `RawHttpRequest`.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Deserialize, Serialize)]
pub enum RawHttpRequestVal {
    Bytes(#[serde(with = "serde_bytes")] Vec<u8>),
    String(String),
    U64(u64),
    Array(Vec<RawHttpRequestVal>),
    Map(BTreeMap<String, RawHttpRequestVal>),
}

/// The reply to an update call.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum HttpReply {
    CodeCall { arg: Blob },
    Empty {},
}

/// The response for a query call from the execution service.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "status")]
pub enum HttpQueryResponse {
    Replied {
        reply: HttpQueryResponseReply,
    },
    Rejected {
        error_code: String,
        reject_code: u64,
        reject_message: String,
    },
}

/// Wraps the hash of a query response as described
/// in the [IC interface-spec](https://internetcomputer.org/docs/current/references/ic-interface-spec#http-query).
pub struct QueryResponseHash([u8; 32]);

impl QueryResponseHash {
    /// Creates a [`QueryResponseHash`] from a given query response, request and timestamp.
    pub fn new(response: &HttpQueryResponse, request: &Query, timestamp: Time) -> Self {
        use RawHttpRequestVal::*;

        let self_map_representation = match response {
            HttpQueryResponse::Replied { reply } => {
                let map_of_reply = btreemap! {
                    "arg".to_string() => RawHttpRequestVal::Bytes(reply.arg.0.clone()),
                };

                btreemap! {
                    "request_id".to_string() => Bytes(request.id().as_bytes().to_vec()),
                    "status".to_string() => String("replied".to_string()),
                    "timestamp".to_string() => U64(timestamp.as_nanos_since_unix_epoch()),
                    "reply".to_string() => Map(map_of_reply)
                }
            }
            HttpQueryResponse::Rejected {
                error_code,
                reject_code,
                reject_message,
            } => {
                btreemap! {
                    "request_id".to_string() => Bytes(request.id().as_bytes().to_vec()),
                    "status".to_string() => String("rejected".to_string()),
                    "timestamp".to_string() => U64(timestamp.as_nanos_since_unix_epoch()),
                    "reject_code".to_string() => U64(*reject_code),
                    "reject_message".to_string() => String(reject_message.to_string()),
                    "error_code".to_string() => String(error_code.to_string()),

                }
            }
        };

        let hash = hash_of_map(&self_map_representation);

        Self(hash)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl SignedBytesWithoutDomainSeparator for QueryResponseHash {
    fn as_signed_bytes_without_domain_separator(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// The response to `/api/v2/canister/_/query`.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpSignedQueryResponse {
    #[serde(flatten)]
    pub response: HttpQueryResponse,

    /// The signature of this replica node for the query response.
    ///
    /// Note:
    /// To follow the IC specification for signed query responses,
    /// the serializer will during serialization:
    /// - rename the field: `node_signature` -> `signatures`.
    /// - Convert the signature to a 1-tuple containing only this signature.
    #[serde(serialize_with = "serialize_node_signature_to_1_tuple")]
    #[serde(rename = "signatures")]
    pub node_signature: NodeSignature,
}

/// Serializes a `NodeSignature` to a 1-tuple containing only that one signature.
fn serialize_node_signature_to_1_tuple<S>(
    signature: &NodeSignature,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut tup = serializer.serialize_tuple(1)?;
    tup.serialize_element(signature)?;
    tup.end()
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct NodeSignature {
    /// The time of creation of the signature (or the batch time).
    pub timestamp: Time,
    /// The actual signature.
    pub signature: Blob,
    /// The node id of the node that created this signature.
    pub identity: NodeId,
}

/// The body of the `QueryResponse`
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpQueryResponseReply {
    pub arg: Blob,
}

/// The response to a `read_state` request.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpReadStateResponse {
    /// The CBOR-encoded `Certificate`.
    pub certificate: Blob,
}

/// A `Certificate` as defined in `<https://internetcomputer.org/docs/current/references/ic-interface-spec#certificate>`
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Certificate {
    pub tree: MixedHashTree,
    pub signature: Blob,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation: Option<CertificateDelegation>,
}

/// A `CertificateDelegation` as defined in `<https://internetcomputer.org/docs/current/references/ic-interface-spec#certification-delegation>`
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct CertificateDelegation {
    pub subnet_id: Blob,
    pub certificate: Blob,
}

/// Different stages required for the full initialization of the HTTPS endpoint.
/// The fields are listed in order of execution/transition.
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRefStr, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplicaHealthStatus {
    /// Marks the start state of the HTTPS endpoint. Some requests will fail
    /// while initialization is on-going.
    Starting,
    /// Waiting for the first non-empty certifited state available on
    /// the node.
    WaitingForCertifiedState,
    /// Waiting for the root delegation in case of non-NNS subnet.
    WaitingForRootDelegation,
    /// Happens when the replica's finalized height is significantly greater
    /// than the certified height.
    /// If the finalized height is significantly greater than the
    /// certified height, this is a signal that execution is lagging
    /// consensus, and that consensus needs to be throttled.
    /// More information can be found in the whitepaper
    /// `<https://internetcomputer.org/whitepaper.pdf>`
    /// under "Per-round certified state" section(s).
    ///
    /// If execution (or certification) is lagging significantly on this replica,
    /// then we better not serve queries because we risk returning stale data.
    /// According to the IC's spec - `<https://internetcomputer.org/docs/current/references/ic-interface-spec#query_call>`,
    /// we should execute queries on "recent enough" state tree.
    CertifiedStateBehind,
    /// Signals that the replica can serve all types of API requests.
    /// When users programmatically access this information they should
    /// check only if 'ReplicaHealthStatus' is equal to 'Healthy' or not.
    Healthy,
}

/// The response to `/api/v2/status`.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impl_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_key: Option<Blob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impl_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_health_status: Option<ReplicaHealthStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified_height: Option<Height>,
}

fn to_authentication<C>(env: &HttpRequestEnvelope<C>) -> Result<Authentication, HttpRequestError> {
    match (&env.sender_pubkey, &env.sender_sig, &env.sender_delegation) {
        (Some(pubkey), Some(signature), delegation) => {
            Ok(Authentication::Authenticated(UserSignature {
                signature: signature.0.clone(),
                signer_pubkey: pubkey.0.clone(),
                sender_delegation: delegation.clone(),
            }))
        }
        (None, None, None) => Ok(Authentication::Anonymous),
        rest => Err(HttpRequestError::MissingPubkeyOrSignature(format!(
            "Got {:?}",
            rest
        ))),
    }
}
