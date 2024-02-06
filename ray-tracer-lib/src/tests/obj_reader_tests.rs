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
        obj_reader.read();
        assert!(obj_reader.triangles.is_empty());
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

    #[test]
    fn triangulating_polygons_test() {
        let str =
            "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0
f 1 2 3 4 5
";
        let mut obj_reader = ObjReader::new(str.as_bytes());
        obj_reader.read();
        assert_eq!(obj_reader.triangles[0].p1, obj_reader.vertices[0]);
        assert_eq!(obj_reader.triangles[0].p2, obj_reader.vertices[1]);
        assert_eq!(obj_reader.triangles[0].p3, obj_reader.vertices[2]);
        assert_eq!(obj_reader.triangles[1].p1, obj_reader.vertices[0]);
        assert_eq!(obj_reader.triangles[1].p2, obj_reader.vertices[2]);
        assert_eq!(obj_reader.triangles[1].p3, obj_reader.vertices[3]);
        assert_eq!(obj_reader.triangles[2].p1, obj_reader.vertices[0]);
        assert_eq!(obj_reader.triangles[2].p2, obj_reader.vertices[3]);
        assert_eq!(obj_reader.triangles[2].p3, obj_reader.vertices[4]);
    }

    #[test]
    fn triangles_in_groups_test() {
        let str =
            "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";
        let mut obj_reader = ObjReader::new(str.as_bytes());
        obj_reader.read();

        assert!(obj_reader.models.contains_key("FirstGroup"));
        assert!(obj_reader.models.contains_key("SecondGroup"));
        let t1 = obj_reader.models["FirstGroup"].triangles[0];
        let t2 = obj_reader.models["SecondGroup"].triangles[0];
        assert_eq!(t1.p1, obj_reader.vertices[0]);
        assert_eq!(t1.p2, obj_reader.vertices[1]);
        assert_eq!(t1.p3, obj_reader.vertices[2]);
        assert_eq!(t2.p1, obj_reader.vertices[0]);
        assert_eq!(t2.p2, obj_reader.vertices[2]);
        assert_eq!(t2.p3, obj_reader.vertices[3]);
    }
}