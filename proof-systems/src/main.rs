use std::str::FromStr;

use mina_hasher::{Hashable, Hasher, ROInput};
use mina_signer::{BaseField, Keypair, NetworkId, Signer};
use o1_utils::FieldHelpers;

#[derive(Clone)]
struct FieldMessage {
    a: BaseField,
}

impl Hashable for FieldMessage {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.a)
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}

// private key is the same as in typescript file
const PRIVATE_KEY: &str = "21111dffe58c42d130119a33840e880d5a1a299d45427280d035f95c5d17736b";

fn main() {
    let keypair = Keypair::from_hex(PRIVATE_KEY).unwrap();

    let mut hasher = mina_hasher::create_kimchi::<FieldMessage>(NetworkId::TESTNET);

    let mut ctx = mina_signer::create_kimchi::<FieldMessage>(NetworkId::TESTNET);

    let msg = FieldMessage {
        a: BaseField::from_str("42").unwrap(),
    };

    println!("field_hash: {}", hasher.hash(&msg).to_biguint());

    let sig = ctx.sign(&keypair, &msg);

    println!("field_sig_rx: {}", sig.rx.to_biguint());
    println!("field_sig_s: {}", sig.s.to_biguint());
}
