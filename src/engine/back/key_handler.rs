#![allow(dead_code)]

use std::collections::HashSet;
use winit::event::MouseButton;
use winit::keyboard::{Key, NamedKey, SmolStr};

pub struct InputHandler {
    left_click: bool,

    pub pressed_keys: HashSet<Key>,
}

impl InputHandler {
    pub(crate) fn clone(&self) -> Self {
        Self {
            left_click: self.left_click,

            pressed_keys: self.pressed_keys.clone(),
        }
    }
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            left_click: false,

            pressed_keys: HashSet::new(),
        }
    }

    pub fn update_keys(&mut self, key: Key, pressed: bool) {
        if pressed {
            self.pressed_keys.insert(key.clone());
        } else {
            self.pressed_keys.remove(&key);
        }
        //Key::Named(NamedKey::ArrowUp) => self.up = pressed,
    }

    pub fn update_mouse(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.left_click = pressed,
            _ => {}
        }
    }
}