use std::fmt::Display;

use deku::{DekuError, DekuRead, DekuWrite, bitvec::*};
use serde::{Deserialize, Serialize};

use crate::ies::{BitRange, Field, IeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(ctx = "len: usize")]
pub struct ExtendedCapabilities {
    #[deku(
        count = "len",
        map = "|bytes: Vec<u8>| -> Result<_, DekuError> { Ok(BitVec::<u8, Lsb0>::from_vec(bytes)) }",
        writer = "bits.clone().into_vec().to_writer(deku::writer, ())"
    )]
    #[serde(
        serialize_with = "serialize_bitvec",
        deserialize_with = "deserialize_bitvec"
    )]
    bits: BitVec<u8, Lsb0>,
}

// Helper functions for serde serialization of BitVec
fn serialize_bitvec<S>(bits: &BitVec<u8, Lsb0>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Convert BitVec to Vec<u8> for serialization
    let bytes = bits.clone().into_vec();
    bytes.serialize(serializer)
}

fn deserialize_bitvec<'de, D>(deserializer: D) -> Result<BitVec<u8, Lsb0>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // Deserialize as Vec<u8>, then convert to BitVec
    let bytes = Vec::<u8>::deserialize(deserializer)?;
    Ok(BitVec::<u8, Lsb0>::from_vec(bytes))
}

impl ExtendedCapabilities {
    pub const NAME: &'static str = "Extended Capabilities";
    pub const ID: u8 = 127;
    pub const ID_EXT: Option<u8> = None;
    pub(crate) const IE_ID: IeId = IeId::new(Self::ID, Self::ID_EXT);

    /// Helper to get a bit value, returning false if bit doesn't exist
    fn bit(&self, index: usize) -> bool {
        self.bits.get(index).as_deref().cloned().unwrap_or(false)
    }

    /// Helper to check if a bit index exists in the data
    fn has_bit(&self, index: usize) -> bool {
        index < self.bits.len()
    }

    pub fn twenty_forty_bss_coexistence_management_support(&self) -> bool {
        self.bit(0)
    }

    pub fn glk(&self) -> bool {
        self.bit(1)
    }

    pub fn extended_channel_switching(&self) -> bool {
        self.bit(2)
    }

    pub fn glk_gcr(&self) -> bool {
        self.bit(3)
    }

    pub fn psmp_capability(&self) -> bool {
        self.bit(4)
    }

    pub fn spsmp_support(&self) -> bool {
        self.bit(6)
    }

    pub fn event(&self) -> bool {
        self.bit(7)
    }

    pub fn diagnostics(&self) -> bool {
        self.bit(8)
    }

    pub fn multicast_diagnostics(&self) -> bool {
        self.bit(9)
    }

    pub fn location_tracking(&self) -> bool {
        self.bit(10)
    }

    pub fn fms(&self) -> bool {
        self.bit(11)
    }

    pub fn proxy_arp_service(&self) -> bool {
        self.bit(12)
    }

    pub fn collocated_interference_reporting(&self) -> bool {
        self.bit(13)
    }

    pub fn civic_location(&self) -> bool {
        self.bit(14)
    }

    pub fn geospatial_location(&self) -> bool {
        self.bit(15)
    }

    pub fn tfs(&self) -> bool {
        self.bit(16)
    }

    pub fn wnm_sleep_mode(&self) -> bool {
        self.bit(17)
    }

    pub fn tim_broadcast(&self) -> bool {
        self.bit(18)
    }

    pub fn bss_transition(&self) -> bool {
        self.bit(19)
    }

    pub fn qos_traffic_capability(&self) -> bool {
        self.bit(20)
    }

    pub fn ac_station_count(&self) -> bool {
        self.bit(21)
    }

    pub fn multiple_bssid(&self) -> bool {
        self.bit(22)
    }

    pub fn timing_measurement(&self) -> bool {
        self.bit(23)
    }

    pub fn channel_usage(&self) -> bool {
        self.bit(24)
    }

    pub fn ssid_list(&self) -> bool {
        self.bit(25)
    }

    pub fn dms(&self) -> bool {
        self.bit(26)
    }

    pub fn utc_tsf_offset(&self) -> bool {
        self.bit(27)
    }

    pub fn tpu_buffer_sta_support(&self) -> bool {
        self.bit(28)
    }

    pub fn tdls_peer_psm_support(&self) -> bool {
        self.bit(29)
    }

    pub fn tdls_channel_switching(&self) -> bool {
        self.bit(30)
    }

    pub fn interworking(&self) -> bool {
        self.bit(31)
    }

    pub fn qos_map(&self) -> bool {
        self.bit(32)
    }

    pub fn ebr(&self) -> bool {
        self.bit(33)
    }

    pub fn sspn_interface(&self) -> bool {
        self.bit(34)
    }

    pub fn msgcf_capability(&self) -> bool {
        self.bit(36)
    }

    pub fn tdls_support(&self) -> bool {
        self.bit(37)
    }

    pub fn tdls_prohibited(&self) -> bool {
        self.bit(38)
    }

    pub fn tdls_channel_switching_prohibited(&self) -> bool {
        self.bit(39)
    }

    pub fn reject_unadmitted_frame(&self) -> bool {
        self.bit(40)
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
        self.bit(44)
    }

    pub fn uapsd_coexistence(&self) -> bool {
        self.bit(45)
    }

    pub fn wnm_notification(&self) -> bool {
        self.bit(46)
    }

    pub fn qab_capability(&self) -> bool {
        self.bit(47)
    }

    pub fn utf8_ssid(&self) -> bool {
        self.bit(48)
    }

    pub fn qmf_activated(&self) -> bool {
        self.bit(49)
    }

    pub fn qmf_reconfiguration_activated(&self) -> bool {
        self.bit(50)
    }

    pub fn robust_av_streaming(&self) -> bool {
        self.bit(51)
    }

    pub fn advanced_gcr(&self) -> bool {
        self.bit(52)
    }

    pub fn mesh_gcr(&self) -> bool {
        self.bit(53)
    }

    pub fn scs(&self) -> bool {
        self.bit(54)
    }

    pub fn qload_report(&self) -> bool {
        self.bit(55)
    }

    pub fn alternate_edca(&self) -> bool {
        self.bit(56)
    }

    pub fn unprotected_txop_negotiation(&self) -> bool {
        self.bit(57)
    }

    pub fn protected_txop_negotiation(&self) -> bool {
        self.bit(58)
    }

    pub fn protected_qload_report(&self) -> bool {
        self.bit(60)
    }

    pub fn tdls_wider_bandwidth(&self) -> bool {
        self.bit(61)
    }

    pub fn operating_mode_notification(&self) -> bool {
        self.bit(62)
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
        self.bit(65)
    }

    pub fn geodatabase_inband_enabling_signal(&self) -> bool {
        self.bit(66)
    }

    pub fn network_channel_control(&self) -> bool {
        self.bit(67)
    }

    pub fn white_space_map(&self) -> bool {
        self.bit(68)
    }

    pub fn channel_availability_query(&self) -> bool {
        self.bit(69)
    }

    pub fn fine_timing_measurement_responder(&self) -> bool {
        self.bit(70)
    }

    pub fn fine_timing_measurement_initiator(&self) -> bool {
        self.bit(71)
    }

    pub fn fils_capability(&self) -> bool {
        self.bit(72)
    }

    pub fn extended_spectrum_management_capable(&self) -> bool {
        self.bit(73)
    }

    pub fn future_channel_guidance(&self) -> bool {
        self.bit(74)
    }

    pub fn pad(&self) -> bool {
        self.bit(75)
    }

    pub fn complete_list_of_non_tx_bssid_profiles(&self) -> bool {
        self.bit(80)
    }

    pub fn sae_password_identifiers_in_use(&self) -> bool {
        self.bit(81)
    }

    pub fn sae_password_identifiers_used_exclusively(&self) -> bool {
        self.bit(82)
    }

    pub fn beacon_protection_enabled(&self) -> bool {
        self.bit(84)
    }

    pub fn mirrored_scs(&self) -> bool {
        self.bit(85)
    }

    pub fn local_mac_address_policy(&self) -> bool {
        self.bit(87)
    }

    pub fn summary(&self) -> String {
        "".to_string()
    }

    pub fn fields(&self) -> Vec<Field> {
        let bytes: Vec<u8> = self.bits.clone().into_vec();
        let mut fields = Vec::new();

        // Define capabilities grouped by octet
        // Format: (bit_index, title, num_bits, is_reserved)
        // num_bits > 1 for multi-bit fields
        let octets: &[&[(usize, &str, usize, bool)]] = &[
            // Octet 1 (bits 0-7)
            &[
                (0, "20/40 BSS Coexistence Management Support", 1, false),
                (1, "GLK", 1, false),
                (2, "Extended Channel Switching", 1, false),
                (3, "GLK-GCR", 1, false),
                (4, "PSMP Capability", 1, false),
                (5, "Reserved", 1, true),
                (6, "S-PSMP Support", 1, false),
                (7, "Event", 1, false),
            ],
            // Octet 2 (bits 8-15)
            &[
                (8, "Diagnostics", 1, false),
                (9, "Multicast Diagnostics", 1, false),
                (10, "Location Tracking", 1, false),
                (11, "FMS", 1, false),
                (12, "Proxy ARP Service", 1, false),
                (13, "Collocated Interference Reporting", 1, false),
                (14, "Civic Location", 1, false),
                (15, "Geospatial Location", 1, false),
            ],
            // Octet 3 (bits 16-23)
            &[
                (16, "TFS", 1, false),
                (17, "WNM Sleep Mode", 1, false),
                (18, "TIM Broadcast", 1, false),
                (19, "BSS Transition", 1, false),
                (20, "QoS Traffic Capability", 1, false),
                (21, "AC Station Count", 1, false),
                (22, "Multiple BSSID", 1, false),
                (23, "Timing Measurement", 1, false),
            ],
            // Octet 4 (bits 24-31)
            &[
                (24, "Channel Usage", 1, false),
                (25, "SSID List", 1, false),
                (26, "DMS", 1, false),
                (27, "UTC TSF Offset", 1, false),
                (28, "TPU Buffer STA Support", 1, false),
                (29, "TDLS Peer PSM Support", 1, false),
                (30, "TDLS Channel Switching", 1, false),
                (31, "Interworking", 1, false),
            ],
            // Octet 5 (bits 32-39)
            &[
                (32, "QoS Map", 1, false),
                (33, "EBR", 1, false),
                (34, "SSPN Interface", 1, false),
                (35, "Reserved", 1, true),
                (36, "MSGCF Capability", 1, false),
                (37, "TDLS Support", 1, false),
                (38, "TDLS Prohibited", 1, false),
                (39, "TDLS Channel Switching Prohibited", 1, false),
            ],
            // Octet 6 (bits 40-47)
            &[
                (40, "Reject Unadmitted Frame", 1, false),
                (41, "Service Interval Granularity", 3, false),
                (44, "Identifier Location", 1, false),
                (45, "U-APSD Coexistence", 1, false),
                (46, "WNM Notification", 1, false),
                (47, "QAB Capability", 1, false),
            ],
            // Octet 7 (bits 48-55)
            &[
                (48, "UTF-8 SSID", 1, false),
                (49, "QMF Activated", 1, false),
                (50, "QMF Reconfiguration Activated", 1, false),
                (51, "Robust AV Streaming", 1, false),
                (52, "Advanced GCR", 1, false),
                (53, "Mesh GCR", 1, false),
                (54, "SCS", 1, false),
                (55, "QLoad Report", 1, false),
            ],
            // Octet 8 (bits 56-63)
            &[
                (56, "Alternate EDCA", 1, false),
                (57, "Unprotected TXOP Negotiation", 1, false),
                (58, "Protected TXOP Negotiation", 1, false),
                (59, "Reserved", 1, true),
                (60, "Protected QLoad Report", 1, false),
                (61, "TDLS Wider Bandwidth", 1, false),
                (62, "Operating Mode Notification", 1, false),
                (63, "Max MSDUs in A-MSDU", 2, false),
            ],
            // Octet 9 (bits 65-71, bit 64 is part of Max MSDUs)
            &[
                (65, "Channel Schedule Management", 1, false),
                (66, "Geodatabase Inband Enabling Signal", 1, false),
                (67, "Network Channel Control", 1, false),
                (68, "White Space Map", 1, false),
                (69, "Channel Availability Query", 1, false),
                (70, "Fine Timing Measurement Responder", 1, false),
                (71, "Fine Timing Measurement Initiator", 1, false),
            ],
            // Octet 10 (bits 72-79)
            &[
                (72, "FILS Capability", 1, false),
                (73, "Extended Spectrum Management Capable", 1, false),
                (74, "Future Channel Guidance", 1, false),
                (75, "PAD", 1, false),
                (76, "Reserved", 1, true),
                (77, "Reserved", 1, true),
                (78, "Reserved", 1, true),
                (79, "Reserved", 1, true),
            ],
            // Octet 11 (bits 80-87)
            &[
                (80, "Complete List of Non-TX BSSID Profiles", 1, false),
                (81, "SAE Password Identifiers In Use", 1, false),
                (82, "SAE Password Identifiers Used Exclusively", 1, false),
                (83, "Reserved", 1, true),
                (84, "Beacon Protection Enabled", 1, false),
                (85, "Mirrored SCS", 1, false),
                (86, "Reserved", 1, true),
                (87, "Local MAC Address Policy", 1, false),
            ],
        ];

        for (octet_index, octet) in octets.iter().enumerate() {
            let octet_num = octet_index + 1;
            let byte_index = octet_index;

            // Check if this octet exists in the data
            if byte_index >= bytes.len() {
                break;
            }

            let byte = bytes[byte_index];
            let mut subfields = Vec::new();

            for &(bit_index, title, num_bits, is_reserved) in *octet {
                // For multi-bit fields spanning octets, check if all bits exist
                let last_bit = bit_index + num_bits - 1;
                if !self.has_bit(last_bit) {
                    continue;
                }

                // Convert global bit index to bit position within this byte (0-7)
                let bit_in_byte = bit_index % 8;

                if is_reserved {
                    subfields.push(Field::reserved(BitRange::from_byte(
                        byte,
                        bit_in_byte,
                        num_bits,
                    )));
                } else if num_bits == 1 {
                    subfields.push(
                        Field::builder()
                            .title(title)
                            .value(self.bit(bit_index))
                            .bits(BitRange::from_byte(byte, bit_in_byte, num_bits))
                            .build(),
                    );
                } else if title == "Service Interval Granularity" {
                    let value = self
                        .service_interval_granularity_ms()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Unknown".to_string());
                    subfields.push(
                        Field::builder()
                            .title(title)
                            .value(&value)
                            .bits(BitRange::from_byte(byte, bit_in_byte, num_bits))
                            .units(if value != "Unknown" { "ms" } else { "" })
                            .build(),
                    );
                } else if title == "Max MSDUs in A-MSDU" {
                    // This field spans bytes 7-8 (bits 63-64), so use a 2-byte slice
                    let value = self
                        .max_msdus_in_amsdu()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Unknown".to_string());
                    subfields.push(
                        Field::builder()
                            .title(title)
                            .value(value)
                            .bits(BitRange::new(
                                &bytes[byte_index..byte_index + 2],
                                bit_in_byte,
                                num_bits,
                            ))
                            .build(),
                    );
                }
            }

            fields.push(
                Field::builder()
                    .title(format!("Extended Capabilities (Octet {})", octet_num))
                    .value("")
                    .byte(byte)
                    .subfields(subfields)
                    .build(),
            );
        }

        fields
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
