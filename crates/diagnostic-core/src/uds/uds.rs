#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdsRequest {
    pub sid: u8,
    pub payload: Vec<u8>,
}

impl UdsRequest {
    pub fn new(sid: u8, payload: Vec<u8>) -> Self {
        Self { sid, payload }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdsResponse {
    pub sid: u8,
    pub payload: Vec<u8>,
}

impl UdsResponse {
    pub fn positive(request_sid: u8, payload: Vec<u8>) -> Self {
        Self {
            sid: request_sid | 0x40,
            payload,
        }
    }

    pub fn negative(request_sid: u8, nrc: u8) -> Self {
        Self {
            sid: 0x7F,
            payload: vec![request_sid, nrc],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_is_created_with_sid_and_payload() {
        let request = UdsRequest::new(0x22, vec![0xF1, 0x90]);

        assert_eq!(request.sid, 0x22);
        assert_eq!(request.payload, vec![0xF1, 0x90]);
    }

    #[test]
    fn positive_response_uses_positive_sid() {
        let response = UdsResponse::positive(0x22, vec![0xF1, 0x90]);

        assert_eq!(response.sid, 0x62);
        assert_eq!(response.payload, vec![0xF1, 0x90]);
    }

    #[test]
    fn negative_response_contains_request_sid_and_nrc() {
        let response = UdsResponse::negative(0x22, 0x13);

        assert_eq!(response.sid, 0x7F);
        assert_eq!(response.payload, vec![0x22, 0x13]);
    }
}
