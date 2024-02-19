use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use crate::core::math::Float;
use crate::core::tuple::{point, Tuple, vector};
use crate::shapes::smooth_triangle::SmoothTriangle;
use crate::shapes::smooth_triangle_model::SmoothTriangleModel;
use crate::shapes::triangle::Triangle;
use crate::shapes::triangle_model::TriangleModel;

pub struct ObjReader<T> {
    pub source : T,
    pub vertices : Vec<Tuple>,
    pub normals : Vec<Tuple>,
    pub triangles : Vec<Triangle>,
    pub smooth_triangles : Vec<SmoothTriangle>,
    pub models: HashMap<String, TriangleModel>,
    pub smooth_models: HashMap<String, SmoothTriangleModel>,
}

impl<T> ObjReader<T> where T: std::io::Read {
    pub fn new(source : T) -> Self  {
        Self{source, vertices: vec![], normals: vec![],
            triangles: vec![], smooth_triangles: vec![],
            models: HashMap::new(), smooth_models: HashMap::new(),
        } }

    pub fn read(&mut self) {
        let reader = BufReader::new(&mut self.source);
        let mut current_name = String::from("Default");

        for line_file in reader.lines() {
            match line_file {
                Err(_) => {continue;}
                Ok(line) => {
                    if line.starts_with("v ") {
                        let items : Vec<&str> = line.split(' ').collect();
                        let x = items[1].parse::<Float>().unwrap();
                        let y = items[2].parse::<Float>().unwrap();
                        let z = items[3].parse::<Float>().unwrap();
                        self.vertices.push(point(x, y, z));
                    }
                    if line.starts_with("vn ") {
                        let items : Vec<&str> = line.split(' ').collect();
                        let x = items[1].parse::<Float>().unwrap();
                        let y = items[2].parse::<Float>().unwrap();
                        let z = items[3].parse::<Float>().unwrap();
                        self.normals.push(vector(x, y, z));
                    }
                    if line.starts_with("f ") {
                        let items: Vec<&str> = line.split(' ').filter(|item| ! item.is_empty()).collect();
                        for i in 1..items.len()-2 {
                            let s1 : Vec<&str> = items[1].split('/').collect();
                            let s2 : Vec<&str> = items[i+1].split('/').collect();
                            let s3 : Vec<&str> = items[i+2].split('/').collect();
                            let i1 = s1[0].parse::<usize>().unwrap() - 1;
                            let i2 = s2[0].parse::<usize>().unwrap() - 1;
                            let i3 = s3[0].parse::<usize>().unwrap() - 1;
                            let v1 = &self.vertices[i1];
                            let v2 = &self.vertices[i2];
                            let v3 = &self.vertices[i3];
                            if s1.len() == 3 {
                                let i_n1 = s1[2].parse::<usize>().unwrap() - 1;
                                let i_n2 = s2[2].parse::<usize>().unwrap() - 1;
                                let i_n3 = s3[2].parse::<usize>().unwrap() - 1;
                                let n1 = self.normals[i_n1];
                                let n2 = self.normals[i_n2];
                                let n3 = self.normals[i_n3];
                                let smooth_triangle = SmoothTriangle::new(*v1, *v2, *v3, n1, n2, n3);
                                self.smooth_triangles.push(smooth_triangle);
                            } else {
                                let triangle = Triangle::new(*v1, *v2, *v3);
                                self.triangles.push(triangle);
                            }
                        }
                    }
                    if line.starts_with("g ") {
                        let items: Vec<&str> = line.split(' ').collect();
                        let name = String::from(items[1]);
                        if self.triangles.len() > 0 {
                            let triangle_model = TriangleModel::new(self.triangles.clone());
                            self.models.insert(current_name.to_string(), triangle_model);
                            self.triangles = vec![];
                        } else {
                            let smooth_triangle_model = SmoothTriangleModel::new(self.smooth_triangles.clone());
                            self.smooth_models.insert(current_name.to_string(), smooth_triangle_model);
                            self.smooth_triangles = vec![];
                        }
                        current_name = name;
                    }
                }
            }
        }
        if self.triangles.len() > 0 {
            let triangle_model = TriangleModel::new(self.triangles.clone());
            self.models.insert(current_name.to_string(), triangle_model);
            self.triangles = vec![];
        } else {
            let smooth_triangle_model = SmoothTriangleModel::new(self.smooth_triangles.clone());
            self.smooth_models.insert(current_name.to_string(), smooth_triangle_model);
            self.smooth_triangles = vec![];
        }
    }
}