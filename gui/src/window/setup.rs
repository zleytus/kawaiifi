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
use kawaiifi::Band;

use crate::objects::BssObject;

use super::KawaiiFiWindow;

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
                window.start_scanning(10);
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
        let imp = self.imp();
        let filter = self.bss_filter();

        let show_hidden = self.settings().boolean("show-hidden-bsss");
        let band_state = imp.bss_filter.band_state();
        let (show_open, security_state) = imp.bss_filter.security_state();
        let width_state = imp.bss_filter.width_state();
        let protocol_state = imp.bss_filter.protocol_state();
        let amendment_state = imp.bss_filter.amendment_state();
        let ssid_query = imp.bss_filter.ssid_query();
        let bssid_query = imp.bss_filter.bssid_query();
        let vendor_query = imp.bss_filter.vendor_query();

        let band_all = band_state.iter().all(|&b| b);
        let security_all = show_open && security_state.is_all();
        let width_all = width_state.len() == 6;
        let protocol_all = protocol_state.is_all();
        let amendment_all = amendment_state.is_all();

        filter.set_filter_func(move |obj| {
            let bss = obj.downcast_ref::<BssObject>().unwrap();

            if !show_hidden && bss.ssid().is_none() {
                return false;
            }

            if !ssid_query.is_empty() {
                let ssid_match = match bss.ssid() {
                    Some(ssid) => ssid.to_lowercase().contains(&ssid_query),
                    None => "hidden".contains(&ssid_query),
                };
                if !ssid_match {
                    return false;
                }
            }

            if !bssid_query.is_empty() && !bss.bssid().to_lowercase().contains(&bssid_query) {
                return false;
            }

            if !vendor_query.is_empty() && !bss.vendor().to_lowercase().contains(&vendor_query) {
                return false;
            }

            // Band filter: [2.4 GHz, 5 GHz, 6 GHz]
            if !band_all {
                let allowed = [Band::TwoPointFourGhz, Band::FiveGhz, Band::SixGhz];
                let passes = allowed
                    .iter()
                    .enumerate()
                    .any(|(i, b)| band_state[i] && *b == bss.band());
                if !passes {
                    return false;
                }
            }

            // Security filter
            if !security_all {
                let sec = bss.security();
                let passes = (show_open && sec.is_empty())
                    || (!sec.is_empty() && !(*sec & *security_state).is_empty());
                if !passes {
                    return false;
                }
            }

            // Channel width filter
            if !width_all && !width_state.contains(&bss.channel_width()) {
                return false;
            }

            // Protocol filter
            if !protocol_all && (*bss.protocols() & *protocol_state).is_empty() {
                return false;
            }

            // Amendment filter
            if !amendment_all && (*bss.amendments() & *amendment_state).is_empty() {
                return false;
            }

            true
        });
    }
}
