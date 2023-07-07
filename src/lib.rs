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
