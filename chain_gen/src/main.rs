use std::collections::HashMap;
use plotlib::{page::Page, repr::Plot, style::{PointMarker, PointStyle}, view::ContinuousView};
use rand::Rng;
use rand_mt::Mt64;
use spin_chain::SpinChain;
use rand::prelude::ThreadRng;
use file_utils::{RunData, Run};
mod spin_chain;
mod calculation_utils;
mod file_utils;

const CHAIN_SIZE:usize = 42;



fn main() {
    // These should be command line arguments
    let number_of_chains = 1000;
    let mut current_size = 8;
    let max_size = 32;
    //

    let run_chains = true;

    let calc_z = false;

    

    let mut run_data: RunData = file_utils::load_data("./the_runs.txt".to_string());

    if run_chains {
        let mut hash_chain_map: HashMap<u64, (u128, Vec<i8>,[char;CHAIN_SIZE])> = HashMap::new();
        let mut unique_spin_chains:  Vec<[char; CHAIN_SIZE]> = Vec::new();

        let mut excited_bond_map = HashMap::<i8,i8>::new();
        excited_bond_map.insert(0, 1);
        excited_bond_map.insert(1,0);
        excited_bond_map.insert(2,0);

        let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();

        while current_size <= max_size {
            for _j in 0..number_of_chains {
                // let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_empty();
                let mut is_unique = false;
                let mut is_alive = true;
                let spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_excited(&excited_bond_map, current_size);
                let mut spin_chain_rep = spin_chain.chain.clone();
                let spin_sector = spin_chain.spin_sector;
                // let mut chain = spin_chain.chain;
    
                // if !hash_chain_map.contains_key(&spin_chain.chain_hash) {
                //     println!("UNIQUE");
                //     hash_chain_map.insert(spin_chain.chain_hash, (1, spin_chain.chain.clone(), spin_chain.chain_bond_rep));
                //     is_unique = true;
                // }
    
                // if is_unique {
                //     println!("PUSHING CHAIN");
                //     unique_spin_chains.push(spin_chain.chain_bond_rep);
                //     println!("VEC LENGTH: {}", unique_spin_chains.len());
                // }
    
                spin_chain_vec.push(spin_chain);
    
                let mut step_count = 0;
                let mut rng_seed: ThreadRng = rand::thread_rng();
                let mut rng = Mt64::new(rng_seed.gen());
    
    
                while is_alive {
                    let random_index = rng.gen_range(0..current_size - 2);
                    is_alive = evolve_chain(&mut spin_chain_rep, random_index, current_size);
                    step_count += 1;
                }
    
                step_count -= 1;
    
                let run = Run{step_count};
    
                update_run_data(run, &mut run_data, spin_sector, current_size);
            }
            println!("completed spin chain of size {current_size}");
            current_size+=2;
        }

        file_utils::save_data("./the_runs.txt".to_string(), &run_data);
    }

    if calc_z {
      let runs_map = run_data.runs.get(&1).unwrap();
      let mut plot_data: Vec<(f64, f64)> = Vec::new();

      for chain_size in runs_map.keys() {
        let mut step_total: u128 = 0;
        let runs = runs_map.get(chain_size).unwrap();
        let total_entries = runs.len() as f64;
        for run in runs {
            step_total += run.step_count;
        }
        let  step_total_conversion= step_total as f64;
        let average_step_count = step_total_conversion/total_entries;
        // println!("average_step_count: {average_step_count}");
        let chain_size_conversion = *chain_size as f64;
        plot_data.push((chain_size_conversion, average_step_count));
      }
      let plot: Plot = Plot::new(plot_data).point_style(PointStyle::new().marker(PointMarker::Square).colour("#DD3355"));

      let v = ContinuousView::new().add(plot).x_range(0.0, 32.0).y_range(0.0, 10000.0);
      Page::single(&v).save("chains.svg").unwrap();
    }

    // for spin_chain in unique_spin_chains {
    //     for symbol in spin_chain{
    //         print!("{}", symbol);
    //     }
    //     println!();
        
    // }

}

fn update_run_data(run: Run, run_data: &mut RunData, spin_sector: usize, chain_size: usize) {
    let contains_spin_sector = run_data.runs.contains_key(&spin_sector);

    if contains_spin_sector {
        // println!("Contains spin sector");
        // println!("Chain size: {chain_size}");
        let contains_chainlength = run_data.runs.get_mut(&spin_sector).unwrap().contains_key(&chain_size);
        if contains_chainlength {
            // println!("Contains chain length");
            let data = run_data.runs.get_mut(&spin_sector).unwrap().get_mut(&chain_size).unwrap();
            data.push(run);
        } else {
            // println!("Does not contain chain length");
            let new_run_vec: Vec<Run> = vec![run];
            // let mut new_data = HashMap::new();
            // new_data.insert(chain_size, new_run_vec);
            run_data.runs.get_mut(&spin_sector).unwrap().insert(chain_size, new_run_vec);
        }
    } else {
        // println!("Does not Contain spin sector");
        let new_run_vec: Vec<Run> = vec![run];
        let mut new_data = HashMap::new();
        new_data.insert(chain_size, new_run_vec);
        run_data.runs.insert(spin_sector, new_data);
    }

}

/// A function to verify that a chain is well formed. That is that the net spin across the chain is zero.
pub fn verify_chain(spin_chain: &SpinChain<CHAIN_SIZE>, hash_chain_map: &mut HashMap<u64, (u128,Vec<i8>,[char;CHAIN_SIZE])>, count_degen_chains: bool) -> bool {

    let mut is_chain_valid = false;
    let mut spin_count = 0;
    let mut _size = 0;
    for spin in &spin_chain.chain {
        _size += 1;
        spin_count = *spin + spin_count; 
    }

    if spin_count == 0{
        is_chain_valid = true;
        // if count_degen_chains {
        //     let chain = spin_chain.chain.clone();
        //     let collision_count = hash_chain_map.entry(spin_chain.chain_hash).or_insert((1, chain, spin_chain.chain_bond_rep));
        //     collision_count.0 += 1;
        // }
    } else {
        // println!("Invalid spin chain produced");
    }

    is_chain_valid

}

/// A function that will print the spins in a chain. (Probably not necessary since I can use {:?} formatter for arrays)
pub fn print_chains(spin_chain_vec: &Vec<SpinChain<CHAIN_SIZE>>) {

    // println!("SPINS");
    for spin_chain in spin_chain_vec {
        for spin in &spin_chain.chain {
            print!("{}, ",*spin);
        }
        // println!("");
        // println!("____");
    }
    
}

/// A function that iterates through the hash_chain_map and prints the relevant information
/// for degenerate chain creation.
pub fn print_degen_counts(hash_chain_map: &HashMap<u64, (u128,[i8;CHAIN_SIZE],[char;CHAIN_SIZE])>) {
    // println!("There were a total of {} unique chains generated", hash_chain_map.len());
    for key_value_pair in hash_chain_map {
        // println!("Bond Rep:");
        for character in key_value_pair.1.2 {
            print!("{}", character)
        }
        // println!("");
        // println!("Hash: {} was generated {} times for chain {:?}", key_value_pair.0, key_value_pair.1.0, key_value_pair.1.1);
    }
}

/// A function that evolves the fredkin chain. It chooses the sites i, i+1, and i+2 and attempts to perform the fredkin swap.
/// * chain: the spin chain that is to be evolved
pub fn evolve_chain(chain: &mut Vec<i8>, random_index: usize, chain_size: usize) -> bool {

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
            if right_spin_index == (chain_size - 1) && (middle_spin == 2 || middle_spin ==1) {
                is_chain_alive = false;
            } else if right_spin_index != chain_size -1 {
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
        let spin_chain = &spin_chain_vec[i].chain;
        for j in 0..spin_chain.len(){
            spin_accum_array[j] += spin_chain[j];
        }
    }
    // println!("{:?}", spin_accum_array);
}