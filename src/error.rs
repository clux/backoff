use std::error;
use std::fmt;

use instant::Duration;

/// Error is the error value in an operation's
/// result.
///
/// Based on the two possible values, the operation
/// may be retried.
pub enum Error<E> {
    /// Permanent means that it's impossible to execute the operation
    /// successfully. This error is immediately returned from `retry()`.
    Permanent(E),
    /// Transient means that the error is temporary. If the second argument is `None`
    /// the operation should be retried according to the backoff policy, else after
    /// the specified duration. Useful for handling ratelimits like a HTTP 429 response.
    Transient(E, Option<Duration>),
}

impl<E> fmt::Display for Error<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Permanent(ref err) | Error::Transient(ref err, _) => err.fmt(f),
        }
    }
}

impl<E> fmt::Debug for Error<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (name, err) = match *self {
            Error::Permanent(ref err) => ("Permanent", err as &dyn fmt::Debug),
            Error::Transient(ref err, _) => ("Transient", err as &dyn fmt::Debug),
        };
        f.debug_tuple(name).field(err).finish()
    }
}

impl<E> error::Error for Error<E>
where
    E: error::Error,
{
    fn description(&self) -> &str {
        match *self {
            Error::Permanent(_) => "permanent error",
            Error::Transient(..) => "transient error",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Permanent(ref err) | Error::Transient(ref err, _) => err.source(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

/// By default all errors are transient. Permanent errors can
/// be constructed explicitly. This implementation is for making
/// the question mark operator (?) and the `try!` macro to work.
impl<E> From<E> for Error<E> {
    fn from(err: E) -> Error<E> {
        Error::Transient(err, None)
    }
}
