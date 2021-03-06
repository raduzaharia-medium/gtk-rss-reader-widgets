pub mod template;

use self::template::FeedListTemplate;

use glib::wrapper;
use gtk4::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct FeedList(ObjectSubclass<FeedListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for FeedList {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedList {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create an instance of FeedList")
    }
}
