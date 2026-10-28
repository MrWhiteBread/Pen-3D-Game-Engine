use std::io::Read;
use std::mem;
use half::f16;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::mesh::Mesh;

struct Offsets {
    vertices_offset: usize,

    normals_offset: usize,
    uvs_offset: usize,
}

impl Offsets {
    fn new() -> Self {
        Self {
            vertices_offset: 1,

            normals_offset: 1,
            uvs_offset: 1,
        }
    }
}


struct ImportData {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,

    radius: f32,

    offsets: Offsets,
}


impl ImportData {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),

            normals: Vec::new(),
            uvs: Vec::new(),

            radius: 0.0,

            offsets: Offsets::new(),
        }
    }
}

pub fn import_mesh(path: &str) -> Vec<Mesh> {
    let mut meshes: Vec<Mesh> = Vec::new();

    let mut import_data = ImportData::new();

    fn add_mesh(data: &mut ImportData, meshes: &mut Vec<Mesh>) {
        if data.vertices.is_empty() { return }

            data.offsets.vertices_offset += data.vertices.len();

        data.offsets.normals_offset += data.normals.len();
        data.offsets.uvs_offset += data.uvs.len();

        meshes.push(Mesh::new(mem::take(&mut data.vertices), mem::take(&mut data.indices), data.radius));

        data.normals.clear();
        data.uvs.clear();
    };

    let mut file = std::fs::File::open(path).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.is_empty() { continue }

        match words[0] {
            "o" => {
                add_mesh(&mut import_data, &mut meshes);
            }

            "v" => {
                let position = [
                    words[1].parse::<f32>().expect("v1 no no???"),
                    words[2].parse::<f32>().expect("v2 no no???"),
                    words[3].parse::<f32>().expect("v3 no no???")
                ];

                let vertex = Vertex {
                    position,
                    normal: 0,
                    uv: [f16::ZERO; 2],
                };

                let new_radius = (position[0].powi(2) + position[1].powi(2) + position[2].powi(2)).sqrt();
                if new_radius > import_data.radius {
                    import_data.radius = new_radius;
                }

                import_data.vertices.push(vertex);
            }

            "vn" => {
                let normal = [
                    words[1].parse::<f32>().expect("vn1 no no???"),
                    words[2].parse::<f32>().expect("vn2 no no???"),
                    words[3].parse::<f32>().expect("vn3 no no???")
                ];

                import_data.normals.push(normal);
            }

            "vt" => {
                let u = words[1].parse::<f32>().expect("vt1 no no???");
                let v = words[2].parse::<f32>().expect("vt2 no no???");

                import_data.uvs.push([u, 1.0 - v]);
            }

            "f" => {
                let mut temp_indices = Vec::new();

                for word in words.iter().skip(1) {
                    let data: Vec<&str> = word.split('/').collect();
                    let index = data[0].parse::<u32>().expect("error parsing vertex index");
                    let vertex = &mut import_data.vertices[index as usize - import_data.offsets.vertices_offset];

                    temp_indices.push(index);

                    if let Some(uvs_index_str) = data.get(1) && let Ok(uvs_index) = uvs_index_str.parse::<usize>() {
                        vertex.uv = [f16::from_f32(import_data.uvs[uvs_index - import_data.offsets.uvs_offset][0]), f16::from_f32(import_data.uvs[uvs_index - import_data.offsets.uvs_offset][1])];
                    }

                    if let Some(normal_index_str) = data.get(2) && let Ok(normal_index) = normal_index_str.parse::<usize>() {
                        vertex.normal = normals_to_u32(import_data.normals[normal_index - import_data.offsets.normals_offset]);
                    }
                }

                import_data.indices.extend(triangulate(&temp_indices));
            }

            _ => {}
        }
    }

    add_mesh(&mut import_data, &mut meshes);

    meshes
}

pub fn normals_to_u32(array: [f32; 3]) -> u32 {
    let x = ((array[0] * 0.5 + 0.5) * 1023.0).round() as u32;
    let y = (((array[1] * 0.5 + 0.5) * 1023.0).round() as u32) << 10;
    let z = (((array[2] * 0.5 + 0.5) * 1023.0).round() as u32) << 20;

    x | y | z
}

#[allow(dead_code)]
fn triangulate(face: &Vec<u32>) -> Vec<u32> {
    let mut triangles = Vec::new();

    if face.len() == 3 {
        triangles.push(face[0]);
        triangles.push(face[1]);
        triangles.push(face[2]);
    } else {
        for i in 1..face.len() - 1 {
            triangles.push(face[0]);
            triangles.push(face[i]);
            triangles.push(face[i + 1]);
        }
    }

    triangles
}