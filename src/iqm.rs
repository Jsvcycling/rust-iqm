extern crate byteorder;

use std::io::{Cursor, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use glium::Display;
use glium::index::{IndexBuffer, IndexBufferAny, PrimitiveType};
use glium::vertex::{VertexBuffer, VertexBufferAny};

#[allow(dead_code)]
enum IQMVertexArrayType {
    Position = 0,
    TexCoord,
    Normal,
    Tangent,
    BlendIndex,
    BlendWeight,
    Color,
}

#[allow(dead_code)]
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
const MAGIC: &'static [u8; 16] = b"INTERQUAKEMODEL\0";
const VERSION: u32 = 2;

// The size (in bytes) of some important IQM structs.
const SIZE_OF_VA_STRUCT: u32 = 20; // Vertex Array

pub struct Mesh {
    pub vertex_buffer: VertexBufferAny,
    pub index_buffer: IndexBufferAny,
}

#[allow(unused_variables)]
pub fn load_iqm(display: &Display, data: &[u8]) -> Vec<Mesh> {
    #[derive(Clone, Copy)]
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

    let mut magic: [u8; 16] = [0; 16];
    cursor.read(&mut magic[..]).unwrap();

    // Verify the type of the file.
    assert_eq!(MAGIC, &magic);
    assert_eq!(VERSION, cursor.read_u32::<LittleEndian>().unwrap());

    // Skip some stuff we don't care about.
    cursor.seek(SeekFrom::Current(16)).unwrap();

    // Read some more header values.
    let num_meshes = cursor.read_u32::<LittleEndian>().unwrap();
    let ofs_meshes = cursor.read_u32::<LittleEndian>().unwrap();
    let num_vertex_arrays = cursor.read_u32::<LittleEndian>().unwrap();
    let num_vertices = cursor.read_u32::<LittleEndian>().unwrap();
    let ofs_vertex_arrays = cursor.read_u32::<LittleEndian>().unwrap();
    let num_triangles = cursor.read_u32::<LittleEndian>().unwrap();
    let ofs_triangles = cursor.read_u32::<LittleEndian>().unwrap();

    // Create the vertex array vectors.
    let mut positions = Vec::new();
    let mut tex_coords = Vec::new();
    let mut normals = Vec::new();

    // Load the vertex arrays.
    for i in 0..num_vertex_arrays {
        cursor.seek(SeekFrom::Start((ofs_vertex_arrays + (i * SIZE_OF_VA_STRUCT)) as u64)).unwrap();

        let va_type = cursor.read_u32::<LittleEndian>().unwrap();
        let va_flags = cursor.read_u32::<LittleEndian>().unwrap();
        let va_format = cursor.read_u32::<LittleEndian>().unwrap();
        let va_size = cursor.read_u32::<LittleEndian>().unwrap();
        let va_offset = cursor.read_u32::<LittleEndian>().unwrap();

        match va_type {
            0 => {
                assert!(va_format == IQMVertexArrayFormat::Float as u32 && va_size == 3);
                cursor.seek(SeekFrom::Start(va_offset as u64)).unwrap();
                
                for j in 0..(num_vertices * va_size) {
                    positions.push(cursor.read_f32::<LittleEndian>().unwrap());
                }
            },
            1 => {
                assert!(va_format == IQMVertexArrayFormat::Float as u32 && va_size == 2);
                cursor.seek(SeekFrom::Start(va_offset as u64)).unwrap();

                for j in 0..(num_vertices * va_size) {
                    tex_coords.push(cursor.read_f32::<LittleEndian>().unwrap_or(0.0));
                }
            },
            2 => {
                assert!(va_format == IQMVertexArrayFormat::Float as u32 && va_size == 3);
                cursor.seek(SeekFrom::Start(va_offset as u64)).unwrap();

                for j in 0..(num_vertices * va_size) {
                    normals.push(cursor.read_f32::<LittleEndian>().unwrap_or(0.0));
                }
            },
            _ => {},
        }
    }

    let mut indices = Vec::new();
    
    // Load the indices.
    cursor.seek(SeekFrom::Start(ofs_triangles as u64)).unwrap();
    
    for i in 0..num_triangles {
        for j in 0..3 {
            indices.push(cursor.read_u32::<LittleEndian>().unwrap());
        }
    }

    // Make sure we've loaded the right number of indices.
    assert_eq!(num_triangles, (indices.len()/3) as u32);
    
    // Create our output vertex buffer vector.
    let mut meshes = Vec::new();

    // Load the IQM meshes and generate the associated vertex buffers.
    cursor.seek(SeekFrom::Start(ofs_meshes as u64)).unwrap();
    
    for i in 0..num_meshes {
        // Skip the name and material fields.
        cursor.seek(SeekFrom::Current(8)).unwrap();

        let first_vertex = cursor.read_u32::<LittleEndian>().unwrap();
        let num_vertices = cursor.read_u32::<LittleEndian>().unwrap();
        let first_triangle = cursor.read_u32::<LittleEndian>().unwrap();
        let num_triangles = cursor.read_u32::<LittleEndian>().unwrap();

        // Load the selected vertices.
        let mut vert_data = Vec::new();

        for j in 0..num_vertices {
            let id = first_vertex + j;
            
            vert_data.push(Vertex {
                position: [ positions[(3 * id) as usize],
                            positions[((3 * id) + 1) as usize],
                            positions[((3 * id) + 2) as usize] ],
                tex_coord: [ tex_coords[(2 * id) as usize],
                             tex_coords[((2 * id) + 1) as usize] ],
                normal: [ normals[(3 * id) as usize],
                          normals[((3 * id) + 1) as usize],
                          normals[((3 * id) + 2) as usize] ],
            });
        }

        // Load the selected indices.
        let mut idx_data = Vec::new();

        for j in 0..num_triangles {
            let id = first_triangle + j;

            idx_data.push(indices[(3 * id) as usize]);
            idx_data.push(indices[((3 * id) + 1) as usize]);
            idx_data.push(indices[((3 * id) + 2) as usize]);
        }

        // Create the buffers.
        let vert_buffer = VertexBuffer::new(display, &vert_data).unwrap().into_vertex_buffer_any();
        let idx_buffer = IndexBufferAny::from(IndexBuffer::new(display, PrimitiveType::TrianglesList, &idx_data).unwrap());
        
        meshes.push(Mesh {
            vertex_buffer: vert_buffer,
            index_buffer: idx_buffer,
        });
    }
    
    // Return the vertex buffers.
    meshes
}
