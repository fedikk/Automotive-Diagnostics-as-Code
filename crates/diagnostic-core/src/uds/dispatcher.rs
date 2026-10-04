use super::constants::UdsService;
use crate::uds::uds::UdsRequest;

#[derive(Debug, Default)]
pub struct UdsDispatcher;

impl UdsDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn dispatch(&self, request: &UdsRequest) -> Result<UdsService, u8> {
        UdsService::try_from(request.sid).map_err(|_| 0x11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatches_read_data_by_identifier() {
        let dispatcher = UdsDispatcher::new();
        let request = UdsRequest::new(0x22, vec![0xF1, 0x90]);

        assert_eq!(
            dispatcher.dispatch(&request),
            Ok(UdsService::ReadDataByIdentifier)
        );
    }

    #[test]
    fn dispatches_diagnostic_session_control() {
        let dispatcher = UdsDispatcher::new();
        let request = UdsRequest::new(0x10, vec![0x03]);

        assert_eq!(
            dispatcher.dispatch(&request),
            Ok(UdsService::DiagnosticSessionControl)
        );
    }

    #[test]
    fn unsupported_service_returns_service_not_supported() {
        let dispatcher = UdsDispatcher::new();
        let request = UdsRequest::new(0x99, vec![]);

        assert_eq!(dispatcher.dispatch(&request), Err(0x11));
    }
}