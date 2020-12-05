/*
 * Created on Sat Nov 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

mod command_processor;

mod header;

use serde::{Deserialize, Serialize};
use crate::{Encode, Decode};

pub use command_processor::CommandProcessor;
pub use header::Header;

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