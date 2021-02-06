fn main() {
    //let a: u128 = (1..=20).product();
    //let b: u128 = (51..=70).product();
    //println!("{} / {}", a, b);

//    for i in (10..301).step_by(2) {
//        let c = lattice(i);
//        //println!("n = {} -> {}", i, c);
//    }

    let a: [usize; 10] = [
        1,
        2,
        2*3,
        2*3*4,
        2*3*4*5,
        2*3*4*5*6,
        2*3*4*5*6*7,
        2*3*4*5*6*7*8,
        2*3*4*5*6*7*8*9,
        2*3*4*5*6*7*8*9*10        
    ];
    println!("{:?}", a);
    println!("{}",fact70());
    let c = (a[9]*a[9]) as u128;
    let b = fact70()/c;
    println!("{}, {}, {}",b, b*c, 1.0/(b as f64))

    /*
    lattice(9);
    lattice(10);
    lattice(17);
    lattice(19);
    lattice(20);
    lattice(26);
    lattice(30);
    lattice(130);
    */
}

fn fact(x1: usize, x2: usize) -> usize {
    (x1+1..=x2).product()
}

fn fact70() -> u128 {
    let mut result = 51;
    for next in 52..=70 {
        result *= next
    }
    result
}

fn lattice(n: usize) -> usize {
    let mut count = 0;
    // known lattice point at (n/2, n/2) with origin at 0,0, and n is even
    // r = radius = n/2 * √2
    // r2 = radius squared = x^2 + y^2 = 2*(n^2/4) = n^2/2
    let r = (n as f64) * (2.0f64).sqrt() / 2.0;
    //println!("n = {}, n/2 = {}, r = {}", n, n/2, r);
    println!("{}", n);
    let r2 = n*n/2;  // will always be an integer even for odd n
    let mut x = r as usize;
    for y in 0..n/2 {
        let y2 = y * y;
        let mut l2 = x * x + y2;
        //println!("  (x,y) = ({},{}), r2 = {}, l2 = {}", x, y, r2, l2);
        if l2 < r2 {
            while l2 < r2 {
                x += 1;
                l2 = x * x + y2;
                //println!("    inc: (x,y) = ({},{}), r2 = {}, l2 = {}", x, y, r2, l2);
            }
        } else {
            while l2 > r2 {
                x -= 1;
                l2 = x * x + y2;
                //println!("    dec: (x,y) = ({},{}), r2 = {}, l2 = {}", x, y, r2, l2);
            }
        }
        if l2 == r2 {
            //println!("FOUND ONE ({},{})", x, y);
            println!("  ({},{})", x, y);
            count += 1;
        }
    }
    count
}

/*
    lattice(10); // -> 1 (12)
    lattice(100); // -> 2 (20)
    lattice(102); // 1
    lattice(104); // 1
    lattice(106); // 1
    lattice(108); // 0
    lattice(110); // 1
    lattice(1000); // -> 3 (28)
    lattice(10000); // -> 4 (36)
    lattice(100000); // -> 5 (44)
    lattice(1000000); // -> 6 (52)
    lattice(10000000); // -> 7 (60)
    lattice(10000002); // -> 1 (12)
    lattice(100000000); // -> 8 (68)
    lattice(1000000000); // -> 9 (76)
    lattice(10_000_000_000);  // fails due to overflow of n^2 in usize
*/
/*
n = 10, n/2 = 5, r = 7.0710678118654755
FOUND ONE (7,1)
n = 100, n/2 = 50, r = 70.71067811865476
FOUND ONE (70,10)
FOUND ONE (62,34)
n = 102, n/2 = 51, r = 72.12489168102785
FOUND ONE (69,21)
n = 104, n/2 = 52, r = 73.53910524340095
FOUND ONE (68,28)
n = 106, n/2 = 53, r = 74.95331880577405
FOUND ONE (73,17)
n = 108, n/2 = 54, r = 76.36753236814714
n = 110, n/2 = 55, r = 77.78174593052023
FOUND ONE (77,11)
n = 1000, n/2 = 500, r = 707.1067811865476
FOUND ONE (700,100)
FOUND ONE (644,292)
FOUND ONE (620,340)
n = 10000, n/2 = 5000, r = 7071.067811865476
FOUND ONE (7000,1000)
FOUND ONE (6904,1528)
FOUND ONE (6440,2920)
FOUND ONE (6200,3400)
n = 100000, n/2 = 50000, r = 70710.67811865476
FOUND ONE (70000,10000)
FOUND ONE (69040,15280)
FOUND ONE (64400,29200)
FOUND ONE (62000,34000)
FOUND ONE (53648,46064)
n = 1000000, n/2 = 500000, r = 707106.7811865476
FOUND ONE (705568,46624)
FOUND ONE (700000,100000)
FOUND ONE (690400,152800)
FOUND ONE (644000,292000)
FOUND ONE (620000,340000)
FOUND ONE (536480,460640)
n = 10000000, n/2 = 5000000, r = 7071067.811865476
FOUND ONE (7055680,466240)
FOUND ONE (7000000,1000000)
FOUND ONE (6904000,1528000)
FOUND ONE (6440000,2920000)
FOUND ONE (6200000,3400000)
FOUND ONE (5924288,3860416)
FOUND ONE (5364800,4606400)
n = 10000002, n/2 = 5000001, r = 7071069.226079038
FOUND ONE (7032939,733341)
n = 100000000, n/2 = 50000000, r = 70710678.11865476
FOUND ONE (70556800,4662400)
FOUND ONE (70000000,10000000)
FOUND ONE (69040000,15280000)
FOUND ONE (66429056,24231808)
FOUND ONE (64400000,29200000)
FOUND ONE (62000000,34000000)
FOUND ONE (59242880,38604160)
FOUND ONE (53648000,46064000)
n = 1000000000, n/2 = 500000000, r = 707106781.1865475
FOUND ONE (705568000,46624000)
FOUND ONE (700000000,100000000)
FOUND ONE (690400000,152800000)
FOUND ONE (676823296,204719872)
FOUND ONE (664290560,242318080)
FOUND ONE (644000000,292000000)
FOUND ONE (620000000,340000000)
FOUND ONE (592428800,386041600)
FOUND ONE (536480000,460640000)

*/