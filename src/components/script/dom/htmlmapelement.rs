/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{DOMString, null_string, ErrorResult, CacheableWrapper};
use dom::htmlcollection::HTMLCollection;
use dom::htmlelement::HTMLElement;
use js::jsapi::{JSObject, JSContext};

pub struct HTMLMapElement {
    parent: HTMLElement
}

impl HTMLMapElement {
    pub fn Name(&self) -> DOMString {
        null_string
    }

    pub fn SetName(&mut self, _name: &DOMString, _rv: &mut ErrorResult) {
    }

        fn get_scope_and_cx(&self) -> (*JSObject, *JSContext) {
        let doc = self.parent.parent.parent.owner_doc.unwrap();
        let win = doc.with_base(|doc| doc.window.unwrap());
        let cx = unsafe {(*win.page).js_info.get_ref().js_compartment.cx.ptr};
        let cache = win.get_wrappercache();
        let scope = cache.get_wrapper();
        (scope, cx)
    }

    pub fn Areas(&self) -> @mut HTMLCollection {
        let (scope, cx) = self.get_scope_and_cx();
        HTMLCollection::new(~[], cx, scope)        
    }
}
