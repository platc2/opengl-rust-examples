macro_rules! generate_key_definitions {
    (@($($t:tt)+)) => {
        impl Key {
            generate_key_definitions!($($t)+);
        }
    };
    ($name:ident -> $value:ident) => {
        pub const $name: Key = Key::new(ImguiKey::$value, SdlKey::$value);
    };
    ($name:ident -> ($imgui:ident, $sdl:ident)) => {
        pub const $name: Key = Key::new(ImguiKey::$imgui, SdlKey::$sdl);
    };
    ($name:ident -> $value:ident, $($tail:tt)*) => {
        generate_key_definitions!($name -> $value);
        generate_key_definitions!($($tail)*);
    };
    ($name:ident -> ($imgui:ident, $sdl:ident), $($tail:tt)*) => {
        generate_key_definitions!($name -> ($imgui, $sdl));
        generate_key_definitions!($($tail)*);
    };
}

macro_rules! generate_key_transformers {
    (@($name:ident -> $value:ident, $($tail:tt)+) $($arms:tt)*) => {
        generate_key_transformers!(
            @($($tail)*)
            $($arms)*
            SdlKey::$value => Ok(Key::$name),
        );
    };
    (@($name:ident -> ($imgui:ident, $sdl:ident), $($tail:tt)*) $($arms:tt)*) => {
        generate_key_transformers!(
            @($($tail)*)
            $($arms)*
            SdlKey::$sdl => Ok(Key::$name),
        );
    };
    (@($name:ident -> $value:ident) $($arms:tt)*) => {
        generate_key_transformers!(
            @()
            $($arms)*
            SdlKey::$value => Ok(Key::$name),
        );
    };
    (@($name:ident -> ($imgui:ident, $sdl:ident)) $($arms:tt)*) => {
        generate_key_transformers!(
            @()
            $($arms)*
            SdlKey::$sdl => Ok(Key::$name),
        );
    };
    (@() $($arms:tt)*) => {
        impl TryInto<Key> for SdlKey {
            type Error = ();
            fn try_into(self) -> Result<Key, Self::Error> {
                match self {
                    $($arms)*
                    _ => {
                        eprintln!("SDL key '{self:?}' not mapped!");
                        Err(())
                    }
                }
            }
        }
    };
}

macro_rules! generate_key_expressions {
    ($($t:tt)+) => {
        generate_key_definitions!(@($($t)+));
        generate_key_transformers!(@($($t)+));
    };
}

pub(crate) use {generate_key_definitions, generate_key_transformers, generate_key_expressions};
