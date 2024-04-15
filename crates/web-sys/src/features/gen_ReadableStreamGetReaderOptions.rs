#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamGetReaderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamGetReaderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
    pub type ReadableStreamGetReaderOptions;
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[wasm_bindgen(method, setter = "mode")]
    fn mode_shim(this: &ReadableStreamGetReaderOptions, val: ReadableStreamReaderMode);
}
impl ReadableStreamGetReaderOptions {
    #[doc = "Construct a new `ReadableStreamGetReaderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"]
    pub fn mode(&mut self, val: ReadableStreamReaderMode) -> &mut Self {
        self.mode_shim(val);
        self
    }
}
impl Default for ReadableStreamGetReaderOptions {
    fn default() -> Self {
        Self::new()
    }
}
