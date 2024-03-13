use godot::prelude::*;

struct RustLib;

#[gdextension]
unsafe impl ExtensionLibrary for RustLib {}