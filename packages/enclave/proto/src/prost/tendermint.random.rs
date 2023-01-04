//// Data contains the set of transactions included in the block
// message EncryptedRandom {
//   bytes random = 1;
//   bytes proof = 2;
// }

/// HasVote is sent to indicate that a particular vote has been received.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedRandom {
    #[prost(bytes="bytes", tag="1")]
    pub random: ::prost::bytes::Bytes,
    #[prost(bytes="bytes", tag="2")]
    pub proof: ::prost::bytes::Bytes,
}
