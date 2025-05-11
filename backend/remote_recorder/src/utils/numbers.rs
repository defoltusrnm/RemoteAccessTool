use anyhow::Context;

pub trait EndianRead {
    type Array;
    fn from_le_bytes(bytes: Self::Array) -> Self;
    fn from_be_bytes(bytes: Self::Array) -> Self;
    fn from_slice(bytes: &[u8]) -> Result<Self::Array, anyhow::Error>;
    fn size() -> usize;
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
        }
    )*
});

impl_EndianRead_for_ints!(u8, u16, u32, u64, usize);
