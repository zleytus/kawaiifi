use std::cell::RefCell;
use std::rc::Rc;

use gtk::subclass::prelude::*;
use gtk::{gio, glib, prelude::*, FilterListModel, SingleSelection};
use kawaiifi::{Band, ChannelWidth};
use plotters::element::DashedPathElement;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters_cairo::CairoBackend;

use crate::objects::BssObject;

/// Data for a single BSS to be displayed on the chart
#[derive(Clone)]
pub struct BssChartData {
    pub ssid: Option<String>,
    pub freq: f64,
    pub signal: f64,
    pub width: f64, // half-width in MHz
    pub color: (u8, u8, u8),
    pub selected: bool,
}

mod imp {
    use super::*;
    use std::cell::{Cell, OnceCell};

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/bss_chart.ui")]
    pub struct BssChart {
        pub(super) band: Cell<Band>,

        // The selection model from the window (all filtered BSSs)
        pub(crate) selection_model: OnceCell<SingleSelection>,

        // Chart-specific filter (only the chart's band)
        pub(crate) band_filter_model: OnceCell<FilterListModel>,

        // Shared data for the chart
        pub(super) chart_data: Rc<RefCell<Vec<BssChartData>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssChart {
        const NAME: &'static str = "BssChart";
        type Type = super::BssChart;
        type ParentType = gtk::DrawingArea;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BssChart {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_plot();
        }

        fn properties() -> &'static [glib::ParamSpec] {
            use std::sync::OnceLock;
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![glib::ParamSpecString::builder("band")
                    .construct_only()
                    .build()]
            })
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                "band" => {
                    let band_str = value.get::<String>().expect("band must be a string");
                    let band = match band_str.as_str() {
                        "2.4" | "2.4 GHz" => kawaiifi::Band::TwoPointFourGhz,
                        "5" | "5 GHz" => kawaiifi::Band::FiveGhz,
                        "6" | "6 GHz" => kawaiifi::Band::SixGhz,
                        _ => {
                            tracing::warn!(
                                band = band_str,
                                "Unknown BssChart band; falling back to 2.4 GHz"
                            );
                            kawaiifi::Band::TwoPointFourGhz
                        }
                    };
                    self.band.set(band);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "band" => {
                    let band = self.band.get();
                    match band {
                        kawaiifi::Band::TwoPointFourGhz => "2.4 GHz".to_value(),
                        kawaiifi::Band::FiveGhz => "5 GHz".to_value(),
                        kawaiifi::Band::SixGhz => "6 GHz".to_value(),
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for BssChart {}
    impl DrawingAreaImpl for BssChart {}
}

glib::wrapper! {
    pub struct BssChart(ObjectSubclass<imp::BssChart>)
        @extends gtk::Widget, gtk::DrawingArea,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget;
}

impl BssChart {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_selection_model(&self, selection_model: &SingleSelection) {
        let imp = self.imp();
        if imp.selection_model.get().is_some() {
            return;
        }

        let band = imp.band.get();

        // Store the selection model
        imp.selection_model.set(selection_model.clone()).unwrap();

        // Create a filter for this chart's band
        let band_filter = gtk::CustomFilter::new(move |obj| {
            let bss = obj.downcast_ref::<BssObject>().unwrap();
            bss.band() == band
        });

        // Wrap the selection model in a band-specific filter
        let band_filter_model =
            gtk::FilterListModel::new(Some(selection_model.clone()), Some(band_filter));

        imp.band_filter_model
            .set(band_filter_model.clone())
            .unwrap();

        // Update the chart when band-filtered data changes
        band_filter_model.connect_items_changed(glib::clone!(
            #[weak(rename_to = chart)]
            self,
            move |_, _, _, _| {
                chart.update_chart_data();
                chart.queue_draw();
            }
        ));

        // Redraw when the selection changes so the bold line updates
        selection_model.connect_selection_changed(glib::clone!(
            #[weak(rename_to = chart)]
            self,
            move |_, _, _| {
                chart.update_chart_data();
                chart.queue_draw();
            }
        ));

        // Trigger initial draw
        self.update_chart_data();
        self.queue_draw();
    }

    fn update_chart_data(&self) {
        let imp = self.imp();

        let Some(model) = imp.band_filter_model.get() else {
            return;
        };

        let selected_bssid = imp
            .selection_model
            .get()
            .and_then(|sm| sm.selected_item())
            .and_then(|obj| obj.downcast::<BssObject>().ok())
            .map(|bss| bss.bssid_bytes());

        let mut data = imp.chart_data.borrow_mut();
        data.clear();

        for bss in model
            .iter::<glib::Object>()
            .filter_map(Result::ok)
            .filter_map(|obj| obj.downcast::<BssObject>().ok())
        {
            let color = bss.color();
            let half_width = match bss.channel_width() {
                ChannelWidth::FortyMhz => 20.0,
                ChannelWidth::EightyMhz => 40.0,
                ChannelWidth::OneSixtyMhz => 80.0,
                ChannelWidth::ThreeHundredTwentyMhz => 160.0,
                _ => 10.0,
            };

            data.push(BssChartData {
                selected: selected_bssid.is_some_and(|id| id == bss.bssid_bytes()),
                ssid: bss.ssid(),
                freq: bss.center_frequency_mhz() as f64,
                signal: bss.signal_strength() as f64,
                width: half_width,
                color: (
                    (color.red() * 255.0) as u8,
                    (color.green() * 255.0) as u8,
                    (color.blue() * 255.0) as u8,
                ),
            });
        }

        self.queue_draw();
    }

    fn frequency_to_channel(freq: i32, band: kawaiifi::Band) -> String {
        match band {
            kawaiifi::Band::TwoPointFourGhz => {
                if freq == 2484 {
                    "14".to_string()
                } else {
                    (((freq - 2412) / 5) + 1).to_string()
                }
            }
            kawaiifi::Band::FiveGhz => ((freq - 5000) / 5).to_string(),
            kawaiifi::Band::SixGhz => ((freq - 5950) / 5).to_string(),
        }
    }

    fn get_band_frequency_range(band: kawaiifi::Band) -> (i32, i32) {
        match band {
            kawaiifi::Band::TwoPointFourGhz => (2392, 2495),
            kawaiifi::Band::FiveGhz => (5150, 5895),
            kawaiifi::Band::SixGhz => (5935, 7115),
        }
    }

    fn bss_shape_points(bss: &BssChartData, band: kawaiifi::Band) -> Vec<(f64, f64)> {
        const CHART_FLOOR: f64 = -100.0;
        const NOISE_FLOOR: f64 = -110.0;

        match band {
            kawaiifi::Band::TwoPointFourGhz => {
                // The 802.11a/g OFDM spectral mask is trapezoidal: flat at 0 dBr out to
                // ±9 MHz from center, then rolling off to -20 dBr at ±11 MHz. We map the
                // rolloff edge to the noise floor. As a ratio of the half-width (10 MHz
                // for a 20 MHz channel): flat top at 0.9×, base at 1.1×.
                let flat = bss.width * 0.9;
                let base = bss.width * 1.1;
                vec![
                    (bss.freq - base, CHART_FLOOR),
                    (bss.freq - flat, bss.signal),
                    (bss.freq + flat, bss.signal),
                    (bss.freq + base, CHART_FLOOR),
                ]
            }
            kawaiifi::Band::FiveGhz | kawaiifi::Band::SixGhz => vec![
                (bss.freq - bss.width, NOISE_FLOOR),
                (bss.freq - bss.width, bss.signal),
                (bss.freq + bss.width, bss.signal),
                (bss.freq + bss.width, NOISE_FLOOR),
            ],
        }
    }

    fn get_channel_frequencies(band: kawaiifi::Band) -> Vec<i32> {
        match band {
            kawaiifi::Band::TwoPointFourGhz => {
                vec![
                    2412, 2417, 2422, 2427, 2432, 2437, 2442, 2447, 2452, 2457, 2462, 2467, 2472,
                    2484,
                ]
            }
            kawaiifi::Band::FiveGhz => {
                vec![
                    5180, 5200, 5220, 5240, 5260, 5280, 5300, 5320, 5500, 5540, 5580, 5620, 5660,
                    5700, 5745, 5785, 5825,
                ]
            }
            kawaiifi::Band::SixGhz => {
                vec![
                    5975, 6055, 6135, 6215, 6295, 6375, 6455, 6535, 6615, 6695, 6775, 6855, 6935,
                    7015, 7095,
                ]
            }
        }
    }

    fn major_signal_gridlines() -> impl Iterator<Item = f64> {
        (-90..=-20).step_by(10).map(f64::from)
    }

    fn setup_plot(&self) {
        let imp = self.imp();
        let band = imp.band.get();
        let chart_data = imp.chart_data.clone();
        let channel_freqs = Self::get_channel_frequencies(band);
        let style_manager = adw::StyleManager::default();
        let style_manager_for_draw = style_manager.clone();

        self.set_draw_func(move |_, cr, width, height| {
            if width <= 0 || height <= 0 {
                return;
            }

            if let Err(err) = Self::draw_plot(
                cr,
                width as u32,
                height as u32,
                band,
                &channel_freqs,
                &chart_data.borrow(),
                style_manager_for_draw.is_dark(),
            ) {
                tracing::warn!(error = %err, "Failed to draw BSS chart");
            }
        });

        self.set_hexpand(true);
        self.set_vexpand(true);

        style_manager.connect_dark_notify(glib::clone!(
            #[weak(rename_to = chart)]
            self,
            move |_| chart.queue_draw()
        ));
    }

    fn draw_plot(
        cr: &gtk::cairo::Context,
        width: u32,
        height: u32,
        band: kawaiifi::Band,
        channel_freqs: &[i32],
        chart_data: &[BssChartData],
        is_dark: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (freq_start, freq_end) = Self::get_band_frequency_range(band);
        // plotters-cairo expects cairo-rs 0.21; GTK provides cairo-rs 0.22.
        // Both wrap the same cairo_t, so borrow the raw pointer for the draw call.
        let cairo_context = unsafe {
            cairo_021::Context::from_raw_none(cr.to_raw_none() as *mut cairo_021::ffi::cairo_t)
        };
        let backend = CairoBackend::new(&cairo_context, (width, height))?;
        let root = backend.into_drawing_area();

        let (background, text, grid) = if is_dark {
            (BLACK, WHITE, RGBColor(82, 82, 82))
        } else {
            (WHITE, RGBColor(46, 52, 54), RGBColor(140, 140, 140))
        };

        root.fill(&background)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(8)
            .x_label_area_size(28)
            .y_label_area_size(42)
            .build_cartesian_2d(freq_start as f64..freq_end as f64, -100.0f64..-15.0f64)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .axis_style(text)
            .label_style(("sans-serif", 12).into_font().color(&text))
            .light_line_style(TRANSPARENT)
            .bold_line_style(TRANSPARENT)
            .x_labels(0)
            .y_label_formatter(&|signal| format!("{signal:.0}"))
            .draw()?;

        chart.draw_series(Self::major_signal_gridlines().map(|signal| {
            DashedPathElement::new(
                vec![(freq_start as f64, signal), (freq_end as f64, signal)],
                2,
                6,
                grid.mix(0.85),
            )
        }))?;

        let x_label_style = ("sans-serif", 12)
            .into_font()
            .color(&text)
            .pos(Pos::new(HPos::Center, VPos::Top));
        chart.draw_series(channel_freqs.iter().map(|freq| {
            EmptyElement::at((*freq as f64, -100.0))
                + Text::new(
                    Self::frequency_to_channel(*freq, band),
                    (0, 8),
                    x_label_style.clone(),
                )
        }))?;

        for bss in chart_data {
            let points = Self::bss_shape_points(bss, band);
            let (red, green, blue) = bss.color;
            let color = RGBColor(red, green, blue);

            if bss.selected {
                chart.draw_series(AreaSeries::new(points.clone(), -130.0, color.mix(0.25)))?;
            }

            chart.draw_series(LineSeries::new(
                points,
                color
                    .mix(0.9)
                    .stroke_width(if bss.selected { 3 } else { 2 }),
            ))?;

            let name = bss.ssid.clone().unwrap_or_else(|| "Hidden".to_string());
            let label_style = ("sans-serif", 16)
                .into_font()
                .color(&color)
                .pos(Pos::new(HPos::Center, VPos::Bottom));
            chart.draw_series(std::iter::once(
                EmptyElement::at((bss.freq, bss.signal)) + Text::new(name, (0, -6), label_style),
            ))?;
        }

        root.present()?;
        Ok(())
    }
}

impl Default for BssChart {
    fn default() -> Self {
        Self::new()
    }
}
