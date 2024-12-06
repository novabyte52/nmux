#[macro_export]
macro_rules! define_error {
    ($name:ident, $($variant:ident($type:ty)),* $(,)?) => {
        #[derive(Debug)]
        pub enum $name {
            $($variant($type),)*
        }

        // Implement the `std::error::Error` trait for the unified error type
        impl std::error::Error for $name {}

        // Implement `std::fmt::Display` for better error descriptions
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(err) => write!(f, "{}", err),)*
                }
            }
        }

        // Automatically implement `From` for each error type
        $(impl From<$type> for $name {
            fn from(err: $type) -> Self {
                Self::$variant(err)
            }
        })*
    };
}
