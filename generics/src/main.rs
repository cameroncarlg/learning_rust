fn find_largest_num(v: &Vec<i32>) -> &i32 {
    let mut current_largest = &v[0];

    for i in v {
        if i > current_largest {
            current_largest = i;
        }
    }
    current_largest
}

fn find_largest_char(v: &Vec<char>) -> &char {
    let mut current_largest = &v[0];

    for i in v {
        if i > current_largest {
            current_largest = i;
        }
    }
    current_largest
}

fn find_largest<T>(v: &[T]) -> &T {
    let mut current_largest = &v[0];

    for i in v {
        if i > current_largest {
            current_largest = i;
        }
    }
    current_largest
}

fn main() {
    let my_vec = vec![11, 13, 53, 60, 999, 100, 1, 501];
    let char_vec = vec!['a', 't', 'g', 'b', 'c', 'z'];

    let largest_int = find_largest_num(&my_vec);
    let largest_char = find_largest_char(&char_vec);

    println!("{largest_int}");
    println!("{largest_char}");
}
