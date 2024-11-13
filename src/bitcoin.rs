use std::sync::Arc;

use bitcoin_ffi::{Script, TxOut};
use payjoin::bitcoin;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PsbtInput {
    pub witness_utxo: Option<TxOut>,
    pub redeem_script: Option<Arc<Script>>,
    pub witness_script: Option<Arc<Script>>,
}

impl PsbtInput {
    pub fn new(
        witness_utxo: Option<TxOut>,
        redeem_script: Option<Arc<Script>>,
        witness_script: Option<Arc<Script>>,
    ) -> Self {
        Self { witness_utxo, redeem_script, witness_script }
    }
}

impl From<bitcoin::psbt::Input> for PsbtInput {
    fn from(psbt_input: bitcoin::psbt::Input) -> Self {
        Self {
            witness_utxo: psbt_input.witness_utxo.map(|s| s.into()),
            redeem_script: psbt_input.redeem_script.clone().map(|s| Arc::new(s.into())),
            witness_script: psbt_input.witness_script.clone().map(|s| Arc::new(s.into())),
        }
    }
}

impl From<PsbtInput> for bitcoin::psbt::Input {
    fn from(psbt_input: PsbtInput) -> Self {
        Self {
            witness_utxo: psbt_input.witness_utxo.map(|s| s.into()),
            redeem_script: psbt_input.redeem_script.map(|s| Arc::unwrap_or_clone(s).into()),
            witness_script: psbt_input.witness_script.map(|s| Arc::unwrap_or_clone(s).into()),
            ..Default::default()
        }
    }
}
