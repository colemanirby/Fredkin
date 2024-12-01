use core::num;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::i8;
use rand::Rng;
use rand::prelude::ThreadRng;
use rand_mt::Mt19937GenRand64;
// Spin chain struct
#[derive(Clone)]
pub struct SpinChain<const N: usize> {
    pub chain: Vec<i8>,
    pub chain_hash: u64,
    pub spin_sector: usize
}

/// Creates a Spin Chain based on "height above horizon".
//  Arrays:
//    Fixed Size: Arrays have a fixed size known at compile time.
//    Stack Allocation: Arrays are allocated on the stack, making them very fast for access and creation.
//    No Allocation Overhead: Since the size is known at compile time, there's no need for dynamic memory allocation, which reduces overhead.
//    Memory Access: Accessing elements in an array is very efficient because the compiler knows the size and can perform bounds checking efficiently.
impl<const N: usize> SpinChain<N> {

    /// A function for generating a spin chain with excited up-cant bonds
    /// 
    /// * 'excited_bond_map': A hashmap that contains 3 key-value pairs in the form (bond type, number of bonds). The keys are 0,1,2 for up-canted, down-canted, and mismatch bond types. 

    pub fn new_excited(excited_bond_map: &HashMap<usize, usize>, chain_size: usize, rng: &mut Mt19937GenRand64) -> Self {

        // println!("Making new excited chain");

        if excited_bond_map.len() < 3 {
            panic!("Not enough arguments supplied! Must have entries with keys 0, 1, 2");
        }

        if excited_bond_map.len() > 3 {
            panic!("Too many arguments supplied! Must only have entries with keys 0, 1, 2");
        }

        let spin_sector = SpinChain::<N>::validate_excited_sites(excited_bond_map, chain_size);

        // Validation should have been successful, now we choose where to place the bonds
        // In the S_tot^z = 1 sector we have that the bonds should have the form
        // (...)[_i (...) ]_j (...)

        let number_of_up_cant_bonds = *excited_bond_map.get(&0).unwrap();
        // let number_of_down_cant_bonds = *excited_bond_map.get(&1).unwrap();
        // let number_of_mismatch_sites = *excited_bond_map.get(&2).unwrap();


        // Nice property of BTreeMap is that it will keep keys in a specific order
        // example: doing insert(10, 20) followed by insert (2, 15) will have the
        // entries stored in the order (2, 15), (10, 20).
        let mut excited_site_indices = BTreeMap::<usize, i8>::new();

        // First, we populate the up_cant sites. This is fairly straightforward since all indices come in pairs meaning that
        // by default they will not be embedded within another up-canted bond.
        // println!("populating map with indices");
        let mut is_valid_map = false;

        // while !is_valid_map {
        //     excited_site_indices.clear();
        //     is_valid_map = SpinChain::<N>::populate_up_cant_site_index_map(&mut excited_site_indices, number_of_up_cant_bonds, chain_size, rng);
        // }

        SpinChain::<N>::populate_up_cant_site_index_map_v2(&mut excited_site_indices, number_of_up_cant_bonds, chain_size, rng);
        

        // println!("excited site indices: {excited_site_indices:?}");

        // The sites for the spin chain have been decided and validated in the previous step. We will now populate the spin
        // chain.
        let chain = SpinChain::<N>::construct_excited_chain(&mut excited_site_indices, chain_size);


        let mut hasher = DefaultHasher::new();
        chain.hash(&mut hasher);
        let chain_hash = hasher.finish();



        SpinChain { chain, chain_hash, spin_sector}
    }

    /// A function that will generate indices that will have an excited bond
    /// * excited_site_indices: An empty map that will be populated with the index for an excited bond as the key and the excitation type for the bond
    /// * number_of_bonds: The number of bonds that one wishes to generate
    fn populate_up_cant_site_index_map(excited_site_indices: &mut BTreeMap<usize, i8>, number_of_bonds: usize, chain_size: usize, rng: &mut Mt19937GenRand64) -> bool {
        let mut odd_number_counter = 0;
        let mut even_number_counter = 0;

        // Here we generate n even numbers and n odd numbers that will be paired with eachother as bond sites
        while odd_number_counter < number_of_bonds || even_number_counter < number_of_bonds {
            let random_number = rng.gen_range(0..chain_size-2);
            if random_number %2 == 0 && even_number_counter < number_of_bonds && !excited_site_indices.contains_key(&random_number) {
                even_number_counter += 1;
                excited_site_indices.insert(random_number, 2);
            } else if random_number %2 !=0 && odd_number_counter < number_of_bonds && !excited_site_indices.contains_key(&random_number) {
                odd_number_counter += 1;
                excited_site_indices.insert(random_number, 2);
            }
        }

        let is_valid_map = validate_site_index_map(excited_site_indices);
        is_valid_map

    }

    fn populate_up_cant_site_index_map_v2(excited_site_indices: &mut BTreeMap<usize, i8>, number_of_bonds: usize, chain_size:usize, rng: &mut Mt19937GenRand64) {

        let mut available_sites: Vec<usize> = Vec::new();

        for integer in 0..chain_size-2 {

            available_sites.push(integer);

        }

        let maximum_index = available_sites.len();

        //i_e_i
        let mut initial_even_index: usize = 0;
        let mut initial_even_index_determined = false;
        //i_o_i
        let mut initial_odd_index: usize = maximum_index;
        let mut initial_odd_index_determined = false;
        let mut number_of_subcovers: usize;

        let mut are_sites_populated = false;

        while !initial_even_index_determined {
            initial_even_index = rng.gen_range(0..maximum_index);
            if initial_even_index % 2 == 0 {
                initial_even_index_determined = true;
            }
        }
        while !initial_odd_index_determined {
            initial_odd_index = rng.gen_range(initial_even_index..maximum_index);
            if initial_odd_index % 2 != 0 {
                initial_odd_index_determined = true;
            }
        }

        // we can now split the chain into multiple logical pieces
        // 0 1 2 .. N - m N - m + 1.. N-3 N-2 N-1 N
        // one piece will be from [0..inital_even_index - 1][initial_even_index..initial_odd_index][initial_odd_index + 1.. N-2]
        let mut chain_pieces: Vec<&[usize]> = Vec::new();
        if initial_even_index == 0 {     
            // If we have i_e_i = 0, then we will only need to split the chain once maximum. If we also have i_o_i = maximum index we don't need to do
            // any splitting since both endpoints will now be part of a bond.
            if initial_odd_index == maximum_index - 1 {
                chain_pieces.push(&available_sites);
                number_of_subcovers = 1;
            } else {
                let (left, right) =  available_sites.split_at(initial_odd_index+1);
                chain_pieces.push(left);
                chain_pieces.push(right);
                number_of_subcovers = 2;
            }
        } else {
            // if we have i_o_i == chain_size -3 we will only need to split once. Otherwise we will have split the chain into 3 pieces
            if initial_odd_index == maximum_index - 1 {
                let (left, right) = available_sites.split_at(initial_even_index);
                chain_pieces.push(left);
                chain_pieces.push(right);
                number_of_subcovers = 2;
            } else {
                let (left, right) = available_sites.split_at(initial_even_index);
                chain_pieces.push(left);
                // the split has created a new logical chain called "right". We need to offset the indexing by the length of the chain
                // called "left" since the original chain structure was permanently modified
                let (middle, right) = right.split_at(initial_odd_index+1 - left.len());
                chain_pieces.push(middle);
                chain_pieces.push(right);     
                number_of_subcovers = 3;
            }
        }

        let mut two_entry_vecs: Vec<&[usize]>= Vec::new();
        if number_of_subcovers < number_of_bonds {

            while !are_sites_populated {
                let random_piece = rng.gen_range(0..chain_pieces.len());
                let chain_to_split = chain_pieces.remove(random_piece);
                let random_odd = rng.gen_range(0..chain_to_split.len());
                // we do not want to attempt to split chains that only have 2 elements
                // nor do we want to split a chain at its endpoint since it will give us an
                // empty slice.
                // Because we remove the "chain to split" from the "chain pieces"
                // we need to be sure that we re-insert them if they are not modified at all
                // otherwise we will lose access to them once the loop continues.
                if random_odd % 2 != 0 && random_odd != chain_to_split.len() - 1{
                    if chain_to_split.len() > 2 {
                        let (left, right) = chain_to_split.split_at(random_odd+1);
                        if left.len() <= 2 && right.len() <= 2{
                            two_entry_vecs.push(left);
                            two_entry_vecs.push(right);
                        } else if left.len() <= 2 && right.len() > 2 {
                            two_entry_vecs.push(left);
                            chain_pieces.push(right);
                        } else if left.len() > 2 && right. len() <= 2 {
                            chain_pieces.push(left);
                            two_entry_vecs.push(right);
                        } else {
                            chain_pieces.push(left);
                            chain_pieces.push(right);
                        }
                        number_of_subcovers+=1;
                    } else {
                        two_entry_vecs.push(chain_to_split);
                    }
                    if number_of_subcovers >= number_of_bonds {
                        are_sites_populated = true;
                
                    }  
                } else {
                    chain_pieces.push(chain_to_split);
                }
            }
        }

       chain_pieces.append(&mut two_entry_vecs);

        for _i in 0..number_of_bonds {
            let random_index = rng.gen_range(0..chain_pieces.len());
            let random_endpoints = chain_pieces.remove(random_index);
            excited_site_indices.insert(*random_endpoints.first().unwrap(), 2);
            excited_site_indices.insert(*random_endpoints.last().unwrap(), 2);
        }
    }

    fn populate_up_cant_site_index_map_k_beach (excited_site_indices: &mut BTreeMap<usize, i8>, number_of_bonds: usize, chain_size:usize, rng: &mut Mt19937GenRand64) {

        let mut me: f64 = 0.0;
        let mut mo: f64 = 0.0;

        let n_o_b_f64: f64 = number_of_bonds as f64;
        let c_s_f64: f64 = chain_size as f64;

        for  i in 0.. N
        {
            let rand: f64 = rng.gen_range(0.0..1f64);
            let i_f64 = i as f64;
           if i%2 == 0
           { // even case
              let p = 2.0*(n_o_b_f64-me)/(c_s_f64-2.0-i_f64);
              if rand < p
              {
                 excited_site_indices.insert(i, 2);
                 me+=1.0;
              }
           }
           else
           { // odd case
              let p = 2.0*(n_o_b_f64-mo)/(c_s_f64-1.0-i_f64);
              if rand < p 
              {
                 excited_site_indices.insert(i, 2);
                 mo+=1.0;
              }
           }
        }
    }

    // up_cant = 2, down_cant = 3, mismatch = 4
    /// A function that will construct the entire excited chain
    /// * excited_site_indices: a map that contains the sites that will have an excited bond endpoint
    fn construct_excited_chain(excited_site_indices: &mut BTreeMap<usize, i8>, chain_size: usize) -> Vec<i8> {
        let mut chain = vec![0;chain_size];
        chain[0] = 1;
        chain[chain_size-1] = -1;
        
        // for excited chains, height above/below the horizon no longer matters. We just need to make 
        // proper Dyck words in between the excited sites.
        let first_excited_bond_position = *excited_site_indices.first_key_value().unwrap().0;

        SpinChain::<N>::populate_excited_sites_of_chain(excited_site_indices, &mut chain);

        SpinChain::<N>::populate_left_side_of_chain(&mut chain, first_excited_bond_position);

        let excited_indices_vec = Vec::from_iter(excited_site_indices.keys());

        let mut index = 1;

        while index < excited_indices_vec.len() {
            let left_bound = *excited_indices_vec.get(index-1).unwrap();
            let left_bound_index = *left_bound as usize;
            let right_bound = *excited_indices_vec.get(index).unwrap();
            let right_bound_index = *right_bound as usize;
            let inner_length =  (right_bound_index - left_bound_index - 1) as u32;
            SpinChain::<N>::generate_arbitrary_dyck_words(&mut chain, left_bound_index+1, right_bound_index, inner_length);           
            index += 1;
        }

        
        let last_excited_bond_position = *excited_site_indices.last_key_value().unwrap().0 as usize;

        let right_side_length = (chain_size - last_excited_bond_position -1) as u32;
        SpinChain::<N>::generate_arbitrary_dyck_words(&mut chain, last_excited_bond_position+1, chain_size, right_side_length);

        chain
        
    }

    // A function that will generate valid Dyck Word States in some given interval [left_bound, right_bound)
    /// * chain: an array that will represent the spin chain
    /// * left_bound: the first spin that will be included in the Dyck word state
    /// * right_bound: the spin after the last spin that will be included in the Dyck word state
    /// * length: the size of then interval
    fn generate_arbitrary_dyck_words(chain: &mut Vec<i8>, left_bound: usize, right_bound: usize, length: u32) {

        // offset index to keep probability calulations correct
        let mut current_index = 1;
        let mut height = 1;

        if length == 0 {
            return;
        }
        else if length == 2 {
            chain[left_bound] = 1;
            chain [right_bound - 1] = -1;
            return;
        } else {
            chain[left_bound] = 1;
            chain[right_bound - 1] = -1;
            for i in left_bound+1..right_bound {
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

    /// A preprocessing function that fills in the bonds before Dyck Word generation is performed
    /// * excited_bond_positions: A map that contains the bond positions and the type of bond
    /// * chain: an array representing the spin chain
    fn populate_excited_sites_of_chain(excited_bond_positions: &mut BTreeMap<usize, i8>, chain: &mut Vec<i8>) {
        for entry  in excited_bond_positions {
            let index = (*entry.0) as usize;
            let excitation_type = *entry.1;
            chain[index] = excitation_type;
        }
    }

    /// A function that handles the special case of populating in the left side of the chain
    /// * chain: an array that represents the spin chain
    /// * first_excited_bond_position: the position of the left most excited bond site
    fn populate_left_side_of_chain(chain: &mut Vec<i8>, first_excited_bond_position: usize) {
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

    /// A function that ensures a user does not pass in more bonds than there are sites
    /// * number_of_bonds: A map containing all bond types and the number of each bond
    fn validate_excited_sites(number_of_bonds: &HashMap<usize, usize>, chain_size: usize) -> usize {

        // the number of available sites is N-2 since the leftmost and rightmost
        // sites cannot be changed
        let available_sites = chain_size-2;
        let mut spin_sector = 0;
        let mut total_number_of_excited_bonds:usize = 0;
        for entry in number_of_bonds {

            total_number_of_excited_bonds += *entry.1 as usize;
            spin_sector +=*entry.1;
        }

        // above we simply find out how many exicted bonds we want. We need to multiply this by 2 
        // since each bond occupies 2 sites.
        let num_of_excited_sites = 2 * total_number_of_excited_bonds;

        if num_of_excited_sites > available_sites {
            panic!("The number of sites needed for excited bonds exceeded the number of available sites in the chain. excited sites: {}, size of chain: {}", num_of_excited_sites, chain_size);
        }

        spin_sector

    }
}

/// A function that ensures the excited site indices are in even, odd, even, odd, even,... order
/// * excited_site_indices: A map that contains the endpoints for excited bonds and the bond type at that index
fn validate_site_index_map(excited_site_indices: &mut BTreeMap<usize, i8>) -> bool {
    let mut is_even = true;
    let mut is_valid_map = true;

    for key in excited_site_indices.keys() {
        if is_even && *key % 2 != 0 {
            // println!("This number is odd and shouldn't be: {}", key);
            is_valid_map = false;
        } else if !is_even && *key % 2 == 0 {
            // println!("This number is even and shouldn't be: {}", key);

            is_valid_map = false;
        }
        is_even = !is_even;
    }

    return is_valid_map;

}



/// Calculates the probability of next spin being up. Based on Eq. 10
/// in arXiv:1805.00532
/// Pr(z_i+1 = up) = (h_i + 2)(N - i - h_i)/[2(h_i + 1)(N - i)]
/// * length: size of the chain (this may be the size of an interval within a chain as well)
/// * current_index: the index of the current site for which the spin has already been determined
/// * height: how high above the horizon are you: up up -> height = 2 up down -> height = 0
fn calculate_next_spin_prob(length: u32, current_index: u32, height: u32) -> f64 {

    let numerator: f64 = ((height + 2) * (length - current_index - height)).try_into().expect("Could not turn u32 into f64 - numerator");

    let denominator: f64 = (2 * (height + 1) * (length - current_index)).try_into().expect("Could not turn u32 into f64 - denominator");

    let spin_up_prob = numerator/denominator;

    spin_up_prob
}

/// Determines the next spin via taking random samples from the unit line.
/// * prob_up: the probability that the particle will have an up spin
/// * trials: how many times do you want to sample in order to determine if the particle is spin up or spin down
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
