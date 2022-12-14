use scrypto::math::Decimal;
use crate::method::{Arg, Method};
use crate::method::Arg::FungibleBucketArg;
use crate::method_args;

pub struct Deposit {
    pub(crate) amount: Decimal,
    pub(crate) resource: String
}

impl Method for Deposit {
    fn name(&self) -> &str {
        "deposit"
    }

    fn args(&self) -> Option<Vec<Arg>> {
        method_args![
            FungibleBucketArg(self.resource.clone(), self.amount.clone())
        ]
    }

    fn needs_admin_badge(&self) -> bool {
        false
    }
}