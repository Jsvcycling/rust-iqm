extern crate byteorder;

use std::io::{Cursor, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use glium::Display;
use glium::vertex::VertexBufferAny;

// Some known constants.
const MAGIC: &[u8; 16] = b"INTERQUAKEMODEL\0";
const VERSION: u32 = 2;

// The sizes of some important structs in bytes.
const IQM_MESH_STRUCT_BYTES: u32 = 24;
const IQM_VERTEX_ARRAY_STRUCT_BYTES: u32 = 20;
const IQM_TRIANGLE_STRUCT_BYTES: u32 = 12;
const IQM_ADJACENCY_STRUCT_BYTES: u32 = 12;

fn load_iqm(display: &Display, data: &[u8]) -> VertexBufferAny {
    // TODO: read the IQM file header
    let mut cursor = Cursor::new(data);

    let mut magic = [u8; 16];
    cursor.read(&mut magic[..])?;

    assert_eq!(MAGIC, magic);
    assert_eq!(VERSION, cursor.read_u32::<LittleEndian>()?);

    // Skip some stuff we don't care about
    try!(cursor.seek(SeekFrom::Current(16)));

    let num_meshes = cursor.read_u32::<LittleEndian>()?;
    let ofs_meshes = cursor.read_u32::<LittleEndian>()?;
    let num_vertex_arrays = cursor.read_u32::<LittleEndian>()?;
    let num_vertices = cursor.read_u32::<LittleEndian>()?;
    let ofs_vertex_arrays = cursor.read_u32::<LittleEndian>()?;
    let num_triangles = cursor.read_u32::<LittleEndian>()?;
    let ofs_triangles = cursor.read_u32::<LittleEndian>()?;
    let ofs_adjacencies = cursor.read_u32::<LittleEndian>()?;

    // TODO: generate a vertex buffer
}
