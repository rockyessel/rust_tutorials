fn main() {

    let n= 3;
    
    let factorial = recursive(n);
    println!("factorial: {:?}", factorial);

    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    let sqr_arr =  sqr_arr_values(arr);
    println!("{:?}", sqr_arr);

    let mean_value_is = cal_mean(arr);

    println!("{}", mean_value_is);

    let x = 32;
    let y = 64;

    let (area, parameter) = find_area_and_parameter(x, y);
    println!("Area: {}, Parameter: {}", area, parameter);

    modify_my_arr(arr);
    println!("Array in Driver Function : {:#?}", arr);

    let mut arr_ref: [u32;9] = [0, 1, 3, 5, 23, 45, 67, 897, 34];

    println!("Array in Function : {:#?}", arr_ref);
    mod_arr_ref(&mut arr_ref);


}

fn cal_mean(arr: [u32; 5]) -> usize {
    let arr_len = arr.len() as usize;

    let mut all_sum_values = 0;

    for i in 0..arr_len {
        all_sum_values += arr[i];
    }

    let mean = all_sum_values as usize / arr_len;

    return mean;
}

fn modify_my_arr(mut arr: [u32; 5]) {
    arr[1] = 8;
    arr[2] = 10;

    println!("Array in Function : {:#?}", arr);
}

fn find_area_and_parameter(x: i32, y: i32) -> (i32, i32) {
    let area = x * y;
    let parameter = 2 * (x + y);

    (area, parameter)
}

fn mod_arr_ref(arr_ref: &mut [u32; 9]) -> [u32;9] {
    arr_ref[3] = 34;
    return *arr_ref
}


fn sqr_arr_values( mut arr: [u32;5]) -> [u32; 5] {
    for i in 1..arr.len() {
        arr[i] *= arr[i];
    }

    return arr
}

fn recursive(n: u64) -> u64 {
    if n == 0 {
        1
    }
    else{
        n * recursive(n-1)
    }
}