use std::fmt::Display;

use deku::{DekuError, DekuRead, DekuWrite, deku_derive};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::{IeId, write_bits_lsb0};

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct HtCapabilities {
    pub ht_capability_information: HtCapabilityInformation,
    pub ampdu_parameters: AmpduParameters,
    #[deku(count = "16")]
    pub supported_mcs_set: Vec<u8>,
    pub ht_extended_capabilities: HtExtendedCapabilities,
    pub transmit_beamforming_capabilities: TransmitBeamformingCapabilities,
    pub asel_capablities: AselCapabilities,
}

impl HtCapabilities {
    pub const NAME: &'static str = "HT Capabilities";
    pub const ID: u8 = 45;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);
    pub const MIN_LENGTH: usize = 26;
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[deku(bit_order = "lsb")]
pub struct HtCapabilityInformation {
    #[deku(bits = 1)]
    pub ldpc_coding_capability: bool,
    #[deku(
        bits = 1,
        map = "|value: u8| SupportedChannelWidthSet::try_from(value).map_err(|_| DekuError::Parse(\"Invalid SupportedChannelWidthSet\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*supported_channel_width_set), 1)"
    )]
    pub supported_channel_width_set: SupportedChannelWidthSet,
    #[deku(
        bits = 2,
        map = "|value: u8| SmPowerSave::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid SmPowerSave\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*sm_power_save), 2)"
    )]
    pub sm_power_save: SmPowerSave,
    #[deku(bits = 1)]
    pub ht_greenfield: bool,
    #[deku(bits = 1)]
    pub short_gi_for_twenty_mhz: bool,
    #[deku(bits = 1)]
    pub short_gi_for_forty_mhz: bool,
    #[deku(bits = 1)]
    pub tx_stbc: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| RxStbc::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid RxStbc\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*rx_stbc), 2)"
    )]
    pub rx_stbc: RxStbc,
    #[deku(bits = 1)]
    reserved_1: bool,
    #[deku(
        bits = 1,
        map = "|value: bool| -> Result<u16, DekuError> { if value { Ok(7935u16) } else { Ok(3839u16) } }",
        writer = "write_bits_lsb0(deku::writer, (*maximum_amsdu_length == 7935u16) as u8, 1)"
    )]
    pub maximum_amsdu_length: u16,
    #[deku(bits = 1)]
    pub dsss_cck_mode_in_forty_mhz: bool,
    #[deku(bits = 1)]
    reserved_2: bool,
    #[deku(bits = 1)]
    pub forty_mhz_intolerant: bool,
    #[deku(bits = 1)]
    reserved_3: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SupportedChannelWidthSet {
    TwentyMhz = 0,
    TwentyOrFortyMhz = 1,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[deku(bit_order = "lsb")]
pub struct AmpduParameters {
    #[deku(bits = 2)]
    pub maximum_ampdu_length_exponent: u8,
    #[deku(
        bits = 3,
        map = "|value: u8| MpduStartSpacing::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid MpduStartSpacing\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*minimum_mpdu_start_spacing), 3)"
    )]
    pub minimum_mpdu_start_spacing: MpduStartSpacing,
    #[deku(bits = 3)]
    reserved: u8,
}

impl AmpduParameters {
    pub fn max_ampdu_length(&self) -> u32 {
        2u32.pow(13 + u32::from(self.maximum_ampdu_length_exponent)) - 1
    }
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[deku(bit_order = "lsb")]
pub struct HtExtendedCapabilities {
    #[deku(bits = 8)]
    reserved_1: u8,
    #[deku(
        bits = 2,
        map = "|value: u8| McsFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid McsFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*mcs_feedback), 2)"
    )]
    pub mcs_feedback: McsFeedback,
    #[deku(bits = 1)]
    pub htc_ht_support: bool,
    #[deku(bits = 1)]
    pub rd_responder: bool,
    #[deku(bits = 4)]
    reserved_2: u8,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[deku(bit_order = "lsb")]
pub struct TransmitBeamformingCapabilities {
    #[deku(bits = 1)]
    pub implicit_transmit_beamforming_receiving_capable: bool,
    #[deku(bits = 1)]
    pub receive_staggered_sounding_capable: bool,
    #[deku(bits = 1)]
    pub transmit_staggered_sounding_capable: bool,
    #[deku(bits = 1)]
    pub receive_ndp_capable: bool,
    #[deku(bits = 1)]
    pub transmit_ndp_capable: bool,
    #[deku(bits = 1)]
    pub implicit_transmit_beamforming_capable: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| Calibration::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid Calibration\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*calibration), 2)"
    )]
    pub calibration: Calibration,
    #[deku(bits = 1)]
    pub explicit_csi_transmit_beamforming_capable: bool,
    #[deku(bits = 1)]
    pub explicit_noncompressed_steering_capable: bool,
    #[deku(bits = 1)]
    pub explicit_compressed_steering_capable: bool,
    #[deku(
        bits = 2,
        map = "|value: u8| BeamformingFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid BeamformingFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*explicit_transmit_beamforming_csi_feedback), 2)"
    )]
    pub explicit_transmit_beamforming_csi_feedback: BeamformingFeedback,
    #[deku(
        bits = 2,
        map = "|value: u8| BeamformingFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid BeamformingFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*explicit_noncompressed_beamforming_feedback_capable), 2)"
    )]
    pub explicit_noncompressed_beamforming_feedback_capable: BeamformingFeedback,
    #[deku(
        bits = 2,
        map = "|value: u8| BeamformingFeedback::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid BeamformingFeedback\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*explicit_compressed_beamforming_feedback_capable), 2)"
    )]
    pub explicit_compressed_beamforming_feedback_capable: BeamformingFeedback,
    #[deku(
        bits = 2,
        map = "|value: u8| MinimalGrouping::try_from(value).map_err(|_| deku::DekuError::Parse(\"Invalid MinimalGrouping\".into()))",
        writer = "write_bits_lsb0(deku::writer, u8::from(*minimal_grouping), 2)"
    )]
    pub minimal_grouping: MinimalGrouping,
    #[deku(bits = 2)]
    pub csi_number_of_beamformer_antennas_supported: u8,
    #[deku(bits = 2)]
    pub noncompressed_steering_number_of_beamformer_antennas_supported: u8,
    #[deku(bits = 2)]
    pub compressed_steering_number_of_beamformer_antennas_supported: u8,
    #[deku(bits = 2)]
    pub csi_max_number_of_rows_beamformer_supported: u8,
    #[deku(bits = 2)]
    pub channel_estimation_capability: u8,
    #[deku(bits = 3)]
    reserved: u8,
}

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[deku(bit_order = "lsb")]
pub struct AselCapabilities {
    #[deku(bits = 1)]
    pub antenna_selection_capable: bool,
    #[deku(bits = 1)]
    pub explicit_csi_feedback_based_transmit_asel_capable: bool,
    #[deku(bits = 1)]
    pub antenna_indices_feedback_based_transmit_asel_capable: bool,
    #[deku(bits = 1)]
    pub explicit_csi_feedback_capable: bool,
    #[deku(bits = 1)]
    pub antenna_indices_feedback_capable: bool,
    #[deku(bits = 1)]
    pub receive_asel_capable: bool,
    #[deku(bits = 1)]
    pub transmit_sounding_ppdus_capable: bool,
    #[deku(bits = 1)]
    reserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum SmPowerSave {
    Static = 0,
    Dynamic = 1,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum RxStbc {
    NotSupported = 0,
    OneSpatialStream,
    OneAndTwoSpatialStreams,
    OneTwoAndThreeSpatialStreams,
}

impl Display for RxStbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RxStbc::NotSupported => write!(f, "Not Supported"),
            RxStbc::OneSpatialStream => write!(f, "One Spatial Stream"),
            RxStbc::OneAndTwoSpatialStreams => write!(f, "One and Two Spatial Streams"),
            RxStbc::OneTwoAndThreeSpatialStreams => {
                write!(f, "One, Two, and Three Spatial Streams")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MpduStartSpacing {
    NoRestriction = 0,
    QuarterMicrosecond,
    HalfMicrosecond,
    OneMicrosecond,
    TwoMicroseconds,
    FourMicroseconds,
    EightMicroseconds,
    SixteenMicroseconds,
}

impl Display for MpduStartSpacing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MpduStartSpacing::NoRestriction => write!(f, "No Restriction"),
            MpduStartSpacing::QuarterMicrosecond => write!(f, "1/4 μs"),
            MpduStartSpacing::HalfMicrosecond => write!(f, "1/2 μs"),
            MpduStartSpacing::OneMicrosecond => write!(f, "1 μs"),
            MpduStartSpacing::TwoMicroseconds => write!(f, "2 μs"),
            MpduStartSpacing::FourMicroseconds => write!(f, "4 μs"),
            MpduStartSpacing::EightMicroseconds => write!(f, "8 μs"),
            MpduStartSpacing::SixteenMicroseconds => write!(f, "16 μs"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum McsFeedback {
    NoMfb = 0,
    UnsolicitedMfb = 2,
    ResponseOrUnsolicitedMfb = 3,
}

impl Display for McsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McsFeedback::NoMfb => write!(f, "No MFB"),
            McsFeedback::UnsolicitedMfb => write!(f, "Unsolicited MFB"),
            McsFeedback::ResponseOrUnsolicitedMfb => {
                write!(f, "Response (Delayed/Immediate) or Unsolicited MFB")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Calibration {
    NotSupported = 0,
    Respond = 1,
    InitiateAndRespond = 3,
}

impl Display for Calibration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Calibration::NotSupported => write!(f, "Not Supported"),
            Calibration::Respond => write!(f, "Respond to Calibration Request"),
            Calibration::InitiateAndRespond => {
                write!(f, "Initiate and Respond to Calibration Request")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum BeamformingFeedback {
    NotSupported = 0,
    Delayed,
    Immediate,
    DelayedAndImmediate,
}

impl Display for BeamformingFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BeamformingFeedback::NotSupported => write!(f, "Not Supported"),
            BeamformingFeedback::Delayed => write!(f, "Delayed"),
            BeamformingFeedback::Immediate => write!(f, "Immediate"),
            BeamformingFeedback::DelayedAndImmediate => write!(f, "Delayed and Immediate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum MinimalGrouping {
    One = 0,
    OneOrTwo,
    OneOrFour,
    OneOrTwoOrFour,
}

impl Display for MinimalGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MinimalGrouping::One => write!(f, "1"),
            MinimalGrouping::OneOrTwo => write!(f, "1 or 2"),
            MinimalGrouping::OneOrFour => write!(f, "1 or 4"),
            MinimalGrouping::OneOrTwoOrFour => write!(f, "1, 2, or 4"),
        }
    }
}
