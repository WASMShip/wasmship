use crate::command::Command;

pub struct RunCommand {
    command: String,
    value: String,
}

impl Command for RunCommand {}
