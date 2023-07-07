pub use typed_command_builder_derive::TypedCommandBuilder;

#[doc(hidden)]
pub trait Optional<T> {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
}

impl<T> Optional<T> for () {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
        default()
    }
}

impl<T> Optional<T> for (T,) {
    fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
        self.0
    }
}

/// Turn a built up object into a Command
///
/// This is implemented by every type that implements [TypedCommandBuilder] via the derive macro
/// shipped by this crate.
///
/// The return type is either the sync `std::process::Command` or `tokio::process::Command`,
/// depending on whether the derive macro was called to use the sync or async backend.
pub trait IntoCommand {
    type Output;
    fn into_command(self) -> Self::Output;
}

/// Helper trait for running the built up command
pub trait Runnable {
    fn output(self) -> Result<std::process::Output, std::io::Error>;
    fn status(self) -> Result<std::process::ExitStatus, std::io::Error>;
}

impl<C> Runnable for C
where
    C: IntoCommand<Output = std::process::Command>,
{
    fn output(self) -> Result<std::process::Output, std::io::Error> {
        let mut command = self.into_command();
        command.output()
    }
    fn status(self) -> Result<std::process::ExitStatus, std::io::Error> {
        let mut command = self.into_command();
        command.status()
    }
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
pub trait AsyncRunnable {
    async fn output(self) -> Result<std::process::Output, std::io::Error>;
    async fn status(self) -> Result<std::process::ExitStatus, std::io::Error>;
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl<C> AsyncRunnable for C
where
    C: IntoCommand<Output = tokio::process::Command>,
{
    async fn output(self) -> Result<std::process::Output, std::io::Error> {
        let mut command = self.into_command();
        command.output().await
    }

    async fn status(self) -> Result<std::process::ExitStatus, std::io::Error> {
        let mut command = self.into_command();
        command.status().await
    }
}
