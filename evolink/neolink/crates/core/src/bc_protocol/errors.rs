use super::bc::model::Bc;
use crate::NomErrorType;
use err_derive::Error;

/// This is the primary error type of the library
#[derive(Debug, Error, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Underlying IO errors
    #[error(display = "IO Error: {:?}", _0)]
    Io(#[error(source)] std::sync::Arc<std::io::Error>),

    /// Raised when fails to parse time from the camera
    #[error(display = "Error in time coversion: {:?}", _0)]
    TimeRange(#[error(source)] time::error::ComponentRange),

    /// Raised when fails to parse time from the camera
    #[error(display = "Error in time parsing")]
    TimeParse,

    /// Raised when fails to parse time from the camera
    #[error(display = "Error in try from NonZeroInt")]
    TryFromInt(#[error(source)] std::num::TryFromIntError),

    /// /// Raised when fails to parse time from the camera
    #[error(display = "Error in time conversion")]
    TimeTryFrom(#[error(source)] time::error::TryFromParsed),

    /// Raised when a Bc reply was not understood
    #[error(display = "Communication error")]
    UnintelligibleReply {
        /// The Bc packet that was not understood
        reply: std::sync::Arc<Box<Bc>>,
        /// The message attached to the error
        why: &'static str,
    },

    /// Raised when the camera responds with a status code over than OK
    #[error(display = "Camera responded with Service Unavaliable")]
    CameraServiceUnavaliable,

    /// Raised when the camera responds with a status code over than OK during login
    #[error(display = "Camera responded with Err during login")]
    CameraLoginFail,

    /// Raised when a connection is dropped.
    #[error(display = "Dropped connection")]
    DroppedConnection,

    /// Raised when a connection is dropped during a tokio mpsc TryRecv event
    #[error(display = "Dropped connection")]
    DroppedConnectionTry(#[error(source)] tokio::sync::mpsc::error::TryRecvError),

    /// Raised when a connection is dropped during a TryRecv event
    #[error(display = "Dropped connection")]
    BroadcastDroppedConnectionTry(#[error(source)] tokio::sync::broadcast::error::TryRecvError),

    /// Raised when a connection is dropped during a TryRecv event
    #[error(display = "Send Error")]
    TokioBcSendError,

    /// Raised when the TIMEOUT is reach
    #[error(display = "Timeout")]
    Timeout(#[error(source)] std::sync::Arc<tokio::time::error::Elapsed>),

    /// Raised when a timeout fails in a non standard way such as timeout during shutdown
    #[error(display = "TimeoutError")]
    TimeoutError(#[error(source)] tokio::time::error::Error),

    /// Raised when connection is dropped because the timeout is reach
    #[error(display = "Dropped connection")]
    TimeoutDisconnected,

    /// Raised when a camera cannot be connected to ay any of the given addresses
    #[error(display = "Cannot contact camera at given address")]
    CannotInitCamera,

    /// Raised when failed to login to the camera
    #[error(display = "Credential error")]
    AuthFailed,

    /// Raised when the given camera url could not be resolved
    #[error(display = "Failed to translate camera address")]
    AddrResolutionError,

    /// Raised non adpcm data is sent to the talk command
    #[error(display = "Talk data is not ADPCM")]
    UnknownTalkEncoding,

    /// Raised when dicovery times out waiting for a reply
    #[error(display = "Timed out while waiting for camera reply")]
    DiscoveryTimeout,

    /// Raised during a (de)seralisation error
    #[error(display = "Cookie GenError")]
    GenError(#[error(source)] std::sync::Arc<cookie_factory::GenError>),

    /// Raised when a connection is subscrbed to more than once for msg_num
    #[error(display = "Simultaneous subscription, {:?}", _0)]
    SimultaneousSubscription {
        /// The message number that was subscribed to
        msg_num: Option<u16>,
    },

    /// Raised when a connection is subscrbed to more than once for msg_id
    #[error(display = "Simultaneous subscription, {}", _0)]
    SimultaneousSubscriptionId {
        /// The message number that was subscribed to
        msg_id: u32,
    },

    /// Raised when a new encyrption byte is observed
    #[error(display = "Unknown encryption: {:x?}", _0)]
    UnknownEncryption(usize),

    /// Raised when the camera cannot be found
    #[error(display = "Camera Not Findable")]
    ConnectionUnavaliable,

    /// Raised when the subscription id dropped too soon
    #[error(display = "Dropped Subscriber")]
    DroppedSubscriber,

    /// Raised when a unknown connection ID attempts to connect with us over UDP
    #[error(display = "Connection with unknown connectionID: {:?}", _0)]
    UnknownConnectionId(i32),

    /// Raised when a unknown SocketAddr attempts to connect with us over UDP
    #[error(display = "Connection from unknown source: {:?}", _0)]
    UnknownSource(std::net::SocketAddr),

    /// Raised when the IP/Hostname cannot be understood
    #[error(display = "Could not parse as IP")]
    AddrParseError(#[error(source)] std::net::AddrParseError),

    /// Raised when a relay connection is not possible
    /// usually happens if the camera has not contacted reolink yet
    #[error(display = "Cannot perform relay connection with this camera")]
    NoDmap,

    /// Raised when a dev connection is not possible
    /// usually happens if the camera has not contacted reolink yet
    #[error(display = "Cannot perform lookup with this camera against reolink servers")]
    NoDev,

    /// Raised when cannot determine local IP address
    #[error(display = "Local IP address is unknown: {}", _0)]
    LocalIpError(#[error(source)] std::sync::Arc<local_ip_address::Error>),

    /// Raised when a discovery fails to be accepted by the register
    #[error(display = "Register refuses to accept us")]
    RegisterError,

    /// Raised when a the relay terminates the connection by sending a R2C_DISC
    #[error(display = "Relay terminated the connection")]
    RelayTerminate,

    /// Raised when a the camera terminates the connection by sending a D2C_DISC
    #[error(display = "Camera terminated the connection")]
    CameraTerminate,

    /// Raised when the stream is not enough to complete a message
    #[error(display = "Nom Parsing incomplete: {}", _0)]
    NomIncomplete(usize),

    /// Raised when a stream cannot be decoded
    #[error(display = "Nom Parsing error: {}", _0)]
    NomError(String),

    /// Raised when a camera/user lacks an ability
    #[error(
        display = "Missing ability: {} with {} permission has only {}",
        name,
        requested,
        actual
    )]
    MissingAbility {
        /// Name of the ability
        name: String,
        /// Requested permission (read/write)
        requested: String,
        /// Actual permission (read/write/none)
        actual: String,
    },

    /// Raised when a thread panics
    #[error(display = "Thread panicked")]
    JoinError(#[error(source)] std::sync::Arc<tokio::task::JoinError>),

    /// A generic catch all error
    #[error(display = "Other error: {}", _0)]
    Other(&'static str),

    /// A generic catch all error
    #[error(display = "Other error: {}", _0)]
    OtherString(String),
}

impl From<std::io::Error> for Error {
    fn from(k: std::io::Error) -> Self {
        Error::Io(std::sync::Arc::new(k))
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::TokioBcSendError
    }
}

impl<T> From<tokio_util::sync::PollSendError<T>> for Error {
    fn from(_: tokio_util::sync::PollSendError<T>) -> Self {
        Error::TokioBcSendError
    }
}

impl From<cookie_factory::GenError> for Error {
    fn from(k: cookie_factory::GenError) -> Self {
        Error::GenError(std::sync::Arc::new(k))
    }
}

impl From<local_ip_address::Error> for Error {
    fn from(k: local_ip_address::Error) -> Self {
        Error::LocalIpError(std::sync::Arc::new(k))
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(k: tokio::task::JoinError) -> Self {
        Error::JoinError(std::sync::Arc::new(k))
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(k: tokio::time::error::Elapsed) -> Self {
        Error::Timeout(std::sync::Arc::new(k))
    }
}

impl<'a> From<nom::Err<NomErrorType<'a>>> for Error {
    fn from(k: nom::Err<NomErrorType<'a>>) -> Self {
        match k {
            nom::Err::Error(e) => Error::NomError(format!("Nom Error: {:X?}", e)),
            nom::Err::Failure(e) => Error::NomError(format!("Nom Error: {:X?}", e)),
            nom::Err::Incomplete(nom::Needed::Size(amount)) => Error::NomIncomplete(amount.get()),
            nom::Err::Incomplete(nom::Needed::Unknown) => Error::NomIncomplete(1),
        }
    }
}
