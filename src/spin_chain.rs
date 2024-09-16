use std::collections::hash_map::DefaultHasher;
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
    /// * 'excited_bond_array': A vec that contains 0, 1, 2, or 3 which represents the types of excited bonds. Can have 4 entries total.
    /// * 'number_of_bonds': A vec that contains the number of each type of excited bond that is wanted. It should have the same number of entries as excited_bond_array where each number corresponds to how many of each type
    ///    of bond is wanted.

    pub fn new_excited(excited_bond_array: &Vec<i8>, number_of_bonds: &Vec<i8>) -> Self {

        if excited_bond_array.len() > 1 {
            panic!("Too many arguments supplied. Multiple excited bond types not supported");
        }

        if *excited_bond_array.get(0).unwrap()!= 0 {
            panic!("Excited bonds that are not canted are currently not supported.")
        }

        if number_of_bonds.len() > 1 {

            panic!("Multiple excited bond types not supported")
        }

        if *number_of_bonds.get(0).unwrap()!=1 {
            panic!("Multiple excited bonds is not currently supported");
        }

        let total_number_of_excited_bonds = SpinChain::<N>::validate_excited_sites(number_of_bonds);
        let number_of_excited_sites = 2 * total_number_of_excited_bonds;


        //Back of envelope:
        // N = total number of excited bonds
        // N = 0 : (0) <- Not applicable
        // N = 1 : (1) [(2)] (3) or (1) [] (2) <- excited bonds don't necessarily have to have a Dyck word within them
        // N = 2 : (1) [(2)] {(3)} (4)
        // let potential_num_of_dyck_words = total_number_of_excited_bonds + 2;
 
        let random_site_vec: Vec<i8> = Vec::new();

        for i in 1..number_of_excited_sites - 1 { 



        }

        

        // Validation should have been successful, now we choose where to place the bonds
        // In the S_tot^z = 1 sector we have that the bonds should have the form
        // (...)[_i (...) ]_j (...)
        

        return SpinChain::new_empty();
    }

    fn validate_excited_sites(number_of_bonds: &Vec<i8>) -> usize {

        // the number of available sites is N-2 since the leftmost and rightmost
        // sites cannot be changed
        let available_sites = N-2;
        let mut total_number_of_excited_bonds:usize = 0;
        for entry in number_of_bonds {

            total_number_of_excited_bonds += *entry as usize;
        }

        // above we simply find out how many exicted bonds we want. We need to multiply this by 2 
        // since each bond occupies 2 sites.
        let num_of_excited_sites = 2 * total_number_of_excited_bonds;

        if num_of_excited_sites > available_sites {
            panic!("The number of sites needed for excited bonds exceeded the number of available sites.");
        }

        total_number_of_excited_bonds

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
