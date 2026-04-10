use gtk::glib;
use gtk::glib::object::CastNone;
use gtk::subclass::prelude::*;

use crate::objects::BssObject;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/bss_capability_info.ui")]
    pub struct BssCapabilityInfo {
        #[template_child]
        pub ess_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub ibss_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub privacy_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub short_preamble_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub critical_update_flag_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub nontx_bssids_critical_update_flag_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub spectrum_management_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub qos_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub short_slot_time_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub apsd_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub radio_measurement_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub epd_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssCapabilityInfo {
        const NAME: &'static str = "BssCapabilityInfo";
        type Type = super::BssCapabilityInfo;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BssCapabilityInfo {}
    impl WidgetImpl for BssCapabilityInfo {}
    impl BoxImpl for BssCapabilityInfo {}
}

glib::wrapper! {
    pub struct BssCapabilityInfo(ObjectSubclass<imp::BssCapabilityInfo>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BssCapabilityInfo {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_selection_model(&self, selection_model: &gtk::SingleSelection) {
        selection_model.connect_selected_notify(glib::clone!(
            #[weak(rename_to = capability_info)]
            self,
            move |selection| {
                if let Some(bss) = selection.selected_item().and_downcast_ref::<BssObject>() {
                    capability_info.set_bss(bss);
                } else {
                    capability_info.clear();
                }
            }
        ));

        // Initial state (nothing selected)
        self.clear();
    }

    pub fn set_bss(&self, bss: &BssObject) {
        let imp = self.imp();
        let capability_info = bss.capability_info();

        imp.ess_label.set_label(&capability_info.ess.to_string());
        imp.ibss_label.set_label(&capability_info.ibss.to_string());
        imp.privacy_label
            .set_label(&capability_info.privacy.to_string());
        imp.short_preamble_label
            .set_label(&capability_info.short_preamble.to_string());
        imp.critical_update_flag_label
            .set_label(&capability_info.critical_update_flag.to_string());
        imp.nontx_bssids_critical_update_flag_label.set_label(
            &capability_info
                .nontransmitted_bssids_critical_update_flag
                .to_string(),
        );
        imp.spectrum_management_label
            .set_label(&capability_info.spectrum_management.to_string());
        imp.qos_label.set_label(&capability_info.qos.to_string());
        imp.short_slot_time_label
            .set_label(&capability_info.short_slot_time.to_string());
        imp.apsd_label.set_label(&capability_info.apsd.to_string());
        imp.radio_measurement_label
            .set_label(&capability_info.radio_measurement.to_string());
        imp.epd_label.set_label(&capability_info.epd.to_string());
    }

    pub fn clear(&self) {
        let imp = self.imp();

        imp.ess_label.set_label("");
        imp.ibss_label.set_label("");
        imp.privacy_label.set_label("");
        imp.short_preamble_label.set_label("");
        imp.critical_update_flag_label.set_label("");
        imp.nontx_bssids_critical_update_flag_label.set_label("");
        imp.spectrum_management_label.set_label("");
        imp.qos_label.set_label("");
        imp.short_slot_time_label.set_label("");
        imp.apsd_label.set_label("");
        imp.radio_measurement_label.set_label("");
        imp.epd_label.set_label("");
    }
}

impl Default for BssCapabilityInfo {
    fn default() -> Self {
        Self::new()
    }
}
