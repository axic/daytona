use evmc_declare::evmc_declare_vm;
use evmc_vm::{EvmcVm, ExecutionContext, ExecutionMessage, ExecutionResult};
use standalone_parity_evm::*;

use std::ops::Deref;
use std::sync::Arc;

#[evmc_declare_vm("Daytona", "evm", "0.1.0")]
pub struct Daytona;

// For some explanation see ethcore/vm/src/tests.rs::FakeExt

struct VMExt<'a> {
    info: EnvInfo,
    schedule: Schedule,
    static_mode: bool,
    depth: i32,
    address: evmc_vm::Address,
    host: &'a mut ExecutionContext<'a>,
}

impl<'a> Ext for VMExt<'a> {
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
        self.host.set_storage(
            &self.address,
            &evmc_vm::Bytes32 { bytes: key.0 },
            &evmc_vm::Bytes32 { bytes: value.0 },
        );
        Ok(())
    }

    /// Determine whether an account exists.
    fn exists(&self, address: &Address) -> Result<bool> {
        unimplemented!()
    }

    /// Determine whether an account exists and is not null (zero balance/nonce, no code).
    fn exists_and_not_null(&self, address: &Address) -> Result<bool> {
        unimplemented!()
    }

    /// Balance of the origin account.
    fn origin_balance(&self) -> Result<U256> {
        unimplemented!()
    }

    /// Returns address balance.
    fn balance(&self, address: &Address) -> Result<U256> {
        unimplemented!()
    }

    /// Returns the hash of one of the 256 most recent complete blocks.
    fn blockhash(&mut self, number: &U256) -> H256 {
        H256::from(self.host.get_block_hash(number.as_u64() as i64).bytes)
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
        let kind = match call_type {
            CallType::Call => evmc_sys::evmc_call_kind::EVMC_CALL,
            CallType::CallCode => evmc_sys::evmc_call_kind::EVMC_CALLCODE,
            CallType::DelegateCall => evmc_sys::evmc_call_kind::EVMC_DELEGATECALL,
            CallType::StaticCall => evmc_sys::evmc_call_kind::EVMC_CALL,
            CallType::None => panic!(),
        };
        let flags = if call_type == CallType::StaticCall {
            // TODO: make this nicer
            evmc_sys::evmc_flags::EVMC_STATIC as u32
        } else {
            0
        };
        let value: [u8; 32] = value.unwrap_or_default().into();
        let message = ExecutionMessage::new(
            /*kind*/ kind,
            /*flags*/ flags,
            /*depth*/ self.depth,
            /*gas*/ gas.as_u64() as i64,
            /*destination*/
            evmc_vm::Address {
                bytes: receive_address.0,
            },
            /*sender*/
            evmc_vm::Address {
                bytes: sender_address.0,
            },
            /*input*/ Some(data),
            /*value*/ evmc_vm::Uint256 { bytes: value },
            /*create2_salt*/ evmc_vm::Bytes32::default(), // FIXME
        );
        //println!("  message: {:?}", message);
        let result = self.host.call(&message);
        //println!("  result: {:?}", result);
        let output = if result.output().is_some() {
            let output = result.output().unwrap().clone();
            let len = output.len();
            ReturnData::new(output, 0, len)
        } else {
            ReturnData::new(vec![], 0, 0)
        };
        match result.status_code() {
            evmc_sys::evmc_status_code::EVMC_SUCCESS => Ok(MessageCallResult::Success(
                U256::from(result.gas_left()),
                output,
            )),
            evmc_sys::evmc_status_code::EVMC_REVERT => Ok(MessageCallResult::Reverted(
                U256::from(result.gas_left()),
                output,
            )),
            evmc_sys::evmc_status_code::EVMC_FAILURE => Ok(MessageCallResult::Failed),
            // TODO: check if these two are correct
            evmc_sys::evmc_status_code::EVMC_INTERNAL_ERROR => panic!(),
            _ => Ok(MessageCallResult::Failed),
        }
    }

    /// Returns code at given address
    fn extcode(&self, address: &Address) -> Result<Option<Arc<Bytes>>> {
        unimplemented!()
    }

    /// Returns code hash at given address
    fn extcodehash(&self, address: &Address) -> Result<Option<H256>> {
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
        self.host.selfdestruct(
            &self.address,
            &evmc_vm::Address {
                bytes: refund_address.0,
            },
        );
        Ok(())
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
        // FIXME: check if this cast is safe
        self.depth as usize
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
        self.static_mode
    }
}

impl EvmcVm for Daytona {
    fn init() -> Self {
        Daytona {}
    }

    fn execute<'a>(
        &self,
        revision: evmc_sys::evmc_revision,
        code: &'a [u8],
        message: &'a ExecutionMessage,
        context: Option<&'a mut ExecutionContext<'a>>,
    ) -> ExecutionResult {
        if context.is_none() {
            return ExecutionResult::failure();
        }

        let context = context.unwrap();

        let tx_context = context.get_tx_context().clone();
        let static_mode = message.flags() == (evmc_sys::evmc_flags::EVMC_STATIC as u32);

        // This is the "message"
        let mut params = ActionParams::default();
        params.call_type = match message.kind() {
            evmc_sys::evmc_call_kind::EVMC_CALL => {
                if static_mode {
                    CallType::StaticCall
                } else {
                    CallType::Call
                }
            }
            evmc_sys::evmc_call_kind::EVMC_CALLCODE => CallType::CallCode,
            evmc_sys::evmc_call_kind::EVMC_DELEGATECALL => CallType::DelegateCall,
            evmc_sys::evmc_call_kind::EVMC_CREATE => CallType::None,
            evmc_sys::evmc_call_kind::EVMC_CREATE2 => CallType::None,
            _ => unimplemented!(),
        };
        // FIXME: add params_type?
        params.code = Some(Arc::new(code.to_vec()));
        // FIXME: add code_hash?
        params.gas = U256::from(message.gas());
        params.data = if let Some(input) = message.input() {
            Some(input.clone())
        } else {
            None
        };
        // FIXME: why are these two different fields?
        params.address = Address::from(message.destination().bytes.clone());
        params.code_address = Address::from(message.destination().bytes);
        params.sender = Address::from(message.sender().bytes);
        params.origin = Address::from(tx_context.tx_origin.bytes);
        params.gas_price = U256::from(tx_context.tx_gas_price.bytes);
        // FIXME: how do we know the difference between Transfer and Apparent?
        params.value = ActionValue::Apparent(U256::from(message.value().bytes));

        let schedule = match revision {
            evmc_sys::evmc_revision::EVMC_FRONTIER => Schedule::new_frontier(),
            evmc_sys::evmc_revision::EVMC_HOMESTEAD => Schedule::new_homestead(),
            evmc_sys::evmc_revision::EVMC_TANGERINE_WHISTLE => {
                Schedule::new_post_eip150(usize::max_value(), false, false, false)
            }
            evmc_sys::evmc_revision::EVMC_SPURIOUS_DRAGON => {
                Schedule::new_post_eip150(24576, true, true, true)
            }
            evmc_sys::evmc_revision::EVMC_BYZANTIUM => {
                let mut schedule = Schedule::new_byzantium();
                // NOTE: fixing a a bug in Parity. See https://github.com/paritytech/parity-ethereum/issues/10914
                // Parity overrides these settings based on a chain config, but the default is wrong.
                schedule.have_create2 = false;
                schedule
            }
            evmc_sys::evmc_revision::EVMC_CONSTANTINOPLE => {
                let mut schedule = Schedule::new_constantinople();
                schedule.eip1283 = true;
                schedule
            }
            // In Parity constantinople is petersburg, because it has eip1283 disabled by default.
            evmc_sys::evmc_revision::EVMC_PETERSBURG => Schedule::new_constantinople(),
            // FIXME: add istanbul
            evmc_sys::evmc_revision::EVMC_ISTANBUL => Schedule::new_constantinople(),
            // FIXME: add berlin
            evmc_sys::evmc_revision::EVMC_BERLIN => Schedule::new_constantinople(),
            _ => unimplemented!(),
        };

        let mut info = EnvInfo::default();
        info.author = Address::from(tx_context.block_coinbase.bytes);
        info.difficulty = U256::from(tx_context.block_difficulty.bytes);
        // FIXME: i64 -> u64 typecasting
        info.number = tx_context.block_number as u64;
        info.timestamp = tx_context.block_timestamp as u64;
        info.gas_limit = U256::from(tx_context.block_gas_limit);

        // This is the wrapper for "context"
        let mut ext = VMExt {
            info: info,
            schedule: schedule,
            static_mode: static_mode,
            depth: message.depth(),
            address: message.destination().clone(),
            host: context,
        };

        let instance = Factory::default().create(params, &ext.schedule, ext.depth());
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
                    ExecutionResult::revert(gas_left, Some(&data.deref()))
                }
            }
            Ok(Err(err)) => {
                let err = match err {
                    Error::OutOfGas => evmc_sys::evmc_status_code::EVMC_OUT_OF_GAS,
                    Error::BadJumpDestination { .. } => {
                        evmc_sys::evmc_status_code::EVMC_BAD_JUMP_DESTINATION
                    }
                    Error::BadInstruction { instruction } => {
                        if instruction == 0xfe {
                            evmc_sys::evmc_status_code::EVMC_INVALID_INSTRUCTION
                        } else {
                            evmc_sys::evmc_status_code::EVMC_UNDEFINED_INSTRUCTION
                        }
                    }
                    Error::StackUnderflow { .. } => {
                        evmc_sys::evmc_status_code::EVMC_STACK_UNDERFLOW
                    }
                    Error::OutOfStack { .. } => evmc_sys::evmc_status_code::EVMC_STACK_OVERFLOW,
                    // FIXME: Support BuiltIn{}?
                    Error::MutableCallInStaticContext => {
                        evmc_sys::evmc_status_code::EVMC_STATIC_MODE_VIOLATION
                    }
                    // FIXME: Support Internal?
                    Error::OutOfBounds { .. } => {
                        evmc_sys::evmc_status_code::EVMC_INVALID_MEMORY_ACCESS
                    }
                    // FIXME: Support Reverted?
                    _ => evmc_sys::evmc_status_code::EVMC_FAILURE,
                };
                ExecutionResult::new(err, 0, None)
            }
            // FIXME: figure out what cases this is called
            Err(err) => ExecutionResult::failure(),
        }
    }
}
