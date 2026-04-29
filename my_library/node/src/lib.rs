use core_lib::generate_msg;
use napi_derive::napi;

#[napi]
pub fn saludar(name: String) -> String {
  generate_msg(&name)
}
