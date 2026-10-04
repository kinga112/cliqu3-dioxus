use futures::executor::block_on;
use tokio::sync::{mpsc::Sender, oneshot};
use xmtp::{AccountIdentifier, IdentifierKind, Result, Signer};

pub type SignRequest = (String, oneshot::Sender<Vec<u8>>);

pub struct WalletConnectXmtpSigner {
    pub address: String,
    pub to_loop: Sender<SignRequest>,
}

impl Signer for WalletConnectXmtpSigner {
    fn identifier(&self) -> AccountIdentifier {
        let addr = self.address.to_lowercase();
        AccountIdentifier {
            address: addr,
            kind: IdentifierKind::Ethereum,
        }
    }

    fn sign(&self, text: &str) -> Result<Vec<u8>> {
        let (response_tx, response_rx) = oneshot::channel();

        self.to_loop
            .blocking_send((text.to_string(), response_tx))
            .expect("Main loop is dead");

        println!("Signer thread is now parked, waiting for MetaMask...");

        let signature =
            block_on(response_rx).expect("Oneshot sender dropped - loop probably crashed");

        println!("Signer thread unparked! Signature received.");
        Ok(signature)
    }
}
