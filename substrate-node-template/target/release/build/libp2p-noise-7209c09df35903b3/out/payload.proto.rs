// Payloads for Noise handshake messages.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identity {
    #[prost(bytes, tag="1")]
    pub pubkey: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub signature: std::vec::Vec<u8>,
}
