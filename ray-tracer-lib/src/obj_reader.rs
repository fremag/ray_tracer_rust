use std::io::{BufRead, BufReader};
use crate::core::math::Float;
use crate::core::tuple::{point, Tuple};
use crate::shapes::group::Group;
use crate::shapes::triangle::Triangle;

pub struct ObjReader<T> where T: std::io::Read {
    pub source : T,
    pub vertices : Vec<Tuple>,
    pub triangles : Vec<crate::shapes::triangle::Triangle>,
}

impl<T> ObjReader<T> where T: std::io::Read {
    pub fn new(source : T) -> Self  { Self{source, vertices: vec![], triangles: vec![] } }

    pub fn read(&mut self) -> Group {
        let reader = BufReader::new(&mut self.source);

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
                        let items: Vec<&str> = line.split(' ').collect();
                        let i1 = items[1].parse::<usize>().unwrap()-1;
                        let i2 = items[2].parse::<usize>().unwrap()-1;
                        let i3 = items[3].parse::<usize>().unwrap()-1;
                        let v1 = &self.vertices[i1];
                        let v2 = &self.vertices[i2];
                        let v3 = &self.vertices[i3];
                        let triangle = Triangle::new(*v1, *v2, *v3);
                        self.triangles.push(triangle);
                    }
                }
            }
        }

        let group = Group::new();
        group
    }
}