use std::cell::RefCell;
use std::rc::Rc;

use gtk::subclass::prelude::*;
use gtk::{FilterListModel, SingleSelection, gio, glib, prelude::*};
use kawaiifi::Band;

use crate::objects::BssObject;

mod data;
mod render;

use data::{BssChartData, channel_width_half_mhz, get_channel_frequencies};

mod imp {
    use super::*;
    use std::cell::{Cell, OnceCell};

    #[derive(gtk::CompositeTemplate, Default)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/bss_chart.ui")]
    pub struct BssChart {
        pub(super) band: Cell<Band>,

        // Observe the shared table selection model through a band-filtered chart view.
        pub(crate) selection_model: OnceCell<SingleSelection>,
        pub(crate) band_filter_model: OnceCell<FilterListModel>,
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
                        "2.4" | "2.4 GHz" => Band::TwoPointFourGhz,
                        "5" | "5 GHz" => Band::FiveGhz,
                        "6" | "6 GHz" => Band::SixGhz,
                        _ => {
                            tracing::warn!(
                                band = band_str,
                                "Unknown BssChart band; falling back to 2.4 GHz"
                            );
                            Band::TwoPointFourGhz
                        }
                    };
                    self.band.set(band);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "band" => match self.band.get() {
                    Band::TwoPointFourGhz => "2.4 GHz".to_value(),
                    Band::FiveGhz => "5 GHz".to_value(),
                    Band::SixGhz => "6 GHz".to_value(),
                },
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

        imp.selection_model.set(selection_model.clone()).unwrap();

        let band_filter = gtk::CustomFilter::new(move |obj| {
            let bss = obj.downcast_ref::<BssObject>().unwrap();
            bss.data().band() == band
        });

        let band_filter_model =
            gtk::FilterListModel::new(Some(selection_model.clone()), Some(band_filter));

        imp.band_filter_model
            .set(band_filter_model.clone())
            .unwrap();

        band_filter_model.connect_items_changed(glib::clone!(
            #[weak(rename_to = chart)]
            self,
            move |_, _, _, _| {
                chart.update_chart_data();
            }
        ));

        selection_model.connect_selection_changed(glib::clone!(
            #[weak(rename_to = chart)]
            self,
            move |_, _, _| {
                chart.update_chart_data();
            }
        ));

        self.update_chart_data();
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
            .map(|bss| *bss.data().bssid());

        let mut data = imp.chart_data.borrow_mut();
        data.clear();

        for bss in model
            .iter::<glib::Object>()
            .filter_map(Result::ok)
            .filter_map(|obj| obj.downcast::<BssObject>().ok())
        {
            let color = bss.data().color();

            data.push(BssChartData {
                selected: selected_bssid.is_some_and(|id| &id == bss.data().bssid()),
                ssid: bss.data().formatted_ssid(),
                freq: bss.data().center_frequency_mhz() as f64,
                signal: bss.data().signal_dbm() as f64,
                width: channel_width_half_mhz(bss.data().channel_width()),
                color: (
                    (color.red() * 255.0) as u8,
                    (color.green() * 255.0) as u8,
                    (color.blue() * 255.0) as u8,
                ),
            });
        }

        self.queue_draw();
    }

    fn setup_plot(&self) {
        let imp = self.imp();
        let band = imp.band.get();
        let chart_data = imp.chart_data.clone();
        let channel_freqs = get_channel_frequencies(band);
        let style_manager = adw::StyleManager::default();
        let style_manager_for_draw = style_manager.clone();

        self.set_draw_func(move |_, cr, width, height| {
            if width <= 0 || height <= 0 {
                return;
            }

            if let Err(err) = render::draw_plot(
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
}

impl Default for BssChart {
    fn default() -> Self {
        Self::new()
    }
}
