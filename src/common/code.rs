use super::Slice;

pub enum Status {
    Ok,
    NotFound(Slice),
    Corruption(Slice),
    NotSupported(Slice),
    InvalidArgument(Slice),
    IOError,
}

impl Status {
    pub fn is_ok(&self) -> bool {
        match self {
            Status::Ok => true,
            _ => false,
        }
    }
    pub fn is_not_found(&self) -> bool {
        match self {
            Status::NotFound(_) => true,
            _ => false,
        }
    }

    pub fn is_corruption(&self) -> bool {
        match self {
            Status::Corruption(_) => true,
            _ => false,
        }
    }

    pub fn is_not_supported(&self) -> bool {
        match self {
            Status::NotSupported(_) => true,
            _ => false,
        }
    }

    pub fn is_invalid_argument(&self) -> bool {
        match self {
            Status::InvalidArgument(_) => true,
            _ => false,
        }
    }

    pub fn is_io_error(&self) -> bool {
        match self {
            Status::IOError => true,
            _ => false,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => "Ok".to_string(),
            Status::NotFound(s) => format!("NotFound({})", s.to_string()),
            Status::Corruption(s) => format!("Corruption({})", s.to_string()),
            Status::NotSupported(s) => format!("NotSupported({})", s.to_string()),
            Status::InvalidArgument(s) => format!("InvalidArgument({})", s.to_string()),
            Status::IOError => "IOError".to_string(),
        }
    }
}
