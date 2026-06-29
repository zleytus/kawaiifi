mod data;
mod id;
mod name;
mod value;

pub use data::*;
pub use id::*;
pub use name::*;
pub use value::*;

use gtk::glib;
use gtk::prelude::*;

use crate::objects::IeTreeItem;

/// Extracts the bound `Label` and its `IeTreeItem` from a tree column's list item.
///
/// Returns `None` for items that aren't tree rows (e.g. during teardown). Only valid for
/// columns whose child is a plain `Label`; the name column wraps its label in a
/// `TreeExpander` and handles extraction itself.
fn label_and_tree_item(list_item: &glib::Object) -> Option<(gtk::Label, IeTreeItem)> {
    let list_item = list_item.downcast_ref::<gtk::ListItem>()?;
    let tree_item = list_item
        .item()
        .and_downcast::<gtk::TreeListRow>()?
        .item()
        .and_downcast::<IeTreeItem>()?;
    let label = list_item.child().and_downcast::<gtk::Label>()?;
    Some((label, tree_item))
}
