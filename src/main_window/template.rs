use super::MainWindow;
use crate::{
    article_list::{template::ArticleListTemplate, ArticleList},
    feed_list::{template::FeedListTemplate, FeedList},
};

use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::{ObjectSubclass, ObjectSubclassExt},
        InitializingObject,
    },
    StaticTypeExt,
};
use gtk4::{
    prelude::{GObjectPropertyExpressionExt, InitializingWidgetExt},
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, Widget,
};
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow, Leaflet};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/main-window.ui")]
pub struct MainWindowTemplate {
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,

    #[template_child]
    pub feed_list: TemplateChild<FeedList>,

    #[template_child]
    pub article_list: TemplateChild<ArticleList>,
}

#[object_subclass]
impl ObjectSubclass for MainWindowTemplate {
    const NAME: &'static str = "MainWindow";

    type Type = MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        FeedList::ensure_type();
        ArticleList::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowTemplate {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let feed_list = self.feed_list.get();
        let feed_list_template = FeedListTemplate::from_instance(&feed_list);

        let article_list = self.article_list.get();
        let article_list_template = ArticleListTemplate::from_instance(&article_list);

        self.leaflet.property_expression("folded").bind(
            &feed_list_template.header_bar.get(),
            "show-end-title-buttons",
            Widget::NONE,
        );

        self.leaflet.property_expression("folded").bind(
            &article_list_template.header_bar.get(),
            "show-start-title-buttons",
            Widget::NONE,
        );

        self.leaflet.property_expression("folded").bind(
            &article_list_template.back_button.get(),
            "visible",
            Widget::NONE,
        );
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
