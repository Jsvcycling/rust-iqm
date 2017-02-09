extern crate byteorder;

use std::io::{Cursor, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use glium::Display;
use glium::vertex::{VertexBuffer, VertexBufferAny};

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
const SIZE_OF_VERTEX_ARRAY_STRUCT: u32 = 20;

fn load_iqm(display: &Display, data: &[u8]) -> Vec<VertexBufferAny> {
    struct Vertex {
        position: [f32; 3],
        tex_coord: [f32; 2],
        normal: [f32; 3],
    }

    implement_vertex!(Vertex,
                      position,
                      tex_coord,
                      normal);
    
    let mut cursor = Cursor::new(data);

    let mut magic = [u8; 16];
    cursor.read(&mut magic[..])?;

    // Verify the type of the file.
    assert_eq!(MAGIC, magic);
    assert_eq!(VERSION, cursor.read_u32::<LittleEndian>()?);

    // Skip some stuff we don't care about.
    cursor.seek(SeekFrom::Current(16))?;

    // Read some more header values.
    let num_meshes = cursor.read_u32::<LittleEndian>()?;
    let ofs_meshes = cursor.read_u32::<LittleEndian>()?;
    let num_vertex_arrays = cursor.read_u32::<LittleEndian>()?;
    let num_vertices = cursor.read_u32::<LittleEndian>()?;
    let ofs_vertex_arrays = cursor.read_u32::<LittleEndian>()?;

    // Create the vertex array vectors.
    let mut positions: Vec<f32> = Vec::with_capacity(num_vertices * 3);
    let mut tex_coords: Vec<f32> = Vec::with_capacity(num_vertices * 2);
    let mut normals: Vec<f32> = Vec::with_capacity(num_vertices * 3);

    // Load the vertex arrays.
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
                    tex_coords[j] = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
                }
            },
            IQMVertexArrayType::Normal => {
                assert_eq!(va_format == IQMVertexArrayFormat::Float && va_size == 3);
                cursor.seek(SeekFrom::Start(va_offset))?;

                for j in 0..(num_vertices * va_size) {
                    normals[j] = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
                }
            },
        }
    }
    
    // Create our output vertex buffer vector.
    let mut buffers: Vec<VertexBufferAny> = Vec::with_capacity(num_meshes);

    // Load the IQM meshes and generate the associated vertex buffers.
    cursor.seek(SeekFrom::Start(ofs_meshes))?;
    
    for i in 0..num_meshes {
        // Skip the name and material fields.
        cursor.seek(SeekFrom::Current(8))?;

        let first_vertex = cursor.read_u32::<LittleEndian>()?;
        let num_vertices = cursor.read_u32::<LittleEndian>()?;
        let first_triangle = cursor.read_u32::<LittleEndian>()?;
        let num_triangles = cursor.read_u32::<LittleEndian>()?;

        // Load the selected vertices
        let mut data: Vec<Vertex> = Vec::with_capacity(num_vertices);

        for j in 0..num_vertices {
            let id = first_vertex + j;
            
            data[j] = Vertex {
                position: [ positions[(3 * id)],
                            positions[(3 * id) + 1],
                            positions[(3 * id) + 2] ],
                tex_coord: [ tex_coords[(2 * id)],
                             tex_coords[(2 * id) + 1] ],
                normal: [ normals[(3 * id)],
                          normals[(3 * id) + 1],
                          normals[(3 * id) + 2] ],
            };
        }
        
        buffers[i] = VertexBuffer::new(display, &data)?.into_vertex_buffer_any();
    }
    
    // Return the vertex buffers.
    buffers
}
