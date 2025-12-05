use nalgebra_glm::Quat;

pub fn quat_from_euler_deg(deg: [f32; 3]) -> Quat {
    let roll = deg[0].to_radians();
    let pitch = deg[1].to_radians();
    let yaw = deg[2].to_radians();

    nalgebra::UnitQuaternion::from_euler_angles(roll, pitch, yaw).into_inner()
}

pub fn euler_deg_from_quat(q: Quat) -> [f32; 3] {
    let uq = nalgebra::UnitQuaternion::new_normalize(q);
    let (roll, pitch, yaw) = uq.euler_angles(); // radians
    [roll.to_degrees(), pitch.to_degrees(), yaw.to_degrees()]
}