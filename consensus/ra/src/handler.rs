use async_trait::async_trait;
use futures_util::SinkExt;
use network::Acknowledgement;
use tokio::sync::mpsc::UnboundedSender;
use types::{ProtMsg, WrapperMsg};

#[derive(Debug, Clone)]
pub struct Handler {
    consensus_tx: UnboundedSender<WrapperMsg<ProtMsg>>,
}

impl Handler {
    pub fn new(consensus_tx: UnboundedSender<WrapperMsg<ProtMsg>>) -> Self {
        Self { consensus_tx }
    }
}

#[async_trait]
impl network::Handler<Acknowledgement, WrapperMsg<ProtMsg>> for Handler {
    async fn dispatch(
        &self,
        msg: WrapperMsg<ProtMsg>,
        writer: &mut network::Writer<Acknowledgement>,
    ) {
        // Forward the message
        // self.consensus_tx
        //     .send(msg)
        //     .expect("Failed to send message to the consensus channel");
        self.consensus_tx.send(msg.clone()).unwrap_or_else(|err| {
            panic!(
                "Failed to send message to the consensus channel: {:?}. Error: {}",
                msg, err
            )
        });

        // Acknowledge
        writer
            .send(Acknowledgement::Pong)
            .await
            .expect("Failed to send an acknowledgement");
    }
}
