use std::collections::HashSet;

use crate::core::resources::inputs::{keycode::KeyCode, InputState, KeyboardEvent};

#[derive(Default)]
pub struct Keyboard {
    pressed_keys: HashSet<KeyCode>,
    keyboard_events: Vec<KeyboardEvent>,
}

impl Keyboard {
    pub fn key_pressed(&self, key: &KeyCode) -> bool { self.pressed_keys.contains(key) }

    pub fn keyboard_events(&self) -> &Vec<KeyboardEvent> { &self.keyboard_events }

    pub fn on_key_pressed<Body>(&self, key: KeyCode, mut action: Body)
    where
        Body: FnMut(), {
        if self
            .keyboard_events
            .iter()
            .filter(|e| e.keycode == key && e.state == InputState::Pressed)
            .count()
            > 0
        {
            action()
        }
    }

    pub fn on_key_released<Body>(&self, key: KeyCode, mut action: Body)
    where
        Body: FnMut(), {
        if self
            .keyboard_events
            .iter()
            .filter(|e| e.keycode == key && e.state == InputState::Released)
            .count()
            > 0
        {
            action()
        }
    }

    pub fn clear_events(&mut self) { self.keyboard_events.clear(); }

    pub fn add_keyboard_event(&mut self, keyboard_event: KeyboardEvent) {
        if match &keyboard_event {
            KeyboardEvent { keycode, state } => {
                match state {
                    InputState::Pressed => self.press(keycode),
                    InputState::Released => self.release(keycode),
                }
            }
        } {
            self.keyboard_events.push(keyboard_event);
        }
    }

    /// Set the given key to pressed status if it is not present in the registered pressed key. Return true if it was updated
    pub(crate) fn press(&mut self, key: &KeyCode) -> bool {
        if self.pressed_keys.contains(key) {
            false
        } else {
            self.pressed_keys.insert(key.clone());
            true
        }
    }

    /// Set the given key to released status if it present in the registered pressed key. Return true if it was updated
    pub(crate) fn release(&mut self, key: &KeyCode) -> bool {
        if self.pressed_keys.contains(key) {
            self.pressed_keys.remove(&key);
            true
        } else {
            false
        }
    }
}
