use bevy::prelude::*;

pub fn control_digger(keys: Res<Input<KeyCode>>) {
	println!("{:?}", keys);
}
