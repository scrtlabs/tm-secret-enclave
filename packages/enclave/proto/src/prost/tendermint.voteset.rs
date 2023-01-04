use tendermint_proto::types::SignedMsgType;
use tendermint_proto::types::ValidatorSet;
use tendermint_proto::types::Vote;
/// HasVote is sent to indicate that a particular vote has been received.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteSet {
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub height: i64,
    #[prost(int32, tag="3")]
    pub round: i32,
    #[prost(enumeration="SignedMsgType", tag="4")]
    pub r#type: i32,
    #[prost(message, tag="5")]
    pub valset: ::core::option::Option<ValidatorSet>,
    #[prost(message, optional, tag="6")]
    pub votes_bit_array: ::core::option::Option<tendermint_proto::libs::bits::BitArray>,
    #[prost(message, repeated, tag="7")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    #[prost(int64, tag="8")]
    pub sum: i64,
}
