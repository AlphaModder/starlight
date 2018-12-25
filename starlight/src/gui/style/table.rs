use std::any::{Any, TypeId};
use std::collections::HashMap;

use typemap::CloneMap;

use gui::{Style, StyleExt, StyleId, StyleValue, StyleKey, StyleMap};

#[derive(Clone)]
pub struct StyleTable {
    style_map: CloneMap,
    else_values: HashMap<TypeId, SpecialStyleValue>,
}

impl StyleTable {
    pub fn new() -> StyleTable {
        StyleTable { 
            style_map: CloneMap::custom(),
            else_values: HashMap::new()
        }
    }

    pub fn set_style<S: Style>(&mut self, value: StyleValue<S>) {
        match value {
            StyleValue::Style(value) => { self.style_map.insert::<StyleKey<S>>(value); },
            StyleValue::Inherit => {
                self.style_map.remove::<StyleKey<S>>();
                self.else_values.insert(S::id().key, SpecialStyleValue::Inherit);
            }
            StyleValue::Initial => {
                self.style_map.remove::<StyleKey<S>>();
                self.else_values.insert(S::id().key, SpecialStyleValue::Initial);
            }
        };
    }

    pub fn remove_style<S: Style>(&mut self) {
        self.style_map.remove::<StyleKey<S>>();
        self.else_values.remove(&S::id().key);
    }

    pub fn or_else(this: &StyleTable, other: &StyleTable) -> StyleTable {
        let mut table = StyleTable::new();
        unsafe {
            for (k, v) in this.style_map.data().iter() {
                table.style_map.data_mut().insert(*k, v.clone());
            }
            for (k, v) in other.style_map.data().iter().filter(|(k, _)| !this.style_map.data().contains_key(k)) {
                table.style_map.data_mut().insert(*k, v.clone());
            }
        }
        for (k, v) in this.else_values.iter() {
            table.else_values.insert(*k, v.clone());
        }
        for (k, v) in other.else_values.iter().filter(|(k, _)| !this.else_values.contains_key(k)) {
            table.else_values.insert(*k, v.clone());
        }
        table
    }
}

impl StyleMap for StyleTable {
    fn get_style_static<S: Style>(&self) -> Option<StyleValue<&S>> {
        match self.style_map.get::<StyleKey<S>>() {
            Some(s) => Some(StyleValue::Style(s)),
            None => match self.else_values.get(&S::id().key) {
                Some(SpecialStyleValue::Inherit) => Some(StyleValue::Inherit),
                Some(SpecialStyleValue::Initial) => Some(StyleValue::Initial),
                None if S::inheritable() => { Some(StyleValue::Inherit) },
                _ => None,
            }
        }
    }

    fn get_style_dyn(&self, style: StyleId) -> Option<StyleValue<&Any>> {
        match unsafe { self.style_map.data().get(&style.key) }.map(|a| (&*a) as &Any) {
            Some(s) => Some(StyleValue::Style(s)),
            None => match self.else_values.get(&style.key) {
                Some(SpecialStyleValue::Inherit) => Some(StyleValue::Inherit),
                Some(SpecialStyleValue::Initial) => Some(StyleValue::Initial),
                None if style.inheritable => Some(StyleValue::Inherit),
                _ => None,
            }
        }
    }

    fn as_table(&self) -> StyleTable {
        self.clone()
    }
}

#[derive(Copy, Clone)]
enum SpecialStyleValue {
    Inherit, Initial
}


