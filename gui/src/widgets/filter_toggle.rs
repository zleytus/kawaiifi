use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use std::sync::OnceLock;

    use adw::ButtonContent;
    use gtk::ToggleButton;

    use super::*;

    pub(super) const PROP_ACTIVE: &str = "active";

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/filter_toggle.ui")]
    pub struct FilterToggle {
        // UI components
        #[template_child]
        pub filter_button: TemplateChild<ToggleButton>,
        #[template_child]
        pub filter_button_content: TemplateChild<ButtonContent>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FilterToggle {
        const NAME: &'static str = "FilterToggle";
        type Type = super::FilterToggle;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FilterToggle {
        fn constructed(&self) {
            self.parent_constructed();

            self.filter_button.connect_active_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self.obj(),
                move |_| {
                    obj.notify(PROP_ACTIVE);
                }
            ));
        }

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![
                    glib::ParamSpecBoolean::builder(PROP_ACTIVE)
                        .default_value(false)
                        .explicit_notify()
                        .build(),
                ]
            })
        }

        fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                PROP_ACTIVE => self.filter_button.is_active().to_value(),
                name => unimplemented!("Unknown property {name}"),
            }
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                PROP_ACTIVE => {
                    self.filter_button.set_active(value.get::<bool>().unwrap());
                }
                name => unimplemented!("Unknown property {name}"),
            }
        }
    }
    impl WidgetImpl for FilterToggle {}
    impl BoxImpl for FilterToggle {}
}

glib::wrapper! {
    pub struct FilterToggle(ObjectSubclass<imp::FilterToggle>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl FilterToggle {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_filtered_count(&self, count: u32) {
        let content = &self.imp().filter_button_content;
        content.set_label(&format!("{count} Filtered"));
    }
}

impl Default for FilterToggle {
    fn default() -> Self {
        Self::new()
    }
}
