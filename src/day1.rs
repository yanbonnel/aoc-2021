
pub fn execute() {
    let input = include_str!("./day1.input");
    let values: Vec<i32> = input.split("\n").map(|val| val.parse::<i32>().unwrap()).collect();
    let (count, _) = nb_increase(&values);
    println!("Step 1 : {}", count);

    let values_window = values.windows(3).map(|value| value.to_vec()).filter(|value| value.len() == 3)
        .map(|values| values.iter().sum()).collect();

    let (count, _) = nb_increase(&values_window);
    println!("Step 2 : {}", count);


}

fn nb_increase(values: &Vec<i32>) -> (i32, i32) {
    values.iter().fold((0, i32::max_value()), |(count, pred), val|
        if *val > pred {
            (count + 1, *val)
        } else {
            (count, *val)
        }
    )
}