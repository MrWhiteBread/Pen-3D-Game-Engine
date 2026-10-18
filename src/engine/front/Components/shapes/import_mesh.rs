use std::io::Read;
use half::f16;
use indexmap::IndexMap;
use crate::engine::back::types::vertex::Vertex;
use crate::engine::front::components::mesh::Mesh;

#[allow(dead_code)]
pub fn import_mesh(path: &str) -> Vec<Mesh> {
    let mut meshes: Vec<Mesh> = Vec::new();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut radius = 0.0;

    let mut index_offset = 0;
    let mut normal_offset = 0;
    let mut uvs_offset = 0;

    let mut file = std::fs::File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() >= 1 {
            match words[0] {
                "o" => {
                    if vertices.len() > 0 {
                        println ! ("Found object: {}, with vertices len: {}", words[1], vertices.len());

                        index_offset += vertices.len();
                        normal_offset += normals.len();
                        uvs_offset += uvs.len();

                        let mesh = Mesh::new(vertices, indices, radius);
                        meshes.push(mesh);

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
                        normal: 0,
                        uv: [f16::ZERO; 2],
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
                        vertices[vertex_index].normal = normals_to_u32(normals[normal_index]);
                        vertices[vertex_index].uv = [f16::from_f32(uvs[uv_index][0]), f16::from_f32(uvs[uv_index][1])];
                    }

                    indices.extend(&ind); //works only for triangulated meshes so yeah i gotta update this
                }

                _ => {}
            }
        }
    }

    println!("Found an object, with vertices len: {}", vertices.len());
    let mesh = Mesh::new(vertices, indices, radius);
    meshes.push(mesh);

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