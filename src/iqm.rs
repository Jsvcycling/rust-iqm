extern crate byteorder;

use std::io::{Cursor, SeekFrom};
use std::vec::Vec;

use byteorder::{LittleEndian, ReadBytesExt};
use glium::Display;
use glium::index::IndexBufferAny;
use glium::vertex::VertexBufferAny;
use glium::uniforms::UniformsStorage;

// Some known constants.
const MAGIC: &[u8; 16] = b"INTERQUAKEMODEL\0";
const VERSION: u32 = 2;

const SIZE_OF_MESHS_STRUCT: u32 = 24;

struct Mesh {
    vertex_buffer: VertexBufferAny,
    index_buffer: IndexBufferAny,
    uniform: UniformStorage,
}

fn load_iqm(display: &Display, data: &[u8]) -> Vec<Mesh> {
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        tangent: [f32; 4],
        texcoord: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, tangent, texcoord);
    
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
    let num_vertexes = cursor.read_u32::<LittleEndian>()?;
    let ofs_vertex_arrays = cursor.read_u32::<LittleEndian>()?;
    let num_triangles = cursor.read_u32::<LittleEndian>()?;
    let ofs_triangles = cursor.read_u32::<LittleEndian>()?;

    // Create all the buffers
    let mut meshes: Vec<Mesh> = Vec::with_capacity(num_meshes);

    // Load all the mesh structs.
    try!(cursor.seek(SeekFrom::Start(ofs_meshes)));

    for i in 0..num_meshes {
        // Go to the current mesh and skip the first 2 fields.
        try!(cursor.seek(SeekFrom::Start(ofs_meshes + (i * SIZE_OF_MESH_STRUCT) + 8)));
        
        let first_vertex = cursor.read_u32<LittleEndian>()?;
        let num_vertexes = cursor.read_u32<LittleEndian>()?;
        let first_triangle = cursor.read_u32<LittleEndian>()?;
        let num_triangles = cursor.read_u32<LittleEndian>()?;

        // TODO
    }

    for i in 0..num_triangles {
        try!(cursor.seek(SeekFrom::Start(ofs_triangles + (i * SIZE_OF_TRIANGLE_STRUCT))));

        
    }

    // Return our meshes
    meshes
}
