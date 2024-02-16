// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! enum_serialize {
    ( $name:ident -> $desc:expr, $( $value:ident => $str:expr $( ; $opt:expr )*, )+ ) => {
        #[allow(deprecated)]
        impl $name {
            /// String representation of the variant.
            pub fn as_str(&self) -> &'static str {
                match *self {
                    $( $name::$value => $str, )*
                }
            }
        }

        #[allow(deprecated)]
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where D: Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;

                match val.as_str() {
                    $( $str $( | $opt )* => Ok($name::$value), )*
                    v => {
                        error!(target: "gitlab", concat!("unknown ", $desc, " from gitlab: {}"), v);
                        Err(D::Error::unknown_variant(v, &[$( $str, $( $opt, )* )*]))
                    },
                }
            }
        }
    };
}
