use std::collections::HashMap;
use std::fs; 
use std::env; 

fn main () {
    let file_name: String = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("input/2019/6.txt"))
        .to_string(); 

    let file_content: String = fs::read_to_string(file_name)
        .expect("could not open file"); 

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

    // count from the root up
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

    let sum = recursive_count(&orbit_graph, orbit_graph.get("COM").expect("COM not found"), 1); 

    println!("{:?}", sum); 
}
