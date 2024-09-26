use std::collections::HashMap;
use spin_chain::SpinChain;
use calculation_utils::*;
use num_complex::Complex;
mod spin_chain;
mod calculation_utils;

const CHAIN_SIZE:usize = 8;

fn main() {
    // These should be command line arguments
    let number_of_chains = 1000;
    let number_of_generations = 1;
    let do_print_chains = true;
    let count_degen_chains = true;
    let calculate_diffs = true;
    //

    // a map that keeps track of how many unique chains have been genearted, their bond representation, how many times
    // the chain has been generated. 
    //Key: hash of chain
    //tuple:         0                                 1                                 2
    //(# of times generated, spin chain rep: [1,1,-1,1...,-1-1,-1], bond representation:[(,(,...,(,),),)] )
    let mut hash_chain_map: HashMap<u64, (u128, [i8;CHAIN_SIZE],[char;CHAIN_SIZE])> = HashMap::new();
    let mut unique_spin_chains:  Vec<[char; CHAIN_SIZE]> = Vec::new();

    // for _i in 1..=number_of_generations{
    //     let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();
    //     let mut is_chain_valid: bool = false;
    //     for _j in 0..number_of_chains {
    //         let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_empty();
    //         let mut is_unique = false;
    
    //         while !is_chain_valid {
    //             is_unique = false;

    //             spin_chain = SpinChain::new();

    //             let mut _size = 0;

    //             if !hash_chain_map.contains_key(&spin_chain.chain_hash) {
    //                 println!("UNIQUE");
    //                 is_unique = true;
    //             }

    //             is_chain_valid = verify_chain(&spin_chain, &mut hash_chain_map, count_degen_chains);

    //         }

    //         if is_unique {
    //             println!("PUSHING CHAIN");
    //             unique_spin_chains.push(spin_chain.chain);
    //             println!("VEC LENGTH: {}", unique_spin_chains.len());
    //         }

    //         spin_chain_vec.push(spin_chain);
    //         is_chain_valid = false;
    //     }

    let mut excited_bond_map = HashMap::<i8,i8>::new();
    excited_bond_map.insert(0, 1);
    excited_bond_map.insert(1,0);
    excited_bond_map.insert(2,0);


    for _i in 1..=number_of_generations{
        let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();
        // let mut is_chain_valid: bool = false;
        for _j in 0..number_of_chains {
            // let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_empty();
            let mut is_unique = false;

            let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_excited(&excited_bond_map);
    
            // while !is_chain_valid {
            //     is_unique = false;

            //     spin_chain = SpinChain::new();

            //     let mut _size = 0;

            if !hash_chain_map.contains_key(&spin_chain.chain_hash) {
                println!("UNIQUE");
                hash_chain_map.insert(spin_chain.chain_hash, (1, spin_chain.chain, spin_chain.chain_bond_rep));
                is_unique = true;
            }

            //     is_chain_valid = verify_chain(&spin_chain, &mut hash_chain_map, count_degen_chains);

            // }

            if is_unique {
                println!("PUSHING CHAIN");
                unique_spin_chains.push(spin_chain.chain_bond_rep);
                println!("VEC LENGTH: {}", unique_spin_chains.len());
            }

            spin_chain_vec.push(spin_chain);
            // is_chain_valid = false;
        }

        if do_print_chains {
        //    print_chains(unique_spin_chains);
        }

        if _i == 1 {
            println!("SPIN CHAIN SPINS: ");
        }
    
        // accumulate_spins_in_chain(&spin_chain_vec);

    }
    if count_degen_chains {
        print_degen_counts(&hash_chain_map);
    }

    if do_print_chains {
        print_unique_chains(unique_spin_chains);
    }

    // if calculate_diffs {
    //     for i in 0..unique_spin_chains.len() {
    //         let spin_chain_1 = unique_spin_chains.get(i).unwrap();
    //         for j in i+1..unique_spin_chains.len() - 1 {
    //             let spin_chain_2 = unique_spin_chains.get(j).unwrap();
    //             calculate_inner_product::<CHAIN_SIZE>(spin_chain_1, spin_chain_2);
    //         }
    //     }
        
    // }
}

fn print_unique_chains(chains: Vec<[char; CHAIN_SIZE]>) {

    println!("UNIQUE SPIN CHAINS:  ");
    println!("Number of unique chains: {}", chains.len());

    for chain in chains {
        let chain_string: String = chain.iter().collect();
        println!("{}", chain_string);
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

/// Function to build the Hamiltonian for a given Fredkin Spin Chain as described in
/// Eq. 6 in arXiv:1805.00532
pub fn build_hamiltonian(spin_chain: &[i8;CHAIN_SIZE]) {
    // H = sum i = 3 to N-2 F_i + P_2,3 + (D_2 D_3)/2 + P_N-2,N-1 + (U_N-2 U_N-1)/2
    // F_i = (U_i-1 P_i,i+1) + (P_i-1 D_i+1)

    for i in 3..spin_chain.len() - 1 {

        println!("{}",i);

    }


}

pub fn build_fredkin_op(site:usize) {

    build_projection_matrix(site);
    build_spin_up_matrix(site - 1);
    build_projection_matrix(site-1);
    build_spin_down_matrix(site + 1);

}

///P_i,i+1= 1/4 (I - sig_i dot sig_i+1) where sig = (sig_x, sig_y, sig_z)
pub fn build_projection_matrix(site:usize) {
    let next_site = site + 1;

}

///D_i = 1/2 (I - sig_i_z)
pub fn build_spin_down_matrix(site:usize) {

}

///U_i = 1/2 (I + sig_i_z)
pub fn build_spin_up_matrix(site:usize) {

}

/// square matrix multiplication AB
pub fn square_matrix_mul(A: Vec<Vec<i32>>, B:Vec<Vec<i32>>) {

}