use crate::command::Command;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct RunCommand {
    command: String,
    value: String,
}

#[async_trait]
impl Command for RunCommand {
    type RealCommand = RunCommand;

    async fn doit(&self) {
        println!("{}", "I'm run command, parse url for call.");
    }
}

impl RunCommand {
    pub fn new() -> RunCommand {
        RunCommand {
            command: "run".to_string(),
            value: "a.wasm".to_string(),
        }
    }
}

unsafe impl Send for RunCommand {}

unsafe impl Sync for RunCommand {}
