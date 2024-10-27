pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    /*
          +---------+--------+----------+
    Stack | pointer | length | capacity |
          |  |      |   3    |    5     |
          +--|  ----+--------+----------+
             |
             |
             v
           +---+---+---+---+---+
    Heap:  | H | e | y | ? | ? |
           +---+---+---+---+---+

    */
    #[test]
    fn string_size() {
        // size of pointer is 8 bytes, size of String is 3 * 8 = 24 bytes
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
