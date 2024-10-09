use std::{collections::HashMap};
use serde::Serialize;
use rand::{Rng};
use rand_mt::Mt64;
use spin_chain::SpinChain;
use rand::prelude::ThreadRng;
mod spin_chain;
mod calculation_utils;

const CHAIN_SIZE:usize = 8;

#[derive(Serialize)]
struct Run {
    pub step_count: u128
}

#[derive(Serialize)]
struct RunData {
    pub runs: HashMap<u128, Vec<Run>>
}

fn main() {
    // These should be command line arguments
    let number_of_chains = 1;
    let do_print_chains = true;
    let count_degen_chains = true;
    let calculate_diffs = true;
    //

    // a map that keeps track of how many unique chains have been genearted, their bond representation, how many times
    // the chain has been generated. 
    //Key: hash of chain
    //tuple:         0                                 1                                 2
    //(# of times generated, spin chain rep: [1,1,-1,1...,-1-1,-1], bond representation:[(,(,...,(,),),)] )
    let runs: HashMap<u128, Vec<Run>> = HashMap::new();
    let run_data = RunData{runs};
    let mut hash_chain_map: HashMap<u64, (u128, [i8;CHAIN_SIZE],[char;CHAIN_SIZE])> = HashMap::new();
    let mut unique_spin_chains:  Vec<[char; CHAIN_SIZE]> = Vec::new();

    let mut excited_bond_map = HashMap::<i8,i8>::new();
    excited_bond_map.insert(0, 1);
    excited_bond_map.insert(1,0);
    excited_bond_map.insert(2,0);

    let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();
    // let mut is_chain_valid: bool = false;
    for _j in 0..number_of_chains {
        // let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_empty();
        let mut is_unique = false;
        let mut is_alive = true;
        let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_excited(&excited_bond_map);
        let mut chain: [i8;CHAIN_SIZE] = spin_chain.chain;

        if !hash_chain_map.contains_key(&spin_chain.chain_hash) {
            println!("UNIQUE");
            hash_chain_map.insert(spin_chain.chain_hash, (1, spin_chain.chain, spin_chain.chain_bond_rep));
            is_unique = true;
        }

        if is_unique {
            println!("PUSHING CHAIN");
            unique_spin_chains.push(spin_chain.chain_bond_rep);
            println!("VEC LENGTH: {}", unique_spin_chains.len());
        }

        spin_chain_vec.push(spin_chain);

        let mut step_count = 0;
        let mut rng_seed: ThreadRng = rand::thread_rng();
        let mut rng = Mt64::new(rng_seed.gen());


        while is_alive {
            let random_index = rng.gen_range(0..CHAIN_SIZE - 2);
            is_alive = evolve_chain(&mut chain, random_index);
            step_count += 1;
        }

        run_data.runs.get(1)
    }
    if do_print_chains {
    //    print_chains(unique_spin_chains);
    }
    
        // accumulate_spins_in_chain(&spin_chain_vec);

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

/// A function that evolves the fredkin chain. It chooses the sites i, i+1, and i+2 and attempts to perform the fredkin swap.
/// * chain: the spin chain that is to be evolved
pub fn evolve_chain(chain: &mut [i8;CHAIN_SIZE], random_index: usize) -> bool {

    let mut is_chain_alive = true;
    let left_spin_index = random_index;
    let middle_spin_index = random_index + 1;
    let right_spin_index = random_index + 2;

    let left_spin = chain[left_spin_index];
    let middle_spin = chain[middle_spin_index];
    let right_spin = chain[right_spin_index];

    if left_spin == 1 || left_spin == 2 {
        let temp_value = chain[middle_spin_index];
        if (middle_spin == 1 || middle_spin == 2) && right_spin == -1 {
            if right_spin_index == (CHAIN_SIZE - 1) && (middle_spin == 2 || middle_spin ==1) {
                is_chain_alive = false;
            } else if right_spin_index != CHAIN_SIZE -1 {
                chain[middle_spin_index] = chain[right_spin_index];
                chain[right_spin_index] = temp_value;
            }
        } else if middle_spin == -1 && (right_spin == 1 || right_spin == 2){
            chain[middle_spin_index] = chain[right_spin_index];
            chain[right_spin_index] = temp_value;
        } else if (middle_spin == -1 && right_spin == -1) && left_spin_index != 0 {
            chain[middle_spin_index] = chain[left_spin_index];
            chain[left_spin_index] = temp_value;
        }
    } else {
        if (middle_spin == 1 || middle_spin == 2) && right_spin == -1 && left_spin_index != 0{
            let temp_value = chain[middle_spin_index];
            chain[middle_spin_index] = chain[left_spin_index];
            chain[left_spin_index] = temp_value;
        }
    }

    is_chain_alive

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