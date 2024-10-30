
/// Calculates the inner product <psi_2|psi_1>
pub fn calculate_inner_product <const CHAIN_SIZE: usize> (spin_chain_1: &[i8;CHAIN_SIZE], spin_chain_2: &[i8;CHAIN_SIZE]) -> i128 {
    let mut spin_vector:Vec<i128> = Vec::new();
    let mut spin_1:i128;
    let mut spin_2:i128;

    for i in 0..CHAIN_SIZE {
        spin_1 = spin_chain_1[i].try_into().expect("could not convert i8 to i128 spin_1");
        spin_2 = spin_chain_2[i].try_into().expect("could not convert i8 to i128 spin_2");
        spin_vector.push(spin_1*spin_2);
    }

    let mut sum_of_products:i128 = 0;

    for spin in spin_vector {
        sum_of_products += spin;
    }

    println!("difference between {:?}", spin_chain_1);
    println!("and");
    println!("{:?}", spin_chain_2);
    println!("is {}", sum_of_products);
    sum_of_products

}

/// a function to calculate the expectation value of an observable O.
/// IE given a state v, we calculate <v|O|v>. It's broken up into 2 pieces
/// calculate O|v> = |v'> then <v|v'>=<v|O|v>
pub fn calculate_expectation_value <const CHAIN_SIZE: usize>(spin_chain_1: &[i8;CHAIN_SIZE], observable:&[[i8;CHAIN_SIZE];CHAIN_SIZE]) -> i128 {

    let v_prime = matrix_mul_with_vector(spin_chain_1, observable);
    let expectation_value = calculate_inner_product(spin_chain_1, &v_prime);
    expectation_value
}

/// A function that takes in a matrix, A, and a vector, v. Produces the result of Av
pub fn matrix_mul_with_vector <const CHAIN_SIZE: usize>(spin_chain_1: &[i8;CHAIN_SIZE], observable:&[[i8;CHAIN_SIZE];CHAIN_SIZE]) -> [i8;CHAIN_SIZE] {

    let mut v_prime:[i8; CHAIN_SIZE] = [0;CHAIN_SIZE];
    for i in 0..CHAIN_SIZE {
        let matrix_row = observable[i];
        let mut sum = 0;
        for j in 0..CHAIN_SIZE {
            let observ_value = matrix_row[j];
            let vec_value = spin_chain_1[j];
            sum += observ_value*vec_value;      
        }
        v_prime[i] = sum;
    }
    v_prime
}