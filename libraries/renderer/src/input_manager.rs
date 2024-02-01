use std::collections::HashSet;

use imgui::Key as ImguiKey;
use sdl2::keyboard::Scancode as SdlKey;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Key {
    pub(crate) imgui: ImguiKey,
    pub(crate) sdl: SdlKey,
}

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

impl Key {
    const fn new(imgui: ImguiKey, sdl: SdlKey) -> Self { Self { imgui, sdl } }

    // Generate key definitions for Mod-Keys
    generate_key_definitions!(
        MOD_CONTROL -> (ModCtrl, LCtrl),
        MOD_SHIFT -> (ModShift, LShift),
        MOD_ALT -> (ModAlt, LAlt)
    );
}

generate_key_expressions!(
    A -> A,
    B -> B,
    C -> C,
    D -> D,
    E -> E,
    F -> F,
    G -> G,
    H -> H,
    I -> I,
    J -> J,
    K -> K,
    L -> L,
    M -> M,
    N -> N,
    O -> O,
    P -> P,
    Q -> Q,
    R -> R,
    S -> S,
    T -> T,
    U -> U,
    V -> V,
    W -> W,
    X -> X,
    Y -> Y,
    Z -> Z,
    ONE -> (Alpha1, Num1),
    TWO -> (Alpha2, Num2),
    THREE -> (Alpha3, Num3),
    FOUR -> (Alpha4, Num4),
    FIVE -> (Alpha5, Num5),
    SIX -> (Alpha6, Num6),
    SEVEN -> (Alpha7, Num7),
    EIGHT -> (Alpha8, Num8),
    NINE -> (Alpha9, Num9),
    ZERO -> (Alpha0, Num0),
    ENTER -> (Enter, Return),
    ESCAPE -> (Escape, Escape),
    BACKSPACE -> Backspace,
    TAB -> Tab,
    SPACE -> Space,
    CAPS_LOCK -> CapsLock,
    F1 -> F1,
    F2 -> F2,
    F3 -> F3,
    F4 -> F4,
    F5 -> F5,
    F6 -> F6,
    F7 -> F7,
    F8 -> F8,
    F9 -> F9,
    F10 -> F10,
    F11 -> F11,
    F12 -> F12,
    PRINT_SCREEN -> PrintScreen,
    SCROLL_LOCK -> ScrollLock,
    PAUSE -> Pause,
    INSERT -> Insert,
    HOME -> Home,
    PAGE_UP -> PageUp,
    DELETE -> Delete,
    END -> End,
    PAGE_DOWN -> PageDown,
    RIGHT_ARROW -> (RightArrow, Right),
    LEFT_ARROW -> (LeftArrow, Left),
    DOWN_ARROW -> (DownArrow, Down),
    UO_ARROW -> (UpArrow, Up),
    LEFT_CONTROL -> (LeftCtrl, LCtrl),
    LEFT_SHIFT -> (LeftShift, LShift),
    LEFT_ALT -> (LeftAlt, LAlt),
    RIGHT_CONTROL -> (RightCtrl, RCtrl),
    RIGHT_SHIFT -> (RightShift, RShift),
    RIGHT_ALT -> (RightAlt, RAlt),
    PERIOD -> Period
);

pub trait InputManager {
    #[must_use]
    fn key_pressed(&self, key: Key) -> bool;

    #[must_use]
    fn key_released(&self, key: Key) -> bool;

    #[must_use]
    fn key_down(&self, key: Key) -> bool;

    #[must_use]
    fn key_up(&self, key: Key) -> bool { !self.key_down(key) }
}

#[derive(Default)]
pub(crate) struct SdlInputManager {
    last_pressed_keys: HashSet<Key>,
    curr_pressed_keys: HashSet<Key>,
}

impl InputManager for SdlInputManager {
    fn key_pressed(&self, key: Key) -> bool {
        self.key_down(key) && !self.last_pressed_keys.contains(&key)
    }

    fn key_released(&self, key: Key) -> bool {
        self.key_up(key) && self.last_pressed_keys.contains(&key)
    }

    fn key_down(&self, key: Key) -> bool {
        self.curr_pressed_keys.contains(&key)
    }

    fn key_up(&self, key: Key) -> bool {
        !self.curr_pressed_keys.contains(&key)
    }
}

impl SdlInputManager {
    pub fn update(&mut self) {
        self.last_pressed_keys.clear();
        for key in self.curr_pressed_keys.clone().into_iter() {
            self.last_pressed_keys.insert(key);
        }
    }

    pub fn set_key_down(&mut self, key: SdlKey) {
        if let Ok(k) = key.try_into() {
            self.curr_pressed_keys.insert(k);
        }
    }

    pub fn set_key_up(&mut self, key: SdlKey) {
        if let Ok(k) = key.try_into() {
            self.curr_pressed_keys.remove(&k);
        }
    }
}
