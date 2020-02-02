use std::fmt;
use super::*;

#[derive(Debug, Serialize, Deserialize, RnetSerde)]
pub struct RnetError {
    payload_kind: Option<PayloadKind>,
    context: String,
    explanation: String,
    pub is_recoverable: bool,
}
impl RnetError {
    pub fn new(payload_kind: Option<PayloadKind>, context: &str, explanation: &str, is_recoverable: bool) -> Self {
        Self {
            payload_kind,
            context: context.to_owned(),
            explanation: explanation.to_owned(),
            is_recoverable,
        }
    }

    pub fn action(datagram: &[u8]) -> RnetResult
    where Self: std::fmt::Debug + Sized
    {
        let ser: Self = Self::payload_from_bytes(datagram);
        ser.debug();
        Ok(None)
    }
}
impl fmt::Display for RnetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let is_recoverable_str = if self.is_recoverable { "non terminating" } else { "Unis_recoverable" };
        write!(f, "{:?} - {} error from {}: {}", self.payload_kind, is_recoverable_str, self.context, self.explanation)
    }
}
impl std::error::Error for RnetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}