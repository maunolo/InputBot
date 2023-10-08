use crate::public::*;
use once_cell::sync::Lazy;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::{Arc, Mutex},
    thread::spawn,
};

pub enum Bind {
    Normal(Handler),
    Block(BlockHandler),
    Blockable(BlockableHandler),
}

pub enum MouseEventBind {
    Normal(MouseEventHandler),
    Block(BlockMouseEventHandler),
    Blockable(BlockableMouseEventHandler),
}

pub type Handler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type MouseEventHandler = Arc<dyn Fn(MouseEvent) + Send + Sync + 'static>;
pub type BlockHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockMouseEventHandler = Arc<dyn Fn(MouseEvent) + Send + Sync + 'static>;
pub type BlockableHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;
pub type BlockableMouseEventHandler = Arc<dyn Fn(MouseEvent) -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseButtonBindMap = HashMap<MouseButton, Bind>;
pub type MouseEventBindMap = HashMap<MouseEvent, MouseEventBind>;

pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static MOUSE_BUTTON_BINDS: Lazy<Mutex<MouseButtonBindMap>> =
    Lazy::new(|| Mutex::new(MouseButtonBindMap::new()));
pub static MOUSE_EVENT_BINDS: Lazy<Mutex<MouseEventBindMap>> =
    Lazy::new(|| Mutex::new(MouseEventBindMap::new()));
