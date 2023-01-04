mod tm_proto;
use prost::{encoding::encoded_len_varint, Message};
use core::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

pub use error::Error;

extern crate alloc;

pub trait Protobuf<T: Message + From<Self> + Default>
    where
        Self: Sized + Clone + TryFrom<T>,
        <Self as TryFrom<T>>::Error: Display,
{
    /// Encode into a buffer in Protobuf format.
    ///
    /// Uses [`prost::Message::encode`] after converting into its counterpart
    /// Protobuf data structure.
    ///
    /// [`prost::Message::encode`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encode
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), Error> {
        T::from(self.clone())
            .encode(buf)
            .map_err(Error::encode_message)
    }

    /// Encode with a length-delimiter to a buffer in Protobuf format.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    ///
    /// Uses [`prost::Message::encode_length_delimited`] after converting into
    /// its counterpart Protobuf data structure.
    ///
    /// [`prost::Message::encode_length_delimited`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encode_length_delimited
    fn encode_length_delimited<B: BufMut>(&self, buf: &mut B) -> Result<(), Error> {
        T::from(self.clone())
            .encode_length_delimited(buf)
            .map_err(Error::encode_message)
    }

    /// Constructor that attempts to decode an instance from a buffer.
    ///
    /// The entire buffer will be consumed.
    ///
    /// Similar to [`prost::Message::decode`] but with additional validation
    /// prior to constructing the destination type.
    ///
    /// [`prost::Message::decode`]: https://docs.rs/prost/*/prost/trait.Message.html#method.decode
    fn decode<B: Buf>(buf: B) -> Result<Self, Error> {
        let raw = T::decode(buf).map_err(Error::decode_message)?;

        Self::try_from(raw).map_err(Error::try_from::<T, Self, _>)
    }

    /// Constructor that attempts to decode a length-delimited instance from
    /// the buffer.
    ///
    /// The entire buffer will be consumed.
    ///
    /// Similar to [`prost::Message::decode_length_delimited`] but with
    /// additional validation prior to constructing the destination type.
    ///
    /// [`prost::Message::decode_length_delimited`]: https://docs.rs/prost/*/prost/trait.Message.html#method.decode_length_delimited
    fn decode_length_delimited<B: Buf>(buf: B) -> Result<Self, Error> {
        let raw = T::decode_length_delimited(buf).map_err(Error::decode_message)?;

        Self::try_from(raw).map_err(Error::try_from::<T, Self, _>)
    }

    /// Returns the encoded length of the message without a length delimiter.
    ///
    /// Uses [`prost::Message::encoded_len`] after converting to its
    /// counterpart Protobuf data structure.
    ///
    /// [`prost::Message::encoded_len`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encoded_len
    fn encoded_len(&self) -> usize {
        T::from(self.clone()).encoded_len()
    }

    /// Encodes into a Protobuf-encoded `Vec<u8>`.
    fn encode_vec(&self) -> Result<Vec<u8>, Error> {
        let mut wire = Vec::with_capacity(self.encoded_len());
        self.encode(&mut wire).map(|_| wire)
    }

    /// Constructor that attempts to decode a Protobuf-encoded instance from a
    /// `Vec<u8>` (or equivalent).
    fn decode_vec(v: &[u8]) -> Result<Self, Error> {
        Self::decode(v)
    }

    /// Encode with a length-delimiter to a `Vec<u8>` Protobuf-encoded message.
    fn encode_length_delimited_vec(&self) -> Result<Vec<u8>, Error> {
        let len = self.encoded_len();
        let lenu64 = len.try_into().map_err(Error::parse_length)?;
        let mut wire = Vec::with_capacity(len + encoded_len_varint(lenu64));
        self.encode_length_delimited(&mut wire).map(|_| wire)
    }

    /// Constructor that attempts to decode a Protobuf-encoded instance with a
    /// length-delimiter from a `Vec<u8>` or equivalent.
    fn decode_length_delimited_vec(v: &[u8]) -> Result<Self, Error> {
        Self::decode_length_delimited(v)
    }
}
