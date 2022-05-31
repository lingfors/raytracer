mod aabb;
mod geometry;
mod material;
mod math;
mod random;

use geometry::*;
use material::*;
use math::*;

use std::rc::Rc;

struct HitRecord {
    geometry_hit_record: geometry::HitRecord,
    material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    fn new_default() -> HitRecord {
        return HitRecord {
            geometry_hit_record: geometry::HitRecord::new_default(),
            material: None,
        };
    }

    fn t(&self) -> f64 {
        return self.geometry_hit_record.t();
    }
}

struct Entity {
    geometry: Box<dyn Geometry>,
    material: Rc<dyn Material>,
}

impl Entity {
    fn new(geometry: Box<dyn Geometry>, material: Rc<dyn Material>) -> Entity {
        return Entity{ geometry, material };
    }

    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if self.geometry.intersect(r, t_min, t_max, &mut rec.geometry_hit_record) {
            rec.material = Some(self.material.clone());
            return true;
        }

        return false;
    }
}

struct World {
    objects: Vec<Entity>,
}

impl World {
    fn new() -> World {
        return World {
            objects: Vec::new()
        }
    }

    fn add(&mut self, object: Entity) {
        self.objects.push(object);
    }

    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.intersect(r, t_min, closest_so_far, &mut *rec) {
                hit_anything = true;
                closest_so_far = rec.t();
            }
        }

        return hit_anything;
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_unit();

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    let a = random::double_range(0.0, 2.0*std::f64::consts::PI);
    let z = random::double_range(-1.0, 1.0);
    let r = (1.0 - z*z).sqrt();

    return Vec3::new(r * a.cos(), r * a.sin(), z);
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random::double_range(-1.0, 1.0), random::double_range(-1.0, 1.0), 0.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn ray_color(r: &Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new_default();

    if world.intersect(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new_default();
        let mut attenuation = Color::new_default();

        match &rec.material {
            Some(material) => {
                if material.scatter(r, &rec.geometry_hit_record, &mut attenuation, &mut scattered)
                {
                    return attenuation * ray_color(&scattered, world, depth-1);
                }
        
                return Color::new(0.0, 0.0, 0.0);
            },
            None => panic!(),
        }
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64, time0: f64, time1: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(v_up, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset, random::double_range(self.time0, self.time1));
    }
}

fn random_scene() -> World {
    let mut world = World::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Entity::new(Box::new(Sphere::new_stationary(Vec3::new(0.0, -1000.0, -0.0), 1000.0)), ground_material));

    let large_radius = 1.0;

    let material1 = Rc::new(Dielectric::new(1.5));
    let position1 = Vec3::new(0.0, large_radius, 0.0);
    let position1_xz = Vec3::new(position1.x(), 0.0, position1.z());
    world.add(Entity::new(Box::new(Sphere::new_stationary(position1, large_radius)), material1));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let position2 = Vec3::new(-4.0, large_radius, 0.0);
    let position2_xz = Vec3::new(position2.x(), 0.0, position2.z());
    world.add(Entity::new(Box::new(Sphere::new_stationary(position2, large_radius)), material2));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let position3 = Vec3::new(4.0, large_radius, 0.0);
    let position3_xz = Vec3::new(position3.x(), 0.0, position3.z());
    world.add(Entity::new(Box::new(Sphere::new_stationary(position3, large_radius)), material3));

    let small_radius = 0.2;
    let min_distance = large_radius + small_radius;
    let min_distance_squared = min_distance * min_distance;

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::double_unit();
            let center0 = Vec3::new((a as f64) + random::double_range(0.0, 0.9), small_radius + random::double_range(0.0, 0.5), (b as f64) + random::double_range(0.0, 0.9));
            let center0_xz = Vec3::new(center0.x(), 0.0, center0.z());

            let reject = (position1_xz - center0_xz).length_squared() < min_distance_squared ||
                    (position2_xz - center0_xz).length_squared() < min_distance_squared ||
                    (position3_xz - center0_xz).length_squared() < min_distance_squared;

            if !reject {
                let center1 = center0 + Vec3::new(0.0, 9.8, 0.0);
                let sphere_material: Option<Rc<dyn Material>>;

                if choose_mat < 0.8 {
                    let albedo = Color::random_unit() * Color::random_unit();
                    sphere_material = Some(Rc::new(Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random::double_range(0.0, 0.5);
                    sphere_material = Some(Rc::new(Metal::new(albedo, fuzz)));
                } else {
                    sphere_material = Some(Rc::new(Dielectric::new(1.5)));
                }

                match sphere_material {
                    Some(material) => world.add(Entity::new(Box::new(Sphere::new(center0, center1, 0.2, 0.0, 1.0)), material)),
                    None => panic!(),
                }
            }
        }
    }

    return world;
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 2048;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let world = random_scene();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_at, v_up, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0 / 60.0);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + random::double_unit()) / ((image_width-1) as f64);
                let v = ((j as f64) + random::double_unit()) / ((image_height-1) as f64);
    
                pixel_color += ray_color(&cam.get_ray(u, v), &world, max_depth);
            }

            pixel_color /= samples_per_pixel as f64;

            println!("{}", pixel_color);
        }
    }

    eprintln!("\nDone.");
}
