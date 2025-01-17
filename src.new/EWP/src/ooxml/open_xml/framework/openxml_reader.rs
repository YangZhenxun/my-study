pub trait AbsOpenXmlReader{
    fn close();
}

#[derive(Default, Debug, Clone, Copy)]
pub struct OpenXmlReader{
    _read_misc_nodes: bool,
}

impl OpenXmlReader {
    pub fn new() -> Self { 
        return OpenXmlReader{..Default::default()} 
    }
    pub fn new(read_misc_nodes: bool) -> Self {
        return OpenXmlReader{_read_misc_nodes: read_misc_nodes}
    }

}

impl Drop for OpenXmlReader{
    fn drop(&mut self, disposing: bool) {
        self.close();
    }
}