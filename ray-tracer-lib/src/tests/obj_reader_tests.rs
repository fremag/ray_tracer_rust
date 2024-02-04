mod tests {
    use crate::core::tuple::point;
    use crate::obj_reader::ObjReader;

    #[test]
    fn test() {
        // let file_path = r"C:\Users\Frédéric\RustroverProjects\rust_tracer\obj\teapot-low.obj";
        // let file = File::open(file_path).unwrap();

        let str = "v 1 2 3\nv 4 5 6\n";

        let mut obj_reader = ObjReader::new(str.as_bytes());
        obj_reader.read();
    }

    #[test]
    fn ignoring_unrecognized_lines_test() {
        let str = "
There was a young lady named Bright\n
who traveled much faster than light.\n
She set out one day\n
in a relative way,\n
and came back the previous night.\n
";
        let mut obj_reader = ObjReader::new(str.as_bytes());
        let group = obj_reader.read();
        assert_eq!(group.len(), 0);
    }

    #[test]
    fn vertex_records_test() {
        let str =
            "
v -1 1 0\n
v -1.0000 0.5000 0.0000\n
v 1 0 0\n
v 1 1 0\n
";
        let mut obj_reader = ObjReader::new(str.as_bytes());
        obj_reader.read();

        let v0 = obj_reader.vertices[0];

        assert_eq!(&v0, &point(-1.0, 1.0, 0.0));
        assert_eq!(&(obj_reader.vertices[1]), &point(-1.0, 0.5, 0.0));
        assert_eq!(&(obj_reader.vertices[2]), &point(1.0, 0.0, 0.0));
        assert_eq!(&(obj_reader.vertices[3]), &point(1.0, 1.0, 0.0));
    }


    #[test]
    fn parsing_triangle_faces_tests() {
        let str =
            "
v -1 1 0\n
v -1 0 0\n
v 1 0 0\n
v 1 1 0\n
f 1 2 3\n
f 1 3 4\n
";

        let mut obj_reader = ObjReader::new(str.as_bytes());
        obj_reader.read();
        assert_eq!(obj_reader.triangles[0].p1, obj_reader.vertices[0]);
        assert_eq!(obj_reader.triangles[0].p2, obj_reader.vertices[1]);
        assert_eq!(obj_reader.triangles[0].p3, obj_reader.vertices[2]);
        assert_eq!(obj_reader.triangles[1].p1, obj_reader.vertices[0]);
        assert_eq!(obj_reader.triangles[1].p2, obj_reader.vertices[2]);
        assert_eq!(obj_reader.triangles[1].p3, obj_reader.vertices[3]);
    }
}