extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt};
use glium::Display;
use glium::vertex::VertexBufferAny;

fn load_iqm(display: &Display, data: &[u8]) -> VertexBufferAny {
    // TODO
}
