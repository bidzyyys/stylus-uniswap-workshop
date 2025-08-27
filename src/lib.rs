//! Constant-Sum Curve calculation for Uniswap V4 Hooks.
//!
//! Based on <https://www.v4-by-example.org/hooks/custom-curve>
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::{evm, prelude::*, storage::StorageString};

/// The currency data type.
pub type Currency = Address;

sol! {
    /// Emitted when the amount of input tokens for an exact-output swap
    /// is calculated.
    #[allow(missing_docs)]
    #[derive(Debug)]
    event AmountInCalculated(
        uint256 amount_out,
        address input,
        address output,
        bool zero_for_one
    );

    /// Emitted when the amount of output tokens for an exact-input swap
    /// is calculated.
    #[allow(missing_docs)]
    #[derive(Debug)]
    event AmountOutCalculated(
        uint256 amount_in,
        address input,
        address output,
        bool zero_for_one
    );
}

sol! {
    /// Indicates a custom error.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error CurveCustomError();
}

#[derive(SolidityError, Debug)]
pub enum Error {
    /// Indicates a custom error.
    CustomError(CurveCustomError),
}
#[storage]
#[entrypoint]
struct ConstantSumCurve {
    version: StorageString,
}

/// Interface of an [`UniswapCurve`] contract.
///
/// NOTE: The contract's interface can be modified in any way.
pub trait IUniswapV4Curve {
    /// The error type associated to the trait implementation.
    type Error: Into<alloc::vec::Vec<u8>>;

    /// Returns the version of the curve.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    fn version(&self) -> String;

    /// Returns the amount of input tokens for an exact-output swap.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `amount_out` the amount of output tokens the user expects to receive.
    /// * `input` - The input token.
    /// * `output` - The output token.
    /// * `zero_for_one` - True if the input token is token0.
    ///
    /// # Errors
    ///
    /// May return an [`Error`].
    ///
    /// # Events
    ///
    /// May emit any event.
    fn get_amount_in_for_exact_output(
        &mut self,
        amount_out: U256,
        input: Currency,
        output: Currency,
        zero_for_one: bool,
    ) -> Result<U256, Self::Error>;

    /// Returns the amount of output tokens for an exact-input swap.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `amount_in` - The amount of input tokens.
    /// * `input` - The input token.
    /// * `output` - The output token.
    /// * `zero_for_one` - True if the input token is `token_0`.
    ///
    /// # Errors
    ///
    /// May return an [`Error`].
    ///
    /// # Events
    ///
    /// May emit any event.
    fn get_amount_out_from_exact_input(
        &mut self,
        amount_in: U256,
        input: Currency,
        output: Currency,
        zero_for_one: bool,
    ) -> Result<U256, Self::Error>;
}

#[public]
#[implements(IUniswapV4Curve<Error = Error>)]
impl ConstantSumCurve {
    #[constructor]
    pub fn constructor(&mut self, version: String) {
        self.version.set_str(version);
    }
}

#[public]
impl IUniswapV4Curve for ConstantSumCurve {
    type Error = Error;

    fn version(&self) -> String {
        self.version.get_string()
    }

    fn get_amount_in_for_exact_output(
        &mut self,
        amount_out: U256,
        input: Currency,
        output: Currency,
        zero_for_one: bool,
    ) -> Result<U256, Self::Error> {
        // Calculate `amount_in` based on swap params.
        let amount_in = self.calculate_amount_in(amount_out, input, output, zero_for_one);

        #[allow(deprecated)]
        evm::log(AmountInCalculated {
            amount_out,
            input,
            output,
            zero_for_one,
        });

        Ok(amount_in)
    }

    fn get_amount_out_from_exact_input(
        &mut self,
        amount_in: U256,
        input: Currency,
        output: Currency,
        zero_for_one: bool,
    ) -> Result<U256, Self::Error> {
        let amount_out = self.calculate_amount_out(amount_in, input, output, zero_for_one);

        #[allow(deprecated)]
        evm::log(AmountOutCalculated {
            amount_in,
            input,
            output,
            zero_for_one,
        });

        Ok(amount_out)
    }
}

impl ConstantSumCurve {
    /// Calculates the amount of input tokens for an exact-output swap.
    ///
    /// # Arguments
    ///
    /// * `&self` - Read access to the contract's state.
    /// * `amount_out` the amount of output tokens the user expects to receive.
    /// * `input` - The input token.
    /// * `output` - The output token.
    /// * `zero_for_one` - True if the input token is `token0`.
    fn calculate_amount_in(
        &self,
        amount_out: U256,
        _input: Currency,
        _output: Currency,
        _zero_for_one: bool,
    ) -> U256 {
        // In constant-sum curve, tokens trade exactly 1:1
        let amount_in = amount_out;

        amount_in
    }

    /// Returns the amount of output tokens for an exact-input swap.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Write access to the contract's state.
    /// * `amount_in` - The amount of input tokens.
    /// * `input` - The input token.
    /// * `output` - The output token.
    /// * `zero_for_one` - True if the input token is `token_0`.
    fn calculate_amount_out(
        &self,
        amount_in: U256,
        _input: Currency,
        _output: Currency,
        _zero_for_one: bool,
    ) -> U256 {
        // in constant-sum curve, tokens trade exactly 1:1
        let amount_out = amount_in;

        amount_out
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use alloy_primitives::{address, uint, Address};
    use motsu::prelude::Contract;

    use super::*;

    const CURRENCY_1: Address = address!("A11CEacF9aa32246d767FCCD72e02d6bCbcC375d");
    const CURRENCY_2: Address = address!("B0B0cB49ec2e96DF5F5fFB081acaE66A2cBBc2e2");

    #[test]
    fn sample_test() {
        assert_eq!(4, 2 + 2);
    }

    #[motsu::test]
    fn calculates_amount_in(contract: Contract<ConstantSumCurve>, alice: Address) {
        let amount_out = uint!(1_U256);
        let expected_amount_in = amount_out; // 1:1 swap
        let amount_in = contract
            .sender(alice)
            .calculate_amount_in(amount_out, CURRENCY_1, CURRENCY_2, true);
        assert_eq!(expected_amount_in, amount_in);
    }

    #[motsu::test]
    fn calculates_amount_out(contract: Contract<ConstantSumCurve>, alice: Address) {
        let amount_in = uint!(2_U256);
        let expected_amount_out = amount_in; // 1:1 swap
        let amount_out = contract
            .sender(alice)
            .calculate_amount_out(amount_in, CURRENCY_1, CURRENCY_2, true);
        assert_eq!(expected_amount_out, amount_out);
    }

    #[motsu::test]
    fn returns_amount_in_for_exact_output(contract: Contract<ConstantSumCurve>, alice: Address) {
        let amount_out = uint!(1_U256);
        let expected_amount_in = amount_out; // 1:1 swap
        let zero_for_one = true;
        let amount_in = contract
            .sender(alice)
            .get_amount_in_for_exact_output(amount_out, CURRENCY_1, CURRENCY_2, zero_for_one)
            .expect("should calculate `amount_in`");
        assert_eq!(expected_amount_in, amount_in);

        // Assert emitted events.
        contract.assert_emitted(&AmountInCalculated {
            amount_out,
            input: CURRENCY_1,
            output: CURRENCY_2,
            zero_for_one,
        });
    }

    #[motsu::test]
    fn returns_amount_out_from_exact_input(contract: Contract<ConstantSumCurve>, alice: Address) {
        let amount_in = uint!(2_U256);
        let expected_amount_out = amount_in; // 1:1 swap
        let zero_for_one = true;
        let amount_out = contract
            .sender(alice)
            .get_amount_out_from_exact_input(amount_in, CURRENCY_1, CURRENCY_2, zero_for_one)
            .expect("should calculate `amount_out`");
        assert_eq!(expected_amount_out, amount_out);

        // Assert emitted events.
        contract.assert_emitted(&AmountOutCalculated {
            amount_in,
            input: CURRENCY_1,
            output: CURRENCY_2,
            zero_for_one,
        });
    }
}
