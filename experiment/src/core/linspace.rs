pub fn linspace(start: i32, stop: i32, number: i32) -> Vec<i32> {
    if number == 0 {
        return vec![];
    }
    if number == 1 {
        return vec![start];
    }

    let step = (stop - start) / (number - 1);

    (0..number).map(|i| start + step * i).collect()
}
