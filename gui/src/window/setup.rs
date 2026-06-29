use std::{cell::RefCell, rc::Rc};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::{
        self,
        prelude::{ActionMapExt, SettingsExt},
    },
    glib::{self},
    prelude::{ButtonExt, ToggleButtonExt, WidgetExt},
};

use crate::widgets::{InterfaceList, InterfaceRefreshResult};

use super::KawaiiFiWindow;

impl KawaiiFiWindow {
    pub fn setup(&self) {
        self.connect_components_to_models();
        self.setup_interface_controls();
        self.setup_actions();
        self.setup_filtering();
        self.setup_scan_controls();
        self.setup_bottom_panel_toggles();
        self.setup_settings();

        let result = self.imp().interface_list.refresh_interfaces();
        self.handle_interface_refresh_result(result);

        if let Some(interface) = self.imp().interface_list.selected_interface() {
            self.load_interface(interface, true);
        }
    }

    fn connect_components_to_models(&self) {
        let imp = self.imp();

        imp.bss_table.setup(self.bss_filter_model());
        if let Some(selection_model) = imp.bss_table.selection_model() {
            imp.bss_ie_table.set_selection_model(selection_model);
            imp.bss_chart_2_4.set_selection_model(selection_model);
            imp.bss_chart_5.set_selection_model(selection_model);
            imp.bss_chart_6.set_selection_model(selection_model);
        }
    }

    fn setup_interface_controls(&self) {
        let imp = self.imp();

        imp.interface_list.connect_interface_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |interface_list: &InterfaceList, _| {
                let interface = interface_list.selected_interface();
                window
                    .imp()
                    .interface_toggle
                    .set_interface(interface.as_ref());

                let restart_scanning = window.imp().scanning_enabled.get();
                window.stop_scanning();
                window.invalidate_scan_generation();
                window.apply_merged_results(Vec::new());
                if let Some(interface) = interface {
                    window.load_interface(interface, restart_scanning);
                }
            }
        ));
        imp.refresh_interfaces_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                let result = window.imp().interface_list.refresh_interfaces();
                window.handle_interface_refresh_result(result);
            }
        ));

        imp.interface_toggle
            .set_interface(imp.interface_list.selected_interface().as_ref());
    }

    fn setup_actions(&self) {
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

    fn setup_filtering(&self) {
        let imp = self.imp();

        imp.bss_filter.connect_filter_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.update_filter();
            }
        ));
    }

    fn setup_scan_controls(&self) {
        let imp = self.imp();

        imp.start_scanning_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                if let Some(interface) = window.imp().interface_list.selected_interface() {
                    window.load_interface(interface, true);
                }
            }
        ));

        imp.stop_scanning_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                window.stop_scanning();
            }
        ));

        imp.status_banner.connect_button_clicked(|banner| {
            banner.set_revealed(false);
        });
    }

    fn handle_interface_refresh_result(&self, result: InterfaceRefreshResult) {
        let interface = self.imp().interface_list.selected_interface();
        self.imp()
            .interface_toggle
            .set_interface(interface.as_ref());

        match result {
            InterfaceRefreshResult::SelectionUnchanged => {}
            InterfaceRefreshResult::SelectionChanged => {
                let restart_scanning = self.imp().scanning_enabled.get();
                self.stop_scanning();
                self.invalidate_scan_generation();
                self.apply_merged_results(Vec::new());
                if let Some(interface) = interface {
                    self.load_interface(interface, restart_scanning);
                }
            }
            InterfaceRefreshResult::NoInterfaces => {
                self.stop_scanning();
                self.invalidate_scan_generation();
                self.apply_merged_results(Vec::new());
                self.imp().interface_toggle.set_interface(None);
                self.imp()
                    .status_banner
                    .set_title("No Wi-Fi interfaces found.");
                self.imp().status_banner.set_revealed(true);
            }
            InterfaceRefreshResult::Error(error) => {
                self.stop_scanning();
                self.invalidate_scan_generation();
                self.apply_merged_results(Vec::new());
                self.show_error("Could Not Load Wi-Fi Interfaces", error);
            }
        }
    }

    fn setup_bottom_panel_toggles(&self) {
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
                window.handle_bottom_panel_toggle(&button, &view_name, &active_button);
            }
        ));
    }

    fn handle_bottom_panel_toggle(
        &self,
        button: &gtk::ToggleButton,
        view_name: &str,
        active_button: &Rc<RefCell<Option<gtk::ToggleButton>>>,
    ) {
        let imp = self.imp();
        let is_active = button.is_active();

        let prev_button = active_button.borrow().clone();

        if is_active {
            if let Some(prev) = prev_button
                && prev != *button
            {
                prev.set_active(false);
            }

            imp.bottom_stack.set_visible_child_name(view_name);
            imp.bottom_stack.set_visible(true);

            *active_button.borrow_mut() = Some(button.clone());
        } else {
            if prev_button.as_ref().is_some_and(|b| *b == *button) {
                imp.bottom_stack.set_visible(false);
                *active_button.borrow_mut() = None;
            }
        }
    }

    fn setup_settings(&self) {
        self.settings().connect_changed(
            Some("show-hidden-bsss"),
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move |_, _| {
                    window.update_filter();
                }
            ),
        );
        self.update_filter();
    }
}
