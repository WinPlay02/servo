/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLCollectionBinding;
use dom::bindings::js::JS;
use dom::bindings::utils::{Reflectable, Reflector, reflect_dom_object};
use dom::element::Element;
use dom::window::Window;
use servo_util::str::DOMString;

#[deriving(Encodable)]
pub struct HTMLCollection {
    elements: ~[JS<Element>],
    reflector_: Reflector,
    window: JS<Window>,
}

impl HTMLCollection {
    pub fn new_inherited(window: JS<Window>, elements: ~[JS<Element>]) -> HTMLCollection {
        HTMLCollection {
            elements: elements,
            reflector_: Reflector::new(),
            window: window,
        }
    }

    pub fn new(window: &JS<Window>, elements: ~[JS<Element>]) -> JS<HTMLCollection> {
        reflect_dom_object(~HTMLCollection::new_inherited(window.clone(), elements),
                           window, HTMLCollectionBinding::Wrap)
    }
}

impl HTMLCollection {
    // http://dom.spec.whatwg.org/#dom-htmlcollection-length
    pub fn Length(&self) -> u32 {
        self.elements.len() as u32
    }

    // http://dom.spec.whatwg.org/#dom-htmlcollection-item
    pub fn Item(&self, index: u32) -> Option<JS<Element>> {
        if index < self.Length() {
            Some(self.elements[index].clone())
        } else {
            None
        }
    }

    // http://dom.spec.whatwg.org/#dom-htmlcollection-nameditem
    pub fn NamedItem(&self, key: DOMString) -> Option<JS<Element>> {
        // Step 1.
        if key.is_empty() {
            return None;
        }

        // Step 2.
        self.elements.iter().find(|elem| {
            let elem = elem.get();
            elem.get_string_attribute("name") == key || elem.get_string_attribute("id") == key
        }).map(|maybe_elem| maybe_elem.clone())
    }
}

impl HTMLCollection {
    pub fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<JS<Element>> {
        let maybe_elem = self.Item(index);
        *found = maybe_elem.is_some();
        maybe_elem
    }

    pub fn NamedGetter(&self, maybe_name: Option<DOMString>, found: &mut bool) -> Option<JS<Element>> {
        match maybe_name {
            Some(name) => {
                let maybe_elem = self.NamedItem(name);
                *found = maybe_elem.is_some();
                maybe_elem
            },
            None => {
                *found = false;
                None
            }
        }
    }
}

impl Reflectable for HTMLCollection {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        &self.reflector_
    }

    fn mut_reflector<'a>(&'a mut self) -> &'a mut Reflector {
        &mut self.reflector_
    }
}
