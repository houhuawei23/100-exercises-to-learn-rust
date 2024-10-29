// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadError> {
        let (response_sender, response_receiver) = sync_channel(1);
        let command = Command::Insert {
            draft: draft.clone(),
            response_channel: response_sender,
        };
        self.sender
            .try_send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");
        let ticket_id = response_receiver.recv().expect("No response received!");
        return ticket_id;
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadError> {
        let (response_sender, response_receiver) = sync_channel(1);
        let command = Command::Get {
            id,
            response_channel: response_sender,
        };
        self.sender
            .try_send(command)
            .expect("Did you actually spawn a thread? The channel is closed!");
        let ticket = response_receiver.recv().expect("No response received!");
        return ticket;
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    return TicketStoreClient { sender };
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadError;

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<Result<TicketId, OverloadError>>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Result<Option<Ticket>, OverloadError>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(Ok(id));
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(Ok(ticket.cloned()));
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
