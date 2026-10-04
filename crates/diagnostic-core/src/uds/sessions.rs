use super::constants::UdsSession;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticSession {
    current: UdsSession,
}

impl Default for DiagnosticSession {
    fn default() -> Self {
        Self {
            current: UdsSession::Default,
        }
    }
}

impl DiagnosticSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> UdsSession {
        self.current
    }

    pub fn set(&mut self, session: UdsSession) {
        self.current = session;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_starts_in_default() {
        let session = DiagnosticSession::new();

        assert_eq!(session.current(), UdsSession::Default);
    }

    #[test]
    fn session_can_be_changed() {
        let mut session = DiagnosticSession::new();

        session.set(UdsSession::Extended);

        assert_eq!(session.current(), UdsSession::Extended);
    }
}
