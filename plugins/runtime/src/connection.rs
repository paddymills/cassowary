
/// Client's connection state
#[derive(Debug)]
pub enum ConnectionState {
    /// TCP Stream has been opened
    Initiated,

    /// Plugin capabilities and subscriptions have been requested
    RequestedCapabilities,

    /// Plugin is fully connected and ready to communicate with the runtime
    Connected,

    /// Plugin TCP stream has been closed
    Disconnected,
}