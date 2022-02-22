use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::net::Ipv4Addr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    Start,
    Exit,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, ">start")?,
            Self::Exit => write!(f, ">exit")?,
        }
        Ok(())
    }
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ">start" => Self::Start,
            ">exit" => Self::Exit,
            _ => return Err(anyhow!("invalid command")),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    Finished,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Finished => write!(f, "<finished")?,
        }
        Ok(())
    }
}

impl std::str::FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "<finished" => Self::Finished,
            _ => return Err(anyhow!("invalid event")),
        })
    }
}

#[async_trait]
pub trait Server: Send + Sized {
    async fn start() -> Result<Self>;
    async fn run(&mut self) -> Result<()> {
        Ok(())
    }
    async fn exit(&mut self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
pub trait Client {
    async fn run(&mut self, addr: Ipv4Addr) -> Result<()>;
}

pub async fn run_server<S: Server>() -> Result<()> {
    // Consume start command
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let mut server = S::start().await?;

    server.run().await?;

    server.exit().await?;

    println!("{}", Event::Finished); // Notify finished
    // Consume exit command
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(())
}

pub async fn run_client<C: Client>(mut client: C) -> Result<()> {
    let addr: Ipv4Addr = std::env::args().nth(1).unwrap().parse()?;

    // Consume start command
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    client.run(addr).await?;

    println!("{}", Event::Finished); // Notify finished
    // Consume exit command
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(())
}
