use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
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

    // need to add new_excited() chain generation. This can be done by passing in an array of numbers 
    //   0    1        2
    // upup downup downdown
    // so if we did something like new_excited([0], [1])
    // we say build me an excited chain that has 1 excited bond of the form "upup"
    // similarly ([0], [2])
    // build excited chian with 2 excited bonds of the form "upup"
    // and ([0,1], [1,1])
    // build excited chain with one upup and one downup
    // ([0,1], [2,1])
    // add two excited upup bonds and one downup bond etc.
    // reject a new_excited call if len(excited_array)!= len(number_of_exc_array)
    // allow user to specify the order they would like the bond to be placed ie [0,1] 
    // will do the upup first then the downup wherea [1,0] will do downup then upup
    // with N total sites (where N = 2n)(N-2 that can be changed) we need to make sure that the sum of the excited states
    // is <= N-2

    /// A function for generating a spin chain with excited bonds
    /// 
    /// * 'excited_bond_map': A hashmap that contains 3 key-value pairs. The keys are 0,1,2 for up-canted, down-canted, and mismatch bond types. 

    pub fn new_excited(excited_bond_map: &HashMap<i8, i8>) -> Self {

        let mut rng = rand::thread_rng();

        if excited_bond_map.len() < 3 {
            panic!("Not enough arguments supplied! Must have entries with keys 0, 1, 2");
        }

        if excited_bond_map.len() > 3 {
            panic!("Too many arguments supplied! Must only have entries with keys 0, 1, 2");
        }

        let total_number_of_excited_bonds = SpinChain::<N>::validate_excited_sites(excited_bond_map);
        let number_of_excited_sites = 2 * total_number_of_excited_bonds;

        // Validation should have been successful, now we choose where to place the bonds
        // In the S_tot^z = 1 sector we have that the bonds should have the form
        // (...)[_i (...) ]_j (...)

        let number_of_up_cant_sites = 2 * (*excited_bond_map.get(&0).unwrap());
        let number_of_down_cant_sites = 2 * (*excited_bond_map.get(&1).unwrap());
        let number_of_mismatch_sites = 2 * (*excited_bond_map.get(&2).unwrap());

        let mut excited_site_indices = BTreeMap::<i8, i8>::new();

        // First, we populate the up_cant sites. This is fairly straightforward since all indices come in pairs meaning that
        // by default they will not be embedded within another up-canted bond.
        SpinChain::<N>::populate_up_cant_site_indices(&mut excited_site_indices, number_of_up_cant_sites);

        // These next two will be more difficult since we have added restrictions:
        // 1. We cannot embed these bonds inside of other excited bonds
        // SpinChain::<N>::populate_down_cant_site_indices(&mut excited_site_indices, number_of_down_cant_sites);
        // Thes bonds can be embedded within eachother, but cannot be embedded within up/down-canted bonds
        // SpinChain::<N>::populate_mismatch_site_indices(&mut excited_site_indices, number_of_mismatch_sites);




        


        

        return SpinChain::new_empty();
    }

    fn validate_excited_sites(number_of_bonds: &HashMap<i8, i8>) -> usize {

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

        total_number_of_excited_bonds

    }


    fn populate_up_cant_site_indices(excited_site_indices: &mut BTreeMap<i8, i8>, number_of_up_cant_sites: i8) {

        let mut rng = rand::thread_rng();

        let mut count = 0;

        // Choose indices that will have an up-canted spin
        while count < number_of_up_cant_sites {

            // N is the size of the chain, but the indices will end on N-1
            // Do not want to include the 2 right most spins in this part of the generation
            let random_site_index = rng.gen_range(0..N-2) as i8;
                if !excited_site_indices.contains_key(&random_site_index) {
                    excited_site_indices.insert(random_site_index, 0);
                    count+=1;
                }
        }

        // Now that indices have been chosen, we need to validate
        let indices = Vec::from_iter(excited_site_indices.keys());
        let mut bad_indices = Vec::<i8>::new();

        let mut index = 1;

        // loop over pairs to make sure that they have proper spacing in the random sequence
        while index < indices.len() {
            let first_index = **indices.get(index-1).unwrap();
            let second_index = **indices.get(index).unwrap();
            let spacing = second_index - first_index - 1;

            // Check for the special condition that the first index is 1, this implies
            // that we have an up canted bond directly next to the right edge which is
            // not allowed. If we have first_index = 0 then second_index has the possibility of being
            // 1 which is fine.
            if spacing %2 != 0 || first_index == 1{
                bad_indices.push(first_index);
                bad_indices.push(second_index);
            }
            index += 2;
        }

        let needed_new_cant_sites = bad_indices.len() as i8;

        // If we found pairs that are not properly spaced, we will remove them from our map (In this case we are looking for things that 
        // will cause broken Dyck word issues such as: [(])
        if needed_new_cant_sites != 0 {
            
            for entry in bad_indices {
                excited_site_indices.remove_entry(&entry);
            }

            SpinChain::<N>::populate_up_cant_site_indices(excited_site_indices, needed_new_cant_sites);
        }
    
    
    }
    
    fn populate_down_cant_site_indices(excited_site_indices:  &BTreeMap<i8, i8>, number_of_down_cant_sites: i8) {
    
    }
    
    fn populate_mismatch_site_indices(excited_site_indices:  &BTreeMap<i8, i8>, number_of_mismatch_sites: i8) {
    
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
