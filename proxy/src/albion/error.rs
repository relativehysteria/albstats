use photon_decode::Value;

/// Errors encountered during decoding
#[derive(Debug, Clone)]
pub enum DecodeError {
    /// A parameter is missing from the `EventData` parameter map
    ParameterMissing,

    /// Got a value that doesn't map to anything expected
    UnexpectedValue(usize),

    /// A parameter was of unexpected photon type
    UnexpectedPhotonType(Value),
}
