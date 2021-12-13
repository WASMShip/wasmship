use async_trait::async_trait;

pub mod list;
pub mod pull;
pub mod push;
pub mod run;

#[async_trait]
pub trait Command {
    type RealCommand;
    async fn doit(&self);
}
