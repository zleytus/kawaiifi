use gtk::{
    glib::object::Cast,
    prelude::{BoxExt, WidgetExt},
};

// Adapted from https://gitlab.com/mission-center-devs/mission-center/-/blob/main/src/table_view/columns/mod.rs
pub fn adjust_header_alignment(row_widget: Option<gtk::Widget>, headers_to_adjust: &[&str]) {
    let mut title = row_widget.and_then(|w| w.first_child());
    while let Some(view_title) = title.take() {
        title = view_title.next_sibling();

        let Some(container) = view_title.first_child() else {
            continue;
        };

        let Some(label) = container
            .first_child()
            .and_then(|l| l.downcast::<gtk::Label>().ok())
        else {
            continue;
        };

        if !headers_to_adjust
            .iter()
            .any(|header| label.label() == *header)
        {
            continue;
        }

        container.set_hexpand(true);
        label.set_halign(gtk::Align::End);
        label.set_justify(gtk::Justification::Right);

        let Some(arrow) = label.next_sibling() else {
            continue;
        };

        if let Some(container) = container.downcast_ref::<gtk::Box>() {
            container.reorder_child_after(&label, Some(&arrow));

            // Setting the arrow to visible is a hack to make column headers without a sort arrow
            // become end-aligned
            arrow.set_visible(true);
            arrow.set_hexpand(true);
        }
    }
}
