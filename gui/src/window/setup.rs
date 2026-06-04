use std::{cell::RefCell, rc::Rc};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::{
        self,
        prelude::{ActionMapExt, SettingsExt},
    },
    glib::{self, object::Cast},
    prelude::{ButtonExt, ToggleButtonExt, WidgetExt},
};

use crate::objects::BssObject;

use super::{KawaiiFiWindow, filtering::BssFilterState};

impl KawaiiFiWindow {
    pub fn setup_settings(&self) {
        self.settings().connect_changed(
            Some("show-hidden-bsss"),
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move |_, _| {
                    window.reapply_filter();
                }
            ),
        );
        self.reapply_filter();
    }

    pub fn setup_scan_controls(&self) {
        let imp = self.imp();

        imp.start_scanning_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.start_scanning();
            }
        ));

        imp.stop_scanning_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.stop_scanning();
            }
        ));
    }

    pub fn setup_actions(&self) {
        let action_save_all = gio::SimpleAction::new("save-all", None);
        action_save_all.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.save_all();
            }
        ));
        self.add_action(&action_save_all);

        let action_save_visible = gio::SimpleAction::new("save-visible", None);
        action_save_visible.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.save_visible();
            }
        ));
        self.add_action(&action_save_visible);

        let action_save_selected = gio::SimpleAction::new("save-selected", None);
        action_save_selected.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.save_selected();
            }
        ));
        self.add_action(&action_save_selected);

        let action_open = gio::SimpleAction::new("open", None);
        action_open.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_, _| {
                window.open();
            }
        ));
        self.add_action(&action_open);
    }

    pub fn setup_search(&self) {
        let imp = self.imp();

        imp.search_toggle.connect_toggled(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |toggle| {
                window
                    .imp()
                    .overlay_split_view
                    .set_show_sidebar(toggle.is_active());
            }
        ));

        imp.bss_filter.connect_filter_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.reapply_filter();
            }
        ));
    }

    pub fn setup_bottom_panel_toggles(&self) {
        let imp = self.imp();

        let active_button: Rc<RefCell<Option<gtk::ToggleButton>>> = Rc::new(RefCell::new(None));

        self.setup_toggle_button(&imp.ies_toggle_button, "ies", &active_button);
        self.setup_toggle_button(
            &imp.two_point_four_ghz_toggle_button,
            "two_point_four_ghz",
            &active_button,
        );
        self.setup_toggle_button(&imp.five_ghz_toggle_button, "five_ghz", &active_button);
        self.setup_toggle_button(&imp.six_ghz_toggle_button, "six_ghz", &active_button);
    }

    fn setup_toggle_button(
        &self,
        button: &gtk::ToggleButton,
        view_name: &str,
        active_button: &Rc<RefCell<Option<gtk::ToggleButton>>>,
    ) {
        let view_name = view_name.to_string();

        button.connect_toggled(glib::clone!(
            #[weak(rename_to = window)]
            self,
            #[weak]
            button,
            #[strong]
            active_button,
            move |_| {
                let imp = window.imp();
                let is_active = button.is_active();

                let prev_button = active_button.borrow().clone();

                if is_active {
                    if let Some(prev) = prev_button
                        && prev != button
                    {
                        prev.set_active(false);
                    }

                    imp.bottom_stack.set_visible_child_name(&view_name);
                    imp.bottom_stack.set_visible(true);

                    *active_button.borrow_mut() = Some(button.clone());
                } else {
                    if prev_button.as_ref().is_some_and(|b| *b == button) {
                        imp.bottom_stack.set_visible(false);
                        *active_button.borrow_mut() = None;
                    }
                }
            }
        ));
    }

    pub fn connect_components_to_models(&self) {
        let imp = self.imp();

        imp.bss_table.setup(self.bss_filter_model());
        if let Some(selection_model) = imp.bss_table.selection_model() {
            imp.bss_elements.set_selection_model(selection_model);
            imp.bss_chart_2_4.set_selection_model(selection_model);
            imp.bss_chart_5.set_selection_model(selection_model);
            imp.bss_chart_6.set_selection_model(selection_model);
        }
    }

    pub fn reapply_filter(&self) {
        let filter = self.bss_filter();
        let state = BssFilterState::from_window(self);

        filter.set_filter_func(move |obj| {
            let bss = obj.downcast_ref::<BssObject>().unwrap();
            state.matches(bss)
        });
    }
}
