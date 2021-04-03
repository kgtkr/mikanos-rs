const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

const fn encode_error(x: usize) -> usize {
    x | ERROR_BIT
}

const fn encode_warning(x: usize) -> usize {
    x
}

#[repr(usize)]
pub enum Status {
    Success = 0,
    LoadError = encode_error(1),
    InvalidParameter = encode_error(2),
    Unsupported = encode_error(3),
    BadBufferSize = encode_error(4),
    BufferTooSmall = encode_error(5),
    NotReady = encode_error(6),
    DeviceError = encode_error(7),
    WriteProtected = encode_error(8),
    OutOfResources = encode_error(9),
    VolumeCorrupted = encode_error(10),
    VolumeFull = encode_error(11),
    NoMedia = encode_error(12),
    MediaChanged = encode_error(13),
    NotFound = encode_error(14),
    AccessDenied = encode_error(15),
    NoResponse = encode_error(16),
    NoMapping = encode_error(17),
    Timeout = encode_error(18),
    NotStarted = encode_error(19),
    AlreadyStarted = encode_error(20),
    Aborted = encode_error(21),
    IcmpError = encode_error(22),
    TftpError = encode_error(23),
    ProtocolError = encode_error(24),
    IncompatibleVersion = encode_error(25),
    SecurityViolation = encode_error(26),
    CrcError = encode_error(27),
    EndOfMedia = encode_error(28),
    EndOfFile = encode_error(31),
    InvalidLanguage = encode_error(32),
    CompromisedData = encode_error(33),
    HttpError = encode_error(35),
    WarnUnknownGlyph = encode_warning(1),
    WarnDeleteFailure = encode_warning(2),
    WarnWriteFailure = encode_warning(3),
    WarnBufferTooSmall = encode_warning(4),
    WarnStaleData = encode_warning(5),
    WarnFileSystem = encode_warning(6),
}
