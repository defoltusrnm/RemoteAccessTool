use anyhow::Context;

pub trait EndianRead
where
    Self: Sized,
{
    type Array;
    fn from_le_bytes(bytes: Self::Array) -> Self;
    fn from_be_bytes(bytes: Self::Array) -> Self;
    fn from_slice(bytes: &[u8]) -> Result<Self::Array, anyhow::Error>;
    fn size() -> usize;

    fn le_bytes(&self) -> Vec<u8>;
    fn be_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_EndianRead_for_ints (( $($int:ident),* ) => {
    $(
        impl EndianRead for $int {
            type Array = [u8; std::mem::size_of::<Self>()];
            fn from_le_bytes(bytes: Self::Array) -> Self { Self::from_le_bytes(bytes) }
            fn from_be_bytes(bytes: Self::Array) -> Self { Self::from_be_bytes(bytes) }
            fn size() -> usize { std::mem::size_of::<Self>() }

            fn from_slice(bytes: &[u8]) -> Result<Self::Array, anyhow::Error> {
                bytes.try_into().with_context(|| "failed to parse &u8")
            }

            fn le_bytes(&self) -> Vec<u8> { self.to_le_bytes().as_slice().to_owned() }
            fn be_bytes(&self) -> Vec<u8> { self.to_be_bytes().as_slice().to_owned() }
        }
    )*
});

impl_EndianRead_for_ints!(u8, u16, u32, u64, usize);
