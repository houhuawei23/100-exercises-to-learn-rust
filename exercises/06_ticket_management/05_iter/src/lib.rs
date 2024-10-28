use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

fn print_vec(v: &Vec<i32>) {
    for n in v.iter() {
        println!("{}", n);
    }
}

fn main() {
    println!("Hello, world!");
    let mut arr = vec![1, 2, 3];
    for n in arr.iter() {
        // n = &5;
        *n = 4;
        println!("{}", n);
    }
    print_vec(&arr);
    for n in arr.iter_mut() {
        *n = 4;
        println!("{}", n);
    }
    print_vec(&arr);

    for n in arr {
        println!("{}", n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
