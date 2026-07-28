use anyhow::Result;

fn main() -> Result<()> {
  if nil_env::generate_ffi_bindings() {
    generate_node()?;
    generate_csharp()?;
  }

  Ok(())
}

fn generate_node() -> Result<()> {
  nil_ffi_node::generate()
    .input("src/lib.rs")
    .output("../../packages/ffi/src/def.ts")
    .call()?;

  Ok(())
}

fn generate_csharp() -> Result<()> {
  csbindgen::Builder::default()
    .input_extern_file("src/lib.rs")
    .input_extern_file("src/status.rs")
    .csharp_dll_name("libnil")
    .csharp_class_name("libnil")
    .csharp_class_accessibility("public")
    .csharp_type_rename(rename_cs_type)
    .generate_csharp_file("gen/libnil.cs")
    .unwrap();

  Ok(())
}

fn rename_cs_type(name: String) -> String {
  match name.as_str() {
    "RequestId" => "uint".into(),
    "Status" => "StatusCode".into(),
    _ => name,
  }
}
