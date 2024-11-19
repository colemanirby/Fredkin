use std::collections::{BTreeMap, HashMap};
use std::env;
use plotlib::{page::Page, repr::Plot, style::{PointMarker, PointStyle}, view::ContinuousView};
use rand::Rng;
use rand_mt::Mt64;
use spin_chain::SpinChain;
use rand::prelude::ThreadRng;
use file_utils::{RunData, Run};
use log::{logger, LevelFilter};
use log::info;
mod spin_chain;
mod calculation_utils;
mod file_utils;
mod data_utils;

const CHAIN_SIZE:usize = 42;



fn main() {
    // These should be command line arguments
    let number_of_chains = 1000;
    // let max_size = 64;
    //

    let args: Vec<String> = env::args().collect();

    let run_chains: bool = args.get(1).unwrap().parse().unwrap();


    
    
    // let run_chains = false;

    let mut excited_bond_map:HashMap<usize, usize> = HashMap::<usize, usize>::new();
    excited_bond_map.insert(0, 1);
    excited_bond_map.insert(1,0);
    excited_bond_map.insert(2,0);

    let mut spin_sector = 0;
    simple_logging::log_to_file("fredkin_logs.log", LevelFilter::Info).unwrap();

    for entry in &excited_bond_map {
        spin_sector += entry.1;
    }

    if run_chains {
        

        info!("testing logging");
        let max_size: usize = args.get(2).unwrap().parse().unwrap();

        let spin_sector_max: usize = args.get(3).unwrap().parse().unwrap(); 

        println!("Running chains up to size {max_size} and spin sector up to {spin_sector_max}");

        let mut rng_seed: ThreadRng = rand::thread_rng();
        let mut rng = Mt64::new(rng_seed.gen());

        for current_spin_sector in 1..=spin_sector_max {
            let mut run_data: RunData = RunData::new();
            println!("spin sector: {current_spin_sector}");
            excited_bond_map.insert(0, current_spin_sector);

            // let mut spin_chain_vec: Vec<SpinChain<CHAIN_SIZE>> = Vec::new();
            let mut current_size= (2 * current_spin_sector) + 2;

            while current_size <= max_size {
                for _j in 0..number_of_chains {
                    let mut is_alive = true;
                    // info!("generating spin chain");
                    let mut spin_chain: SpinChain<CHAIN_SIZE> = SpinChain::new_excited(&excited_bond_map, current_size);
                
                    let mut step_count = 0;

                
                    while is_alive {
                        let random_index = rng.gen_range(0..current_size - 2);
                        is_alive = evolve_chain(&mut spin_chain.chain, random_index, current_size);
                        step_count += 1;
                    }

                    update_run_data(&mut run_data, current_size, step_count);
                    
                }
                println!("completed spin chain of size {current_size}");
                current_size+=2;
            }

            let mut directory_string = String::from("./data/runs/run_ss_");
            directory_string.push_str(current_spin_sector.to_string().as_str());
            directory_string.push_str(".json");

            file_utils::save_data(directory_string, &run_data);
            // let runs_map: &BTreeMap<usize, Vec<Run>> = run_data.runs.get(&current_spin_sector).unwrap();
            // data_utils::generate_plot(runs_map, &current_spin_sector);
        }


    }
    // if not wanting to run chains, then perform data analysis
    // else {
    // //   let runs_map: &BTreeMap<usize, Vec<Run>> = run_data.runs.get(&spin_sector).unwrap();
    // //   data_utils::generate_plot(runs_map, &spin_sector);
    // }
}

fn update_run_data(run_data: &mut RunData, chain_size: usize, steps: u128) {
    let contains_chain_size = run_data.runs.contains_key(&chain_size);
    if contains_chain_size {
        run_data.runs.get_mut(&chain_size).unwrap().push(steps);

    } else {
        let new_run_vec: Vec<u128> = vec![steps];
        run_data.runs.insert(chain_size, new_run_vec);
    }
}

/// A function that will print the spins in a chain. (Probably not necessary since I can use {:?} formatter for arrays)
pub fn print_chains(spin_chain_vec: &Vec<SpinChain<CHAIN_SIZE>>) {
    for spin_chain in spin_chain_vec {
        for spin in &spin_chain.chain {
            print!("{}, ",*spin);
        }
    }
}

/// A function that iterates through the hash_chain_map and prints the relevant information
/// for degenerate chain creation.
pub fn print_degen_counts(hash_chain_map: &HashMap<u64, (u128,[i8;CHAIN_SIZE],[char;CHAIN_SIZE])>) {
    for key_value_pair in hash_chain_map {
        for character in key_value_pair.1.2 {
            print!("{}", character)
        }
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
}