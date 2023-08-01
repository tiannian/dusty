pub struct DtyIRHeader {
    pub endian: u8,
    pub version: u8,
    pub width: u16,
    pub target: Vec<u8>,
    pub compiler: Vec<u8>,
    pub numbers: SegNumbers,
    pub init: u32,
}
