const arr = [23, 32, 54, 67, 456, 87, 675, 66, 4576, 7, 5645, 345];

const calculate_mean = (arr) => {
  const arr_len = arr.length;
  console.log(arr_len);
  let d;
  let total_sum = 0;

  for (let i = 1; i <= arr_len; i++) {
    total_sum += arr[i];
    console.log('inner', total_sum);
  }

  console.log('outer', total_sum);
//   const mean_value = d / arr_len;
//   return mean_value;
};

console.log('where is it',calculate_mean(arr));
