/*
 * Created on Sat Nov 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod command_processor;

mod header;

use serde::{Deserialize, Serialize};
use crate::command::header::Header;
use crate::{Encode, Decode};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Command {
    pub header: Header,
    pub data: Vec<u8>,
}

pub trait NamedCommand {
    const NAME: &'static str;
}

pub trait CommandData: Encode + Decode + NamedCommand{}

pub trait CommandPair: NamedCommand {}

pub trait PairedCommandRequest<T: CommandPair>: Encode + Decode + NamedCommand {}

pub trait PairedCommandResponse<T: CommandPair>: Encode + Decode + NamedCommand {}