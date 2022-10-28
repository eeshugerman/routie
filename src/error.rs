use cairo;

pub type CairoError = cairo::Error;

#[derive(Debug)]
pub enum RoutieError {
    AlreadyLinkedSegment,
    UnlinkedSegment,
    InvalidId, // TODO: be more specific
}

#[derive(Debug)]
pub enum GenericError {
    Routie(RoutieError),
    Cairo(CairoError)
}

impl From<RoutieError> for GenericError {
    fn from(e: RoutieError) -> GenericError {
        GenericError::Routie(e)
    }
}

impl From<CairoError> for GenericError {
    fn from(e: CairoError) -> GenericError {
        GenericError::Cairo(e)
    }
}
