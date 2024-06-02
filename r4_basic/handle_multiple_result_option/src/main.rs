fn main() {
    // Result
    println!("---- Result ----");
    let random_results = [Ok(1), Ok(2), Err(3)];

    let filter_results = random_results
        .iter()
        .filter(|e| e.is_ok())
        .collect::<Vec<_>>();

    let filter_map_results = random_results
        .iter()
        .filter_map(|e| e.as_ref().ok())
        .collect::<Vec<_>>();

    #[allow(clippy::all)]
    let flat_map_results = random_results.iter().flat_map(|e| e).collect::<Vec<_>>();

    let flatten_results = random_results.iter().flatten().collect::<Vec<_>>();

    println!("filter_results: {:?}", filter_results);
    println!("filter_map_results: {:?}", filter_map_results);
    println!("flat_map_results: {:?}", flat_map_results);
    println!("flatten_results: {:?}", flatten_results);

    // Option
    println!("---- Option---- ");
    let random_options = [Some(1), Some(2), None];

    let filter_options = random_options
        .iter()
        .filter(|e| e.is_some())
        .collect::<Vec<_>>();

    let filter_map_err_options = random_options
        .iter()
        .filter_map(|r| match r {
            Some(r) => Some(r),
            None => None,
        })
        .collect::<Vec<_>>();

    #[allow(clippy::all)]
    let flat_map_options = random_options.iter().flat_map(|e| e).collect::<Vec<_>>();

    let flatten_options = random_options.iter().flatten().collect::<Vec<_>>();

    println!("filter_options: {:?}", filter_options);
    println!("filter_map_err_options: {:?}", filter_map_err_options);
    println!("flat_map_options: {:?}", flat_map_options);
    println!("flatten_options: {:?}", flatten_options);

    // Convert Result <--> Option
    println!("---- Convert between `Result` and `Option` ---- ");

    // How to convert from `Result` to `Option`.
    let random_results = [Ok(1), Ok(2), Err(3)];
    println!("1️⃣ random_results: {:?}", random_results);

    let map_option_foo = random_results.iter().map(|e| e.ok()).collect::<Vec<_>>();
    println!("2️⃣ map_option_foo: {:?}", map_option_foo);

    // How to convert from `Option` to `Result`.
    let random_options = [Some(1), Some(2), None];
    println!("3️⃣ random_options: {:?}", random_options);

    let map_result_foo = random_options
        .iter()
        .map(|e| e.ok_or("Error!!!"))
        .collect::<Vec<_>>();
    println!("4️⃣ map_result_foo: {:?}", map_result_foo);
}
