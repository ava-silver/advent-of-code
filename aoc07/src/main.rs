use std::{fs::read_to_string, collections::HashMap};

#[derive(Clone, Debug)]
enum Fs {
    File(u64), // size
    Dir(String, Vec<Fs>), // name, files
}

impl Fs {
    fn has_name(&self, name: &String) -> bool {
        matches!(self, Fs::Dir(n, _) if name == n)
    } 
    fn get(&mut self, path: &[String]) -> &mut Self {
        let Some(next_folder) = path.first() else {
            return self;
        };
        let Fs::Dir(_, contents) = self else {
            panic!("Path must be only folders");
        };
        let Some(fs) = contents.iter_mut().find(|f| (*f).has_name(next_folder)) else {
            panic!("could not find folder {}", next_folder);
        };
        fs.get(&path[1..])
    }
}

fn parse_line(line: &str, wd: &mut Vec<String>, current_contents: &mut Vec<Fs>) {
    if line.starts_with("$ cd ") {
        match &line[5..] {
            ".." => {
                wd.pop();
            },
            folder => {
                wd.push(folder.to_owned());
            },
        };
    } else {
        let mut tokens = line.split(' ');
        match tokens.next() {
            None => (),
            Some("$") => (),
            Some("dir") => {
                let Some(new_dir) = tokens.next().and_then(|s| Some(s.to_owned())) else {
                    panic!("Missing dir name token in ls dir");
                };
                if current_contents.iter().find(|f| f.has_name(&new_dir)).is_none() {
                    current_contents.push(Fs::Dir(new_dir, vec![]))
                }
            },
            Some(val) => {
                let size = val.parse().expect("file size must be an int");
                current_contents.push(Fs::File(size));
            }
        }
    }
}

fn parse_fs(input: String) -> Fs {
    let mut fs = Fs::Dir("/".to_owned(), vec![]);
    let mut wd = vec![];
    for line in input.lines().skip(1) {
        let Fs::Dir(_, current_contents) = fs.get(&wd) else {
            panic!("not a folder")
        };
        parse_line(line, &mut wd, current_contents);
    }
    fs
}

fn folder_sizes(fs: &Fs, sizes: &mut HashMap<String, u64>, wd: String) {
    let Fs::Dir(_, contents) = fs else {
        return;
    };
    let mut size = 0;
    for item in contents {
        match item {
            Fs::Dir(sub_name, _) => {
                let sub_full_name = wd.clone() + "/" +  sub_name;
                if !sizes.contains_key(&sub_full_name) {
                    folder_sizes(item, sizes, sub_full_name.clone())
                }
                size += sizes[&sub_full_name];
            },
            Fs::File(s) => size += s,
        }
    }
    sizes.insert(wd, size);
}

fn part_1(sizes: &HashMap<String, u64>) -> u64 {
    sizes.values().filter(|size| **size <= 100000).sum()
}

fn part_2(sizes: &HashMap<String, u64>) -> u64 {
    let total_size = 70000000;
    let needed_space = 30000000;
    let used_space = sizes[""];
    let space_to_delete = needed_space - (total_size - used_space);
    *sizes.values().filter(|size| **size >= space_to_delete).min().expect("there should be at least one value")
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let fs = parse_fs(input);
    let mut sizes = HashMap::new();
    folder_sizes(&fs, &mut sizes, "".to_owned());
    println!("Part 1: {}", part_1(&sizes));
    println!("Part 2: {}", part_2(&sizes));
}
