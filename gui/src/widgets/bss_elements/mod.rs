mod columns;

use std::cell::OnceCell;

use gtk::gio::ListStore;
use gtk::subclass::prelude::*;
use gtk::{SingleSelection, SortListModel, gio, glib, prelude::*};

use crate::objects::{BssObject, IeObject, IeTreeItem};
use crate::widgets::column_view;
use columns::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/fi/kawaii/kawaiifi/ui/bss_elements.ui")]
    pub struct BssElements {
        pub list_store: OnceCell<ListStore>,
        pub sort_model: OnceCell<SortListModel>,

        #[template_child]
        pub column_view: TemplateChild<gtk::ColumnView>,
        #[template_child]
        pub id_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub ie_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub value_column: TemplateChild<gtk::ColumnViewColumn>,
        #[template_child]
        pub data_column: TemplateChild<gtk::ColumnViewColumn>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BssElements {
        const NAME: &'static str = "BssElements";
        type Type = super::BssElements;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BssElements {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_model();
            obj.setup_columns();
            obj.setup_column_sorters();
        }
    }

    impl WidgetImpl for BssElements {
        fn realize(&self) {
            self.parent_realize();

            let column_view_title = self.column_view.first_child();
            column_view::adjust_header_alignment(column_view_title, &["ID", "Data", "Value"]);
        }
    }

    impl BoxImpl for BssElements {}
}

glib::wrapper! {
    pub struct BssElements(ObjectSubclass<imp::BssElements>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl BssElements {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_selection_model(&self, selection_model: &SingleSelection) {
        selection_model.connect_selected_notify(glib::clone!(
            #[weak(rename_to = elements)]
            self,
            move |selection| {
                if let Some(bss) = selection.selected_item().and_downcast_ref::<BssObject>() {
                    elements.set_bss(bss);
                } else {
                    elements.clear();
                }
            }
        ));

        self.clear();
    }

    pub fn list_store(&self) -> ListStore {
        self.imp()
            .list_store
            .get()
            .expect("List store should be set during construction")
            .clone()
    }

    fn setup_model(&self) {
        let imp = self.imp();

        // Model chain: ListStore -> SortListModel -> TreeListModel -> NoSelection
        // Sort model must wrap the root ListStore so sorting only affects top-level items
        // and the tree structure remains intact
        let model = gio::ListStore::new::<IeTreeItem>();
        imp.list_store.set(model.clone()).unwrap();

        let sort_model = SortListModel::new(Some(model), imp.column_view.sorter());
        imp.sort_model.set(sort_model.clone()).unwrap();

        let tree_model = gtk::TreeListModel::new(sort_model, false, false, |item| {
            let tree_item = item.downcast_ref::<IeTreeItem>().unwrap();
            let children = tree_item.children();

            if children.is_empty() {
                None
            } else {
                let child_store = ListStore::new::<IeTreeItem>();
                for child in children {
                    child_store.append(&child);
                }
                Some(child_store.upcast())
            }
        });
        tree_model.set_autoexpand(false);

        let selection_model = gtk::NoSelection::new(Some(tree_model));
        imp.column_view.set_model(Some(&selection_model));
    }

    fn setup_columns(&self) {
        let imp = self.imp();

        imp.id_column.set_factory(Some(&create_id_factory()));
        imp.ie_column.set_factory(Some(&create_name_factory()));
        imp.value_column.set_factory(Some(&create_value_factory()));
        imp.data_column.set_factory(Some(&create_data_factory()));
    }

    fn setup_column_sorters(&self) {
        let imp = self.imp();

        imp.id_column.set_sorter(Some(&create_id_sorter()));
        imp.ie_column.set_sorter(Some(&create_name_sorter()));

        imp.column_view
            .sort_by_column(Some(&imp.id_column), gtk::SortType::Ascending);
    }

    pub fn set_bss(&self, bss: &BssObject) {
        let list_store = self.list_store();
        list_store.remove_all();
        for ie in bss.data().ies() {
            let ie_obj = IeObject::new(ie.clone());
            let tree_item = IeTreeItem::from_ie(ie_obj);
            list_store.append(&tree_item);
        }
    }

    pub fn clear(&self) {
        let model = self.list_store();
        model.remove_all();
    }
}

impl Default for BssElements {
    fn default() -> Self {
        Self::new()
    }
}
