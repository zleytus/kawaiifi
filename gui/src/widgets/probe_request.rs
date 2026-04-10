use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/probe_request.ui")]
    pub struct ProbeRequest;

    #[glib::object_subclass]
    impl ObjectSubclass for ProbeRequest {
        const NAME: &'static str = "ProbeRequest";
        type Type = super::ProbeRequest;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ProbeRequest {}
    impl WidgetImpl for ProbeRequest {}
    impl BoxImpl for ProbeRequest {}
}

glib::wrapper! {
    pub struct ProbeRequest(ObjectSubclass<imp::ProbeRequest>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ProbeRequest {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for ProbeRequest {
    fn default() -> Self {
        Self::new()
    }
}
