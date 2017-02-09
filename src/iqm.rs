extern crate byteorder;

use std::io::{Cursor, SeekFrom};
use std::vec::Vec;

use byteorder::{LittleEndian, ReadBytesExt};
use glium::Display;
use glium::index::IndexBufferAny;
use glium::vertex::VertexBufferAny;
use glium::uniforms::UniformsStorage;

enum IQMVertexArrayType {
    Position = 0,
    TexCoord,
    Normal,
    Tangent,
    BlendIndex,
    BlendWeight,
    Color,
}

enum IQMVertexArrayFormat {
    Byte = 0,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Half,
    Float,
    Double,
}

// Some known constants.
const MAGIC: &[u8; 16] = b"INTERQUAKEMODEL\0";
const VERSION: u32 = 2;

// The size (in bytes) of some important IQM structs.
const SIZE_OF_MESH_STRUCT: u32 = 24;
const SIZE_OF_VERTEX_ARRAY_STRUCT: u32 = 20;

struct Mesh {
    vertex_buffer: VertexBufferAny,
    index_buffer: IndexBufferAny,
    uniform: UniformStorage,
}

fn load_iqm(display: &Display, data: &[u8]) -> Vec<Mesh> {
    struct Vertex {
        position: [f32; 3],
        tex_coord: [f32; 2],
        normal: [f32; 3],
        tangent: [f32; 4],
        blend_index: [u8; 4],
        blend_weight: [u8; 4],
        color: [u8; 4],
    }

    implement_vertex!(Vertex,
                      position,
                      tex_coord,
                      normal,
                      tangent,
                      blend_index,
                      blend_weight,
                      color);
    
    let mut cursor = Cursor::new(data);

    let mut magic = [u8; 16];
    cursor.read(&mut magic[..])?;

    assert_eq!(MAGIC, magic);
    assert_eq!(VERSION, cursor.read_u32::<LittleEndian>()?);

    // Skip some stuff we don't care about.
    cursor.seek(SeekFrom::Current(16))?;

    let num_meshes = cursor.read_u32::<LittleEndian>()?;
    let ofs_meshes = cursor.read_u32::<LittleEndian>()?;
    let num_vertex_arrays = cursor.read_u32::<LittleEndian>()?;
    let num_vertices = cursor.read_u32::<LittleEndian>()?;
    let ofs_vertex_arrays = cursor.read_u32::<LittleEndian>()?;
    let num_triangles = cursor.read_u32::<LittleEndian>()?;
    let ofs_triangles = cursor.read_u32::<LittleEndian>()?;

    // Create the mesh objects.
    let mut meshes: Vec<Mesh> = Vec::with_capacity(num_meshes);

    // Create the vertex array vectors.
    let mut positions: Vec<f32> = Vec::with_capacity(num_vertices * 3);
    let mut tex_coords: Vec<f32> = Vec::with_capacity(num_vertices * 2);
    let mut normals: Vec<f32> = Vec::with_capacity(num_vertices * 3);
    let mut tangents: Vec<f32> = Vec::with_capacity(num_vertices * 4);
    let mut blend_indices: Vec<u8> = Vec::with_capacity(num_vertices * 4);
    let mut blend_weights: Vec<u8> = Vec::with_capacity(num_vertices * 4);
    let mut colors: Vec<u8> = Vec::with_capacity(num_vertices * 4);

    // Load our vertex arrays
    for i in 0..num_vertex_arrays {
        cursor.seek(SeekFrom::Start(ofs_vertex_arrays + (i * SIZE_OF_VERTEX_ARRAY_STRUCT)))?;

        let va_type = cursor.read_u32::<LittleEndian>()?;
        let va_flags = cursor.read_u32::<LittleEndian>()?;
        let va_format = cursor.read_u32::<LittleEndian>()?;
        let va_size = cursor.read_u32::<LittleEndian>()?;
        let va_offset = cursor.read_u32::<LittleEndian>()?;

        match va_type {
            IQMVertexArrayType::Position => {
                assert_eq!(va_format == IQMVertexArrayFormat::Float && va_size == 3);
                cursor.seek(SeekFrom::Start(va_offset))?;
                
                for j in 0..(num_vertices * va_size) {
                    positions[j] = cursor.read_f32::<LittleEndian>()?;
                }
            },
            IQMVertexArrayType::TexCoord => {
                assert_eq!(va_format == IQMVertexArrayFormat::Float && va_size == 2);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    tex_coords[j] = cursor.read_f32::<LittleEndian>()?;
                }
            },
            IQMVertexArrayType::Normal => {
                assert_eq!(va_format == IQMVertexArrayFormat::Float && va_size == 3);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    normals[j] = cursor.read_f32::<LittleEndian>()?;
                }
            },
            IQMVertexArrayType::Tangent => {
                assert_eq!(va_format == IQMVertexArrayFormat::Float && va_size == 4);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    tangents[j] = cursor.read_f32::<LittleEndian>()?;
                }
            },
            IQMVertexArrayType::BlendIndex => {
                assert_eq!(va_format == IQMVertexArrayFormat::UByte && va_size == 4);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    blend_indices[j] = cursor.read_u8()?;
                }
            },
            IQMVertexArrayType::BlendWeight => {
                assert_eq!(va_format == IQMVertexArrayFormat::UByte && va_size == 4);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    blend_weights[j] = cursor.read_u8()?;
                }
            },
            IQMVertexArrayType::Color => {
                assert_eq!(va_format == IQMVertexArrayFormat::UByte && va_size = 4);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    colors[j] = cursor.read_u8()?;
                }
            },
        }
    }

    // Return our meshes
    meshes
}
