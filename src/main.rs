use rand::Rng;
use rand::prelude::ThreadRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

const CHAIN_SIZE:usize = 32;
fn main() {
    // These should be command line arguments
    let number_of_chains = 100;
    let number_of_generations = 1;
    let do_print_chains = false;
    let count_degen_chains = true;
    //

    let mut hash_chain_map: HashMap<u64, (u128, [i8;CHAIN_SIZE],[char;CHAIN_SIZE])> = HashMap::new();

    for _i in 1..=number_of_generations{
        let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();
        let mut is_chain_valid: bool = false;
        for _j in 0..number_of_chains {
            let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_empty();
    
            while !is_chain_valid {

                spin_chain = SpinChain::new();

                let mut _size = 0;

                is_chain_valid = verify_chain(&spin_chain, &mut hash_chain_map, count_degen_chains);

            }

            spin_chain_vec.push(spin_chain);
            is_chain_valid = false;
        }

        if do_print_chains {
           print_chains(&spin_chain_vec)
        }

        if _i == 1 {
            println!("SPIN CHAIN SPINS: ");
        }
    
        accumulate_spins_in_chain(&spin_chain_vec);

    }
    if count_degen_chains {
        print_degen_counts(&hash_chain_map);
    }
}


// Spin chain struct
pub struct SpinChain<const N: usize> {
    pub chain: [i8;N],
    pub chain_hash: u64,
    pub chain_bond_rep: [char; N]
}

/// Creates a Spin Chain based on "height above horizon".
//  Arrays:
//    Fixed Size: Arrays have a fixed size known at compile time.
//    Stack Allocation: Arrays are allocated on the stack, making them very fast for access and creation.
//    No Allocation Overhead: Since the size is known at compile time, there's no need for dynamic memory allocation, which reduces overhead.
//    Memory Access: Accessing elements in an array is very efficient because the compiler knows the size and can perform bounds checking efficiently.
impl<const N: usize> SpinChain<N> {

    pub fn new_empty() -> Self {
        let chain = [0;N];
        // let chain_bond_rep: [String; N] = 
        let chain_bond_rep: [char; N] = [' '; N];
        let chain_hash = 0;
        SpinChain { chain, chain_hash, chain_bond_rep }
    }

    pub fn new() -> Self {
        let mut hasher = DefaultHasher::new();
        let chain = SpinChain::construct_chain();
        let chain_bond_rep = SpinChain::construct_bond_rep(chain);
        chain.hash(&mut hasher);
        let chain_hash = hasher.finish();
        SpinChain { chain, chain_hash, chain_bond_rep }
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
            
            let is_up_spin = determine_next_spin(prob_up, 10002);

            if is_up_spin {
                chain[i] = 1;
                height += 1;
            } else {
                chain[i] = -1;
                height -= 1;
            }
        }
        chain
    }

    /// Temporary function that needs to be re-written
    fn construct_bond_rep(chain: [i8;N]) -> [char;N] {

        let mut chain_rep : [char;N] = [' '; N];

       for i in 0..chain.len() {
        if chain[i] == 1 {
            chain_rep[i] = '(';
        } else {
            chain_rep[i] = ')';
        }
       }

       chain_rep

    }

}


/// A function to verify that a chain is well formed. That is that the net spin across the chain is zero.
pub fn verify_chain(spin_chain: &SpinChain<CHAIN_SIZE>, hash_chain_map: &mut HashMap<u64, (u128,[i8;CHAIN_SIZE],[char;CHAIN_SIZE])>, count_degen_chains: bool) -> bool {

    let mut is_chain_valid = false;
    let mut spin_count = 0;
    let mut _size = 0;
    for spin in spin_chain.chain {
        _size += 1;
        spin_count = spin + spin_count; 
    }

    if spin_count == 0{
        is_chain_valid = true;
        if count_degen_chains {
            let collision_count = hash_chain_map.entry(spin_chain.chain_hash).or_insert((1, spin_chain.chain, spin_chain.chain_bond_rep));
            collision_count.0 += 1;
        }
    } else {
        println!("Invalid spin chain produced");
    }

    is_chain_valid

}

/// A function that will print the spins in a chain. (Probably not necessary since I can use {:?} formatter for arrays)
pub fn print_chains(spin_chain_vec: &Vec<SpinChain<CHAIN_SIZE>>) {

    println!("SPINS");
    for spin_chain in spin_chain_vec {
        for spin in spin_chain.chain {
            print!("{}, ",spin);
        }
        println!("");
        println!("____");
    }
    
}

/// A function that iterates through the hash_chain_map and prints the relevant information
/// for degenerate chain creation.
pub fn print_degen_counts(hash_chain_map: &HashMap<u64, (u128,[i8;CHAIN_SIZE],[char;CHAIN_SIZE])>) {
    println!("There were a total of {} unique chains generated", hash_chain_map.len());
    for key_value_pair in hash_chain_map {
        println!("Bond Rep:");
        for character in key_value_pair.1.2 {
            print!("{}", character)
        }
        println!("");
        println!("Hash: {} was generated {} times for chain {:?}", key_value_pair.0, key_value_pair.1.0, key_value_pair.1.1);
    }
}

/// A method that iterates through the collection of generated spin chains.
/// It sums spins at the same site in each chain to see what the "net" spin is.
/// Say the chain is of length 20 and this method returns that for index i there is a 20,
/// this means that every chain generated had an up spin at this position.
pub fn accumulate_spins_in_chain(spin_chain_vec: &Vec<SpinChain<CHAIN_SIZE>>) {
    let mut spin_accum_array = [0;CHAIN_SIZE];

    for i in 0..spin_chain_vec.len() {
        let spin_chain = spin_chain_vec[i].chain;
        for j in 0..spin_chain.len(){
            spin_accum_array[j] += spin_chain[j];
        }
    }
    println!("{:?}", spin_accum_array);
}

/// Calculates the probability of next spin being up. Based on Eq. 10
/// in arXiv:1805.00532
/// Pr(z_i+1 = up) = (h_i + 2)(N - i - h_i)/[2(h_i + 1)(N - i)]
fn calculate_next_spin_prob(length: u32, current_index: u32, height: u32) -> f64 {

    let numerator: f64 = ((height + 2) * (length - current_index - height)).try_into().expect("Could not turn u32 into f64 - numerator");

    let denominator: f64 = (2 * (height + 1) * (length - current_index)).try_into().expect("Could not turn u32 into f64 - denominator");

    let spin_up_prob = numerator/denominator;

    spin_up_prob
}

/// Determines the next spin via taking random samples from the unit line.
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

    return up_spin > down_spin;
}
