use distances::sets::jaccard;
use distances::sets::kulsinski;

/// Generates a length 1000 set with each bit flipped randomly on or off
fn gen_set() -> Vec<u16> {
    let mut vec = Vec::new();
    for i in 0..1000 {
        if rand::random() {
            vec.push(i);
        }
    }
    vec
}

/// Random exhaustive testing of set distances, manually creating union and intersection values
#[test]
fn sets_test() {
    for _ in 0..10000 {
        let x: Vec<u16> = gen_set();
        let y: Vec<u16> = gen_set();
        let mut union: f32 = 0.0;
        let mut intersection: f32 = 0.0;
        for i in 0_u16..1000 {
            if x.contains(&i) || y.contains(&i) {
                union += 1.0;
            }
            if x.contains(&i) && y.contains(&i) {
                intersection += 1.0;
            }
        }
        let mut distance: f32;
        let mut real_distance: f32;

        distance = jaccard(&x, &y);
        if union == 0.0 {
            real_distance = 0.0;
        } else {
            real_distance = 1_f32 - intersection / union;
        }
        assert!((distance - real_distance).abs() < f32::EPSILON);

        distance = kulsinski(&x, &y);
        if union == 0.0 {
            real_distance = 0.0;
        } else {
            real_distance = 1_f32 - intersection / (union + union - intersection);
        }
        assert!((distance - real_distance).abs() < f32::EPSILON);
    }
}