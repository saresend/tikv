#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec::Vec;

use raftserver::{Result, ConnData, Sender, TimerMsg};

pub trait ServerHandler :Sized {
    // Handle messages reading from connection with Token.
    // Return some messages for later writing to this connection.
    fn handle_read_data(&mut self,
                        sender: &Sender,
                        msgs: Vec<ConnData>)
                        -> Result<(Vec<ConnData>)> {
        Ok((msgs))
    }

    fn handle_tick(&mut self, sender: &Sender) -> Result<()> {
        Ok(())
    }

    fn handle_timer(&mut self, sender: &Sender, msg: TimerMsg) -> Result<()> {
        Ok(())
    }
}
