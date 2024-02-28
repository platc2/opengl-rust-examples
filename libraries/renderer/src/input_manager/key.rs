use imgui::Key as ImguiKey;
use sdl2::keyboard::Scancode as SdlKey;

use super::key_helpers::{generate_key_definitions, generate_key_expressions, generate_key_transformers};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Key {
    pub(crate) imgui: ImguiKey,
    pub(crate) sdl: SdlKey,
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
    UP_ARROW -> (UpArrow, Up),
    LEFT_CONTROL -> (LeftCtrl, LCtrl),
    LEFT_SHIFT -> (LeftShift, LShift),
    LEFT_ALT -> (LeftAlt, LAlt),
    RIGHT_CONTROL -> (RightCtrl, RCtrl),
    RIGHT_SHIFT -> (RightShift, RShift),
    RIGHT_ALT -> (RightAlt, RAlt),
    PERIOD -> Period
);
