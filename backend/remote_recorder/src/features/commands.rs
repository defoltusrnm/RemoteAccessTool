use crate::utils::reading::ReadByte;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Login,
}

pub trait ReadCommand {
    fn read_command(&mut self) -> impl Future<Output = Result<Command, anyhow::Error>> + Send;
}

impl<T: ReadByte + Send> ReadCommand for T {
    async fn read_command(&mut self) -> Result<Command, anyhow::Error> {
        let command_byte = self.read_single_byte().await?;

        match command_byte {
            1 => Ok(Command::Login),
            _ => anyhow::bail!("unknown command"),
        }
    }
}
