use std::fmt::Display;

use deku::{DekuError, DekuRead, DekuWrite, bitvec::*};

use super::IeId;

#[derive(Debug, Clone, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(ctx = "len: usize")]
pub struct ExtendedCapabilities {
    #[deku(
        count = "len",
        map = "|bytes: Vec<u8>| -> Result<_, DekuError> { Ok(BitVec::<u8, Lsb0>::from_vec(bytes)) }",
        writer = "bits.clone().into_vec().to_writer(deku::writer, ())"
    )]
    bits: BitVec<u8, Lsb0>,
}

impl ExtendedCapabilities {
    pub const NAME: &'static str = "Extended Capabilities";
    pub const ID: u8 = 127;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    pub fn twenty_forty_bss_coexistence_management_support(&self) -> bool {
        self.bits.get(0).as_deref().cloned().unwrap_or(false)
    }

    pub fn glk(&self) -> bool {
        self.bits.get(1).as_deref().cloned().unwrap_or(false)
    }

    pub fn extended_channel_switching(&self) -> bool {
        self.bits.get(2).as_deref().cloned().unwrap_or(false)
    }

    pub fn glk_gcr(&self) -> bool {
        self.bits.get(3).as_deref().cloned().unwrap_or(false)
    }

    pub fn psmp_capability(&self) -> bool {
        self.bits.get(4).as_deref().cloned().unwrap_or(false)
    }

    pub fn spsmp_support(&self) -> bool {
        self.bits.get(6).as_deref().cloned().unwrap_or(false)
    }

    pub fn event(&self) -> bool {
        self.bits.get(7).as_deref().cloned().unwrap_or(false)
    }

    pub fn diagnostics(&self) -> bool {
        self.bits.get(8).as_deref().cloned().unwrap_or(false)
    }

    pub fn multicast_diagnostics(&self) -> bool {
        self.bits.get(9).as_deref().cloned().unwrap_or(false)
    }

    pub fn location_tracking(&self) -> bool {
        self.bits.get(10).as_deref().cloned().unwrap_or(false)
    }

    pub fn fms(&self) -> bool {
        self.bits.get(11).as_deref().cloned().unwrap_or(false)
    }

    pub fn proxy_arp_service(&self) -> bool {
        self.bits.get(12).as_deref().cloned().unwrap_or(false)
    }

    pub fn collocated_interference_reporting(&self) -> bool {
        self.bits.get(13).as_deref().cloned().unwrap_or(false)
    }

    pub fn civic_location(&self) -> bool {
        self.bits.get(14).as_deref().cloned().unwrap_or(false)
    }

    pub fn geospatial_location(&self) -> bool {
        self.bits.get(15).as_deref().cloned().unwrap_or(false)
    }

    pub fn tfs(&self) -> bool {
        self.bits.get(16).as_deref().cloned().unwrap_or(false)
    }

    pub fn wnm_sleep_mode(&self) -> bool {
        self.bits.get(17).as_deref().cloned().unwrap_or(false)
    }

    pub fn tim_broadcast(&self) -> bool {
        self.bits.get(18).as_deref().cloned().unwrap_or(false)
    }

    pub fn bss_transition(&self) -> bool {
        self.bits.get(19).as_deref().cloned().unwrap_or(false)
    }

    pub fn qos_traffic_capability(&self) -> bool {
        self.bits.get(20).as_deref().cloned().unwrap_or(false)
    }

    pub fn ac_station_count(&self) -> bool {
        self.bits.get(21).as_deref().cloned().unwrap_or(false)
    }

    pub fn multiple_bssid(&self) -> bool {
        self.bits.get(22).as_deref().cloned().unwrap_or(false)
    }

    pub fn timing_measurement(&self) -> bool {
        self.bits.get(23).as_deref().cloned().unwrap_or(false)
    }

    pub fn channel_usage(&self) -> bool {
        self.bits.get(24).as_deref().cloned().unwrap_or(false)
    }

    pub fn ssid_list(&self) -> bool {
        self.bits.get(25).as_deref().cloned().unwrap_or(false)
    }

    pub fn dms(&self) -> bool {
        self.bits.get(26).as_deref().cloned().unwrap_or(false)
    }

    pub fn utc_tsf_offset(&self) -> bool {
        self.bits.get(27).as_deref().cloned().unwrap_or(false)
    }

    pub fn tpu_buffer_sta_support(&self) -> bool {
        self.bits.get(28).as_deref().cloned().unwrap_or(false)
    }

    pub fn tdls_peer_psm_support(&self) -> bool {
        self.bits.get(29).as_deref().cloned().unwrap_or(false)
    }

    pub fn tdls_channel_switching(&self) -> bool {
        self.bits.get(30).as_deref().cloned().unwrap_or(false)
    }

    pub fn interworking(&self) -> bool {
        self.bits.get(31).as_deref().cloned().unwrap_or(false)
    }

    pub fn qos_map(&self) -> bool {
        self.bits.get(32).as_deref().cloned().unwrap_or(false)
    }

    pub fn ebr(&self) -> bool {
        self.bits.get(33).as_deref().cloned().unwrap_or(false)
    }

    pub fn sspn_interface(&self) -> bool {
        self.bits.get(34).as_deref().cloned().unwrap_or(false)
    }

    pub fn msgcf_capability(&self) -> bool {
        self.bits.get(36).as_deref().cloned().unwrap_or(false)
    }

    pub fn tdls_support(&self) -> bool {
        self.bits.get(37).as_deref().cloned().unwrap_or(false)
    }

    pub fn tdls_prohibited(&self) -> bool {
        self.bits.get(38).as_deref().cloned().unwrap_or(false)
    }

    pub fn tdls_channel_switching_prohibited(&self) -> bool {
        self.bits.get(39).as_deref().cloned().unwrap_or(false)
    }

    pub fn reject_unadmitted_frame(&self) -> bool {
        self.bits.get(40).as_deref().cloned().unwrap_or(false)
    }

    pub fn service_interval_granularity_ms(&self) -> Option<u8> {
        if let Some(bit_slice) = self.bits.get(41..=43) {
            match bit_slice.load::<u8>() {
                0 => Some(5),
                1 => Some(10),
                2 => Some(15),
                3 => Some(20),
                4 => Some(25),
                5 => Some(30),
                6 => Some(35),
                7 => Some(40),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn identifier_location(&self) -> bool {
        self.bits.get(44).as_deref().cloned().unwrap_or(false)
    }

    pub fn uapsd_coexistence(&self) -> bool {
        self.bits.get(45).as_deref().cloned().unwrap_or(false)
    }

    pub fn wnm_notification(&self) -> bool {
        self.bits.get(46).as_deref().cloned().unwrap_or(false)
    }

    pub fn qab_capability(&self) -> bool {
        self.bits.get(47).as_deref().cloned().unwrap_or(false)
    }

    pub fn utf8_ssid(&self) -> bool {
        self.bits.get(48).as_deref().cloned().unwrap_or(false)
    }

    pub fn qmf_activated(&self) -> bool {
        self.bits.get(49).as_deref().cloned().unwrap_or(false)
    }

    pub fn qmf_reconfiguration_activated(&self) -> bool {
        self.bits.get(50).as_deref().cloned().unwrap_or(false)
    }

    pub fn robust_av_streaming(&self) -> bool {
        self.bits.get(51).as_deref().cloned().unwrap_or(false)
    }

    pub fn advanced_gcr(&self) -> bool {
        self.bits.get(52).as_deref().cloned().unwrap_or(false)
    }

    pub fn mesh_gcr(&self) -> bool {
        self.bits.get(53).as_deref().cloned().unwrap_or(false)
    }

    pub fn scs(&self) -> bool {
        self.bits.get(54).as_deref().cloned().unwrap_or(false)
    }

    pub fn qload_report(&self) -> bool {
        self.bits.get(55).as_deref().cloned().unwrap_or(false)
    }

    pub fn alternate_edca(&self) -> bool {
        self.bits.get(56).as_deref().cloned().unwrap_or(false)
    }

    pub fn unprotected_txop_negotiation(&self) -> bool {
        self.bits.get(57).as_deref().cloned().unwrap_or(false)
    }

    pub fn protected_txop_negotiation(&self) -> bool {
        self.bits.get(58).as_deref().cloned().unwrap_or(false)
    }

    pub fn protected_qload_report(&self) -> bool {
        self.bits.get(60).as_deref().cloned().unwrap_or(false)
    }

    pub fn tdls_wider_bandwidth(&self) -> bool {
        self.bits.get(61).as_deref().cloned().unwrap_or(false)
    }

    pub fn operating_mode_notification(&self) -> bool {
        self.bits.get(62).as_deref().cloned().unwrap_or(false)
    }

    pub fn max_msdus_in_amsdu(&self) -> Option<MaxMsdus> {
        if let Some(bit_slice) = self.bits.get(63..=64) {
            match bit_slice.load::<u8>() {
                0 => Some(MaxMsdus::NoLimit),
                1 => Some(MaxMsdus::ThirtyTwo),
                2 => Some(MaxMsdus::Sixteen),
                3 => Some(MaxMsdus::Eight),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn channel_schedule_management(&self) -> bool {
        self.bits.get(65).as_deref().cloned().unwrap_or(false)
    }

    pub fn geodatabase_inband_enabling_signal(&self) -> bool {
        self.bits.get(66).as_deref().cloned().unwrap_or(false)
    }

    pub fn network_channel_control(&self) -> bool {
        self.bits.get(67).as_deref().cloned().unwrap_or(false)
    }

    pub fn white_space_map(&self) -> bool {
        self.bits.get(68).as_deref().cloned().unwrap_or(false)
    }

    pub fn channel_availability_query(&self) -> bool {
        self.bits.get(69).as_deref().cloned().unwrap_or(false)
    }

    pub fn fine_timing_measurement_responder(&self) -> bool {
        self.bits.get(70).as_deref().cloned().unwrap_or(false)
    }

    pub fn fine_timing_measurement_initiator(&self) -> bool {
        self.bits.get(71).as_deref().cloned().unwrap_or(false)
    }

    pub fn fils_capability(&self) -> bool {
        self.bits.get(72).as_deref().cloned().unwrap_or(false)
    }

    pub fn extended_spectrum_management_capable(&self) -> bool {
        self.bits.get(73).as_deref().cloned().unwrap_or(false)
    }

    pub fn future_channel_guidance(&self) -> bool {
        self.bits.get(74).as_deref().cloned().unwrap_or(false)
    }

    pub fn pad(&self) -> bool {
        self.bits.get(75).as_deref().cloned().unwrap_or(false)
    }

    pub fn complete_list_of_non_tx_bssid_profiles(&self) -> bool {
        self.bits.get(80).as_deref().cloned().unwrap_or(false)
    }

    pub fn sae_password_identifiers_in_use(&self) -> bool {
        self.bits.get(81).as_deref().cloned().unwrap_or(false)
    }

    pub fn sae_password_identifiers_used_exclusively(&self) -> bool {
        self.bits.get(82).as_deref().cloned().unwrap_or(false)
    }

    pub fn beacon_protection_enabled(&self) -> bool {
        self.bits.get(84).as_deref().cloned().unwrap_or(false)
    }

    pub fn mirrored_scs(&self) -> bool {
        self.bits.get(85).as_deref().cloned().unwrap_or(false)
    }

    pub fn local_mac_address_policy(&self) -> bool {
        self.bits.get(87).as_deref().cloned().unwrap_or(false)
    }
}

pub enum MaxMsdus {
    NoLimit,
    ThirtyTwo,
    Sixteen,
    Eight,
}

impl Display for MaxMsdus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxMsdus::NoLimit => write!(f, "No Limit"),
            MaxMsdus::ThirtyTwo => write!(f, "32"),
            MaxMsdus::Sixteen => write!(f, "16"),
            MaxMsdus::Eight => write!(f, "8"),
        }
    }
}
