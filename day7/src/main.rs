extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

struct Relation {
    id: char,
    dep: char,
}

struct ComplexRelation {
    id: char,
    deps: Vec<char>
}

#[derive(Clone, Debug)]
struct Worker {
    id: char,
    time: i32
}

fn main() {
    first_part();
    second_part();
}

fn first_part() {
    let relations = load_relations("input.txt");
    let tags = get_tags(&relations);
    let deps = get_deps(&relations);
    for relation in relations.iter() {
        println!("id {} dep {}", relation.id, relation.dep);
    }
    let free_tags = get_free_tags(&tags, &deps);
    println!("Free tags: {:?}", free_tags);
    let mut complex_relations = create_complex_relations(&relations);
    for relation in complex_relations.iter() {
        println!("id {} deps {:?}", relation.id, relation.deps);
    }
    let mut available_actions: Vec<char> = Vec::new();
    available_actions.append(&mut free_tags.clone());
    let mut instructions: Vec<char> = Vec::new();
    while instructions.len() != tags.len() + free_tags.len() {
        available_actions.sort();

        let mut current_key = '#';
        for action in available_actions.iter() {
            if !instructions.contains(&action) {
                current_key = *action;
                break;
            }
        }
        instructions.push(current_key);


        for relation in complex_relations.iter() {
            let mut should_unlock = true;
            for dep in relation.deps.iter() {
                if !instructions.contains(&dep) {
                    should_unlock = false;
                }
            }

            if should_unlock {
                available_actions.push(relation.id);
            }
        }
        complex_relations.retain(|ref x| x.id != current_key);
    }
    let mut result = String::new();
    for instruction in instructions {
        result.push(instruction);
    }
    println!("{}", result);
}

fn second_part() {
    let relations = load_relations("input.txt");
    let tags = get_tags(&relations);
    let deps = get_deps(&relations);
    for relation in relations.iter() {
        println!("id {} dep {}", relation.id, relation.dep);
    }
    let free_tags = get_free_tags(&tags, &deps);
    println!("Free tags: {:?}", free_tags);
    let complex_relations = create_complex_relations(&relations);
    for relation in complex_relations.iter() {
        println!("id {} deps {:?}", relation.id, relation.deps);
    }
    let mut available_actions: Vec<char> = Vec::new();
    available_actions.append(&mut free_tags.clone());
    let mut instructions: Vec<char> = Vec::new();
    let mut current_time = 0;
    let mut workers: Vec<Worker> = vec![Worker { id: '#', time: 0 }; 5];
    let mut in_construction: Vec<char> = Vec::new();
    while instructions.len() != tags.len() + free_tags.len() {
        println!("Second: {}", current_time);
        //check if anything built
        let mut built: Vec<char> = Vec::new();
        for worker in &mut workers {
            if worker.time == time_to_build(worker.id) && worker.id != '#' {
                println!("Finished working on {}", worker.id);
                built.push(worker.id);
                worker.time = 0;
                worker.id = '#';
            } else if worker.id != '#' {
                println!("Currently working on {} ({})", worker.id, worker.time);
            }
        }
        built.sort();

        for block in built.iter() {
            instructions.push(block.clone());
        }

        for relation in complex_relations.iter() {
            let mut should_unlock = true;
            for dep in relation.deps.iter() {
                if !instructions.contains(&dep) {
                    should_unlock = false;
                }
            }

            if should_unlock {
                available_actions.push(relation.id);
            }
        }

        available_actions.sort();

        // find building blocks to assign
        let mut keys: Vec<char> = Vec::new();
        for action in available_actions.iter() {
            if !instructions.contains(&action) && !in_construction.contains(&action) {
                println!("Available key: {}", *action);
                keys.push(*action);
            }
        }
        keys.sort();

        // find workers to assign the blocks
        for worker in &mut workers {
            if worker.id == '#' && !keys.is_empty() {
                println!("Assigned to: {}", *keys.first().unwrap());
                worker.id = *keys.first().unwrap();
                worker.time = 0;
                in_construction.push(*keys.first().unwrap());
                keys.remove(0);
            }
        }

        // work
        current_time += 1;
        for worker in &mut workers {
            worker.time += 1;
        }

        println!("Next turn");
    }
    let mut result = String::new();
    for instruction in instructions {
        result.push(instruction);
    }
    println!("{}", result);
}

fn get_free_tags(tags: &Vec<char>, deps: &Vec<char>) -> Vec<char> {
    let mut free_tags: Vec<char> = Vec::new();
    for dep in deps.iter() {
        if !tags.contains(dep) {
            free_tags.push(dep.clone());
        }
    }

    free_tags
}

fn get_deps(relations: &Vec<Relation>) -> Vec<char> {
    let mut deps: Vec<char> = relations.iter().map(|ref x| x.dep).collect();
    deps.sort();
    deps.dedup();
    deps
}

fn get_tags(relations: &Vec<Relation>) -> Vec<char> {
    let mut tags: Vec<char> = relations.iter().map(|ref x| x.id).collect();
    tags.sort();
    tags.dedup();
    tags
}

fn load_relations(filename: &str) -> Vec<Relation> {
    let f = File::open(filename).expect("input file not found");
    let f = BufReader::new(f);

    let re = Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
    let mut relations: Vec<Relation> = Vec::new();
    for line in f.lines() {
        match re.captures(&line.unwrap()) {
            Some(caps) => relations.push(Relation { id: caps[2].parse().unwrap(), dep: caps[1].parse().unwrap()}),
            None => panic!("Messy input!")
        }
    }
    relations.sort_by(|a,b| a.id.cmp(&b.id));
    relations
}

fn create_complex_relations(relations: &Vec<Relation>) -> Vec<ComplexRelation> {
    let mut complex_relations: Vec<ComplexRelation> = Vec::new();

    let mut current_id = relations.first().unwrap().id;
    let mut current_deps: Vec<char> = Vec::new();
    for relation in relations.iter() {
        if relation.id == current_id {
            current_deps.push(relation.dep);
        } else {
            complex_relations.push(ComplexRelation{id: current_id, deps: current_deps});

            current_id = relation.id;
            current_deps = Vec::new();
            current_deps.push(relation.dep);
        }
    }
    complex_relations.push(ComplexRelation{id: current_id, deps: current_deps});
    complex_relations
}

fn time_to_build(block: char) -> i32 {
    60 + (block as i32) - 65 + 1
}
