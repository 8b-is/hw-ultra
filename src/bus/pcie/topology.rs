pub struct Topology {
    pub links: [Option<crate::bus::pcie::Link>; 8],
}

impl Default for Topology {
    fn default() -> Self {
        Self::new()
    }
}

impl Topology {
    pub fn new() -> Self {
        Topology {
            links: [None, None, None, None, None, None, None, None],
        }
    }

    pub fn add_link(&mut self, idx: usize, link: crate::bus::pcie::Link) -> bool {
        if idx >= self.links.len() {
            return false;
        }
        self.links[idx] = Some(link);
        true
    }

    pub fn get_link(&self, idx: usize) -> Option<&crate::bus::pcie::Link> {
        if idx >= self.links.len() {
            return None;
        }
        self.links[idx].as_ref()
    }
}
