use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Lock(pub u8, pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Key(pub u8, pub u8, pub u8, pub u8, pub u8);

pub fn usable_key_on_lock(key: &Key, lock: &Lock) -> bool {
    (lock.0 + key.0) <= 7
        && (lock.1 + key.1) <= 7
        && (lock.2 + key.2) <= 7
        && (lock.3 + key.3) <= 7
        && (lock.4 + key.4) <= 7
}
