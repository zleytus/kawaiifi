use kawaiifi::{Band, ChannelWidth};

pub(super) const CHART_FLOOR: f64 = -100.0;
pub(super) const NOISE_FLOOR: f64 = -110.0;

/// Data for a single BSS to be displayed on the chart.
#[derive(Clone)]
pub struct BssChartData {
    pub ssid: Option<String>,
    pub freq: f64,
    pub signal: f64,
    pub width: f64,
    pub color: (u8, u8, u8),
    pub selected: bool,
}

pub(super) fn channel_width_half_mhz(width: ChannelWidth) -> f64 {
    match width {
        ChannelWidth::FortyMhz => 20.0,
        ChannelWidth::EightyMhz => 40.0,
        ChannelWidth::OneSixtyMhz => 80.0,
        ChannelWidth::ThreeHundredTwentyMhz => 160.0,
        _ => 10.0,
    }
}

pub(super) fn frequency_to_channel(freq: i32, band: Band) -> String {
    match band {
        Band::TwoPointFourGhz => {
            if freq == 2484 {
                "14".to_string()
            } else {
                (((freq - 2412) / 5) + 1).to_string()
            }
        }
        Band::FiveGhz => ((freq - 5000) / 5).to_string(),
        Band::SixGhz => ((freq - 5950) / 5).to_string(),
    }
}

pub(super) fn get_band_frequency_range(band: Band) -> (i32, i32) {
    match band {
        Band::TwoPointFourGhz => (2392, 2495),
        Band::FiveGhz => (5150, 5895),
        Band::SixGhz => (5935, 7115),
    }
}

pub(super) fn bss_shape_points(bss: &BssChartData, band: Band) -> Vec<(f64, f64)> {
    match band {
        Band::TwoPointFourGhz => {
            // The 802.11a/g OFDM spectral mask is trapezoidal: flat at 0 dBr out to
            // +/-9 MHz from center, then rolling off to -20 dBr at +/-11 MHz. We map the
            // rolloff edge to the noise floor. As a ratio of the half-width (10 MHz
            // for a 20 MHz channel): flat top at 0.9x, base at 1.1x.
            let flat = bss.width * 0.9;
            let base = bss.width * 1.1;
            vec![
                (bss.freq - base, CHART_FLOOR),
                (bss.freq - flat, bss.signal),
                (bss.freq + flat, bss.signal),
                (bss.freq + base, CHART_FLOOR),
            ]
        }
        Band::FiveGhz | Band::SixGhz => vec![
            (bss.freq - bss.width, NOISE_FLOOR),
            (bss.freq - bss.width, bss.signal),
            (bss.freq + bss.width, bss.signal),
            (bss.freq + bss.width, NOISE_FLOOR),
        ],
    }
}

// Frequencies used for x-axis labels only.
pub(super) fn get_channel_frequencies(band: Band) -> Vec<i32> {
    match band {
        Band::TwoPointFourGhz => {
            vec![
                2412, 2417, 2422, 2427, 2432, 2437, 2442, 2447, 2452, 2457, 2462, 2467, 2472, 2484,
            ]
        }
        Band::FiveGhz => {
            vec![
                5180, 5200, 5220, 5240, 5260, 5280, 5300, 5320, 5500, 5540, 5580, 5620, 5660, 5700,
                5745, 5785, 5825,
            ]
        }
        Band::SixGhz => {
            vec![
                5975, 6055, 6135, 6215, 6295, 6375, 6455, 6535, 6615, 6695, 6775, 6855, 6935, 7015,
                7095,
            ]
        }
    }
}

pub(super) fn major_signal_gridlines() -> impl Iterator<Item = f64> {
    (-90..=-20).step_by(10).map(f64::from)
}

pub(super) fn is_dfs_frequency(freq_mhz: i32) -> bool {
    (5250..=5350).contains(&freq_mhz) || (5470..=5725).contains(&freq_mhz)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_bss_data() -> BssChartData {
        BssChartData {
            ssid: Some("Test".to_string()),
            freq: 2412.0,
            signal: -50.0,
            width: 10.0,
            color: (255, 0, 0),
            selected: false,
        }
    }

    #[test]
    fn frequency_to_channel_handles_2_4_ghz_channels() {
        assert_eq!(frequency_to_channel(2412, Band::TwoPointFourGhz), "1");
        assert_eq!(frequency_to_channel(2484, Band::TwoPointFourGhz), "14");
    }

    #[test]
    fn frequency_to_channel_handles_5_and_6_ghz_channels() {
        assert_eq!(frequency_to_channel(5180, Band::FiveGhz), "36");
        assert_eq!(frequency_to_channel(5975, Band::SixGhz), "5");
    }

    #[test]
    fn band_frequency_ranges_match_chart_bounds() {
        assert_eq!(
            get_band_frequency_range(Band::TwoPointFourGhz),
            (2392, 2495)
        );
        assert_eq!(get_band_frequency_range(Band::FiveGhz), (5150, 5895));
        assert_eq!(get_band_frequency_range(Band::SixGhz), (5935, 7115));
    }

    #[test]
    fn major_signal_gridlines_are_ten_dbm_apart() {
        assert_eq!(
            major_signal_gridlines().collect::<Vec<_>>(),
            vec![-90.0, -80.0, -70.0, -60.0, -50.0, -40.0, -30.0, -20.0]
        );
    }

    #[test]
    fn two_point_four_ghz_shape_uses_trapezoid_mask() {
        let bss = test_bss_data();

        assert_eq!(
            bss_shape_points(&bss, Band::TwoPointFourGhz),
            vec![
                (2401.0, CHART_FLOOR),
                (2403.0, -50.0),
                (2421.0, -50.0),
                (2423.0, CHART_FLOOR),
            ]
        );
    }

    #[test]
    fn five_and_six_ghz_shape_uses_vertical_edges() {
        let bss = test_bss_data();

        let expected = vec![
            (2402.0, NOISE_FLOOR),
            (2402.0, -50.0),
            (2422.0, -50.0),
            (2422.0, NOISE_FLOOR),
        ];

        assert_eq!(bss_shape_points(&bss, Band::FiveGhz), expected);
    }
}
