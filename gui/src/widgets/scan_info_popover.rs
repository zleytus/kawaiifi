use gtk::glib;
use gtk::subclass::prelude::*;
use kawaiifi::{self, Scan};

mod imp {
    use gtk::Label;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/scan_info_popover.ui")]
    pub struct ScanInfoPopover {
        #[template_child]
        pub(crate) frequencies_scanned_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) ies_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) start_time_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) end_time_label: TemplateChild<Label>,
        #[template_child]
        pub(crate) duration_label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScanInfoPopover {
        const NAME: &'static str = "ScanInfoPopover";
        type Type = super::ScanInfoPopover;
        type ParentType = gtk::Popover;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ScanInfoPopover {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for ScanInfoPopover {}
    impl PopoverImpl for ScanInfoPopover {}
}

glib::wrapper! {
    pub struct ScanInfoPopover(ObjectSubclass<imp::ScanInfoPopover>)
        @extends gtk::Widget, gtk::Popover,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::ShortcutManager;
}

impl ScanInfoPopover {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_scan_info(&self, scan: &Scan) {
        let imp = self.imp();

        imp.frequencies_scanned_label
            .set_label(&format!("{} frequencies", scan.freqs_mhz().len()));

        imp.ies_label.set_label(
            &scan
                .ies()
                .iter()
                .map(|ie| ie.name().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );

        imp.start_time_label.set_label(
            &scan
                .start_time()
                .with_timezone(&chrono::Local)
                .format("%H:%M:%S")
                .to_string(),
        );
        imp.end_time_label.set_label(
            &scan
                .end_time()
                .with_timezone(&chrono::Local)
                .format("%H:%M:%S")
                .to_string(),
        );
        imp.duration_label
            .set_label(&format!("{:.01} seconds", &scan.duration().as_secs_f32()));
    }

    pub fn clear(&self) {
        let imp = self.imp();

        imp.frequencies_scanned_label.set_label("");
        imp.ies_label.set_label("");
        imp.start_time_label.set_label("");
        imp.end_time_label.set_label("");
        imp.duration_label.set_label("");
    }
}

impl Default for ScanInfoPopover {
    fn default() -> Self {
        Self::new()
    }
}
