pub type FeatureRaw = u16;

pub trait Feature {
    fn into_feature(self) -> FeatureRaw;
}

macro_rules! impl_cast {
    ( $( $typ:ty $(,)* )* ) => {
        $(
            impl Feature for $typ {
                fn into_feature(self) -> FeatureRaw {
                    self as FeatureRaw
                }
            }
        )*
    };
}

impl_cast!(u8, i8, u16);
