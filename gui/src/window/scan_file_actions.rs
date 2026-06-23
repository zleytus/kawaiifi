use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::prelude::{FileExt, ListModelExtManual};
use gtk::glib::object::Cast;
use gtk::prelude::WidgetExt;
use gtk::{gio, glib};

use crate::objects::{BssInternal, BssObject};
use crate::scan::{ScanFile, ScanFileError};

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

        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                let Ok(file) = dialog.open_future(Some(&window)).await else {
                    return;
                };

                match ScanFile::open(&file).await {
                    Ok(scan_file) => {
                        window.stop_scanning();
                        window.apply_loaded_scan(scan_file, &file);
                    }
                    Err(err) => window.show_scan_file_open_error(err),
                }
            }
        ));
    }

    fn apply_loaded_scan(&self, scan_file: ScanFile, file: &gio::File) {
        self.invalidate_scan_generation();
        let label = file
            .basename()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| file.uri().to_string());
        self.imp().file_label.set_label(&label);
        self.imp().file_label.set_visible(true);
        self.imp().interface_box.set_visible(false);

        self.apply_merged_results(scan_file.bss_list());
    }

    fn show_scan_file_open_error(&self, error: ScanFileError) {
        match error {
            ScanFileError::File(e) => {
                self.show_error(
                    "Could Not Open Scan File",
                    format!("The scan file could not be opened.\n\n{e}"),
                );
            }
            ScanFileError::Json(e) => {
                self.show_error(
                    "Could Not Open Scan",
                    format!("The scan file could not be parsed.\n\n{e}"),
                );
            }
            ScanFileError::UnsupportedPlatform(platform) => {
                self.show_error(
                    "Unsupported Scan File",
                    format!(
                        "This scan file was saved on {platform}, but this version of KawaiiFi can only open Linux scan files."
                    ),
                );
            }
            ScanFileError::UnsupportedVersion(version) => {
                self.show_error(
                    "Unsupported Scan File",
                    format!(
                        "This scan file uses version {version}, but this version of KawaiiFi only supports version 1."
                    ),
                );
            }
        }
    }

    pub fn save_all(&self) {
        self.save_bss_list(
            "All",
            "All-BSSs",
            self.bss_list_store()
                .iter::<BssObject>()
                .filter_map(|obj| obj.ok())
                .map(|bss_obj| bss_obj.data().clone()),
        );
    }

    pub fn save_visible(&self) {
        self.save_bss_list(
            "Visible",
            "Visible-BSSs",
            self.bss_filter_model()
                .snapshot()
                .into_iter()
                .filter_map(|obj| obj.downcast::<BssObject>().ok())
                .map(|obj| obj.data().clone()),
        );
    }

    pub fn save_selected(&self) {
        let Some(bss) = self.imp().bss_table.selected_bss() else {
            return;
        };
        self.save("Selected", "Selected-BSSs", vec![bss.data().clone()]);
    }

    fn save_bss_list(
        &self,
        title: &str,
        initial_name: &str,
        bss_list: impl Iterator<Item = BssInternal>,
    ) {
        let bss_list: Vec<BssInternal> = bss_list.collect();
        if !bss_list.is_empty() {
            self.save(title, initial_name, bss_list);
        }
    }

    fn save(&self, title: &str, initial_name: &str, bss_list: Vec<BssInternal>) {
        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&Self::kwifi_file_filter());

        let dialog = gtk::FileDialog::builder()
            .title(title)
            .initial_name(format!("{}.kwifi", initial_name))
            .filters(&filters)
            .build();

        glib::spawn_future_local(glib::clone!(
            #[weak(rename_to = window)]
            self,
            async move {
                let Ok(file) = dialog.save_future(Some(&window)).await else {
                    return;
                };
                let scan_file = ScanFile::new(bss_list);
                if let Err(err) = scan_file.save(&file).await {
                    window.show_scan_file_save_error(err);
                }
            }
        ));
    }

    fn show_scan_file_save_error(&self, error: ScanFileError) {
        match error {
            ScanFileError::File(err) => {
                self.show_error(
                    "Could Not Save File",
                    format!("The scan file could not be written.\n\n{err}"),
                );
            }
            ScanFileError::Json(err) => {
                self.show_error(
                    "Could Not Save Scan",
                    format!("The scan could not be serialized.\n\n{err}"),
                );
            }
            ScanFileError::UnsupportedPlatform(_) | ScanFileError::UnsupportedVersion(_) => {
                self.show_error("Could Not Save Scan", error.to_string());
            }
        }
    }
}
