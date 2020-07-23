extern crate alloc;
use alloc::string::ToString;

use serde::{Deserialize, Serialize};

use crate::abi::*;
use crate::types::*;
use crate::utils::*;

/// A primitive piece of state which has a single owner, and behaves like a physical object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    address: Address,
}

impl From<Address> for Resource {
    fn from(address: Address) -> Self {
        Self { address }
    }
}

impl Resource {
    pub fn new(
        symbol: &str,
        name: &str,
        description: &str,
        url: &str,
        icon_url: &str,
        minter: Option<Address>,
        supply: Option<U256>,
    ) -> Resource {
        let input = CreateResourceInput {
            info: ResourceInfo {
                symbol: symbol.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                url: url.to_string(),
                icon_url: icon_url.to_string(),
                minter,
                supply,
            },
        };
        let output: CreateResourceOutput = syscall(CREATE_RESOURCE, input);

        Resource::from(output.resource)
    }

    pub fn get_info(&self) -> ResourceInfo {
        let input = GetResourceInfoInput {
            resource: self.address.clone(),
        };
        let output: GetResourceInfoOutput = syscall(GET_RESOURCE_INFO, input);

        output.result.unwrap()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }
}
