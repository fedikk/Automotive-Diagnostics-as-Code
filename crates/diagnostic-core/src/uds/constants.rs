#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdsService {
    DiagnosticSessionControl,
    EcuReset,
    ClearDiagnosticInformation,
    ReadDtcInformation,
    ReadDataByIdentifier,
}

impl UdsService {
    pub fn sid(self) -> u8 {
        match self {
            Self::DiagnosticSessionControl => 0x10,
            Self::EcuReset => 0x11,
            Self::ClearDiagnosticInformation => 0x14,
            Self::ReadDtcInformation => 0x19,
            Self::ReadDataByIdentifier => 0x22,
        }
    }
}

impl TryFrom<u8> for UdsService {
    type Error = ();

    fn try_from(sid: u8) -> Result<Self, Self::Error> {
        match sid {
            0x10 => Ok(Self::DiagnosticSessionControl),
            0x11 => Ok(Self::EcuReset),
            0x14 => Ok(Self::ClearDiagnosticInformation),
            0x19 => Ok(Self::ReadDtcInformation),
            0x22 => Ok(Self::ReadDataByIdentifier),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdsNrc {
    GeneralReject,
    ServiceNotSupported,
    SubFunctionNotSupported,
    IncorrectMessageLengthOrInvalidFormat,
    ConditionsNotCorrect,
    RequestOutOfRange,
}

impl UdsNrc {
    pub fn code(self) -> u8 {
        match self {
            Self::GeneralReject => 0x10,
            Self::ServiceNotSupported => 0x11,
            Self::SubFunctionNotSupported => 0x12,
            Self::IncorrectMessageLengthOrInvalidFormat => 0x13,
            Self::ConditionsNotCorrect => 0x22,
            Self::RequestOutOfRange => 0x31,
        }
    }
}

impl TryFrom<u8> for UdsNrc {
    type Error = ();

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            0x10 => Ok(Self::GeneralReject),
            0x11 => Ok(Self::ServiceNotSupported),
            0x12 => Ok(Self::SubFunctionNotSupported),
            0x13 => Ok(Self::IncorrectMessageLengthOrInvalidFormat),
            0x22 => Ok(Self::ConditionsNotCorrect),
            0x31 => Ok(Self::RequestOutOfRange),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdsSession {
    Default,
    Programming,
    Extended,
}

impl UdsSession {
    pub fn sub_function(self) -> u8 {
        match self {
            Self::Default => 0x01,
            Self::Programming => 0x02,
            Self::Extended => 0x03,
        }
    }
}

impl TryFrom<u8> for UdsSession {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Default),
            0x02 => Ok(Self::Programming),
            0x03 => Ok(Self::Extended),
            _ => Err(()),
        }
    }
}

pub const SID_DIAGNOSTIC_SESSION_CONTROL: u8 = 0x10;
pub const SID_ECU_RESET: u8 = 0x11;
pub const SID_CLEAR_DIAGNOSTIC_INFORMATION: u8 = 0x14;
pub const SID_READ_DTC_INFORMATION: u8 = 0x19;
pub const SID_READ_DATA_BY_IDENTIFIER: u8 = 0x22;

pub const NRC_GENERAL_REJECT: u8 = 0x10;
pub const NRC_SERVICE_NOT_SUPPORTED: u8 = 0x11;
pub const NRC_SUB_FUNCTION_NOT_SUPPORTED: u8 = 0x12;
pub const NRC_INCORRECT_MESSAGE_LENGTH_OR_INVALID_FORMAT: u8 = 0x13;
pub const NRC_CONDITIONS_NOT_CORRECT: u8 = 0x22;
pub const NRC_REQUEST_OUT_OF_RANGE: u8 = 0x31;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_returns_correct_sid() {
        assert_eq!(UdsService::DiagnosticSessionControl.sid(), 0x10);
        assert_eq!(UdsService::EcuReset.sid(), 0x11);
        assert_eq!(UdsService::ClearDiagnosticInformation.sid(), 0x14);
        assert_eq!(UdsService::ReadDtcInformation.sid(), 0x19);
        assert_eq!(UdsService::ReadDataByIdentifier.sid(), 0x22);
    }

    #[test]
    fn sid_converts_to_service() {
        assert_eq!(
            UdsService::try_from(0x10),
            Ok(UdsService::DiagnosticSessionControl)
        );
        assert_eq!(
            UdsService::try_from(0x11),
            Ok(UdsService::EcuReset)
        );
        assert_eq!(
            UdsService::try_from(0x14),
            Ok(UdsService::ClearDiagnosticInformation)
        );
        assert_eq!(
            UdsService::try_from(0x19),
            Ok(UdsService::ReadDtcInformation)
        );
        assert_eq!(
            UdsService::try_from(0x22),
            Ok(UdsService::ReadDataByIdentifier)
        );
    }

    #[test]
    fn unsupported_sid_is_rejected() {
        assert!(UdsService::try_from(0x99).is_err());
    }

    #[test]
    fn nrc_returns_correct_code() {
        assert_eq!(UdsNrc::GeneralReject.code(), 0x10);
        assert_eq!(UdsNrc::ServiceNotSupported.code(), 0x11);
        assert_eq!(UdsNrc::SubFunctionNotSupported.code(), 0x12);
        assert_eq!(
            UdsNrc::IncorrectMessageLengthOrInvalidFormat.code(),
            0x13
        );
        assert_eq!(UdsNrc::ConditionsNotCorrect.code(), 0x22);
        assert_eq!(UdsNrc::RequestOutOfRange.code(), 0x31);
    }

    #[test]
    fn nrc_code_converts_to_nrc() {
        assert_eq!(UdsNrc::try_from(0x10), Ok(UdsNrc::GeneralReject));
        assert_eq!(
            UdsNrc::try_from(0x11),
            Ok(UdsNrc::ServiceNotSupported)
        );
        assert_eq!(
            UdsNrc::try_from(0x12),
            Ok(UdsNrc::SubFunctionNotSupported)
        );
        assert_eq!(
            UdsNrc::try_from(0x13),
            Ok(UdsNrc::IncorrectMessageLengthOrInvalidFormat)
        );
        assert_eq!(
            UdsNrc::try_from(0x22),
            Ok(UdsNrc::ConditionsNotCorrect)
        );
        assert_eq!(
            UdsNrc::try_from(0x31),
            Ok(UdsNrc::RequestOutOfRange)
        );
    }

    #[test]
    fn unsupported_nrc_is_rejected() {
        assert!(UdsNrc::try_from(0x99).is_err());
    }

    #[test]
    fn session_returns_correct_sub_function() {
        assert_eq!(UdsSession::Default.sub_function(), 0x01);
        assert_eq!(UdsSession::Programming.sub_function(), 0x02);
        assert_eq!(UdsSession::Extended.sub_function(), 0x03);
    }

    #[test]
    fn session_sub_function_converts_to_session() {
        assert_eq!(UdsSession::try_from(0x01), Ok(UdsSession::Default));
        assert_eq!(
            UdsSession::try_from(0x02),
            Ok(UdsSession::Programming)
        );
        assert_eq!(UdsSession::try_from(0x03), Ok(UdsSession::Extended));
    }

    #[test]
    fn unsupported_session_is_rejected() {
        assert!(UdsSession::try_from(0x04).is_err());
    }
}
