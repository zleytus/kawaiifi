use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::prelude::{FileExt, ListModelExtManual};
use gtk::glib::object::Cast;
use gtk::{gio, glib};

use crate::objects::BssObject;
use crate::scan_file::{ScanFile, ScanFileError};

use super::KawaiiFiWindow;

impl KawaiiFiWindow {
    fn kwifi_file_filter() -> gtk::FileFilter {
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("KawaiiFi Scan (.kwifi)"));
        filter.add_suffix("kwifi");
        filter
    }

    pub fn open(&self) {
        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&Self::kwifi_file_filter());

        let dialog = gtk::FileDialog::builder()
            .title("Open")
            .filters(&filters)
            .build();

        dialog.open(
            Some(self),
            None::<&gio::Cancellable>,
            glib::clone!(
                #[weak(rename_to = window)]
                self,
                move |result| {
                    let Ok(file) = result else {
                        return;
                    };
                    let Some(path) = file.path() else {
                        window.show_error(
                            "Could Not Open File",
                            "The selected file is not available as a local path.",
                        );
                        return;
                    };
                    match std::fs::read_to_string(&path) {
                        Ok(json) => match ScanFile::from_json(&json) {
                            Ok(scan_file) => {
                                window.stop_scanning();
                                window.apply_loaded_scan(scan_file, &path);
                            }
                            Err(e) => {
                                tracing::error!(error = %e, "Failed to parse scan file");
                                match e {
                                    ScanFileError::UnsupportedVersion(version) => {
                                        window.show_error(
                                            "Unsupported Scan File",
                                            format!(
                                                "This scan file uses version {version}, but this version of KawaiiFi only supports version 1."
                                            ),
                                        );
                                    }
                                    ScanFileError::UnsupportedPlatform(platform) => {
                                        window.show_error(
                                            "Unsupported Scan File",
                                            format!(
                                                "This scan file was saved on {platform}, but this version of KawaiiFi can only open Linux scan files."
                                            ),
                                        );
                                    }
                                    ScanFileError::Json(e) => {
                                        window.show_error(
                                            "Could Not Open Scan",
                                            format!("The scan file could not be parsed.\n\n{e}"),
                                        );
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            tracing::error!(error = %e, "Failed to read scan file");
                            window.show_error(
                                "Could Not Read File",
                                format!("The scan file could not be read.\n\n{e}"),
                            );
                        }
                    }
                }
            ),
        );
    }

    pub fn save_all(&self) {
        self.save_bss_objects(
            "All",
            "All-BSSs",
            self.bss_list_store()
                .iter::<BssObject>()
                .filter_map(|obj| obj.ok()),
        );
    }

    pub fn save_visible(&self) {
        self.save_bss_objects(
            "Visible",
            "Visible-BSSs",
            self.bss_filter_model()
                .snapshot()
                .into_iter()
                .filter_map(|obj| obj.downcast::<BssObject>().ok()),
        );
    }

    pub fn save_selected(&self) {
        let Some(bss) = self.imp().bss_table.selected_bss() else {
            return;
        };
        self.save(
            "Selected",
            "Selected-BSSs",
            vec![kawaiifi::Bss::clone(&bss.bss())],
        );
    }

    fn save_bss_objects(
        &self,
        title: &str,
        initial_name: &str,
        bss_objects: impl Iterator<Item = BssObject>,
    ) {
        let bss_list: Vec<kawaiifi::Bss> = bss_objects
            .map(|obj| kawaiifi::Bss::clone(&obj.bss()))
            .collect();
        if !bss_list.is_empty() {
            self.save(title, initial_name, bss_list);
        }
    }

    fn save(&self, title: &str, initial_name: &str, bss_list: Vec<kawaiifi::Bss>) {
        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&Self::kwifi_file_filter());

        let dialog = gtk::FileDialog::builder()
            .title(title)
            .initial_name(format!("{}.kwifi", initial_name))
            .filters(&filters)
            .build();

        let window = self.clone();
        dialog.save(Some(self), None::<&gio::Cancellable>, move |result| {
            let Ok(file) = result else {
                return;
            };
            let Some(path) = file.path() else {
                window.show_error(
                    "Could Not Save File",
                    "The selected file is not available as a local path.",
                );
                return;
            };
            let scan_file = ScanFile::new(bss_list);
            match scan_file.to_json() {
                Ok(json) => {
                    if let Err(e) = std::fs::write(&path, json) {
                        tracing::error!(error = %e, "Failed to write scan file");
                        window.show_error(
                            "Could Not Save File",
                            format!("The scan file could not be written.\n\n{e}"),
                        );
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to serialize scan file");
                    window.show_error(
                        "Could Not Save Scan",
                        format!("The scan file could not be serialized.\n\n{e}"),
                    );
                }
            }
        });
    }
}
