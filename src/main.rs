fn main() {
    let Some(s) = std::env::args().nth(1) else {
        return;
    };
    let mut program: Vec<u8> = s.as_bytes().to_vec();
    while !program.is_empty() {
        match program.remove(0) {
            b'\\' => {
                if program.is_empty() {
                    return;
                }
                print!("{}", char::from(program.remove(0)));
            }
            b'/' => {
                let Some(pattern) = read_slash(&mut program) else {
                    return;
                };
                let Some(replacement) = read_slash(&mut program) else {
                    return;
                };
                program = substitute(&pattern, &replacement, program);
            }
            c => print!("{}", char::from(c)),
        }
    }
}

fn read_slash(program: &mut Vec<u8>) -> Option<Vec<u8>> {
    let mut acc = Vec::new();
    while !program.is_empty() {
        match program.remove(0) {
            b'\\' => {
                if program.is_empty() {
                    break;
                }
                acc.push(program.remove(0));
            }
            b'/' => return Some(acc),
            c => acc.push(c),
        }
    }
    None
}

fn substitute(pattern: &[u8], replacement: &[u8], mut program: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut j = 0;
    let mut s = 0;
    loop {
        if j == pattern.len() {
            program = [&program[..s], replacement, &program[i..]].concat();
            i = s;
            j = 0;
            continue;
        } else if i >= program.len() {
            return program;
        } else if program[i] == pattern[j] {
            if j == 0 {
                s = i;
            }
            j += 1;
        } else {
            j = 0;
        }
        i += 1;
    }
}
