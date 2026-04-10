use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/kawaiifi/ui/bss_filter.ui")]
    pub struct BssFilter;

    #[glib::object_subclass]
    impl ObjectSubclass for BssFilter {
        const NAME: &'static str = "BssFilter";
        type Type = super::BssFilter;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BssFilter {}
    impl WidgetImpl for BssFilter {}
    impl BoxImpl for BssFilter {}
}

glib::wrapper! {
    pub struct BssFilter(ObjectSubclass<imp::BssFilter>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BssFilter {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for BssFilter {
    fn default() -> Self {
        Self::new()
    }
}
