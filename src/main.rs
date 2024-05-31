use rand::Rng;
use rand::prelude::ThreadRng;
use std::collections::{HashSet,hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

const CHAIN_SIZE:usize = 32;
fn main() {

    // const CHAIN_SIZE:usize = 32;

    // Build chain with 1 for z-up and -1 for z-down


    let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();
    let mut chain_hash_set: HashSet<u64> = HashSet::new();
    let mut is_chain_valid = false;
    for _i in 0..20 {
        let mut spin_chain: SpinChain<32> = SpinChain::new_empty();

        while !is_chain_valid {

         spin_chain = SpinChain::new();

         // populate_spin_chain(&spin_chain);

            let mut spin_count = 0;
            let mut size = 0;

            for spin in spin_chain.chain {
                size += 1;
                spin_count = spin + spin_count; 
            }

    
            if spin_count == 0 && !chain_hash_set.contains(&spin_chain.chain_hash){
                is_chain_valid = true;
                chain_hash_set.insert(spin_chain.chain_hash);
                println!("Spin count {}", spin_count);
                println!("Size: {}", size);
                print!("{:?}", spin_chain.chain);
                println!("");
            } else {
                let collision = chain_hash_set.contains(&spin_chain.chain_hash);
                println!("Current Chain Hash: {}", spin_chain.chain_hash);
                println!("Hash Set: ");
                for hash in &chain_hash_set {
                    print!("{}, ", hash);
                }
                println!("");
                println!("Was there a collision?: {}", collision);
                println!("spin count: {}", spin_count);
                // panic!("INVALID INVALID INVALIDDDDDD!!!!!!")
            }
            

        }
        spin_chain_vec.push(spin_chain);
        is_chain_valid = false;
    }
    
    println!("SPINS");
    for spin_chain in spin_chain_vec {
        for spin in spin_chain.chain {
            print!("{},",spin);
        }
        println!("");
        println!("____");
    }
}

// Spin chain struct
pub struct SpinChain<const N: usize> {
    pub chain: [i8;N],
    pub chain_hash: u64
}

// Creates a Spin Chain based on height above horizon.
//  Arrays:
//    Fixed Size: Arrays have a fixed size known at compile time.
//    Stack Allocation: Arrays are allocated on the stack, making them very fast for access and creation.
//    No Allocation Overhead: Since the size is known at compile time, there's no need for dynamic memory allocation, which reduces overhead.
//    Memory Access: Accessing elements in an array is very efficient because the compiler knows the size and can perform bounds checking efficiently.
impl<const N: usize> SpinChain<N> {

    pub fn new_empty() -> Self {
        let chain = [0;N];
        let chain_hash = 0;
        SpinChain { chain, chain_hash }
    }

    pub fn new() -> Self {
        let mut hasher = DefaultHasher::new();
        let chain = SpinChain::construct_chain();
        chain.hash(&mut hasher);
        let chain_hash = hasher.finish();
        SpinChain { chain, chain_hash }
    }

    fn construct_chain() -> [i8;N] {
        let mut chain = [0; N];
        chain[0] = 1;
        chain[N-1] = -1;
        let chain_size: u32 = chain.len().try_into().expect("Could not turn usize into u32");
        let mut height :u32= 1;
        
        // "Starting" at i = 0 with an up spin -> height starts as 1;
        for i in 1..N - 1 {

            let current_index: u32 = i.try_into().expect("could not make index into u32");
            let prob_up = calculate_next_spin_prob(chain_size, current_index, height);
            println!("prob_up: {}", prob_up);
            
            let is_up_spin = determine_next_spin(prob_up, 10002);

            if is_up_spin {
                chain[i] = 1;
                height += 1;
            } else {
                chain[i] = -1;
                height -= 1;
            }
            println!("Height: {}", height);
        }
        chain
    }     
}
///
/// Pr(z_i+1 = up) = (h_i + 2)(N - i - h_i)/[2(h_i + 1)(N - i)]
/// 
fn calculate_next_spin_prob(length: u32, current_index: u32, height: u32) -> f64 {

    let numerator: f64 = ((height + 2) * (length - current_index - height)).try_into().expect("Could not turn u32 into f64 - numerator");

    let denominator: f64 = (2 * (height + 1) * (length - current_index)).try_into().expect("Could not turn u32 into f64 - denominator");

    let spin_up_prob = numerator/denominator;

    spin_up_prob
}

fn determine_next_spin(prob_up: f64, trials: u128) -> bool {

    let mut up_spin:u128 = 0;
    let mut down_spin:u128 = 0;
    let mut rng: ThreadRng = rand::thread_rng();

    for _i in 1..trials {

        let random_num:f64 = rng.gen();
        if random_num <= prob_up {
            up_spin += 1;
        } else {
            down_spin +=1;
        }
    }

    println!("up_spin: {}", up_spin);
    println!("down_spin: {}", down_spin);

    return up_spin > down_spin;

}
