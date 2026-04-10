use std::{cell::RefCell, rc::Rc};

use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    gio::{self, prelude::ActionMapExt},
    glib::{self, object::Cast},
    prelude::{
        BoxExt, ButtonExt, CheckButtonExt, EditableExt, PopoverExt, ToggleButtonExt, WidgetExt,
    },
};
use kawaiifi::{Band, ChannelWidth, SecurityProtocol, WifiProtocol};

use crate::objects::BssObject;

use super::KawaiiFiWindow;

impl KawaiiFiWindow {
    pub fn setup_scan_controls(&self) {
        let imp = self.imp();

        // Connect start scanning button
        imp.start_scanning_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                let interval_seconds = 10;
                window.start_scanning(interval_seconds);
            }
        ));

        // Connect stop scanning button
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
                // window.imp().search_bar.set_search_mode(toggle.is_active());
                window.imp().search_bar.set_reveal_child(toggle.is_active());
            }
        ));

        imp.search_entry.connect_search_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |entry| {
                *window.imp().text_query.borrow_mut() = entry.text().to_lowercase();
                window.reapply_filter();
            }
        ));
    }

    pub fn setup_bottom_panel_toggles(&self) {
        let imp = self.imp();

        // Track which button is active
        let active_button: Rc<RefCell<Option<gtk::ToggleButton>>> = Rc::new(RefCell::new(None));

        // Set up each toggle
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

                // Get previous active button
                let prev_button = active_button.borrow().clone();

                if is_active {
                    // This button is being activated

                    // Deactivate previous button if different
                    if let Some(prev) = prev_button
                        && prev != button
                    {
                        prev.set_active(false);
                    }

                    // Switch view and show container
                    imp.bottom_stack.set_visible_child_name(&view_name);
                    imp.bottom_stack.set_visible(true);

                    // Update tracker
                    *active_button.borrow_mut() = Some(button.clone());
                } else {
                    // This button is being deactivated

                    // Only hide if this was the active button
                    if prev_button.as_ref().map_or(false, |b| *b == button) {
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
            // imp.bss_capability_info.set_selection_model(selection_model);
            imp.bss_elements.set_selection_model(selection_model);
            imp.bss_chart_2_4.set_selection_model(selection_model);
            imp.bss_chart_5.set_selection_model(selection_model);
            imp.bss_chart_6.set_selection_model(selection_model);
        }
    }

    pub fn setup_filter_buttons(&self) {
        let imp = self.imp();

        Self::setup_multi_select_button(
            &imp.band_filter_button,
            "Band",
            &["2.4 GHz", "5 GHz", "6 GHz"],
            Some(" GHz"),
            &imp.band_filter_state,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || window.reapply_filter()
            ),
        );

        Self::setup_multi_select_button(
            &imp.security_filter_button,
            "Security",
            &["Open", "WEP", "WPA", "WPA2", "WPA3"],
            None,
            &imp.security_filter_state,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || window.reapply_filter()
            ),
        );

        Self::setup_multi_select_button(
            &imp.width_filter_button,
            "Width",
            &[
                "20 MHz",
                "40 MHz",
                "80 MHz",
                "80+80 MHz",
                "160 MHz",
                "320 MHz",
            ],
            Some(" MHz"),
            &imp.width_filter_state,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || window.reapply_filter()
            ),
        );

        Self::setup_multi_select_button(
            &imp.protocols_filter_button,
            "Protocols",
            &["b", "a", "g", "n", "ac", "ax", "be"],
            None,
            &imp.protocols_filter_state,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move || window.reapply_filter()
            ),
        );

        imp.reset_filter_button.connect_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_| {
                let imp = window.imp();
                // Reset all filter states to all-true by rebuilding the popovers
                for state in [
                    &imp.band_filter_state,
                    &imp.security_filter_state,
                    &imp.width_filter_state,
                    &imp.protocols_filter_state,
                ] {
                    let mut s = state.borrow_mut();
                    s.iter_mut().for_each(|b| *b = true);
                }
                // Uncheck-then-recheck each checkbox to fire the active-notify signal
                for button in [
                    imp.band_filter_button.get(),
                    imp.security_filter_button.get(),
                    imp.width_filter_button.get(),
                    imp.protocols_filter_button.get(),
                ] {
                    if let Some(popover) = button.popover() {
                        if let Some(child) = popover.child() {
                            if let Some(box_widget) = child.downcast_ref::<gtk::Box>() {
                                let mut widget = box_widget.first_child();
                                while let Some(w) = widget {
                                    if let Some(check) = w.downcast_ref::<gtk::CheckButton>() {
                                        check.set_active(true);
                                    }
                                    widget = w.next_sibling();
                                }
                            }
                        }
                    }
                }
                imp.search_entry.set_text("");
                *imp.text_query.borrow_mut() = String::new();
                window.reapply_filter();
            }
        ));
    }

    fn setup_multi_select_button(
        button: &gtk::MenuButton,
        label_prefix: &str,
        item_labels: &[&str],
        unit: Option<&str>,
        state: &Rc<RefCell<Vec<bool>>>,
        on_change: impl Fn() + 'static,
    ) {
        let item_labels_owned: Vec<String> = item_labels.iter().map(|s| s.to_string()).collect();
        let label_prefix_owned = label_prefix.to_string();
        let unit_owned: Option<String> = unit.map(|s| s.to_string());
        let on_change = Rc::new(on_change);

        *state.borrow_mut() = vec![true; item_labels.len()];

        // Set initial button label from items (overrides any blueprint placeholder)
        let unit = unit_owned.as_deref().unwrap_or("");
        let all_values: Vec<String> = item_labels
            .iter()
            .map(|l| l.strip_suffix(unit).unwrap_or(l).to_string())
            .collect();
        button.set_label(&format!(
            "{}: {}{}",
            label_prefix,
            all_values.join("/"),
            unit
        ));

        let popover_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
        popover_box.set_margin_top(6);
        popover_box.set_margin_bottom(6);
        popover_box.set_margin_start(6);
        popover_box.set_margin_end(6);

        let checks: Vec<gtk::CheckButton> = item_labels
            .iter()
            .map(|label| {
                let check = gtk::CheckButton::with_label(label);
                check.set_active(true);
                popover_box.append(&check);
                check
            })
            .collect();

        let popover = gtk::Popover::new();
        popover.set_child(Some(&popover_box));
        button.set_popover(Some(&popover));

        for (i, check) in checks.iter().enumerate() {
            check.connect_active_notify(glib::clone!(
                #[weak]
                button,
                #[strong]
                state,
                #[strong]
                on_change,
                #[strong]
                item_labels_owned,
                #[strong]
                label_prefix_owned,
                #[strong]
                unit_owned,
                move |check| {
                    state.borrow_mut()[i] = check.is_active();
                    let unit = unit_owned.as_deref().unwrap_or("");
                    let value = |label: &str| label.strip_suffix(unit).unwrap_or(label).to_string();
                    let all_values: Vec<String> =
                        item_labels_owned.iter().map(|l| value(l)).collect();
                    let selected_values: Vec<String> = item_labels_owned
                        .iter()
                        .enumerate()
                        .filter_map(|(j, label)| {
                            if state.borrow()[j] {
                                Some(value(label))
                            } else {
                                None
                            }
                        })
                        .collect();
                    let new_label = if selected_values.len() == item_labels_owned.len() {
                        format!("{}: {}{}", label_prefix_owned, all_values.join("/"), unit)
                    } else if selected_values.is_empty() {
                        format!("{}: None", label_prefix_owned)
                    } else {
                        format!(
                            "{}: {}{}",
                            label_prefix_owned,
                            selected_values.join("/"),
                            unit
                        )
                    };
                    button.set_label(&new_label);
                    on_change();
                }
            ));
        }
    }

    pub fn reapply_filter(&self) {
        let imp = self.imp();
        let filter = self.bss_filter();

        let query = imp.text_query.borrow().clone();
        let band_state = imp.band_filter_state.borrow().clone();
        let security_state = imp.security_filter_state.borrow().clone();
        let width_state = imp.width_filter_state.borrow().clone();
        let protocol_state = imp.protocols_filter_state.borrow().clone();

        let band_all = band_state.iter().all(|&b| b);
        let security_all = security_state.iter().all(|&b| b);
        let width_all = width_state.iter().all(|&b| b);
        let protocol_all = protocol_state.iter().all(|&b| b);

        filter.set_filter_func(move |obj| {
            let bss = obj.downcast_ref::<BssObject>().unwrap();

            // Text query
            if !query.is_empty() {
                let ssid_match = match bss.ssid() {
                    Some(ssid) => ssid.to_lowercase().contains(&query),
                    None => "hidden".contains(&query),
                };
                if !ssid_match && !bss.bssid().to_lowercase().contains(&query) {
                    return false;
                }
            }

            // Band filter: [2.4 GHz, 5 GHz, 6 GHz]
            if !band_all {
                let allowed = [Band::TwoPointFourGhz, Band::FiveGhz, Band::SixGhz];
                let passes = allowed
                    .iter()
                    .enumerate()
                    .any(|(i, b)| band_state.get(i).copied().unwrap_or(true) && *b == bss.band());
                if !passes {
                    return false;
                }
            }

            // Security filter: [Open, WEP, WPA, WPA2, WPA3]
            if !security_all {
                let sec = bss.security();
                let passes = (security_state.get(0).copied().unwrap_or(true) && sec.is_empty())
                    || (security_state.get(1).copied().unwrap_or(true)
                        && sec.contains(SecurityProtocol::WEP))
                    || (security_state.get(2).copied().unwrap_or(true)
                        && sec.contains(SecurityProtocol::WPA))
                    || (security_state.get(3).copied().unwrap_or(true)
                        && sec.contains(SecurityProtocol::WPA2))
                    || (security_state.get(4).copied().unwrap_or(true)
                        && sec.contains(SecurityProtocol::WPA3));
                if !passes {
                    return false;
                }
            }

            // Width filter: [20, 40, 80, 80+80, 160, 320 MHz]
            if !width_all {
                let widths = [
                    ChannelWidth::TwentyMhz,
                    ChannelWidth::FortyMhz,
                    ChannelWidth::EightyMhz,
                    ChannelWidth::EightyPlusEightyMhz,
                    ChannelWidth::OneSixtyMhz,
                    ChannelWidth::ThreeHundredTwentyMhz,
                ];
                let passes = widths.iter().enumerate().any(|(i, w)| {
                    width_state.get(i).copied().unwrap_or(true) && *w == bss.channel_width()
                });
                if !passes {
                    return false;
                }
            }

            // Protocol filter: [b, a, g, n, ac, ax, be]
            if !protocol_all {
                let protocols = [
                    WifiProtocol::B,
                    WifiProtocol::A,
                    WifiProtocol::G,
                    WifiProtocol::N,
                    WifiProtocol::AC,
                    WifiProtocol::AX,
                    WifiProtocol::BE,
                ];
                let bss_protos = bss.protocols();
                let passes = protocols.iter().enumerate().any(|(i, p)| {
                    protocol_state.get(i).copied().unwrap_or(true) && bss_protos.contains(*p)
                });
                if !passes {
                    return false;
                }
            }

            true
        });
    }

    pub fn connect_components_to_signals(&self) {
        let imp = self.imp();
        imp.interface_box.set_signal_handlers(&self);
    }
}
