use gui::{Style, StyleId, StyleValue, StyleMap, StyleTable};

use std::any::Any;

pub trait StyleMapExt {
    fn or_else<M: StyleMap>(&self, other: &M) -> OrElse<Self, M>;
    fn inherit_from<M: StyleMap>(&self, other: &M) -> InheritFrom<Self, M>;
}

impl<T: StyleMap> StyleMapExt for T {
    fn or_else<'a, M: StyleMap>(&'a self, other: &'a M) -> OrElse<Self, M> {
        OrElse {
            a: self,
            b: other,
        }
    }
}

pub struct OrElse<'a, A: StyleMap, B: StyleMap> {
    a: &'a A,
    b: &'a B,
}

impl<'a, A: StyleMap, B: StyleMap> StyleMap for OrElse<'a, A, B> {
    fn get_style_static<S: Style>(&self) -> Option<StyleValue<&S>> {
        self.a.get_style_static().or_else(|| self.b.get_style_static())
    }

    fn get_style_dyn(&self, style: StyleId) -> Option<StyleValue<&Any>> {
        self.a.get_style_dyn(style).or_else(|| self.b.get_style_dyn(style))
    }

    fn as_table(&self) -> StyleTable {
        StyleTable::or_else(&self.a.as_table(), &self.b.as_table())
    }
}

pub struct InheritFrom<'a, A: StyleMap, B: StyleMap> {
    a: &'a A,
    b: &'a B,
}

impl<'a, A: StyleMap, B: StyleMap> StyleMap for OrElse<'a, A, B> {
    fn get_style_static<S: Style>(&self) -> Option<StyleValue<&S>> {
        match self.a.get_style_static() {
            None if S::inheritable() => self.b.get_style_static(),
            s @ _ => s
        }
    }

    fn get_style_dyn(&self, style: StyleId) -> Option<StyleValue<&Any>> {
        self.a.get_style_dyn(style).or_else(|| self.b.get_style_dyn(style))
    }

    fn as_table(&self) -> StyleTable {
        StyleTable::or_else(&self.a.as_table(), &self.b.as_table())
    }
}