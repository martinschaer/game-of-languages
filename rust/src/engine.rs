use crate::ActionKeys;
use crate::State;

pub struct Action {
    pub key: ActionKeys,
    pub desc: &'static str,
    pub cb: Option<fn(state: &mut State)>,
}

macro_rules! actions {
    ($($name:ident => ($value:expr, $label:expr, $desc:expr, $fn:expr)),*) => {
        pub enum ActionKeys {
            $($name = $value,)*
        }
        struct Actions;
        impl Actions {
            $(
                pub const $name: Action = Action {
                    key: ActionKeys::$name,
                    desc: $desc,
                    cb: $fn,
                };
            )*
        }
        pub const MENU: &'static str = concat![$(stringify!($value), ": ", $label, ", "),*];
        pub fn get_action(key: i32) -> Option<Action> {
            match key {
                $($value => Some(Actions::$name),)*
                _ => None
            }
        }
    };
}

pub(crate) use actions;
