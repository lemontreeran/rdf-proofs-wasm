use ark_std::rand::{prelude::StdRng, SeedableRng};
use rdf_proofs::{BlindSignRequestString, CircuitString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
pub(crate) use log;

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeriveProofVcPair {
    #[serde(rename = "originalDocument")]
    pub original_document: String,
    #[serde(rename = "originalProof")]
    pub original_proof: String,
    #[serde(rename = "disclosedDocument")]
    pub disclosed_document: String,
    #[serde(rename = "disclosedProof")]
    pub disclosed_proof: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeriveProofPredicate {
    #[serde(rename = "circuitId")]
    pub circuit_id: String,
    #[serde(rename = "circuitR1CS")]
    pub circuit_r1cs: String,
    #[serde(rename = "circuitWasm")]
    pub circuit_wasm: String,
    #[serde(rename = "snarkProvingKey")]
    pub snark_proving_key: String,
    pub private: Vec<(String, String)>,
    pub public: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct DeriveProofRequest {
    #[serde(rename = "vcPairs")]
    pub vc_pairs: Vec<DeriveProofVcPair>,
    #[serde(rename = "deanonMap")]
    pub deanon_map: HashMap<String, String>,
    #[serde(rename = "keyGraph")]
    pub key_graph: String,
    pub challenge: Option<String>,
    pub domain: Option<String>,
    pub secret: Option<Vec<u8>>,
    #[serde(rename = "blindSignRequest")]
    pub blind_sign_request: Option<BlindSignRequestString>,
    #[serde(rename = "withPpid")]
    pub with_ppid: Option<bool>,
    pub predicates: Option<Vec<String>>,
    pub circuits: Option<HashMap<String, CircuitString>>,
    #[serde(rename = "openerPubKey")]
    pub opener_pub_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyProofRequest {
    pub vp: String,
    #[serde(rename = "keyGraph")]
    pub key_graph: String,
    pub challenge: Option<String>,
    pub domain: Option<String>,
    #[serde(rename = "snarkVerifyingKeys")]
    pub snark_verifying_keys: Option<HashMap<String, String>>,
    #[serde(rename = "openerPubKey")]
    pub opener_pub_key: Option<String>,
}

////////////////////////////////////////////////////////////////////////////////////
// copied from [docknetwork/crypto-wasm](https://github.com/docknetwork/crypto-wasm)
////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyResult {
    pub verified: bool,
    pub error: Option<String>,
}

pub fn get_seeded_rng() -> StdRng {
    let mut buf = [0u8; 32];
    use rand::{thread_rng, RngCore as RngCoreOld};
    let mut rng = thread_rng();
    rng.fill_bytes(&mut buf);
    // getrandom is using node-js crypto module which doesn't work when building for target web. It
    // works for `wasm-pack test` with chrome in headless and normal mode
    // getrandom::getrandom(&mut buf).unwrap();
    StdRng::from_seed(buf)
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    {
        console_error_panic_hook::set_once();
    }
}
