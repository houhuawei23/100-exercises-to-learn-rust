// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

// impl TryFrom<String> for Status {
//     type Error = String;
//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         if s.to_lowercase() == "todo" {
//             Ok(Status::ToDo)
//         } else if s.to_lowercase() == "inprogress" {
//             Ok(Status::InProgress)
//         } else if s.to_lowercase() == "done" {
//             Ok(Status::Done)
//         } else {
//             Err(format!("Invalid status: {}", s))
//         }
//     }
// }

// impl TryFrom<&str> for Status {
//     type Error = String;
//     fn try_from(s: &str) -> Result<Self, Self::Error> {
//         return Status::try_from(s.to_string());
//     }
// }

// impl TryFrom<String> for Status {
//     type Error = String;
//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         return s.as_str().try_into();
//     }
// }

// impl TryFrom<&str> for Status {
//     type Error = String;
//     fn try_from(s: &str) -> Result<Self, Self::Error> {
//         match s.to_lowercase().as_str() {
//             "todo" => Ok(Status::ToDo),
//             "inprogress" => Ok(Status::InProgress),
//             "done" => Ok(Status::Done),
//             _ => Err(format!("Invalid status: {}", s)),
//         }
//     }
// }
#[derive(Debug, thiserror::Error)]
#[error("{status} is not a valid status")]
struct ParseStatusError {
    status: String,
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        return s.as_str().try_into();
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(ParseStatusError {
                status: s.to_string(),
            }),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
