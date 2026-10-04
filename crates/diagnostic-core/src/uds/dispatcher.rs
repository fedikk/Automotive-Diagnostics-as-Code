use super::constants::{UdsNrc, UdsService, UdsSession};
use super::sessions::DiagnosticSession;
use crate::uds::uds::{UdsRequest, UdsResponse};

#[derive(Debug, Default)]
pub struct UdsDispatcher {
session: DiagnosticSession,
}

impl UdsDispatcher {
pub fn new() -> Self {
Self::default()
}

 
pub fn dispatch(&mut self, request: &UdsRequest) -> UdsResponse {
    let service = match UdsService::try_from(request.sid) {
        Ok(service) => service,
        Err(_) => {
            return UdsResponse::negative(
                request.sid,
                UdsNrc::ServiceNotSupported.code(),
            );
        }
    };

    match service {
        UdsService::DiagnosticSessionControl => {
            self.handle_session_control(request)
        }
        _ => UdsResponse::negative(
            request.sid,
            UdsNrc::ServiceNotSupported.code(),
        ),
    }
}

pub fn session(&self) -> UdsSession {
    self.session.current()
}

fn handle_session_control(&mut self, request: &UdsRequest) -> UdsResponse {
    if request.payload.len() != 1 {
        return UdsResponse::negative(
            request.sid,
            UdsNrc::IncorrectMessageLengthOrInvalidFormat.code(),
        );
    }

    let requested_session = match UdsSession::try_from(request.payload[0]) {
        Ok(session) => session,
        Err(_) => {
            return UdsResponse::negative(
                request.sid,
                UdsNrc::SubFunctionNotSupported.code(),
            );
        }
    };

    self.session.set(requested_session);

    UdsResponse::positive(
        request.sid,
        vec![requested_session.sub_function()],
    )
}
 

}

#[cfg(test)]
mod tests {
use super::*;

 
#[test]
fn diagnostic_session_control_switches_to_extended_session() {
    let mut dispatcher = UdsDispatcher::new();
    let request = UdsRequest::new(0x10, vec![0x03]);

    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x50);
    assert_eq!(response.payload, vec![0x03]);
    assert_eq!(dispatcher.session(), UdsSession::Extended);
}

#[test]
fn diagnostic_session_control_switches_to_programming_session() {
    let mut dispatcher = UdsDispatcher::new();
    let request = UdsRequest::new(0x10, vec![0x02]);

    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x50);
    assert_eq!(response.payload, vec![0x02]);
    assert_eq!(dispatcher.session(), UdsSession::Programming);
}

#[test]
fn diagnostic_session_control_rejects_invalid_session() {
    let mut dispatcher = UdsDispatcher::new();
    let request = UdsRequest::new(0x10, vec![0x99]);

    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x7F);
    assert_eq!(response.payload, vec![0x10, 0x12]);
    assert_eq!(dispatcher.session(), UdsSession::Default);
}

#[test]
fn diagnostic_session_control_rejects_invalid_length() {
    let mut dispatcher = UdsDispatcher::new();
    let request = UdsRequest::new(0x10, vec![]);

    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x7F);
    assert_eq!(response.payload, vec![0x10, 0x13]);
    assert_eq!(dispatcher.session(), UdsSession::Default);
}

#[test]
fn unsupported_service_returns_negative_response() {
    let mut dispatcher = UdsDispatcher::new();
    let request = UdsRequest::new(0x99, vec![]);

    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x7F);
    assert_eq!(response.payload, vec![0x99, 0x11]);
}
 

}
