use evmc_declare::evmc_declare_vm;
use evmc_vm::*;

#[evmc_declare_vm("Daytona", "evm")]
pub struct Daytona;

impl EvmcVm for Daytona {
    fn init() -> Self {
        Daytona {}
    }

    fn execute(&self, code: &[u8], context: &ExecutionContext) -> ExecutionResult {
        let is_create = context.get_message().kind == evmc_sys::evmc_call_kind::EVMC_CREATE;

        if is_create {
            ExecutionResult::failure()
        } else {
            ExecutionResult::success(66, Some(vec![0xc0, 0xff, 0xee]))
        }
    }
}
