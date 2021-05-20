use crate::hittable::*;
use crate::materials::*;
use crate::texture::*;
use crate::*;
use std::sync::Arc;

fn random_scene() -> hittable_list::HittableList {
    let mut world = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::new_clr(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let mat_gnd: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new_txtr(&checker));
    world.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&mat_gnd),
    )));

    let mut spheres = hittable_list::HittableList {
        objects: Vec::with_capacity(484),
    };

    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;

            let choose_mat = random_double();
            let center = Point::new(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat_sphere: Arc<dyn Material + Sync + Send>;

                if choose_mat < 0.5 {
                    // difuse
                    let albedo = random() * random();
                    mat_sphere = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    if choose_mat < 0.25 {
                        spheres.add(Arc::new(moving_sphere::MovingSphere::new(
                            center, center2, 0.0, 1.0, 0.2, mat_sphere,
                        )));
                    } else {
                        spheres.add(Arc::new(sphere::Sphere::new(center, 0.2, mat_sphere)))
                    }
                } else if choose_mat < 0.75 {
                    // metal
                    let albedo = random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    mat_sphere = Arc::new(Metal::new(albedo, fuzz));
                    spheres.add(Arc::new(sphere::Sphere::new(
                        center,
                        0.2,
                        Arc::clone(&mat_sphere),
                    )));
                } else {
                    // glass
                    mat_sphere = Arc::new(Dialectric::new(1.5));
                    spheres.add(Arc::new(sphere::Sphere::new(
                        center,
                        0.2,
                        Arc::clone(&mat_sphere),
                    )));
                }
            }
        }
    }

    let mat1: Arc<dyn Material + Sync + Send> = Arc::new(Dialectric::new(1.5));
    world.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat1),
    )));

    let mat2: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(sphere::Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat2),
    )));

    let mat3: Arc<dyn Material + Sync + Send> =
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(sphere::Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&mat3),
    )));

    //let mut out = hittable_list::HittableList {objects: Vec::with_capacity(10)};
    world.add(Arc::new(bvh::BvhNode::new_l(&mut spheres, 0.0, 1.0)));

    world
}

fn two_spheres() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let checker: Arc<dyn Texture + Sync + Send> = Arc::new(CheckerTexture::new_clr(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_txtr(&checker)),
    )));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_txtr(&checker)),
    )));

    objects
}

fn two_perlin_spheres() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc(4.0));
    let pertext2: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new_sc(5.0));

    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_txtr(&pertext2)),
    )));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_txtr(&pertext)),
    )));

    objects
}

fn earth() -> hittable_list::HittableList {
    let earth_txtr: Arc<dyn Texture + Sync + Send> = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new_txtr(&earth_txtr));
    let globe = Arc::new(sphere::Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        2.0,
        Arc::clone(&earth_surface),
    ));

    hittable_list::HittableList::new(globe)
}

fn simple_light() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc(0.5));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_txtr(&pertext)),
    )));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc_clr(
        3.5,
        Color::new(1.0, 1.0, 1.0),
    ));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_txtr(&pertext)),
    )));

    let difflight: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    //let grnlight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(0.0, 12.0, 0.0)));
    objects.add(Arc::new(aarect::XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::clone(&difflight),
    )));
    //objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&difflight))));

    objects
}

fn cornell_box() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let red: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(aarect::YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    objects.add(Arc::new(aarect::YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    objects.add(Arc::new(aarect::XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light),
    )));
    objects.add(Arc::new(aarect::XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(aarect::XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(aarect::XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    let mut box1: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let red: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(aarect::YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    objects.add(Arc::new(aarect::YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    objects.add(Arc::new(aarect::XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::clone(&light),
    )));
    objects.add(Arc::new(aarect::XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(aarect::XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(aarect::XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    let mut box1: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable + Sync + Send> = Arc::new(boxes::Box::new(
        &Point::new(0.0, 0.0, 0.0),
        &Point::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(constant_medium::ConstantMedium::new(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(constant_medium::ConstantMedium::new(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    objects
}

fn final_scene() -> hittable_list::HittableList {
    let mut boxes1 = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };
    let ground: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(boxes::Box::new(
                &Point::new(x0, y0, z0),
                &Point::new(x1, y1, z1),
                Arc::clone(&ground),
            )));
        }
    }

    let mut objects =
        hittable_list::HittableList::new(Arc::new(bvh::BvhNode::new_l(&mut boxes1, 0.0, 1.0)));

    let light: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(aarect::XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::clone(&light),
    )));

    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let mov_sph_mat: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(moving_sphere::MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        Arc::clone(&mov_sph_mat),
    )));

    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dialectric::new(1.5)),
    )));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 10.0)),
    )));

    let mut boundary: Arc<dyn Hittable + Sync + Send> = Arc::new(sphere::Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dialectric::new(1.5)),
    ));
    objects.add(Arc::clone(&boundary));
    objects.add(Arc::new(constant_medium::ConstantMedium::new(
        Arc::clone(&boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(sphere::Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dialectric::new(1.5)),
    ));
    objects.add(Arc::new(constant_medium::ConstantMedium::new(
        Arc::clone(&boundary),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat: Arc<dyn Material + Sync + Send> = Arc::new(Lambertian::new_txtr(
        &(Arc::new(ImageTexture::new("earthmap.jpg")) as Arc<dyn Texture + Sync + Send>),
    ));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc(0.1));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_txtr(&pertext)),
    )));

    let mut boxes2 = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };
    let white: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(sphere::Sphere::new(
            random_range(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(bvh::BvhNode::new_l(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

fn glow_earth() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::new(),
    };

    let earth_txtr: Arc<dyn Texture + Sync + Send> = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Arc<dyn Material + Sync + Send> =
        Arc::new(DiffuseLight::new_txtr(earth_txtr));
    let globe = Arc::new(sphere::Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        2.0,
        Arc::clone(&earth_surface),
    ));
    objects.add(globe);

    let _wall: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    //objects.add(Arc::new(aarect::YZRect))

    objects
}

fn bg() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc(0.5));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_txtr(&pertext)),
    )));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc_clr(
        3.5,
        Color::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0) * 1.5,
    ));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 3.0, 0.0),
        2.0,
        Arc::new(DiffuseLight::new_txtr(pertext)),
    )));

    //let difflight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    //let grnlight: Arc<dyn Material  + Sync + Send> = Arc::new(DiffuseLight::new(Color::new(0.0, 12.0, 0.0)));
    //objects.add(Arc::new(aarect::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, Arc::clone(&difflight))));
    //objects.add(Arc::new(sphere::Sphere::new(Point::new(0.0, 7.0, 0.0), 2.0, Arc::clone(&difflight))));

    let wall: Arc<dyn Material + Sync + Send> =
        Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
    objects.add(Arc::new(aarect::YZRect::new(
        0.0, 6.0, -3.0, 3.0, -4.0, wall,
    )));

    objects
}

fn pandorba() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc_clr(
        0.5,
        Color::new(0.6078, 0.4627, 0.3255),
    ));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_txtr(&pertext)),
    )));
    //let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(DualMarbleNoiseTexture::new_sc_clr_weight(1.0, Color::new(0.0, 0.0, 1.0)*1.0, Color::new(1.0, 1.0, 1.0)*1.0, 1.5));
    let pertext: Arc<dyn Texture + Sync + Send> = Arc::new(FragmentNoiseTexture::new(
        3,
        Color::new(0.0, 0.6, 0.8) * 1.0,
        Color::new(0.8, 0.8, 1.0) * 1.0,
        -0.1,
        0.05,
        0.1,
        1.0,
        1.0,
        2.0,
    ));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 3.0, 0.0),
        2.0,
        Arc::new(DiffuseLight::new_txtr(pertext)),
    )));

    //let _fog: Arc<dyn Hittable + Sync + Send> = Arc::new(sphere::Sphere::new(Point::new(0.0, 3.0, 0.0), 10.0, Arc::new(Lambertian::new(Color::new_e()))));
    //objects.add(Arc::new(constant_medium::ConstantMedium::new(_fog, 0.1, Color::new(1.0, 1.0, 1.0))));

    objects
}

fn noises() -> hittable_list::HittableList {
    let mut objects = hittable_list::HittableList {
        objects: Vec::with_capacity(10),
    };

    let perlin: Arc<dyn Texture + Sync + Send> = Arc::new(NoiseTexture::new_sc(10.0));
    let turb: Arc<dyn Texture + Sync + Send> = Arc::new(TurbNoiseTexture::new_sc(10.0));
    let marble: Arc<dyn Texture + Sync + Send> = Arc::new(MarbleNoiseTexture::new_sc(10.0));

    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, -2.0, 0.0),
        1.0,
        Arc::new(Lambertian::new_txtr(&perlin)),
    )));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        1.0,
        Arc::new(Lambertian::new_txtr(&turb)),
    )));
    objects.add(Arc::new(sphere::Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        1.0,
        Arc::new(Lambertian::new_txtr(&marble)),
    )));

    objects
}

arg_enum! {
    #[derive(Debug, PartialEq)]
    pub enum Scene{
        RandomSpheres,
        TwoSpheres,
        PerlinSpheres,
        Earth,
        SimpleLight,
        CornellBox,
        CornellSmoke,
        FinalScene,
        Earth2,
        Background,
        Pandorba,
        Noises,
    }
}

pub struct SceneData {
    pub image_width: u32,
    pub background: Color,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Point,
    pub dist_to_focus: f64,
    pub vfov: f64,
    pub aperture: f64,
    pub sample_per_pixel: u32,
    pub aspect_ratio: f64,
}

/**
 * Returns the scene, along with scene defined options, if present
 *
 */
pub fn match_scene(scene: Scene, scene_dat: &mut SceneData) -> hittable_list::HittableList {
    //Create empty return types to modify in the match
    let world;

    //Fill world and scene_dat
    match scene {
        Scene::RandomSpheres => {
            world = random_scene();
            scene_dat.image_width = 1200;
            scene_dat.background = Color::new(0.70, 0.80, 1.00);
            scene_dat.lookfrom = Point::new(13.0, 2.0, 3.0);
            scene_dat.lookat = Point::new(0.0, 0.0, 0.0);
            scene_dat.vfov = 20.0;
            scene_dat.aperture = 0.1;
        }

        Scene::TwoSpheres => {
            world = two_spheres();
            scene_dat.background = Color::new(0.70, 0.80, 1.00);
            scene_dat.lookfrom = Point::new(13.0, 2.0, 3.0);
            scene_dat.lookat = Point::new(0.0, 0.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::PerlinSpheres => {
            world = two_perlin_spheres();
            scene_dat.background = Color::new(0.70, 0.80, 1.00);
            scene_dat.lookfrom = Point::new(13.0, 2.0, 3.0);
            scene_dat.lookat = Point::new(0.0, 0.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::Earth => {
            world = earth();
            scene_dat.background = Color::new(0.70, 0.80, 1.00);
            scene_dat.lookfrom = Point::new(13.0, 2.0, 3.0);
            scene_dat.lookat = Point::new(0.0, 0.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::SimpleLight => {
            world = simple_light();
            scene_dat.sample_per_pixel = 400;
            scene_dat.background = Color::new(0.0, 0.0, 0.0);
            scene_dat.lookfrom = Point::new(26.0, 3.0, 6.0);
            scene_dat.lookat = Point::new(0.0, 2.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::CornellBox => {
            world = cornell_box();
            scene_dat.aspect_ratio = 1.0;
            scene_dat.image_width = 600;
            scene_dat.sample_per_pixel = 200;
            scene_dat.background = Color::new(0.0, 0.0, 0.0);
            scene_dat.lookfrom = Point::new(278.0, 278.0, -800.0);
            scene_dat.lookat = Point::new(278.0, 278.0, 0.0);
            scene_dat.vfov = 40.0;
        }

        Scene::CornellSmoke => {
            world = cornell_smoke();
            scene_dat.aspect_ratio = 1.0;
            scene_dat.image_width = 600;
            scene_dat.sample_per_pixel = 200;
            scene_dat.lookfrom = Point::new(278.0, 278.0, -800.0);
            scene_dat.lookat = Point::new(278.0, 278.0, 0.0);
            scene_dat.vfov = 40.0;
        }

        Scene::FinalScene => {
            world = final_scene();
            scene_dat.aspect_ratio = 1.0;
            scene_dat.image_width = 800;
            scene_dat.sample_per_pixel = 10000;
            scene_dat.background = Color::new(0.0, 0.0, 0.0);
            scene_dat.lookfrom = Point::new(478.0, 278.0, -600.0);
            scene_dat.lookat = Point::new(278.0, 278.0, 0.0);
            scene_dat.vfov = 400.0;
        }

        Scene::Earth2 => {
            world = earth();
            scene_dat.background = Color::new(0.0, 0.0, 0.00);
            scene_dat.lookfrom = Point::new(13.0, 2.0, 3.0);
            scene_dat.lookat = Point::new(0.0, 0.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::Background => {
            world = bg();
            scene_dat.sample_per_pixel = 400;
            scene_dat.background = Color::new(0.0, 0.0, 0.0);
            scene_dat.lookfrom = Point::new(26.0, 3.0, 6.0);
            scene_dat.lookat = Point::new(0.0, 2.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::Pandorba => {
            world = pandorba();
            scene_dat.sample_per_pixel = 400;
            scene_dat.background = Color::new(1.0, 1.0, 1.0) * 0.01;
            scene_dat.lookfrom = Point::new(26.0, 3.0, 6.0);
            scene_dat.lookat = Point::new(0.0, 2.0, 0.0);
            scene_dat.vfov = 20.0;
        }

        Scene::Noises => {
            world = noises();
            scene_dat.background = Color::new(0.70, 0.80, 1.00);
            scene_dat.lookfrom = Point::new(13.0, 2.0, 3.0);
            scene_dat.lookat = Point::new(0.0, 0.0, 0.0);
            scene_dat.vfov = 20.0;
        }
    };

    world
}
