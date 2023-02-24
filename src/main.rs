use rand;
use rand::Rng;


const Boltzmann: f32 = 1.38e-23; 

#[derive(Debug)]
struct Particle<'a> {
    x : u32,
    y : u32,
    energy : f32,
    mass : u32,
    vel : f32,
    w : &'a World
}


#[derive(Debug)]
struct World {
    xM : u32,
    xm : u32,
    yM : u32,
    ym : u32,
    rng : rand::rngs::ThreadRng
}

impl World {
    fn new(xM:u32, yM : u32) -> World {
        let mut rng = rand::thread_rng(); 
        World { xM: xM, xm: 1, yM: yM, ym: 1, rng: rng }
    }
    fn igen(&mut self) -> u32 {
        self.rng.gen_range(1..=self.xM)
    }
    fn fgen(&mut self) -> f32 {
        self.rng.gen()
    }
}



impl Particle<'_>{
    fn new( w : &mut World, mass : u32, temp : u32) -> Particle {
        Particle{ w: w , x: w.igen(), y: w.igen(), energy: Boltzmann * temp as f32  , mass: mass, vel: w.fgen() }
    }


    fn recalculate_energy(&mut self) {
        self.energy = 0.5 * self.mass as f32 * self.vel.powi(2);
    }
    fn mover(&mut self) {
        self.x += self.w.igen();
        self.y += self.w.igen();
    }

}





fn main() {
    let mut w  = World::new(100, 100);
    let mut hydrogen_atom = Particle::new(&mut w, 1, 23);
    hydrogen_atom.recalculate_energy();
    println!("{:?}", hydrogen_atom);
    hydrogen_atom.mover();
    // hydrogen_atom.print_rand_n();
    println!("{:?}", hydrogen_atom);
}
