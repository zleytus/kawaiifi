use std::cell::RefCell;
use std::rc::Rc;

use egui_plot::{Line, Text};
use gtk::subclass::prelude::*;
use gtk::{FilterListModel, SingleSelection, gio, glib, prelude::*};
use gtk_egui_area::egui::{Color32, Visuals};
use gtk_egui_area::{EguiArea, egui};
use kawaiifi::{Band, ChannelWidth};

use crate::objects::BssObject;

/// Data for a single BSS to be displayed on the chart
#[derive(Clone)]
pub struct BssChartData {
    pub ssid: Option<String>,
    pub freq: f64,
    pub signal: f64,
    pub width: f64, // half-width in MHz
    pub color: egui::Color32,
    pub selected: bool,
}

mod imp {
    use super::*;
    use std::cell::{Cell, OnceCell};

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/com/github/kawaiifi/ui/bss_chart.ui")]
    pub struct BssChart {
        pub(super) band: Cell<Band>,

        // The selection model from the window (all filtered BSSs)
        pub(crate) selection_model: OnceCell<SingleSelection>,

        // Chart-specific filter (only the chart's band)
        pub(crate) band_filter_model: OnceCell<FilterListModel>,

        // Shared data for the egui chart
        pub(super) chart_data: Rc<RefCell<Vec<BssChartData>>>,

        // Store the EguiArea so we can trigger redraws
        pub(super) egui_area: RefCell<Option<EguiArea>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssChart {
        const NAME: &'static str = "BssChart";
        type Type = super::BssChart;
        type ParentType = gtk::Box;

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
            obj.setup_egui_plot();
        }

        fn properties() -> &'static [glib::ParamSpec] {
            use std::sync::OnceLock;
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![
                    glib::ParamSpecString::builder("band")
                        .construct_only()
                        .build(),
                ]
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
                        _ => kawaiifi::Band::TwoPointFourGhz,
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
    impl BoxImpl for BssChart {}
}

glib::wrapper! {
    pub struct BssChart(ObjectSubclass<imp::BssChart>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}

impl BssChart {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_selection_model(&self, selection_model: &SingleSelection) {
        let imp = self.imp();
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
                chart.update_egui_chart_data();
                chart.queue_draw();
            }
        ));

        // Redraw when the selection changes so the bold line updates
        selection_model.connect_selection_changed(glib::clone!(
            #[weak(rename_to = chart)]
            self,
            move |_, _, _| {
                chart.update_egui_chart_data();
                chart.queue_draw();
            }
        ));

        // Trigger initial draw
        self.update_egui_chart_data();
        self.queue_draw();
    }

    fn update_egui_chart_data(&self) {
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
                color: egui::Color32::from_rgba_unmultiplied(
                    (color.red() * 255.0) as u8,
                    (color.green() * 255.0) as u8,
                    (color.blue() * 255.0) as u8,
                    200,
                ),
            });
        }

        // Trigger egui redraw
        if let Some(egui_area) = imp.egui_area.borrow().as_ref() {
            egui_area.queue_draw();
        }
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

    fn bss_shape_points(bss: &BssChartData, band: kawaiifi::Band) -> Vec<[f64; 2]> {
        const NOISE_FLOOR: f64 = -110.0;
        const STEP: f64 = 1.0;

        let corners = match band {
            kawaiifi::Band::TwoPointFourGhz => {
                // The 802.11a/g OFDM spectral mask is trapezoidal: flat at 0 dBr out to
                // ±9 MHz from center, then rolling off to -20 dBr at ±11 MHz. We map the
                // rolloff edge to the noise floor. As a ratio of the half-width (10 MHz
                // for a 20 MHz channel): flat top at 0.9×, base at 1.1×.
                let flat = bss.width * 0.9;
                let base = bss.width * 1.1;
                vec![
                    [bss.freq - base, NOISE_FLOOR],
                    [bss.freq - flat, bss.signal],
                    [bss.freq + flat, bss.signal],
                    [bss.freq + base, NOISE_FLOOR],
                ]
            }
            kawaiifi::Band::FiveGhz | kawaiifi::Band::SixGhz => {
                vec![
                    [bss.freq - bss.width, NOISE_FLOOR],
                    [bss.freq - bss.width, bss.signal],
                    [bss.freq + bss.width, bss.signal],
                    [bss.freq + bss.width, NOISE_FLOOR],
                ]
            }
        };

        Self::densify_points(&corners, STEP)
    }

    /// Inserts interpolated points along each segment at roughly `step`-unit
    /// intervals, applied independently to both axes so that vertical segments
    /// are densified by dBm and horizontal segments by MHz.
    fn densify_points(points: &[[f64; 2]], step: f64) -> Vec<[f64; 2]> {
        let mut result = Vec::new();
        for pair in points.windows(2) {
            let [x0, y0] = pair[0];
            let [x1, y1] = pair[1];
            let steps = ((x1 - x0).abs() / step)
                .ceil()
                .max((y1 - y0).abs() / step)
                .ceil() as usize;
            let steps = steps.max(1);
            for i in 0..steps {
                let t = i as f64 / steps as f64;
                result.push([x0 + t * (x1 - x0), y0 + t * (y1 - y0)]);
            }
        }
        result.push(*points.last().unwrap());
        result
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

    fn setup_egui_plot(&self) {
        let imp = self.imp();
        let band = imp.band.get();
        let (freq_start, freq_end) = Self::get_band_frequency_range(band);

        let chart_data = imp.chart_data.clone();

        // Get channel frequencies for axis labels and grid
        let channel_freqs: Vec<i32> = Self::get_channel_frequencies(band);

        // Prepare font definitions outside the closure
        let font_definitions = self.load_gtk_font().map(|font_data| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "gtk_font".to_owned(),
                egui::FontData::from_owned(font_data).into(),
            );
            // Insert at front so it's preferred
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "gtk_font".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, "gtk_font".to_owned());
            fonts
        });

        let style_manager = adw::StyleManager::default();

        let egui_area = EguiArea::new(move |ui| {
            if style_manager.is_dark() {
                ui.set_visuals(Visuals::dark());
            } else {
                ui.set_visuals(Visuals::light());
            }
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.inner_margin(egui::Margin {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 5,
                }))
                .show_inside(ui, |ui| {
                    egui_plot::Plot::new("bss_chart")
                        .allow_zoom(false)
                        .allow_drag(false)
                        .allow_scroll(false)
                        .allow_boxed_zoom(false)
                        .allow_double_click_reset(false)
                        .auto_bounds(egui::Vec2b::FALSE)
                        .show_axes([true, true])
                        .show_grid([false, true])
                        .cursor_color(Color32::TRANSPARENT)
                        .include_x(freq_start as f64)
                        .include_x(freq_end as f64)
                        .include_y(-100.0)
                        .include_y(-15.0)
                        .default_y_bounds(-100.0, -15.0)
                        .label_formatter(|name, value| {
                            if !name.is_empty() {
                                format!("{}\nx = {:.1} MHz\ny = {:.1} dBm", name, value.x, value.y)
                            } else {
                                format!("x = {:.1} MHz\ny = {:.1} dBm", value.x, value.y)
                            }
                        })
                        .custom_x_axes(vec![
                            egui_plot::AxisHints::new_x()
                                .formatter(move |x, _range| {
                                    Self::frequency_to_channel(x.value as i32, band)
                                })
                                .placement(egui_plot::VPlacement::Bottom),
                        ])
                        .x_grid_spacer({
                            let channel_freqs = channel_freqs.clone();
                            // Use larger step_size for wider bands so marks aren't filtered out
                            let step = (freq_end - freq_start) as f64;
                            move |input| {
                                channel_freqs
                                    .iter()
                                    .filter_map(|&f| {
                                        let f = f as f64;
                                        if f >= input.bounds.0 && f <= input.bounds.1 {
                                            Some(egui_plot::GridMark {
                                                value: f,
                                                step_size: step,
                                            })
                                        } else {
                                            None
                                        }
                                    })
                                    .collect()
                            }
                        })
                        .show(ui, |plot_ui| {
                            let data = chart_data.borrow();
                            for bss in data.iter() {
                                let points = Self::bss_shape_points(bss, band);
                                let name = bss.ssid.clone().unwrap_or_else(|| "Hidden".to_string());
                                plot_ui.line(
                                    Line::new(name.clone(), points)
                                        .color(bss.color)
                                        .width(2.0)
                                        .fill(-130.0)
                                        .fill_alpha(if bss.selected { 0.25 } else { 0.0 }),
                                );
                                plot_ui.text(
                                    Text::new(
                                        name.clone(),
                                        egui_plot::PlotPoint::new(bss.freq, bss.signal + 0.75),
                                        egui::RichText::new(name).size(16.0),
                                    )
                                    .anchor(egui::Align2::CENTER_BOTTOM)
                                    .color(bss.color),
                                );
                            }
                        });
                });
        });

        if let Some(fonts) = font_definitions {
            egui_area.egui_ctx().set_fonts(fonts);
        }

        egui_area.set_hexpand(true);
        egui_area.set_vexpand(true);

        self.append(&egui_area);
        imp.egui_area.replace(Some(egui_area));
    }

    fn load_gtk_font(&self) -> Option<Vec<u8>> {
        use fontconfig::Fontconfig;

        // Get font family from GTK's Pango context
        let pango_context = self.pango_context();
        let font_desc = pango_context.font_description()?;
        let family = font_desc.family()?.to_string();

        // Use fontconfig to find the font file
        let fc = Fontconfig::new()?;
        let font = fc.find(&family, None)?;

        // Load the font file
        std::fs::read(&font.path).ok()
    }
}

impl Default for BssChart {
    fn default() -> Self {
        Self::new()
    }
}
