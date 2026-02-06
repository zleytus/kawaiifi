//! Wi-Fi Information Element (IE) parsing and serialization.
//!
//! Information Elements are the building blocks of Wi-Fi management frames
//! (beacons, probe responses, etc.). Each IE contains specific information
//! about an access point or station's capabilities and configuration.
//!
//! # Parsing IEs
//!
//! Use [`from_bytes`] to parse a sequence of IEs from raw bytes.
//! Returns a `Vec<Ie>` containing all successfully parsed IEs:
//!
//! ```
//! # use kawaiifi::ies;
//! // Two IEs: SSID "Hello" + DS Parameter Set (channel 6)
//! let ie_bytes = &[
//!     0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,  // SSID IE
//!     0x03, 0x01, 0x06,                           // DS Parameter Set IE
//! ];
//! let ies = ies::from_bytes(ie_bytes);
//! assert_eq!(ies.len(), 2);
//! ```
//!
//! # Accessing IE Data
//!
//! Each [`Ie`] has `id`, `id_ext`, and `data` fields.
//! Use the `name()` method to get a human-readable IE name:
//!
//! ```
//! # use kawaiifi::ies::{self, IeData};
//! # let ie_bytes = &[
//! #     0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
//! #     0x03, 0x01, 0x06,
//! # ];
//! # let ies = ies::from_bytes(ie_bytes);
//! for ie in &ies {
//!     println!("IE: {} (id={}, id_ext={:?})", ie.name(), ie.id, ie.id_ext);
//!
//!     match &ie.data {
//!         IeData::Ssid(ssid) => println!("  SSID: {}", ssid.to_string_lossy()),
//!         IeData::DsParameterSet(ds) => println!("  Channel: {}", ds.current_channel),
//!         _ => {}
//!     }
//! }
//! ```

mod elements;
mod field;

pub use elements::*;
pub use field::{BitRange, Field};

use deku::{DekuContainerRead, DekuContainerWrite, DekuError, DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

/// A Wi-Fi Information Element.
///
/// Contains the element ID, length, optional extension ID, and parsed data.
#[derive(Clone, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
pub struct Ie {
    pub id: u8,
    pub len: u8,
    #[deku(cond = "*id == 255")]
    pub id_ext: Option<u8>,
    #[deku(
        bytes = "Ie::compute_data_len(*len, *id_ext)?",
        ctx = "IeId { id: *id, id_ext: *id_ext }"
    )]
    pub data: IeData,
}

impl Ie {
    pub const fn name(&self) -> &'static str {
        match self.data {
            IeData::AdvertisementProtocol(_) => AdvertisementProtocol::NAME,
            IeData::Antenna(_) => Antenna::NAME,
            IeData::ApChannelReport(_) => ApChannelReport::NAME,
            IeData::ApConfigurationSequenceNumber(_) => ApConfigurationSequenceNumber::NAME,
            IeData::AwakeWindow(_) => AwakeWindow::NAME,
            IeData::BssLoad(_) => BssLoad::NAME,
            IeData::ChallengeText(_) => ChallengeText::NAME,
            IeData::ChannelSwitchAnnouncement(_) => ChannelSwitchAnnouncement::NAME,
            IeData::Country(_) => Country::NAME,
            IeData::DsParameterSet(_) => DsParameterSet::NAME,
            IeData::EhtCapabilities(_) => EhtCapabilities::NAME,
            IeData::EhtOperation(_) => EhtOperation::NAME,
            IeData::ErpInfo(_) => ErpInfo::NAME,
            IeData::ExtendedCapabilities(_) => ExtendedCapabilities::NAME,
            IeData::ExtendedChannelSwitchAnnouncement(_) => ExtendedChannelSwitchAnnouncement::NAME,
            IeData::ExtendedSupportedRates(_) => ExtendedSupportedRates::NAME,
            IeData::FilsIndication(_) => FilsIndication::NAME,
            IeData::HeCapabilities(_) => HeCapabilities::NAME,
            IeData::HeOperation(_) => HeOperation::NAME,
            IeData::HeSixGhzBandCapabilities(_) => HeSixGhzBandCapabilities::NAME,
            IeData::HtCapabilities(_) => HtCapabilities::NAME,
            IeData::HtOperation(_) => HtOperation::NAME,
            IeData::IbssParameterSet(_) => IbssParameterSet::NAME,
            IeData::Interworking(_) => Interworking::NAME,
            IeData::MeasurementPilotTransmission(_) => MeasurementPilotTransmission::NAME,
            IeData::MeasurementRequest(_) => MeasurementRequest::NAME,
            IeData::MeshConfiguration(_) => MeshConfiguration::NAME,
            IeData::MeshId(_) => MeshId::NAME,
            IeData::MuEdcaParameterSet(_) => MuEdcaParameterSet::NAME,
            IeData::OverlappingBssScanParams(_) => OverlappingBssScanParams::NAME,
            IeData::PowerCapability(_) => PowerCapability::NAME,
            IeData::PowerConstraint(_) => PowerConstraint::NAME,
            IeData::ReducedNeighborReport(_) => ReducedNeighborReport::NAME,
            IeData::RmEnabledCapabilities(_) => RmEnabledCapabilities::NAME,
            IeData::RoamingConsortium(_) => RoamingConsortium::NAME,
            IeData::Rsn(_) => Rsn::NAME,
            IeData::RsnExtension(_) => RsnExtension::NAME,
            IeData::SpatialReuseParameterSet(_) => SpatialReuseParameterSet::NAME,
            IeData::Ssid(_) => Ssid::NAME,
            IeData::SupportedOperatingClasses(_) => SupportedOperatingClasses::NAME,
            IeData::SupportedRates(_) => SupportedRates::NAME,
            IeData::Tim(_) => Tim::NAME,
            IeData::TimeAdvertisement(_) => TimeAdvertisement::NAME,
            IeData::TimeZone(_) => TimeZone::NAME,
            IeData::TpcReport(_) => TpcReport::NAME,
            IeData::TransmitPowerEnvelope(_) => TransmitPowerEnvelope::NAME,
            IeData::TwentyFortyBssCoexistence(_) => TwentyFortyBssCoexistence::NAME,
            IeData::TwentyFortyBssIntolerantChannelReport(_) => {
                TwentyFortyBssIntolerantChannelReport::NAME
            }
            IeData::VendorSpecific(_) => VendorSpecific::NAME,
            IeData::VhtCapabilities(_) => VhtCapabilities::NAME,
            IeData::VhtOperation(_) => VhtOperation::NAME,
            IeData::Unknown { .. } => Unknown::NAME,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.to_bytes().unwrap_or_default()
    }

    pub fn summary(&self) -> String {
        match &self.data {
            IeData::AdvertisementProtocol(data) => data.summary(),
            IeData::Antenna(data) => data.summary(),
            IeData::ApChannelReport(data) => data.summary(),
            IeData::ApConfigurationSequenceNumber(data) => data.summary(),
            IeData::AwakeWindow(data) => data.summary(),
            IeData::BssLoad(data) => data.summary(),
            IeData::ChallengeText(data) => data.summary(),
            IeData::ChannelSwitchAnnouncement(data) => data.summary(),
            IeData::Country(data) => data.summary(),
            IeData::DsParameterSet(data) => data.summary(),
            IeData::EhtCapabilities(data) => data.summary(),
            IeData::EhtOperation(data) => data.summary(),
            IeData::ErpInfo(data) => data.summary(),
            IeData::ExtendedCapabilities(data) => data.summary(),
            IeData::ExtendedChannelSwitchAnnouncement(data) => data.summary(),
            IeData::ExtendedSupportedRates(data) => data.summary(),
            IeData::FilsIndication(data) => data.summary(),
            IeData::HeCapabilities(data) => data.summary(),
            IeData::HeOperation(data) => data.summary(),
            IeData::HeSixGhzBandCapabilities(data) => data.summary(),
            IeData::HtCapabilities(data) => data.summary(),
            IeData::HtOperation(data) => data.summary(),
            IeData::IbssParameterSet(data) => data.summary(),
            IeData::Interworking(data) => data.summary(),
            IeData::MeasurementPilotTransmission(data) => data.summary(),
            IeData::MeasurementRequest(data) => data.summary(),
            IeData::MeshConfiguration(data) => data.summary(),
            IeData::MeshId(data) => data.summary(),
            IeData::MuEdcaParameterSet(data) => data.summary(),
            IeData::OverlappingBssScanParams(data) => data.summary(),
            IeData::PowerCapability(data) => data.summary(),
            IeData::PowerConstraint(data) => data.summary(),
            IeData::ReducedNeighborReport(data) => data.summary(),
            IeData::RmEnabledCapabilities(data) => data.summary(),
            IeData::RoamingConsortium(data) => data.summary(),
            IeData::Rsn(data) => data.summary(),
            IeData::RsnExtension(data) => data.summary(),
            IeData::SpatialReuseParameterSet(data) => data.summary(),
            IeData::Ssid(data) => data.summary(),
            IeData::SupportedOperatingClasses(data) => data.summary(),
            IeData::SupportedRates(data) => data.summary(),
            IeData::Tim(data) => data.summary(),
            IeData::TimeAdvertisement(data) => data.summary(),
            IeData::TimeZone(data) => data.summary(),
            IeData::TpcReport(data) => data.summary(),
            IeData::TransmitPowerEnvelope(data) => data.summary(),
            IeData::TwentyFortyBssCoexistence(data) => data.summary(),
            IeData::TwentyFortyBssIntolerantChannelReport(data) => data.summary(),
            IeData::VendorSpecific(data) => data.summary(),
            IeData::VhtCapabilities(data) => data.summary(),
            IeData::VhtOperation(data) => data.summary(),
            IeData::Unknown { unknown, .. } => unknown.summary(),
        }
    }

    pub fn fields(&self) -> Vec<Field> {
        match &self.data {
            IeData::AdvertisementProtocol(data) => data.fields(),
            IeData::Antenna(data) => data.fields(),
            IeData::ApChannelReport(data) => data.fields(),
            IeData::ApConfigurationSequenceNumber(data) => data.fields(),
            IeData::AwakeWindow(data) => data.fields(),
            IeData::BssLoad(data) => data.fields(),
            IeData::ChallengeText(data) => data.fields(),
            IeData::ChannelSwitchAnnouncement(data) => data.fields(),
            IeData::Country(data) => data.fields(),
            IeData::DsParameterSet(data) => data.fields(),
            IeData::EhtCapabilities(data) => data.fields(),
            IeData::EhtOperation(data) => data.fields(),
            IeData::ErpInfo(data) => data.fields(),
            IeData::ExtendedCapabilities(data) => data.fields(),
            IeData::ExtendedChannelSwitchAnnouncement(data) => data.fields(),
            IeData::ExtendedSupportedRates(data) => data.fields(),
            IeData::FilsIndication(data) => data.fields(),
            IeData::HeCapabilities(data) => data.fields(),
            IeData::HeOperation(data) => data.fields(),
            IeData::HeSixGhzBandCapabilities(data) => data.fields(),
            IeData::HtCapabilities(data) => data.fields(),
            IeData::HtOperation(data) => data.fields(),
            IeData::IbssParameterSet(data) => data.fields(),
            IeData::Interworking(data) => data.fields(),
            IeData::MeasurementPilotTransmission(data) => data.fields(),
            IeData::MeasurementRequest(data) => data.fields(),
            IeData::MeshConfiguration(data) => data.fields(),
            IeData::MeshId(data) => data.fields(),
            IeData::MuEdcaParameterSet(data) => data.fields(),
            IeData::OverlappingBssScanParams(data) => data.fields(),
            IeData::PowerCapability(data) => data.fields(),
            IeData::PowerConstraint(data) => data.fields(),
            IeData::ReducedNeighborReport(data) => data.fields(),
            IeData::RmEnabledCapabilities(data) => data.fields(),
            IeData::RoamingConsortium(data) => data.fields(),
            IeData::Rsn(data) => data.fields(),
            IeData::RsnExtension(data) => data.fields(),
            IeData::SpatialReuseParameterSet(data) => data.fields(),
            IeData::Ssid(data) => data.fields(),
            IeData::SupportedOperatingClasses(data) => data.fields(),
            IeData::SupportedRates(data) => data.fields(),
            IeData::Tim(data) => data.fields(),
            IeData::TimeAdvertisement(data) => data.fields(),
            IeData::TimeZone(data) => data.fields(),
            IeData::TpcReport(data) => data.fields(),
            IeData::TransmitPowerEnvelope(data) => data.fields(),
            IeData::TwentyFortyBssCoexistence(data) => data.fields(),
            IeData::TwentyFortyBssIntolerantChannelReport(data) => data.fields(),
            IeData::VendorSpecific(data) => data.fields(),
            IeData::VhtCapabilities(data) => data.fields(),
            IeData::VhtOperation(data) => data.fields(),
            IeData::Unknown { unknown, .. } => unknown.fields(),
        }
    }

    fn compute_data_len(len: u8, id_ext: Option<u8>) -> Result<usize, DekuError> {
        if id_ext.is_some() {
            (usize::from(len)).checked_sub(1).ok_or_else(|| {
                DekuError::Assertion("IE with ID extension has len=0 (invalid)".into())
            })
        } else {
            Ok(usize::from(len))
        }
    }
}

/// Parses a sequence of Information Elements from raw bytes.
///
/// Returns all successfully parsed IEs. If parsing fails partway through,
/// returns the IEs that were successfully parsed before the error.
/// Parse failures are logged at the `warn` level.
///
/// # Example
///
/// ```
/// # use kawaiifi::ies;
/// // Two IEs: SSID "Hello" + DS Parameter Set (channel 6)
/// let ie_bytes = &[
///     0x00, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f,  // SSID IE
///     0x03, 0x01, 0x06,                           // DS Parameter Set IE
/// ];
/// let ies = ies::from_bytes(ie_bytes);
/// assert_eq!(ies.len(), 2);
/// ```
pub fn from_bytes(bytes: &[u8]) -> Vec<Ie> {
    let mut ies = Vec::new();
    let mut input = bytes;

    while !input.is_empty() {
        let offset = bytes.len() - input.len();
        match Ie::from_bytes((input, 0)) {
            Ok(((rest, _), ie)) => {
                let bytes_read = input.len() - rest.len();
                let expected_bytes = usize::from(ie.len) + 2;
                if bytes_read != expected_bytes {
                    log::warn!(
                        "Incorrect number of bytes read for IE at offset {}: read {}, expected {}. IE: {:?}",
                        offset,
                        bytes_read,
                        expected_bytes,
                        &ie
                    );
                }
                #[cfg(debug_assertions)]
                if let Ok(serialized) = ie.to_bytes() {
                    if serialized.as_slice() != &input[..expected_bytes.min(input.len())] {
                        log::warn!(
                            "Mismatch between raw IE bytes and parsed Ie::to_bytes at offset {}. IE: {:?}",
                            offset,
                            &ie
                        );
                    }
                }
                ies.push(ie);
                input = &input[expected_bytes..];
            }
            Err(error) => {
                let failed_bytes = bytes
                    .get(offset..offset.saturating_add(20).min(bytes.len()))
                    .unwrap_or(&[]);
                log::warn!(
                    "Failed to parse IE at offset {} (parsed {} IEs successfully): {:?}. Failed bytes: {:02x?}",
                    offset,
                    ies.len(),
                    error,
                    failed_bytes
                );
                break;
            }
        }
    }

    ies
}

/// Parsed Information Element data.
///
/// Each variant corresponds to a specific IE type defined in the Wi-Fi standards.
/// Unknown or unrecognized IEs are captured in the `Unknown` variant.
#[derive(Clone, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: deku::ctx::ByteSize, ie_id: IeId", id = "ie_id")]
pub enum IeData {
    #[deku(id = "AdvertisementProtocol::IE_ID")]
    AdvertisementProtocol(#[deku(ctx = "len.0")] AdvertisementProtocol),
    #[deku(id = "Antenna::IE_ID")]
    Antenna(Antenna),
    #[deku(id = "ApChannelReport::IE_ID")]
    ApChannelReport(#[deku(ctx = "len.0")] ApChannelReport),
    #[deku(id = "ApConfigurationSequenceNumber::IE_ID")]
    ApConfigurationSequenceNumber(ApConfigurationSequenceNumber),
    #[deku(id = "AwakeWindow::IE_ID")]
    AwakeWindow(AwakeWindow),
    #[deku(id = "BssLoad::IE_ID")]
    BssLoad(BssLoad),
    #[deku(id = "ChallengeText::IE_ID")]
    ChallengeText(#[deku(ctx = "len.0")] ChallengeText),
    #[deku(id = "ChannelSwitchAnnouncement::IE_ID")]
    ChannelSwitchAnnouncement(ChannelSwitchAnnouncement),
    #[deku(id = "Country::IE_ID")]
    Country(#[deku(ctx = "len.0")] Country),
    #[deku(id = "DsParameterSet::IE_ID")]
    DsParameterSet(DsParameterSet),
    #[deku(id = "EhtCapabilities::IE_ID")]
    EhtCapabilities(#[deku(ctx = "len.0")] Box<EhtCapabilities>),
    #[deku(id = "EhtOperation::IE_ID")]
    EhtOperation(EhtOperation),
    #[deku(id_pat = "&ErpInfo::IE_ID | &ErpInfo::IE_ID_ALT")]
    ErpInfo(ErpInfo),
    #[deku(id = "ExtendedCapabilities::IE_ID")]
    ExtendedCapabilities(#[deku(ctx = "len.0")] ExtendedCapabilities),
    #[deku(id = "ExtendedChannelSwitchAnnouncement::IE_ID")]
    ExtendedChannelSwitchAnnouncement(ExtendedChannelSwitchAnnouncement),
    #[deku(id = "ExtendedSupportedRates::IE_ID")]
    ExtendedSupportedRates(#[deku(ctx = "len.0")] ExtendedSupportedRates),
    #[deku(id = "FilsIndication::IE_ID")]
    FilsIndication(Box<FilsIndication>),
    #[deku(id = "HeCapabilities::IE_ID")]
    HeCapabilities(Box<HeCapabilities>),
    #[deku(id = "HeOperation::IE_ID")]
    HeOperation(HeOperation),
    #[deku(id = "HeSixGhzBandCapabilities::IE_ID")]
    HeSixGhzBandCapabilities(HeSixGhzBandCapabilities),
    #[deku(id = "HtCapabilities::IE_ID")]
    HtCapabilities(Box<HtCapabilities>),
    #[deku(id = "HtOperation::IE_ID")]
    HtOperation(HtOperation),
    #[deku(id = "IbssParameterSet::IE_ID")]
    IbssParameterSet(IbssParameterSet),
    #[deku(id = "Interworking::IE_ID")]
    Interworking(#[deku(ctx = "len.0")] Interworking),
    #[deku(id = "MeasurementPilotTransmission::IE_ID")]
    MeasurementPilotTransmission(#[deku(ctx = "len.0")] MeasurementPilotTransmission),
    #[deku(id = "MeasurementRequest::IE_ID")]
    MeasurementRequest(#[deku(ctx = "len.0")] MeasurementRequest),
    #[deku(id = "MeshConfiguration::IE_ID")]
    MeshConfiguration(MeshConfiguration),
    #[deku(id = "MeshId::IE_ID")]
    MeshId(#[deku(ctx = "len.0")] MeshId),
    #[deku(id = "MuEdcaParameterSet::IE_ID")]
    MuEdcaParameterSet(MuEdcaParameterSet),
    #[deku(id = "OverlappingBssScanParams::IE_ID")]
    OverlappingBssScanParams(OverlappingBssScanParams),
    #[deku(id = "PowerCapability::IE_ID")]
    PowerCapability(PowerCapability),
    #[deku(id = "PowerConstraint::IE_ID")]
    PowerConstraint(PowerConstraint),
    #[deku(id = "ReducedNeighborReport::IE_ID")]
    ReducedNeighborReport(#[deku(ctx = "len.0")] ReducedNeighborReport),
    #[deku(id = "RmEnabledCapabilities::IE_ID")]
    RmEnabledCapabilities(RmEnabledCapabilities),
    #[deku(id = "RoamingConsortium::IE_ID")]
    RoamingConsortium(#[deku(ctx = "len.0")] Box<RoamingConsortium>),
    #[deku(id = "Rsn::IE_ID")]
    Rsn(#[deku(ctx = "len.0")] Box<Rsn>),
    #[deku(id = "RsnExtension::IE_ID")]
    RsnExtension(#[deku(ctx = "len.0")] RsnExtension),
    #[deku(id = "SpatialReuseParameterSet::IE_ID")]
    SpatialReuseParameterSet(SpatialReuseParameterSet),
    #[deku(id = "Ssid::IE_ID")]
    Ssid(#[deku(ctx = "len.0")] Ssid),
    #[deku(id = "SupportedOperatingClasses::IE_ID")]
    SupportedOperatingClasses(#[deku(ctx = "len.0")] SupportedOperatingClasses),
    #[deku(id = "SupportedRates::IE_ID")]
    SupportedRates(#[deku(ctx = "len.0")] SupportedRates),
    #[deku(id = "Tim::IE_ID")]
    Tim(#[deku(ctx = "len.0")] Tim),
    #[deku(id = "TimeAdvertisement::IE_ID")]
    TimeAdvertisement(#[deku(ctx = "len.0")] Box<TimeAdvertisement>),
    #[deku(id = "TimeZone::IE_ID")]
    TimeZone(#[deku(ctx = "len.0")] TimeZone),
    #[deku(id = "TpcReport::IE_ID")]
    TpcReport(TpcReport),
    #[deku(id = "TransmitPowerEnvelope::IE_ID")]
    TransmitPowerEnvelope(#[deku(ctx = "len.0")] TransmitPowerEnvelope),
    #[deku(id = "TwentyFortyBssCoexistence::IE_ID")]
    TwentyFortyBssCoexistence(TwentyFortyBssCoexistence),
    #[deku(id = "TwentyFortyBssIntolerantChannelReport::IE_ID")]
    TwentyFortyBssIntolerantChannelReport(
        #[deku(ctx = "len.0")] TwentyFortyBssIntolerantChannelReport,
    ),
    #[deku(id = "VendorSpecific::IE_ID")]
    VendorSpecific(#[deku(ctx = "len.0")] VendorSpecific),
    #[deku(id = "VhtCapabilities::IE_ID")]
    VhtCapabilities(VhtCapabilities),
    #[deku(id = "VhtOperation::IE_ID")]
    VhtOperation(VhtOperation),
    #[deku(id_pat = "_")]
    Unknown {
        #[deku(skip)]
        ie_id: IeId,
        #[deku(ctx = "len.0")]
        unknown: Unknown,
    },
}

/// Information Element identifier consisting of an ID and optional extension ID.
///
/// Used internally for IE type discrimination during parsing.
/// Standard IEs use only `id`, while extended IEs (id=255) also have `id_ext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, DekuWrite, Serialize, Deserialize)]
pub struct IeId {
    #[deku(skip)]
    pub id: u8,
    #[deku(skip)]
    pub id_ext: Option<u8>,
}

impl IeId {
    pub const fn new(id: u8, id_ext: Option<u8>) -> Self {
        Self { id, id_ext }
    }
}

pub(crate) fn write_bits_lsb0<W: std::io::Write + std::io::Seek>(
    writer: &mut deku::writer::Writer<W>,
    field: u8,
    bit_size: usize,
) -> Result<(), DekuError> {
    let bits = deku::bitvec::BitVec::from_element(field);
    writer.write_bits_order(&bits[bits.len() - bit_size..], deku::ctx::Order::Lsb0)
}
