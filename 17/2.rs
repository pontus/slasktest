fn main() {
    let mut count = 0;
    let mut initvx = 0;
    while initvx < 300 {
        let mut initvy = -100;
        while initvy < 4096 {
            let (hit, highest) = simulate(initvx, initvy);

            if hit {
                count += 1;
            }

            initvy += 1;
        }

        initvx += 1;
    }
    println!("{}", count);
}

fn simulate(vx: isize, vy: isize) -> (bool, isize) {
    let mut x = 0;
    let mut y = 0;
    let mut highest = 0;
    let mut cont = true;
    let mut dx = vx;
    let mut dy = vy;

    let mut within = false;

    // let targetminx = 20;
    // let targetmaxx = 30;
    // let targetminy = -10;
    // let targetmaxy = -5;

    let targetminx = 241;
    let targetmaxx = 273;
    let targetminy = -97;
    let targetmaxy = -63;

    while cont {
        x += dx;

        if dx.abs() > 0 {
            dx += -dx.signum();
        }

        y += dy;
        dy -= 1;

        if y > highest {
            highest = y;
        }

        if x >= targetminx && x <= targetmaxx && y >= targetminy && y <= targetmaxy {
            within = true;
        }

        if y < -120 {
            cont = false;
        }
    }

    return (within, highest);
}
