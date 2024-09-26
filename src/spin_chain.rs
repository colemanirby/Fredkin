use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::i8;
use std::thread::current;
use rand::{random, Rng};
use rand::prelude::ThreadRng;
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

    /// A function for generating a spin chain with excited up-cant bonds
    /// 
    /// * 'excited_bond_map': A hashmap that contains 3 key-value pairs in the form (bond type, number of bonds). The keys are 0,1,2 for up-canted, down-canted, and mismatch bond types. 

    pub fn new_excited(excited_bond_map: &HashMap<i8, i8>) -> Self {

        println!("Making new excited chain");

        if excited_bond_map.len() < 3 {
            panic!("Not enough arguments supplied! Must have entries with keys 0, 1, 2");
        }

        if excited_bond_map.len() > 3 {
            panic!("Too many arguments supplied! Must only have entries with keys 0, 1, 2");
        }

        println!("validating map");
        SpinChain::<N>::validate_excited_sites(excited_bond_map);

        // Validation should have been successful, now we choose where to place the bonds
        // In the S_tot^z = 1 sector we have that the bonds should have the form
        // (...)[_i (...) ]_j (...)

        let number_of_up_cant_bonds = *excited_bond_map.get(&0).unwrap();
        let number_of_down_cant_bonds = *excited_bond_map.get(&1).unwrap();
        let number_of_mismatch_sites = *excited_bond_map.get(&2).unwrap();


        // Nice property of BTreeMap is that it will keep keys in a specific order
        // example: doing insert(10, 20) followed by insert (2, 15) will have the
        // entries stored in the order (2, 15), (10, 20).
        let mut excited_site_indices = BTreeMap::<i8, i8>::new();

        // First, we populate the up_cant sites. This is fairly straightforward since all indices come in pairs meaning that
        // by default they will not be embedded within another up-canted bond.
        println!("populating map with indices");
        
        SpinChain::<N>::populate_up_cant_site_index_map(&mut excited_site_indices, number_of_up_cant_bonds);

        // The sites for the spin chain have been decided and validated in the previous step. We will now populate the spin
        // chain.
        let chain = SpinChain::<N>::construct_excited_chain(&mut excited_site_indices);

        // These next two will be more difficult since we have added restrictions:
        // 1. We cannot embed these bonds inside of other excited bonds
        // NOTE: Maybe merge the up and down cant population? Can generate the indices for all of the sites, then randomly choose
        // if up or down cant in paired sites. This will prevent embedding of bonds.
        // SpinChain::<N>::populate_down_cant_site_indices(&mut excited_site_indices, number_of_down_cant_sites);
        // Thes bonds can be embedded within eachother, but cannot be embedded within up/down-canted bonds
        // SpinChain::<N>::populate_mismatch_site_indices(&mut excited_site_indices, number_of_mismatch_sites);

        let mut hasher = DefaultHasher::new();
        let chain_bond_rep = SpinChain::construct_bond_rep(chain);
        chain.hash(&mut hasher);
        let chain_hash = hasher.finish();

        SpinChain { chain, chain_hash, chain_bond_rep }
    }

    fn populate_up_cant_site_index_map(excited_site_indices: &mut BTreeMap<i8, i8>, number_of_bonds: i8) {

        let mut rng = rand::thread_rng();
        let mut count = 0;

        // Choose indices that will have an up-canted spin
        // Use number of bonds to produce a pair of indices, (i,j) such that i<j, i is even, and j is odd.
        while count < number_of_bonds {
            let mut index_counter = 1;
            let mut left_bond_index: i8 = 0;

            while index_counter <=2 {

                println!("bond number: {}", index_counter);

                let random_site_index = rng.gen_range(0..N-2) as i8;

                if index_counter == 1 && random_site_index % 2 == 0 
                                    && !excited_site_indices.contains_key(&random_site_index){
                    println!("left index: {}", random_site_index);
                    excited_site_indices.insert(random_site_index, 2);
                    left_bond_index = random_site_index;
                    index_counter = 2;
                } else if index_counter == 2 && random_site_index > left_bond_index 
                                           && random_site_index % 2 != 0 
                                           && !excited_site_indices.contains_key(&random_site_index) {

                        println!("right index: {}", random_site_index);
                        excited_site_indices.insert(random_site_index, 2);
                        index_counter = 3;
                }
            }
            println!("count: {}", count);
            count += 1;
        }
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
            
            let is_up_spin = determine_next_spin(prob_up, 15);

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

    // up_cant = 2, down_cant = 3, mismatch = 4
    fn construct_excited_chain(excited_site_indices: &mut BTreeMap<i8, i8>) -> [i8;N] {

        println!("Construct excited chain");
        println!("excited_site_indices map: {:?}", excited_site_indices);

        let mut chain = [0;N];
        chain[0] = 1;
        chain[N-1] = -1;
        // let chain_size: u32 = chain.len().try_into().expect("Could not turn usize into u32");
        
        // for excited chains, height above/below the horizon no longer matters. We just need to make 
        // proper Dyck words in between the excited sites.
        // let first_excited_bond_position = *excited_site_indices.first_entry().unwrap().get();    
        let first_excited_bond_position = *excited_site_indices.first_key_value().unwrap().0;

        SpinChain::<N>::populate_excited_sites_of_chain(excited_site_indices, &mut chain);

        println!("populating left side of chain");
        SpinChain::<N>::populate_left_side_of_chain(&mut chain, first_excited_bond_position);

        println!("chain after populating left side: {:?}", chain);

        let excited_indices_vec = Vec::from_iter(excited_site_indices.keys());

        println!("excited indices: {:?}", excited_indices_vec);

        let mut index = 1;

        println!("filling in bulk");
        while index < excited_indices_vec.len() {
            let left_bound = *excited_indices_vec.get(index-1).unwrap();
            let left_bound_index = *left_bound as usize;
            let right_bound = *excited_indices_vec.get(index).unwrap();
            let right_bound_index = *right_bound as usize;
            let inner_length =  (right_bound_index - left_bound_index - 1) as u32;
            SpinChain::<N>::generate_arbitrary_dyck_words(&mut chain, left_bound_index, right_bound_index, inner_length);           
            index += 1;
        }

        println!("chain after filling in bulk: {:?}", chain);

        
        let last_excited_bond_position = *excited_site_indices.last_key_value().unwrap().0 as usize;

        println!("last excited bond position {}", last_excited_bond_position);
        // if last_excited_bond_position == N-3 {
        //     chain[N-1] = 1;
        // }
        println!("Filling in right side of chain");
        let right_side_length = (N - last_excited_bond_position -1) as u32;
        SpinChain::<N>::generate_arbitrary_dyck_words(&mut chain, last_excited_bond_position, N, right_side_length);

        println!("chain created successfully!");

        chain
        
    }

    fn generate_arbitrary_dyck_words(chain: &mut [i8;N], left_bound: usize, right_bound: usize, length: u32) {

        // let length = (right_bound - left_bound - 1) as u32;

        // chain[left_bound + 1] = 1;
        // offset index to keep probability calulations correct
        let mut current_index = 1;
        let mut height = 1;

        if length == 0 {
            return;
        }
        else if length == 2 {
            chain[left_bound + 1] = 1;
            chain [right_bound - 1] = -1;
            return;
        } else {
            chain[left_bound + 1] =1;
            chain[right_bound - 1] = 1;
            for i in left_bound+2..right_bound {
                // println!("Offset index: {}", )
                println!("Dyck Word actualy index: {}", i);
                let prob_up = calculate_next_spin_prob(length, current_index, height);
                let is_up_spin = determine_next_spin(prob_up, 15);
                if is_up_spin {
                    chain[i] = 1;
                    height += 1;
                } else {
                    chain[i] = -1;
                    height -= 1;
                }
                current_index = current_index+1;
            }
        }

        

        

    }

    fn populate_excited_sites_of_chain(excited_bond_positions: &mut BTreeMap<i8, i8>, chain: &mut [i8;N]) {
        for entry  in excited_bond_positions {
            let index = (*entry.0) as usize;
            let excitation_type = *entry.1;
            chain[index] = excitation_type;
        }
    }

    fn populate_left_side_of_chain(chain: &mut [i8;N], first_excited_bond_position: i8) {
        println!("First excited postion: {}", first_excited_bond_position);
        if first_excited_bond_position == 0 {
            return;
        } else if first_excited_bond_position == 2 {
            chain [1] = -1;
            return;
        }


        let right_bound = first_excited_bond_position as usize;
        let length = right_bound as u32;

        SpinChain::<N>::generate_arbitrary_dyck_words(chain, 0, right_bound, length);

    }

    fn validate_excited_sites(number_of_bonds: &HashMap<i8, i8>) {

        // the number of available sites is N-2 since the leftmost and rightmost
        // sites cannot be changed
        let available_sites = N-2;
        let mut total_number_of_excited_bonds:usize = 0;
        for entry in number_of_bonds {

            total_number_of_excited_bonds += *entry.1 as usize;
        }

        // above we simply find out how many exicted bonds we want. We need to multiply this by 2 
        // since each bond occupies 2 sites.
        let num_of_excited_sites = 2 * total_number_of_excited_bonds;

        if num_of_excited_sites > available_sites {
            panic!("The number of sites needed for excited bonds exceeded the number of available sites in the chain. excited sites: {}, size of chain: {}", num_of_excited_sites, N);
        }

    }
    
    fn populate_down_cant_site_indices(excited_site_indices:  &BTreeMap<i8, i8>, number_of_down_cant_sites: i8) {
    
    }
    
    fn populate_mismatch_site_indices(excited_site_indices:  &BTreeMap<i8, i8>, number_of_mismatch_sites: i8) {
    
    }


    /// Temporary function that needs to be re-written
    fn construct_bond_rep(chain: [i8;N]) -> [char;N] {

        let mut chain_rep : [char;N] = [' '; N];
        let mut paired = false;

       for i in 0..chain.len() {
        if chain[i] == 1 {
            chain_rep[i] = '(';
        } else if chain[i] == 2 {
            if !paired {
                chain_rep[i] = '[';
                paired = true;
            } else {
                chain_rep[i] = ']';
                paired = false;
            }
            
        } 
        else {
            chain_rep[i] = ')';
        }
       } 

       chain_rep

    }

}



/// Calculates the probability of next spin being up. Based on Eq. 10
/// in arXiv:1805.00532
/// Pr(z_i+1 = up) = (h_i + 2)(N - i - h_i)/[2(h_i + 1)(N - i)]
fn calculate_next_spin_prob(length: u32, current_index: u32, height: u32) -> f64 {

    println!("Calculate_next_spin_prob");
    println!("length: {}", length);
    println!("current_index: {}", current_index);
    println!("height: {}", height);

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
