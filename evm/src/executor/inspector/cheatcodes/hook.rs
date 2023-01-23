use super::Cheatcodes;
use crate::{
    abi::HEVMCalls,
    // error::{SolError, ERROR_PREFIX, REVERT_PREFIX},
    executor::backend::DatabaseExt,
};
use bytes::Bytes;

use std::cmp::Ordering;
use ethers::{
        abi::AbiEncode,
        types::{Address, U256},
    };
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HookCallDataContext {
    /// The partial calldata to match for mock
    pub calldata: Bytes,
    /// The value to match for mock
    pub value: Option<U256>,
}
#[derive(Clone, Debug, Default)]
pub struct HookCallBackData {
    pub address: Address,
    pub calldata: Bytes,
}

#[derive(Clone, Debug, Default)]
pub struct HookCallExecutionContext {
    pub caller: Address,
}

impl Ord for HookCallDataContext {
    fn cmp(&self, other: &Self) -> Ordering {
        // Calldata matching is reversed to ensure that a tighter match is
        // returned if an exact match is not found. In case, there is
        // a partial match to calldata that is more specific than
        // a match to a msg.value, then the more specific calldata takes
        // precedence.
        self.calldata.cmp(&other.calldata).reverse().then(self.value.cmp(&other.value).reverse())
    }
}

impl PartialOrd for HookCallDataContext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn apply<DB: DatabaseExt>(
    state: &mut Cheatcodes,
    caller: Address,
    call: &HEVMCalls,
) -> Option<Result<Bytes, Bytes>> {
    Some(match call {
        HEVMCalls::HookCall(inner) => {
            state.hooked_calls.entry(inner.0).or_default().insert(
                HookCallDataContext {
                    calldata: inner.1.to_vec().into(),
                    value: None
                },
                HookCallBackData {
                    address: caller,
                    calldata: inner.2.to_vec().into(),
                }
            );
            Ok(Bytes::new())
        },
        HEVMCalls::ClearHookedCalls(_) => {
            state.hooked_calls = Default::default();
            Ok(Bytes::new())
        },
        HEVMCalls::ExecuteHookedCall(_) => {
            let execute_hook = HookCallExecutionContext {
                caller: caller.clone(),
            };
            if state.execute_hook.is_some() {
                return Some(Err("You can't execute a hook twice.".encode().into()));
            }
            state.execute_hook = Some(execute_hook);
            Ok(Bytes::new())
        }
       _ => return None,
    })
}