use std::collections::HashMap;
use spin_chain::SpinChain;
mod spin_chain;

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
pub fn calculate_sums_of_products(spin_chain_1: [i8;CHAIN_SIZE], spin_chain_2: [i8;CHAIN_SIZE]) -> i128 {
    let mut spin_vector:Vec<i128> = Vec::new();
    let mut spin_1:i128;
    let mut spin_2:i128;

    for i in 0..CHAIN_SIZE {
        spin_1 = spin_chain_1[i].try_into().expect("could not convert i8 to i128 spin_1");
        spin_2 = spin_chain_2[i].try_into().expect("could not convert i8 to i128 spin_1");
        spin_vector[i] = spin_1*spin_2;
    }

    let mut sum_of_products:i128 = 0;

    for spin in spin_vector {
        sum_of_products += spin;
    }

    sum_of_products
}