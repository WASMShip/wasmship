use command::Command;

pub struct RunCommand {
    command: str,
    value: String,
}

impl Command for RunCommand {}
