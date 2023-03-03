use std::collections::HashMap;
use std::collections::HashSet;
use std::fs; 
use std::env; 

fn main () {
    // parsing cli args
    let file_name: String = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("input/2019/6.txt"))
        .to_string(); 

    // reading file into string
    let file_content: String = fs::read_to_string(file_name)
        .expect("could not open file"); 

    part_one(&file_content);
    part_two(&file_content);
}

fn part_one(file_content: &String){
    // create hashmap with center_of_orbit -> list_of_orbiting_objects
    let mut orbit_graph: HashMap<&str,Vec<&str>> = HashMap::new();
    for line in file_content.lines() {
        let mut split = line.split(")");
        let com = split.next().unwrap();
        let orbiter = split.next().unwrap();

        match orbit_graph.get_mut(com) {
            Some(vec) => vec.push(orbiter),
            None => {orbit_graph.insert(com, vec![orbiter]);}
        }
    }

    // start with COM(center of all mass). for each orbiting object add 1 + recursive call with 1->1+1
    fn recursive_count(orbit_graph: &HashMap<&str, Vec<&str>>, orbiters: &Vec<&str>, length: usize) -> usize {
        let mut res: usize =  orbiters.len() * length;
        res += orbiters.iter()
            .map(|object| {
                match orbit_graph.get(object) {
                    Some(vec) => recursive_count(orbit_graph, vec, length + 1),
                    None => 0
                }})
            .sum::<usize>();
        res
    }

    // initial call
    let sum = recursive_count(&orbit_graph, orbit_graph.get("COM").expect("COM not found"), 1); 
    println!("{:?}", sum); 
}

fn part_two(file_content: &String){
    // object -> center of orbit
    let orbits: HashMap<&str, &str> = file_content.lines()
        .map(|line| {
            let mut split = line.split(")"); 
            let center = split.next().unwrap();
            let object = split.next().unwrap();
            (object, center)})
        .collect();

    // path: YOU -> COM
    let mut path_you: Vec<&str> = Vec::new();
    let mut coord: &str = "YOU";
    loop {
        match orbits.get(&coord) {
            Some(next) => {
                path_you.push(coord); 
                coord = next;
            },
            None => break
        }
    }

    // path: SAN -> COM
    let mut path_san: Vec<&str> = Vec::new();
    let mut coord: &str = "SAN";
    loop {
        match orbits.get(&coord) {
            Some(next) => {
                path_san.push(coord); 
                coord = next;
            },
            None => break
        }
    }

    let hash_you: HashSet<&str> = HashSet::from_iter(path_you); 
    let hash_san: HashSet<&str> = HashSet::from_iter(path_san); 

    let sym_diff: usize = hash_san.symmetric_difference(&hash_you).count() - 2; 

    
    println!("min lenght (YOU -> SAN): {:?}", sym_diff); 

    // println!("YOU -> COM: {:?}", path_you); 
    // println!("SAN -> COM: {:?}", path_san); 
}
