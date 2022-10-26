let elems = params[0];
let n = params[1];

let res = [];
let tmp = [];
let i = 0;
for elem in elems {
    tmp += elem;
    i += 1;
    
    if i == n {
        res += [tmp];
        tmp = [];
        i = 0;
    }
}

if tmp.len() != 0 {
    res += [tmp]
}

res
