use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Field, IeData, IeId, elements::*};

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
