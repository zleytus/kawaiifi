use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{IeId, elements::*};

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
