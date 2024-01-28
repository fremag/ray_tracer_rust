use crate::core::math::{Float, rotation};
use crate::object::{build_cylinder, Object};
use crate::shapes::group::Group;
use crate::core::transform::{scaling, translation};
use crate::core::tuple::{point, Tuple, vector};
use crate::shapes::shape::Shape;
use crate::shapes::triangle::Triangle;

pub fn make_cylinder(p1: Tuple, p2: Tuple, radius: Float) -> Object {
    let v = p2 - p1;
    let vector_y = vector(0.0, 1.0, 0.0);
    let rot = rotation(vector_y, v.normalize());
    let scale = scaling(radius, v.magnitude(), radius);
    let translation = translation(p1.x, p1.y, p1.z);
    let mut cyl = build_cylinder(0.0, 1.0);
    cyl.set_transformation(&translation * &(&rot * &scale));
    cyl
}

pub struct Mesh {
    n: usize,
    m: usize,
    points: Vec<Vec<Tuple>>,
}

pub fn curve_sweep_mesh(n: usize, m: usize, path: fn(t: Float) -> Tuple, curve: fn(u: Float, v: Float) -> (Float, Float)) -> Mesh
{
    let vector_y = vector(0.0, 1.0, 0.0);
    let mut points = vec![vec![point(0.0, 0.0, 0.0); m]; n];
    for i in 0..n
    {
        let u = (i as Float) * 1.0 / (n as Float);
        let p0 = path(u);
        let p1 = path(u + 0.001);
        let tgt = p1 - p0;
        let rotation = rotation(vector_y, tgt.normalize());
        let vec1 = &mut points[i];

        for j in 0..m
        {
            let v = (j as Float) * 1.0 / (m as Float);
            let c = curve(u, v);
            let transform_point = &rotation * &vector(c.0, 0.0, c.1);
            let x = p0.x + transform_point.x;
            let y = p0.y + transform_point.y;
            let z = p0.z + transform_point.z;

            vec1[j] = point(x, y, z);
        }
    }
    Mesh { n, m, points }
}

pub fn build_mesh(mesh: &Mesh, r: Float, close_u: bool, close_v: bool) -> Object
{
    let mut group = Group::new();

    for i in 0..mesh.n
    {
        let mut sub_group = Group::new();
        let vec = &mesh.points[i];
        for j in 0..mesh.m-1
        {
            let p0 = vec[j];
            let p1 = vec[j + 1];
            let c1 = make_cylinder(p0, p1, r);
            sub_group.add(c1);
        }

        if close_v
        {
            let c1 = make_cylinder(vec[0], vec[mesh.m - 1], r);
            sub_group.add(c1);
        }
        let sub_group_obj = Object::new_group(sub_group);
        group.add(sub_group_obj);
    }

    for i in 0..mesh.m
    {
        let mut sub_group = Group::new();
        for j in 0..mesh.n - 1
        {
            let c1 = make_cylinder(mesh.points[j][i], mesh.points[j + 1][i], r);
            sub_group.add(c1);
        }

        if close_u
        {
            let c1 = make_cylinder(mesh.points[0][i], mesh.points[mesh.n - 1][i], r);
            sub_group.add(c1);
        }
        let sub_group_obj = Object::new_group(sub_group);
        group.add(sub_group_obj);
    }

    Object::new_group(group)
}

pub fn build_mesh_tri(mesh: &Mesh, close_u: bool, close_v: bool) -> Object
{
    let mut group = Group::new();
    let mut  m = mesh.m;
    let mut n = mesh.n;
    if !close_u
    {
        m -= 1;
    }

    if ! close_v
    {
        n -= 1;
    }

    for i in 0..n
    {
        let mut sub_group = Group::new();
        for j in 0..m
        {
            let  p0 = mesh.points[i][j];
            let next_i = (i + 1) % mesh.n;
            let next_j = (j + 1) % mesh.m;
            let p1 = mesh.points[next_i][next_j];
            let tri1 = Triangle::new(p0, p1, mesh.points[i][next_j]);
            let tri2 = Triangle::new(p0, p1, mesh.points[next_i][j]);
            let object = Object::new(Shape::Triangle(tri1));
            sub_group.add(object);
            let object1 = Object::new(Shape::Triangle(tri2));
            sub_group.add(object1);
        }

        group.add( Object::new_group(sub_group));
    }
    Object::new_group(group)
}
