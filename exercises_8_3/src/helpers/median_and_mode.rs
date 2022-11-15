use std::collections::HashMap;

pub fn vector_median(v: &mut Vec<i32>) {
    println!("===== pulling vector median =====");
    let mut to_append: Vec<i32> = vec![1, 2, 1, 55, 4, 55, 55, 2];
    println!("Median value => {}", median(v));
    println!("Vector to append is {:?}", v);
    append_me(v, &mut to_append);
    println!("Median value after append => {}", median(v));
}

pub fn mode_variant_1(v: &Vec<i32>) -> Option<(usize, i32)> {
    println!("===== pulling vector mode =====");
    let occurences = count_occurences(v);
    let sc = occurences.into_iter();
    sc.reduce(|accum, item| if accum.1 >= item.1 { accum } else { item })
}

//TODO study it again
pub fn mode_variant_2(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);
    println!("Max value is {}", max_value);
    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}

fn count_occurences(v: &Vec<i32>) -> HashMap<usize, i32> {
    let mut store_counter = HashMap::new();
    for el in v {
        let counter = store_counter.entry(*el as usize).or_insert(0);
        *counter += 1;
    }
    println!("Occurences {:?}", &store_counter);
    store_counter
}

fn median(v: &mut Vec<i32>) -> &i32 {
    println!(" Vector is {:?}", v);
    println!(" Vector size is {}", v.len());
    v.sort();
    println!("Sorted {:?}", v);

    let mid: usize = v.len() / 2;
    v.get(mid).unwrap_or(&0)
}

fn append_me(v: &mut Vec<i32>, to_append: &mut Vec<i32>) {
    v.append(to_append);
}
