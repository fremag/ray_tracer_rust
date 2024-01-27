#[cfg(test)]
pub mod tests {
    use crate::colors::Color;
    use crate::core::intersections::Intersections;
    use crate::core::math::INFINITY;
    use crate::core::matrix::Matrix;
    use crate::core::transform::scaling;
    use crate::core::tuple::point;
    use crate::lights::point_light::PointLight;
    use crate::material::Material;
    use crate::object::{build_sphere, Object};
    use crate::object::ObjectType::ObjectGroup;
    use crate::patterns::pattern::Pattern;
    use crate::shapes::cone::Cone;
    use crate::shapes::cylinder::Cylinder;
    use crate::shapes::group::Group;
    use crate::world::World;

    pub fn build_world() -> World {
        let mut sphere_1 = build_sphere();
        sphere_1.set_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            ambient: 0.1,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: Pattern::new()
        });

        let mut sphere_2 = build_sphere();
        sphere_2.set_transformation(scaling(0.5, 0.5, 0.5));

        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        World { objects: vec!(sphere_1, sphere_2), lights: vec!(light) }
    }

    impl<'a> Intersections<'a> {
        pub fn count(&self) -> usize {
            self.intersections.len()
        }
    }

    impl Cone {
        pub(crate) fn new() -> Self {
            Cone { min: -INFINITY, max: INFINITY, closed: false }
        }
    }

    impl Cylinder {
        pub(crate) fn new() -> Self {
            Cylinder {min: -INFINITY, max: INFINITY, closed: false}
        }
    }

    impl Group {

        pub fn len(&self) -> usize {
            self.children.len()
        }

        pub(crate) fn child(&self, p0: usize) -> &Object {
            &self.children[p0]
        }
        pub fn from(objects: Vec<Object>, transformation: Matrix<4>) -> Object {
            let mut group = Group::new();
            group.set_transformation(transformation);
            objects.iter().for_each( |a|  { group.add(a.clone())});
            Object::new_group(group)
        }
    }

    pub fn build_glass_sphere() -> Object {
        let mut sphere = build_sphere();
        let mut material = Material::new();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        sphere.set_material(material);
        sphere
    }
    impl Object {
        pub fn group(&self) -> Option<&Group> {
            match &self.object_type {
                ObjectGroup(group) => Some(&group),
                _ => None,
            }
        }
    }
}
