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

struct IQMMesh {
    name: u32,
    material: u32,
    first_vertex: u32,
    num_vertices: u32,
    first_triangle: u32,
    num_triangles: u32,
}

struct IQMVertexArray {
    type: u32,
    flags: u32,
    format: u32,
    size: u32,
    offset: u32,
}

struct IQMTriangle {
    vertices: [u32; 3],
}

fn load_iqm(display: &Display, data: &[u8]) -> VertexBufferAny {
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

    // Create all the arrays
    let meshes: [mut IQMMesh; num_meshes];
    let vertex_arrays: [mut IQMVertexArray; num_vertex_arrays];
    let triangles: [mut IQMTriangle; num_triangles];
    let mut adjacencies: [u8; num_adjacencies * IQM_ADJACENCY_STRUCT_BYTES];

    // Load all the mesh structs.
    try!(cursor.seek(SeekFrom::Start(ofs_meshes)));

    for i..num_meshes {
        meshes[i].name = cursor.read_u32<LittleEndian>()?;
        meshes[i].material = cursor.read_u32<LittleEndian>()?;
        meshes[i].first_vertex = cursor.read_u32<LittleEndian>()?;
        meshes[i].num_vertices = cursor.read_u32<LittleEndian>()?;
        meshes[i].first_triangle = cursor.read_u32<LittleEndian>()?;
        meshes[i].num_triangles = cursor.read_u32<LittleEndian>()?;
    }

    // Load all the vertex array structs.
    try!(cursor.seek(SeekFrom::Start(ofs_vertex_arrays)));

    for i in 0..num_vertex_arrays {
        vertex_arrays[i].type = cursor.read_u32<LittleEndian>()?;
        vertex_arrays[i].flags = cursor.read_u32<LittleEndian>()?;
        vertex_arrays[i].format = cursor.read_u32<LittleEndian>()?;
        vertex_arrays[i].size = cursor.read_u32<LittleEndian>()?;
        vertex_arrays[i].offset = cursor.read_u32<LittleEndian>()?;
    }

    // load all the triangle structs.
    try!(cursor.seek(SeekFrom::Start(ofs_triangles)));

    for i in 0..num_triangles {
        for j in 0..3 {
            triangles[i].vertices[j] = cursor.read_u32<LittleEndian>()?;
        }
    }
}
