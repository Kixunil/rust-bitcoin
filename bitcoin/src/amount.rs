// SPDX-License-Identifier: CC0-1.0

//! Bitcoin amounts.
//!
//! This module mainly introduces the [Amount] and [SignedAmount] types.
//! We refer to the documentation on the types for more information.

#[cfg(feature = "serde")]
pub use units::amount::serde;
pub use units::amount::{
    Amount, CheckedSum, Denomination, Display, ParseAmountError, SignedAmount,
};

use crate::consensus::{encode, Decodable, Encodable};
use crate::io;

impl Decodable for Amount {
    #[inline]
    fn consensus_decode<R: io::Read + ?Sized>(r: &mut R) -> Result<Self, encode::Error> {
        Ok(Amount::from_sat(Decodable::consensus_decode(r)?))
    }
}

impl Encodable for Amount {
    #[inline]
    fn consensus_encode<W: io::Write + ?Sized>(&self, w: &mut W) -> Result<usize, io::Error> {
        self.to_sat().consensus_encode(w)
    }
}
