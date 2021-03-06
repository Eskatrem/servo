/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! DOM bindings for `CharacterData`.

use dom::bindings::utils::{DOMString, str, ErrorResult};
use dom::bindings::utils::{BindingObject, CacheableWrapper, WrapperCache};
use dom::node::{Node, NodeTypeId, ScriptView};
use js::jsapi::{JSObject, JSContext};

pub struct CharacterData {
    parent: Node<ScriptView>,
    data: ~str
}

impl CharacterData {
    pub fn new(id: NodeTypeId, data: ~str) -> CharacterData {
        CharacterData {
            parent: Node::new(id),
            data: data
        }
    }
    
    pub fn Data(&self) -> DOMString {
        str(self.data.clone())
    }

    pub fn SetData(&mut self, arg: &DOMString, _rv: &mut ErrorResult) {
        self.data = arg.unwrap();
    }

    pub fn Length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn SubstringData(&self, offset: u32, count: u32, _rv: &mut ErrorResult) -> DOMString {
        str(self.data.slice(offset as uint, count as uint).to_str())
    }

    pub fn AppendData(&mut self, arg: &DOMString, _rv: &mut ErrorResult) {
        self.data.push_str(arg.unwrap());
    }

    pub fn InsertData(&mut self, _offset: u32, _arg: &DOMString, _rv: &mut ErrorResult) {
        fail!("CharacterData::InsertData() is unimplemented")
    }

    pub fn DeleteData(&mut self, _offset: u32, _count: u32, _rv: &mut ErrorResult) {
        fail!("CharacterData::DeleteData() is unimplemented")
    }

    pub fn ReplaceData(&mut self, _offset: u32, _count: u32, _arg: &DOMString, _rv: &mut ErrorResult) {
        fail!("CharacterData::ReplaceData() is unimplemented")
    }
}

impl CacheableWrapper for CharacterData {
    fn get_wrappercache(&mut self) -> &mut WrapperCache {
        self.parent.get_wrappercache()
    }

    fn wrap_object_shared(@mut self, _cx: *JSContext, _scope: *JSObject) -> *JSObject {
        fail!(~"need to implement wrapping");
    }
}

impl BindingObject for CharacterData {
    fn GetParentObject(&self, cx: *JSContext) -> Option<@mut CacheableWrapper> {
        self.parent.GetParentObject(cx)
    }
}
