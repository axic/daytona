use evmc_declare::evmc_declare_vm;
use evmc_vm::{EvmcVm, ExecutionContext, ExecutionMessage, ExecutionResult};
use standalone_parity_evm::*;

use std::ops::Deref;
use std::sync::Arc;

#[evmc_declare_vm("Daytona", "evm", "0.1.0")]
pub struct Daytona;

// For some explanation see ethcore/vm/src/tests.rs::FakeExt

#[derive(Default)]
struct VMExt {
    pub info: EnvInfo,
    pub schedule: Schedule,
}

impl Ext for VMExt {
    /// Returns the storage value for a given key if reversion happens on the current transaction.
    fn initial_storage_at(&self, key: &H256) -> Result<H256> {
        unimplemented!()
    }

    /// Returns a value for given key.
    fn storage_at(&self, key: &H256) -> Result<H256> {
        unimplemented!()
    }

    /// Stores a value for given key.
    fn set_storage(&mut self, key: H256, value: H256) -> Result<()> {
        unimplemented!()
    }

    /// Determine whether an account exists.
    fn exists(&self, address: &Address) -> Result<bool> {
        // NOTE: used by SELFDESTRUCT/CALL for gas metering (not used here now since we don't charge gas)
        unimplemented!()
    }

    /// Determine whether an account exists and is not null (zero balance/nonce, no code).
    fn exists_and_not_null(&self, address: &Address) -> Result<bool> {
        // NOTE: used by SELFDESTRUCT/CALL for gas metering (not used here now since we don't charge gas)
        unimplemented!()
    }

    /// Balance of the origin account.
    fn origin_balance(&self) -> Result<U256> {
        // NOTE: used by SLEFDESTRUCT for gas metering (not used here now since we don't charge gas)
        unimplemented!()
    }

    /// Returns address balance.
    fn balance(&self, address: &Address) -> Result<U256> {
        unimplemented!()
    }

    /// Returns the hash of one of the 256 most recent complete blocks.
    fn blockhash(&mut self, number: &U256) -> H256 {
        unimplemented!()
    }

    /// Creates new contract.
    ///
    /// Returns gas_left and contract address if contract creation was succesfull.
    fn create(
        &mut self,
        gas: &U256,
        value: &U256,
        code: &[u8],
        address: CreateContractAddress,
        trap: bool,
    ) -> ::std::result::Result<ContractCreateResult, TrapKind> {
        unimplemented!()
        // Could just return ContractCreateResult::Failed for now
    }

    /// Message call.
    ///
    /// Returns Err, if we run out of gas.
    /// Otherwise returns call_result which contains gas left
    /// and true if subcall was successfull.
    fn call(
        &mut self,
        gas: &U256,
        sender_address: &Address,
        receive_address: &Address,
        value: Option<U256>,
        data: &[u8],
        code_address: &Address,
        call_type: CallType,
        trap: bool,
    ) -> ::std::result::Result<MessageCallResult, TrapKind> {
        unimplemented!()
    }

    /// Returns code at given address
    fn extcode(&self, address: &Address) -> Result<Option<Arc<Bytes>>> {
        unimplemented!()
    }

    /// Returns code hash at given address
    fn extcodehash(&self, address: &Address) -> Result<Option<H256>> {
        // NOTE: only used by constantinople's EXTCODEHASH
        unimplemented!()
    }

    /// Returns code size at given address
    fn extcodesize(&self, address: &Address) -> Result<Option<usize>> {
        unimplemented!()
    }

    /// Creates log entry with given topics and data
    fn log(&mut self, topics: Vec<H256>, data: &[u8]) -> Result<()> {
        unimplemented!()
    }

    /// Should be called when transaction calls `RETURN` opcode.
    /// Returns gas_left if cost of returning the data is not too high.
    fn ret(self, gas: &U256, data: &ReturnData, apply_state: bool) -> Result<U256> {
        // NOTE: this is only called through finalize(), but we are not using it
        // so it should be safe to ignore it here
        unimplemented!()
    }

    /// Should be called when contract commits suicide.
    /// Address to which funds should be refunded.
    fn suicide(&mut self, refund_address: &Address) -> Result<()> {
        unimplemented!()
    }

    /// Returns schedule.
    fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    /// Returns environment info.
    fn env_info(&self) -> &EnvInfo {
        &self.info
    }

    /// Returns current depth of execution.
    ///
    /// If contract A calls contract B, and contract B calls C,
    /// then A depth is 0, B is 1, C is 2 and so on.
    fn depth(&self) -> usize {
        // FIXME: implement
        0
    }

    /// Increments sstore refunds counter.
    fn add_sstore_refund(&mut self, value: usize) {
        unimplemented!()
    }

    /// Decrements sstore refunds counter.
    fn sub_sstore_refund(&mut self, value: usize) {
        unimplemented!()
    }

    /// Decide if any more operations should be traced. Passthrough for the VM trace.
    fn trace_next_instruction(&mut self, _pc: usize, _instruction: u8, _current_gas: U256) -> bool {
        false
    }

    /// Prepare to trace an operation. Passthrough for the VM trace.
    fn trace_prepare_execute(
        &mut self,
        _pc: usize,
        _instruction: u8,
        _gas_cost: U256,
        _mem_written: Option<(usize, usize)>,
        _store_written: Option<(U256, U256)>,
    ) {
    }

    /// Trace the finalised execution of a single instruction.
    fn trace_executed(&mut self, _gas_used: U256, _stack_push: &[U256], _mem: &[u8]) {}

    /// Check if running in static context.
    fn is_static(&self) -> bool {
        // NOTE: this is used by CREATE/CALL*, but since ewasm in the upper layer will handle this anyway, we can just ignore it here
        false
    }
}

impl EvmcVm for Daytona {
    fn init() -> Self {
        Daytona {}
    }

    fn execute(&self, code: &[u8], message: &ExecutionMessage, context: &ExecutionContext) -> ExecutionResult {
        // This is the "message"
        let mut params = ActionParams::default();
        // FIXME: fill out params
        params.code = Some(Arc::new(code.to_vec()));
        params.gas = U256::from(message.gas());
        params.data = if let Some(input) = message.input() {
           Some(input.clone())
        } else {
           None
        };

        // This is the wrapper for "context"
        let mut ext = VMExt::default();

        let mut instance = Factory::default().create(params, ext.schedule(), ext.depth());
        let result = instance.exec(&mut ext);

        // Could run `result.finalize(ext)` here, but processing manually seemed simpler.
        match result {
            Ok(Ok(GasLeft::Known(gas_left))) => {
                // NOTE: as_u64 is safe here given the incoming gas limit is u64
                // FIXME: casting from u64 to i64 is unsafe
                ExecutionResult::success(gas_left.as_u64() as i64, None)
            }
            Ok(Ok(GasLeft::NeedsReturn {
                gas_left,
                data,
                apply_state,
            })) => {
                let gas_left = gas_left.as_u64() as i64;
                if apply_state {
                    ExecutionResult::success(gas_left, Some(&data.deref()))
                } else {
                    ExecutionResult::revert(Some(&data.deref()))
                }
            }
            // FIXME: not sure what this state means
            Ok(Err(err)) => ExecutionResult::failure(),
            // FIXME: add support for pushing the error message as revert data
            Err(err) => ExecutionResult::failure(),
        }
    }
}
