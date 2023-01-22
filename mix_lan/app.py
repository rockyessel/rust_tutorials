arr = [0,2,3,543,3,4,3,23,21,31,4,345,45,456,5,678,67,57,5,45,645,545,5345,5,4,564,54,5,453,3434]


def cal_mean(arr):
    arr_len = len(arr)

    sum_of_all_value = 0

    for i in range(0, arr_len):
        sum_of_all_value = sum_of_all_value + arr[i]

    mean_value = sum_of_all_value / arr_len

    return mean_value


mean_values_is = cal_mean(arr)
print(f'The mean of the array is: {mean_values_is}')