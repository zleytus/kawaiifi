use gtk::prelude::*;
use gtk::SignalListItemFactory;

use crate::objects::BssObject;

pub fn create_color_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_width_request(3);
        drawing_area.set_height_request(3);
        drawing_area.set_halign(gtk::Align::Center);
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&drawing_area));
    });

    factory.connect_bind(move |_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let bss = list_item.item().and_downcast::<BssObject>().unwrap();
        let drawing_area = list_item
            .child()
            .and_downcast::<gtk::DrawingArea>()
            .unwrap();

        let color = bss.color();

        drawing_area.set_draw_func(move |_, cr, width, height| {
            cr.set_source_rgba(
                f64::from(color.red()),
                f64::from(color.green()),
                f64::from(color.blue()),
                f64::from(color.alpha()),
            );
            cr.rectangle(0.0, 0.0, width as f64, height as f64);
            let _ = cr.fill();
        });
    });

    factory
}
