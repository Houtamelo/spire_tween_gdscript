# include_gdext_lib = []
  #include_gdscript_bridge = []

cargo check
cargo check -F include_gdext_lib
cargo check -F include_gdext_lib,include_gdscript_bridge
cargo check -F include_gdscript_bridge
