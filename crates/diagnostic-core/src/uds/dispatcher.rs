use super::constants::{UdsNrc, UdsService, UdsSession};
use super::sessions::DiagnosticSession;
use crate::uds::uds::{UdsRequest, UdsResponse};
use crate::model::DidsDefinition;

#[derive(Debug)]
pub struct UdsDispatcher {
session: DiagnosticSession,
dids: DidsDefinition,
}

impl UdsDispatcher {
pub fn new(dids: DidsDefinition) -> Self {
Self {
session: DiagnosticSession::new(),
dids,
}
}

 
pub fn session(&self) -> UdsSession {
    self.session.current()
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

        UdsService::ReadDataByIdentifier => {
            self.handle_read_data_by_identifier(request)
        }

        _ => UdsResponse::negative(
            request.sid,
            UdsNrc::ServiceNotSupported.code(),
        ),
    }
}

fn handle_session_control(
    &mut self,
    request: &UdsRequest,
) -> UdsResponse {
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

fn handle_read_data_by_identifier(
    &self,
    request: &UdsRequest,
) -> UdsResponse {
    if request.payload.len() != 2 {
        return UdsResponse::negative(
            request.sid,
            UdsNrc::IncorrectMessageLengthOrInvalidFormat.code(),
        );
    }

    let did = u16::from_be_bytes([
        request.payload[0],
        request.payload[1],
    ]);

    let definition = match self.dids.dids.iter().find(|item| item.uds_did == did) {
        Some(definition) => definition,
        None => {
            return UdsResponse::negative(
                request.sid,
                UdsNrc::RequestOutOfRange.code(),
            );
        }
    };

    if !definition.access.read {
        return UdsResponse::negative(
            request.sid,
            UdsNrc::RequestOutOfRange.code(),
        );
    }

    let mut payload = vec![
        (did >> 8) as u8,
        (did & 0xFF) as u8,
    ];

    payload.extend(vec![0; definition.length as usize]);

    UdsResponse::positive(request.sid, payload)
}
 

}

#[cfg(test)]
mod tests {
use super::*;

 
fn test_dids() -> DidsDefinition {
    DidsDefinition {
        dids: vec![
            crate::model::Did {
                id: "VIN".to_string(),
                uds_did: 0xF190,
                name: "Vehicle Identification Number".to_string(),
                description: "Vehicle identification number".to_string(),
                data_type: "string".to_string(),
                length: 17,
                access: crate::model::DidAccess { read: true },
            },
        ],
    }
}

#[test]
fn diagnostic_session_control_switches_to_extended_session() {
    let mut dispatcher = UdsDispatcher::new(test_dids());

    let request = UdsRequest::new(0x10, vec![0x03]);
    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x50);
    assert_eq!(response.payload, vec![0x03]);
    assert_eq!(dispatcher.session(), UdsSession::Extended);
}

#[test]
fn read_data_by_identifier_returns_did() {
    let mut dispatcher = UdsDispatcher::new(test_dids());

    let request = UdsRequest::new(0x22, vec![0xF1, 0x90]);
    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x62);
    assert_eq!(&response.payload[0..2], &[0xF1, 0x90]);
    assert_eq!(response.payload.len(), 19);
}

#[test]
fn unknown_did_returns_request_out_of_range() {
    let mut dispatcher = UdsDispatcher::new(test_dids());

    let request = UdsRequest::new(0x22, vec![0x12, 0x34]);
    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x7F);
    assert_eq!(response.payload, vec![0x22, 0x31]);
}

#[test]
fn read_data_by_identifier_rejects_invalid_length() {
    let mut dispatcher = UdsDispatcher::new(test_dids());

    let request = UdsRequest::new(0x22, vec![0xF1]);
    let response = dispatcher.dispatch(&request);

    assert_eq!(response.sid, 0x7F);
    assert_eq!(response.payload, vec![0x22, 0x13]);
}
 

}
