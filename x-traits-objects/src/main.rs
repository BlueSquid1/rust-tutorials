#[derive(Debug)]
struct Gun {
    range: u32,
}

#[derive(Debug)]
struct Grenade {
    timeDelay: f32,
}

#[derive(Debug)]
enum Weapons {
    GunType(Gun),
    GrenadeType(Grenade),
}

fn main() {
    // When you know all the types in a vector you can define a enum with all the types.
    // simular to a union in C++.
    let inventory = vec![
        Weapons::GunType(Gun { range: 100 }),
        Weapons::GrenadeType(Grenade { timeDelay: 2.5 }),
    ];

    // When you don't know all the types then you need to use a trait
    println!("{:?}", inventory);
}
