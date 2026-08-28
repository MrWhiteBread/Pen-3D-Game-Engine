use std::io::Read;
use indexmap::IndexMap;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::mesh::Mesh;

#[allow(dead_code)]
pub fn import_obj(path: &str, color: &[f32; 4], material: &[f32; 2]) -> IndexMap<u32, Mesh> {
    let mut meshes: IndexMap<u32, Mesh> = IndexMap::new();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut radius = 0.0;

    let mut index_offset = 0;
    let mut normal_offset = 0;
    let mut uvs_offset = 0;

    let mut counter = 0;

    let mut file = std::fs::File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() >= 1 {
            match words[0] {
                "o" => {
                    println ! ("Found object: {}", words[1]);

                    if vertices.len() > 0 {
                        index_offset += vertices.len();
                        normal_offset += normals.len();
                        uvs_offset += uvs.len();

                        let mesh = Mesh::new(vertices, indices, radius);
                        meshes.insert(counter, mesh);
                        counter += 1;

                        vertices = Vec::new();
                        indices = Vec::new();
                        normals = Vec::new();
                        radius = 0.0;
                    }
                }

                "v" => {
                    let position = [
                        words[1].parse::<f32>().expect("v1 no no???"),
                        words[2].parse::<f32>().expect("v2 no no???"),
                        words[3].parse::<f32>().expect("v3 no no???")
                    ];

                    let vertex = Vertex {
                        position,
                        old_position: position,
                        color: *color,
                        old_color: *color,
                        normal: [0.0, 0.0, 0.0],
                        old_normal: [0.0, 0.0, 0.0],
                        material: *material,
                        old_material: *material,
                        uv: [0.0, 0.0], 
                        old_uv: [0.0, 0.0],
                        skeleton: 0,
                    };

                    let new_radius = (position[0].powi(2) + position[1].powi(2) + position[2].powi(2)).sqrt();
                    if new_radius > radius {
                        radius = new_radius;
                    }

                    vertices.push(vertex);
                }

                "vn" => {
                    let normal = [
                        words[1].parse::<f32>().expect("vn1 no no???"),
                        words[2].parse::<f32>().expect("vn2 no no???"),
                        words[3].parse::<f32>().expect("vn3 no no???")
                    ];
                    normals.push(normal);
                }

                "vt" => {
                    let u = words[1].parse::<f32>().expect("vt1 no no???");
                    let v = words[2].parse::<f32>().expect("vt2 no no???");
                    uvs.push([u, 1.0 - v]);
                }

                "f" => {
                    let mut ind: Vec<u32> = Vec::new();

                    for word in words.iter().skip(1) {
                        let vertex_index = word.split("/").next().expect("no vi").parse::<usize>().expect("no parse") - 1 - index_offset;
                        let mut uv_index = word.split("/").skip(1).next().expect("no ui").parse::<usize>().expect("no parse");
                        let mut normal_index = word.split("/").skip(2).next().expect("no ni").parse::<usize>().expect("no parse");

                        if (normal_index as i32 - 1 - normal_offset as i32) < 0 {
                            normal_offset = 0;
                        }
                        normal_index -= 1;
                        normal_index -= normal_offset;

                        if (uv_index as i32 - 1 - uvs_offset as i32) < 0 {
                            uvs_offset = 0;
                        }
                        uv_index -= 1;
                        uv_index -= uvs_offset;


                        ind.push(vertex_index as u32);
                        vertices[vertex_index].normal = normals[normal_index];
                        vertices[vertex_index].old_normal = normals[normal_index];
                        vertices[vertex_index].uv = uvs[uv_index];
                        vertices[vertex_index].old_uv = uvs[uv_index];
                    }

                    indices.extend(triangulate(&ind));
                }

                _ => {}
            }
        }
    }

    let mesh = Mesh::new(vertices, indices, radius);
    meshes.insert(counter, mesh);

    meshes
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