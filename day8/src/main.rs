fn main() {
    let data = load_data();

    let (result,_) = go1(&data);
    println!("{}", result);

    let (result,_) = go2(&data);
    println!("{}", result);
}

fn go1(data: &[i32]) -> (i32, &[i32]) {
    let node_count = data[0] as usize;
    let meta_count = data[1] as usize;

    if meta_count < 1 {
        panic!("parsing error");
    }

//    println!("Nodes: {}, metas: {}", node_count, meta_count);

    match node_count {
        0 => {
            let mut meta_sum =data[2..2+meta_count].iter().sum();
//            println!("Summed: {:?}",&data[2..2+meta_count] );
//            println!("Zero nodes! Winding up with: {}", meta_sum);
            (meta_sum, &data[meta_count..])
        },
        _ => {
//            println!("Found {} nodes!", node_count);
            let mut sum = 0;
            let mut current_data = data;
            for i in 0..node_count {
//                println!("Analyzing node#{}, current sum: {}", i, sum);

                let result = go1(&current_data[2..]);
                sum += result.0;
                current_data = result.1;
            }

            let last_sum: i32 = current_data[2..2+ meta_count].iter().sum();
            sum += last_sum;

//            println!("Exiting level with sum: {}!", sum);
            (sum,&current_data[meta_count..])
        }

    }
}

fn go2(data: &[i32]) -> (i32, &[i32]) {
    let node_count = data[0] as usize;
    let meta_count = data[1] as usize;

    if meta_count < 1 {
        panic!("parsing error");
    }

//    println!("Nodes: {}, metas: {}", node_count, meta_count);

    match node_count {
        0 => {
            let mut meta_sum =data[2..2+meta_count].iter().sum();
//            println!("Summed: {:?}",&data[2..2+meta_count] );
//            println!("Zero nodes! Winding up with: {}", meta_sum);
            (meta_sum, &data[meta_count..])
        },
        _ => {
//            println!("Found {} nodes!", node_count);

            let mut child_nodes_values: Vec<i32> = Vec::new();

            let mut current_data = data;
            for i in 0..node_count {
//                println!("Analyzing node#{}", i);

                let result = go2(&current_data[2..]);
//                println!("node#{} sum: {}",i, result.0);
                child_nodes_values.push(result.0);
                current_data = result.1;
            }

            let mut sum= 0;
            let metas = &current_data[2..2+meta_count];
            for meta in metas.iter() {
                if *meta as usize <= child_nodes_values.len() {
                    sum += child_nodes_values[*meta as usize - 1];
                }
            }

//            println!("Exiting level with sum: {}!", sum);
            (sum,&current_data[meta_count..])
        }

    }
}


fn load_data() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");

    let input: Result<Vec<i32>, _> = input
        .trim()
        .split(' ')
        .map(|x| x.parse())
        .collect();

    input.expect("Could not parse data!")
}
