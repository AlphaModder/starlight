use std::any::{Any, TypeId};
use typemap::Key;

mod table;
mod util;

pub use self::util::*;
pub use self::table::*;

pub trait Style: Clone + 'static {
    fn inheritable() -> bool;
}

pub struct StyleId {
    pub(crate) key: TypeId,
    pub(crate) inheritable: bool,
}

pub(crate) enum StyleKey<S: Style> { }

impl<S: Style> Key for StyleKey<S> {
    type Value = S;
}

pub trait StyleExt: Style {
    fn id() -> StyleId;
}

impl<T: Style> StyleExt for T {
    fn id() -> StyleId {
        StyleId {
            key: TypeId::of::<StyleKey<T>>(),
            inheritable: T::inheritable(),
        }
    }
}

pub trait StyleMap {
    fn get_style_static<S: Style>(&self) -> Option<StyleValue<&S>> where Self: Sized;
    fn get_style_dyn(&self, style: StyleId) -> Option<StyleValue<&Any>>;
    fn as_table(&self) -> StyleTable;
}

pub trait StyleMapExt {
    fn get_style<S: Style>(&self) -> Option<StyleValue<&S>>;
}

impl<T: StyleMap> StyleMapExt for T {
    fn get_style<S: Style>(&self) -> Option<StyleValue<&S>> {
        self.get_style_static()
    }
}

impl StyleMapExt for StyleMap {
    fn get_style<S: Style>(&self) -> Option<StyleValue<&S>> {
        self.get_style_dyn(S::id()).map(|inner| inner.map(|any| any.downcast_ref().unwrap()))
    }
}

pub enum StyleValue<S> {
    Style(S),
    Inherit,
    Initial,
}

impl<S> StyleValue<S> {
    pub fn map<T, F: FnOnce(S) -> T>(self, f: F) -> StyleValue<T> {
        match self {
            StyleValue::Style(s) => StyleValue::Style(f(s)),
            StyleValue::Inherit => StyleValue::Inherit,
            StyleValue::Initial => StyleValue::Initial,
        }
    }

    pub fn as_ref(&self) -> StyleValue<&S> {
        match self {
            StyleValue::Style(s) => StyleValue::Style(&s),
            StyleValue::Inherit => StyleValue::Inherit,
            StyleValue::Initial => StyleValue::Initial,
        }
    }
}