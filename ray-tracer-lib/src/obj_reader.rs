use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use crate::core::math::Float;
use crate::core::tuple::{point, Tuple};
use crate::shapes::triangle::Triangle;
use crate::shapes::triangle_model::TriangleModel;

pub struct ObjReader<T> {
    pub source : T,
    pub vertices : Vec<Tuple>,
    pub triangles : Vec<Triangle>,
    pub models: HashMap<String, TriangleModel>,
}

impl<T> ObjReader<T> where T: std::io::Read {
    pub fn new(source : T) -> Self  { Self{source, vertices: vec![], triangles: vec![], models: HashMap::new() } }

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
                            let triangle = Triangle::new(*v1, *v2, *v3);
                            self.triangles.push(triangle);
                        }
                    }
                    if line.starts_with("g ") {
                        let items: Vec<&str> = line.split(' ').collect();
                        let name = String::from(items[1]);
                        let triangle_model = TriangleModel::new(self.triangles.clone());
                        self.models.insert(current_name.to_string(), triangle_model);

                        current_name = name;
                        self.triangles = vec![];
                    }
                }
            }
        }
        if ! self.models.is_empty() {
            let triangle_model = TriangleModel::new(self.triangles.clone());
            self.models.insert(current_name.to_string(), triangle_model);
            self.triangles = vec![];
        }
    }
}