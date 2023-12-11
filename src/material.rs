    use crate::colors::Color;
use crate::light::Light;
use crate::math::{equals, Float};
use crate::tuple::Tuple;

pub struct Material {
    pub color : Color,
    pub ambient : Float,
    pub diffuse : Float,
    pub specular : Float,
    pub shininess : Float,
}

impl Material {
    pub(crate) fn lighting(&self, light : &Light, point : Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity();
        // find the direction to the light source
        let lightv = (light.position() - point).normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;
        let specular: Color;
        let diffuse : Color;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = lightv.dot(&normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::new(0.0, 0.0, 0.0);
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            // compute the diffuse contribution
            diffuse = effective_color * (self.diffuse * &light_dot_normal);
            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            //  light reflects away from the eye.
            let reflectv = (-lightv).reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
        }
        // Add the three contributions together to get the final shading
        return ambient + diffuse + specular
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color.eq(&other.color)
        && equals(self.ambient, other.ambient)
        && equals(self.diffuse, other.diffuse)
        && equals(self.specular, other.specular)
        && equals(self.shininess, other.shininess)
    }
}

pub fn material() -> Material {
    Material { color : Color {r: 1.0, g: 1.0, b: 1.0}, ambient : 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0}
}

