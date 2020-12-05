/*
 * Created on Sat Nov 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::io::{Write, Read};

pub mod command;

pub trait Encode {
	fn encode<T: Write>(&self, buffer: &mut T);
}

pub trait Decode {
	fn decode<T: Read>(&self, buffer: &mut T) -> Self;
}