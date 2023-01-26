




fn cal_mean(arr: [u32; 5])-> usize{
    let arr_len = arr.len() as usize;

    let mut all_sum_values  = 0;

    for i in 0..arr_len {

        all_sum_values  += arr[i];

    }
    
    let mean = all_sum_values as usize / arr_len;

    return mean 
}


fn main() {
let arr [u32,5] = [1,2,3,4,5]

let values: Vec<u32> = Vec::new();

cal_mean(arr)
}
