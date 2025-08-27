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
use stylus_sdk::prelude::*;

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
struct ConstantSumCurve {}

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
impl ConstantSumCurve {}

#[cfg(test)]
mod test {}
