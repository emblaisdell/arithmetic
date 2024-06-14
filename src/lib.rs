use wasm_bindgen::prelude::*;

#[no_mangle]
pub extern "C" fn alloc(len: u32) -> u32 {
  let layout = std::alloc::Layout::array::<u8>(len.try_into().unwrap()).unwrap();
  unsafe {
    (std::alloc::alloc(layout) as usize).try_into().unwrap()
  }
}

#[wasm_bindgen]
pub fn add(a_bytes: &[u8], b_bytes: &[u8]) -> Vec<u8> {
    let a = i32::from_be_bytes(a_bytes.try_into().unwrap());
    let b = i32::from_be_bytes(b_bytes.try_into().unwrap());
    return (a + b).to_be_bytes().to_vec()
}

#[wasm_bindgen]
pub fn int32_to_str(num_bytes: &[u8]) -> Vec<u8> {
    let num = i32::from_be_bytes(num_bytes.try_into().unwrap());
    num.to_string().as_bytes().to_vec()
}

#[wasm_bindgen]
pub fn hello_world() -> Vec<u8> {
    "Hello World...".as_bytes().to_vec()
}


#[wasm_bindgen]
pub fn id(arr:&[u8]) -> Vec<u8> {
    arr.to_vec()
}
