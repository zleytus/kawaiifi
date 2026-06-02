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
    /// Advertisement Protocol element.
    #[deku(id = "AdvertisementProtocol::IE_ID")]
    AdvertisementProtocol(#[deku(ctx = "len.0")] AdvertisementProtocol),
    /// Antenna element.
    #[deku(id = "Antenna::IE_ID")]
    Antenna(Antenna),
    /// AP Channel Report element.
    #[deku(id = "ApChannelReport::IE_ID")]
    ApChannelReport(#[deku(ctx = "len.0")] ApChannelReport),
    /// AP Configuration Sequence Number element.
    #[deku(id = "ApConfigurationSequenceNumber::IE_ID")]
    ApConfigurationSequenceNumber(ApConfigurationSequenceNumber),
    /// Awake Window element.
    #[deku(id = "AwakeWindow::IE_ID")]
    AwakeWindow(AwakeWindow),
    /// BSS Load element.
    #[deku(id = "BssLoad::IE_ID")]
    BssLoad(BssLoad),
    /// Challenge Text element.
    #[deku(id = "ChallengeText::IE_ID")]
    ChallengeText(#[deku(ctx = "len.0")] ChallengeText),
    /// Channel Switch Announcement element.
    #[deku(id = "ChannelSwitchAnnouncement::IE_ID")]
    ChannelSwitchAnnouncement(ChannelSwitchAnnouncement),
    /// Country element.
    #[deku(id = "Country::IE_ID")]
    Country(#[deku(ctx = "len.0")] Country),
    /// DS Parameter Set element.
    #[deku(id = "DsParameterSet::IE_ID")]
    DsParameterSet(DsParameterSet),
    /// EHT Capabilities element.
    #[deku(id = "EhtCapabilities::IE_ID")]
    EhtCapabilities(#[deku(ctx = "len.0")] Box<EhtCapabilities>),
    /// EHT Operation element.
    #[deku(id = "EhtOperation::IE_ID")]
    EhtOperation(EhtOperation),
    /// ERP Information element.
    #[deku(id_pat = "&ErpInfo::IE_ID | &ErpInfo::IE_ID_ALT")]
    ErpInfo(ErpInfo),
    /// Extended Capabilities element.
    #[deku(id = "ExtendedCapabilities::IE_ID")]
    ExtendedCapabilities(#[deku(ctx = "len.0")] ExtendedCapabilities),
    /// Extended Channel Switch Announcement element.
    #[deku(id = "ExtendedChannelSwitchAnnouncement::IE_ID")]
    ExtendedChannelSwitchAnnouncement(ExtendedChannelSwitchAnnouncement),
    /// Extended Supported Rates element.
    #[deku(id = "ExtendedSupportedRates::IE_ID")]
    ExtendedSupportedRates(#[deku(ctx = "len.0")] ExtendedSupportedRates),
    /// FILS Indication element.
    #[deku(id = "FilsIndication::IE_ID")]
    FilsIndication(Box<FilsIndication>),
    /// HE Capabilities element.
    #[deku(id = "HeCapabilities::IE_ID")]
    HeCapabilities(Box<HeCapabilities>),
    /// HE Operation element.
    #[deku(id = "HeOperation::IE_ID")]
    HeOperation(HeOperation),
    /// HE 6 GHz Band Capabilities element.
    #[deku(id = "HeSixGhzBandCapabilities::IE_ID")]
    HeSixGhzBandCapabilities(HeSixGhzBandCapabilities),
    /// HT Capabilities element.
    #[deku(id = "HtCapabilities::IE_ID")]
    HtCapabilities(Box<HtCapabilities>),
    /// HT Operation element.
    #[deku(id = "HtOperation::IE_ID")]
    HtOperation(HtOperation),
    /// IBSS Parameter Set element.
    #[deku(id = "IbssParameterSet::IE_ID")]
    IbssParameterSet(IbssParameterSet),
    /// Interworking element.
    #[deku(id = "Interworking::IE_ID")]
    Interworking(#[deku(ctx = "len.0")] Interworking),
    /// Measurement Pilot Transmission element.
    #[deku(id = "MeasurementPilotTransmission::IE_ID")]
    MeasurementPilotTransmission(#[deku(ctx = "len.0")] MeasurementPilotTransmission),
    /// Measurement Request element.
    #[deku(id = "MeasurementRequest::IE_ID")]
    MeasurementRequest(#[deku(ctx = "len.0")] MeasurementRequest),
    /// Mesh Configuration element.
    #[deku(id = "MeshConfiguration::IE_ID")]
    MeshConfiguration(MeshConfiguration),
    /// Mesh ID element.
    #[deku(id = "MeshId::IE_ID")]
    MeshId(#[deku(ctx = "len.0")] MeshId),
    /// Mobility Domain element.
    #[deku(id = "MobilityDomain::IE_ID")]
    MobilityDomain(MobilityDomain),
    /// MU EDCA Parameter Set element.
    #[deku(id = "MuEdcaParameterSet::IE_ID")]
    MuEdcaParameterSet(MuEdcaParameterSet),
    /// Overlapping BSS Scan Parameters element.
    #[deku(id = "OverlappingBssScanParams::IE_ID")]
    OverlappingBssScanParams(OverlappingBssScanParams),
    /// Power Capability element.
    #[deku(id = "PowerCapability::IE_ID")]
    PowerCapability(PowerCapability),
    /// Power Constraint element.
    #[deku(id = "PowerConstraint::IE_ID")]
    PowerConstraint(PowerConstraint),
    /// Reduced Neighbor Report element.
    #[deku(id = "ReducedNeighborReport::IE_ID")]
    ReducedNeighborReport(#[deku(ctx = "len.0")] ReducedNeighborReport),
    /// RM Enabled Capabilities element.
    #[deku(id = "RmEnabledCapabilities::IE_ID")]
    RmEnabledCapabilities(RmEnabledCapabilities),
    /// Roaming Consortium element.
    #[deku(id = "RoamingConsortium::IE_ID")]
    RoamingConsortium(#[deku(ctx = "len.0")] Box<RoamingConsortium>),
    /// RSN element.
    #[deku(id = "Rsn::IE_ID")]
    Rsn(#[deku(ctx = "len.0")] Box<Rsn>),
    /// RSN Extension element.
    #[deku(id = "RsnExtension::IE_ID")]
    RsnExtension(#[deku(ctx = "len.0")] RsnExtension),
    /// Spatial Reuse Parameter Set element.
    #[deku(id = "SpatialReuseParameterSet::IE_ID")]
    SpatialReuseParameterSet(SpatialReuseParameterSet),
    /// SSID element.
    #[deku(id = "Ssid::IE_ID")]
    Ssid(#[deku(ctx = "len.0")] Ssid),
    /// Supported Operating Classes element.
    #[deku(id = "SupportedOperatingClasses::IE_ID")]
    SupportedOperatingClasses(#[deku(ctx = "len.0")] SupportedOperatingClasses),
    /// Supported Rates element.
    #[deku(id = "SupportedRates::IE_ID")]
    SupportedRates(#[deku(ctx = "len.0")] SupportedRates),
    /// TIM element.
    #[deku(id = "Tim::IE_ID")]
    Tim(#[deku(ctx = "len.0")] Tim),
    /// Time Advertisement element.
    #[deku(id = "TimeAdvertisement::IE_ID")]
    TimeAdvertisement(#[deku(ctx = "len.0")] Box<TimeAdvertisement>),
    /// Time Zone element.
    #[deku(id = "TimeZone::IE_ID")]
    TimeZone(#[deku(ctx = "len.0")] TimeZone),
    /// TPC Report element.
    #[deku(id = "TpcReport::IE_ID")]
    TpcReport(TpcReport),
    /// Transmit Power Envelope element.
    #[deku(id = "TransmitPowerEnvelope::IE_ID")]
    TransmitPowerEnvelope(#[deku(ctx = "len.0")] TransmitPowerEnvelope),
    /// 20/40 BSS Coexistence element.
    #[deku(id = "TwentyFortyBssCoexistence::IE_ID")]
    TwentyFortyBssCoexistence(TwentyFortyBssCoexistence),
    /// 20/40 BSS Intolerant Channel Report element.
    #[deku(id = "TwentyFortyBssIntolerantChannelReport::IE_ID")]
    TwentyFortyBssIntolerantChannelReport(
        #[deku(ctx = "len.0")] TwentyFortyBssIntolerantChannelReport,
    ),
    /// Vendor Specific element.
    #[deku(id = "VendorSpecific::IE_ID")]
    VendorSpecific(#[deku(ctx = "len.0")] VendorSpecific),
    /// VHT Capabilities element.
    #[deku(id = "VhtCapabilities::IE_ID")]
    VhtCapabilities(VhtCapabilities),
    /// VHT Operation element.
    #[deku(id = "VhtOperation::IE_ID")]
    VhtOperation(VhtOperation),
    /// Unknown or unsupported Information Element.
    #[deku(id_pat = "_")]
    Unknown {
        /// The original Information Element identifier.
        #[deku(skip)]
        ie_id: IeId,
        /// The raw unknown element payload.
        #[deku(ctx = "len.0")]
        unknown: Unknown,
    },
}
