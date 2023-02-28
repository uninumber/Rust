struct MyByteArray<const N: usize>([u8; N]);

impl<const N: usize> TryFrom<&str> for MyByteArray<N> {
    type Error = std::array::TryFromSliceError;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(s.as_bytes().try_into()?))
    }
}

fn main() {
}
