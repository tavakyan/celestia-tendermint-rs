#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Txs {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeenTx {
    #[prost(bytes = "vec", tag = "1")]
    pub tx_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WantTx {
    #[prost(bytes = "vec", tag = "1")]
    pub tx_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof = "message::Sum", tags = "1, 2, 3")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        Txs(super::Txs),
        #[prost(message, tag = "2")]
        SeenTx(super::SeenTx),
        #[prost(message, tag = "3")]
        WantTx(super::WantTx),
    }
}
