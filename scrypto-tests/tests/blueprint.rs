#![cfg_attr(not(feature = "std"), no_std)]

use scrypto::buffer::*;
use scrypto::constructs::*;
use scrypto::kernel::*;
use scrypto::types::rust::vec;
use scrypto::types::*;
use scrypto::*;
use serde::Serialize;
use serde_json::{json, to_value, Value};

blueprint! {
    struct Simple {
        state: u32,
    }

    impl Simple {
        pub fn new() -> Component {
            Self {
                state: 0
            }.into()
        }

        pub fn get_state(&self) -> u32 {
            self.state
        }

        pub fn set_state(&mut self, new_state: u32) {
            self.state = new_state;
        }
    }
}

fn assert_json_eq<T: Serialize>(actual: T, expected: Value) {
    assert_eq!(to_value(&actual).unwrap(), expected);
}

#[test]
fn test_simple_abi() {
    let ptr = Simple_abi();
    let abi: scrypto::abi::Blueprint = scrypto_consume(ptr, |slice| scrypto_decode(slice).unwrap());

    assert_json_eq(
        abi,
        json!({
          "package": "056967d3d49213394892980af59be76e9b3e7cc4cb78237460d0c7",
          "name": "Simple",
          "functions": [
            {
              "name": "new",
              "inputs": [],
              "output": {
                "type": "SystemType",
                "name": "::scrypto::constructs::Component"
              }
            }
          ],
          "methods": [
            {
              "name": "get_state",
              "mutability": "Immutable",
              "inputs": [],
              "output": {
                "type": "U32"
              }
            },
            {
              "name": "set_state",
              "mutability": "Mutable",
              "inputs": [
                {
                  "type": "U32"
                }
              ],
              "output": {
                "type": "Unit"
              }
            }
          ]
        }),
    );
}

#[no_mangle]
pub extern "C" fn kernel(_op: u32, _input_ptr: *const u8, _input_len: usize) -> *mut u8 {
    let response = GetPackageAddressOutput {
        address: Address::from_hex("056967d3d49213394892980af59be76e9b3e7cc4cb78237460d0c7")
            .unwrap(),
    };
    scrypto_wrap(&scrypto_encode(&response))
}