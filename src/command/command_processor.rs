/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//todo error handle

use std::io::{Read, Write};

use std::ops::{Deref, DerefMut};
use bytes::Buf;
use crate::command::Command;

/// Like BufReader and BufWriter, provide optimized Command read/write operation to stream.
pub struct CommandProcessor<S: Read + Write> {
    
    pub stream: S,
    read_buffer: [u8; 2048]
    
}

impl<S: Read + Write> CommandProcessor<S> {

    pub fn new(stream: S) -> Self {
        Self {
            stream,
            read_buffer: [0u8; 2048]
        }
    }

    fn read_command(&mut self) -> Option<Command> {
        match self.stream.read(&mut self.read_buffer) {
            Ok(read_bytes) => {
                if read_bytes < 22 {
                    return None;
                }
                let buffer = &self.read_buffer[0..read_bytes];
                Some(bincode::deserialize(buffer).unwrap())
            },
            Err(e) => None
        }
    }

    /// Write command to stream.
    pub fn write_command(&mut self, command: Command) {
        let mut cursor = Vec::with_capacity(command.header.data_size as usize + 22);
        bincode::serialize_into(&mut cursor, &command);

        self.write(&cursor).unwrap();
    }

}

impl<T: Write + Read> Deref for CommandProcessor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<T: Write + Read> DerefMut for CommandProcessor<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}