use evmc_declare::evmc_declare_vm;
use evmc_vm::{EvmcVm, ExecutionContext, ExecutionMessage, ExecutionResult};

mod parity;

#[evmc_declare_vm("Daytona", "evm", "0.1.0")]
pub struct Daytona;

impl EvmcVm for Daytona {
    fn init() -> Self {
        Daytona {}
    }

    fn execute(
        &self,
        revision: evmc_sys::evmc_revision,
        code: &[u8],
        message: &ExecutionMessage,
        context: &ExecutionContext,
    ) -> ExecutionResult {
        parity::execute(revision, code, message, context)
    }
}
