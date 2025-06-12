pub const fn calc_modulo_cycle_u(dividend: u32, divisor: u32) -> usize {
    if divisor <= 0 {
        return 0; // error
    }

    let dividend = dividend as u32; // Cast to unsigned, preserving the underlying bit pattern

    if dividend < divisor {
        return 18; // 2*5 + 8 fo branch on 6th instruction
    }
    let mut cycles = 24; // Time to get into first loop and between first/second loops
    let mut r0 = dividend;
    let mut r1 = divisor;
    let mut r3: u32 = 1;
    let mut r2: u32;
    let mut r12: u32;
    let mut r4: u32 = 0x10000000;
    // Enter into first loop at offest 0x12
    loop {
        if r1 >= r4 {
            cycles += 10;
            break;
        }
        if r1 >= r0 {
            cycles += 14;
            break;
        }
        r1 <<= 4;
        r3 <<= 4;
        cycles += 20;
    }
    r4 <<= 3;
    loop {
        if r1 >= r4 {
            cycles += 10;
            break;
        }
        if r1 >= r0 {
            cycles += 14;
            break;
        }
        r1 <<= 1;
        r3 <<= 1;
        cycles += 20;
    }
    loop {
        // Entering loop at 0x30
        r2 = 0;
        cycles += 48;
        if r0 >= r1 {
            r0 -= r1;
            cycles -= 4;
        }
        r4 = r1 >> 1; // Now at 0x38
        if r0 >= r4 {
            r0 -= r4;
            r12 = r3;
            r3 = (r3 << (32 - 1)) | (r3 >> 1);
            r2 |= r3;
            r3 = r12;
            cycles += 7;
        }
        r4 = r1 >> 2; // Now at 0x4A
        if r0 >= r4 {
            r0 -= r4;
            r12 = r3;
            r3 = (r3 << (32 - 2)) | (r3 >> 2);
            r2 |= r3;
            r3 = r12;
            cycles += 7;
        }
        r4 = r1 >> 3; // Now at 0x5C
        if r0 >= r4 {
            r0 -= r4;
            r12 = r3;
            r3 = (r3 << (32 - 3)) | (r3 >> 3);
            r2 |= r3;
            r3 = r12;
            cycles += 7;
        }
        r12 = r3; // Now at 0x6E
        if r0 == 0 {
            cycles += 12;
            break;
        }
        r3 >>= 4;
        if r3 == 0 {
            cycles += 16;
            break;
        }
        r1 >>= 4;
        cycles += 20;
    }

    r2 &= 0xE0000000; // Now at 0x7C
    if r2 == 0 {
        return cycles + 18;
    }

    r3 = r12; // Now at 0x88
    r3 = (r3 << (32 - 3)) | (r3 >> 3);
    if (r2 & r3) != 0 {
        //r0 += r1 >> 3;
        cycles -= 2;
    }
    r3 = r12;
    r3 = (r3 << (32 - 2)) | (r3 >> 2);
    if (r2 & r3) != 0 {
        //r0 += r1 >> 2;
        cycles -= 2;
    }
    r3 = r12;
    r3 = (r3 << (32 - 1)) | (r3 >> 1);
    if (r2 & r3) != 0 {
        // r0 += r1 >> 1;
        cycles -= 2;
    }
    cycles + 75
}

pub const fn calc_modulo_cycle_u_from_lua(dividend: i32, divisor: u32) -> usize {
    // Convert while preserving the underlying bit pattern
    calc_modulo_cycle_u(dividend as u32, divisor)
}

pub const fn calc_modulo_cycle_s(dividend: i32, divisor: i32) -> usize {
    let mut r0: u32;
    let mut r1: u32;
    let mut r2: u32;
    let mut r3: u32;
    let mut r4: u32;
    let mut r12: u32;
    let mut cycles = 10;
    r1 = divisor.abs() as u32;
    r0 = dividend.abs() as u32;
    r3 = 1;
    if divisor > 0 {
        cycles += 4;
    }

    cycles += 10;
    if dividend > 0 {
        cycles += 4;
    }

    if r0 < r1 {
        if dividend > 0 {
            return cycles + 32;
        }
        return cycles + 28;
    }
    r4 = 0x10000000;

    cycles += 8;

    loop {
        if r1 >= r4 {
            cycles += 10;
            break;
        }
        if r1 >= r0 {
            cycles += 14;
            break;
        }
        r1 <<= 4;
        r3 <<= 4;
        cycles += 20;
    }

    r4 <<= 3;
    cycles += 2;

    loop {
        if r1 >= r4 {
            cycles += 10;
            break;
        }
        if r1 >= r0 {
            cycles += 14;
            break;
        }
        r1 <<= 1;
        r3 <<= 1;
        cycles += 20;
    }

    loop {
        r2 = 0;
        cycles += 48;
        if r0 >= r1 {
            r0 -= r1;
            cycles -= 4;
        }

        r4 = r1 >> 1;
        if r0 >= r4 {
            r0 -= r4;
            r12 = r3;
            r3 = (r3 << (32 - 1)) | (r3 >> 1);
            r2 |= r3;
            r3 = r12;
            cycles += 7;
        }

        r4 = r1 >> 2;

        if r0 >= r4 {
            r0 -= r4;
            r12 = r3;
            r3 = (r3 << (32 - 2)) | (r3 >> 2);
            r2 |= r3;
            r3 = r12;
            cycles += 7;
        }

        r4 = r1 >> 3;
        if r0 >= r4 {
            r0 -= r4;
            r12 = r3;
            r3 = (r3 << (32 - 3)) | (r3 >> 3);
            r2 |= r3;
            r3 = r12;
            cycles += 7;
        }

        r12 = r3;
        if r0 == 0 {
            cycles += 12;
            break;
        }
        r3 >>= 4;
        if r3 == 0 {
            cycles += 16;
            break;
        }
        r1 >>= 4;
        cycles += 20;
    }

    r2 &= 0xE0000000;
    if r2 == 0 {
        if dividend >= 0 {
            return cycles + 36;
        }
        return cycles + 32;
    }
    cycles += 8;

    r3 = r12;
    cycles += 17;
    r3 = (r3 << (32 - 3)) | (r3 >> 3);
    if (r2 & r3) != 0 {
        //r0 += r1 >> 3;
        cycles -= 2;
    }
    r3 = r12;

    cycles += 17;
    r3 = (r3 << (32 - 2)) | (r3 >> 2);
    if (r2 & r3) != 0 {
        //r0 += r1 >> 2;
        cycles -= 2;
    }
    r3 = r12;

    cycles += 17;
    r3 = (r3 << (32 - 1)) | (r3 >> 1);
    if (r2 & r3) != 0 {
        // r0 += r1 >> 1;
        cycles -= 2;
    }

    cycles += 18;
    if dividend >= 0 {
        cycles += 4;
    }
    cycles
}

// if divisor is 25, it returns 0x5d555550
pub fn find_longest_modulo_cycle_u(divisor: u32) -> u32 {
    let mut max = 0;
    let mut dividend_for_max = 0;
    for dividend in 0..=u32::MAX {
        let cycles = calc_modulo_cycle_u(dividend, divisor);
        if cycles > max {
            max = cycles;
            dividend_for_max = dividend;
        }
    }
    dividend_for_max
}

pub fn calculate_distribution_modulo_cycle_u_24() -> Vec<u32> {
    let mut res: [u32; 1000] = [0; 1000];
    for dividend in 0..=u32::MAX {
        let cycles = calc_modulo_cycle_u(dividend, 24);
        res[cycles] += 1;
    }
    return res.to_vec();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_modulo_cycle_u() {
        assert_eq!(calculate_distribution_modulo_cycle_u_24(), vec![]);

        assert_eq!(calc_modulo_cycle_u(0x4747745, 1), 868);
        assert_eq!(calc_modulo_cycle_s(0x4747745, 1), 888);

        // assert_eq!(find_longest_modulo_cycle_u(24), 0x59999995); // very long
        assert_eq!(calc_modulo_cycle_u(0x59999995, 24), 900);

        // assert_eq!(find_longest_modulo_cycle_u(25), 0x5d555550); // very long
        assert_eq!(calc_modulo_cycle_u(0x5d555550, 25), 900);

        assert_eq!(calc_modulo_cycle_u(24, 25), 18);
        assert_eq!(calc_modulo_cycle_u(25, 25), 126);

        assert_eq!(calc_modulo_cycle_u(1140479406, 25), 767);
        assert_eq!(calc_modulo_cycle_u(1141359974, 25), 836);
        assert_eq!(calc_modulo_cycle_u(1270576878, 25), 777);
        assert_eq!(calc_modulo_cycle_u(1355424535, 25), 767);
        assert_eq!(calc_modulo_cycle_u(1584375516, 25), 807);
        assert_eq!(calc_modulo_cycle_u(1708021406, 25), 776);
        assert_eq!(calc_modulo_cycle_u(1749665817, 25), 754);
        assert_eq!(calc_modulo_cycle_u(2081426142, 25), 804);
        assert_eq!(calc_modulo_cycle_u(524763481, 25), 777);
        assert_eq!(calc_modulo_cycle_u(927365657, 25), 735);

        assert_eq!(calc_modulo_cycle_u(1455995688, 100), 799);
        assert_eq!(calc_modulo_cycle_u(1969433148, 100), 783);
        assert_eq!(calc_modulo_cycle_u(704919059, 100), 803);
        assert_eq!(calc_modulo_cycle_u(1025776836, 100), 768);
        assert_eq!(calc_modulo_cycle_u(765851278, 100), 762);
        assert_eq!(calc_modulo_cycle_u(1609208851, 100), 774);
        assert_eq!(calc_modulo_cycle_u(1915624704, 100), 729);

        assert_eq!(calc_modulo_cycle_u(133070802, 44), 773);
        assert_eq!(calc_modulo_cycle_u(2690473360, 91), 780);
        assert_eq!(calc_modulo_cycle_u(517978802, 82), 777);
        assert_eq!(calc_modulo_cycle_u(932746226, 64), 801);
        assert_eq!(calc_modulo_cycle_u(1447158151, 94), 808);
        assert_eq!(calc_modulo_cycle_u(1586160591, 81), 798);
        assert_eq!(calc_modulo_cycle_u(2533948937, 55), 848);
    }

    #[test]
    fn test_calc_modulo_cycle_u_from_lua() {
        assert_eq!(calc_modulo_cycle_u_from_lua(-5304908, 74), 783);
        assert_eq!(calc_modulo_cycle_u_from_lua(-3153559, 76), 808);
        assert_eq!(calc_modulo_cycle_u_from_lua(-10278414, 10), 854);
        assert_eq!(calc_modulo_cycle_u_from_lua(-11024636, 87), 728);
        assert_eq!(calc_modulo_cycle_u_from_lua(-3041458, 22), 750);
        assert_eq!(calc_modulo_cycle_u_from_lua(-2424550, 41), 803);
        assert_eq!(calc_modulo_cycle_u_from_lua(-10575121, 75), 727);
        assert_eq!(calc_modulo_cycle_u_from_lua(-3202050, 10), 866);
        assert_eq!(calc_modulo_cycle_u_from_lua(-9188001, 97), 764);
        assert_eq!(calc_modulo_cycle_u_from_lua(-1854680, 4), 843);
    }

    #[test]
    fn test_calc_modulo_cycle_s() {
        assert_eq!(calc_modulo_cycle_s(1881135926, 25), 836);
        assert_eq!(calc_modulo_cycle_s(375357918, 25), 792);
        assert_eq!(calc_modulo_cycle_s(1413825380, 25), 801);
        assert_eq!(calc_modulo_cycle_s(-118428064, 25), 781);
        assert_eq!(calc_modulo_cycle_s(1657444058, 25), 827);
        assert_eq!(calc_modulo_cycle_s(38557744, 25), 782);
        assert_eq!(calc_modulo_cycle_s(-1372116835, 25), 762);

        assert_eq!(calc_modulo_cycle_s(1321724843, 99), 811);
        assert_eq!(calc_modulo_cycle_s(-974761848, 99), 782);
        assert_eq!(calc_modulo_cycle_s(660664920, 99), 751);
        assert_eq!(calc_modulo_cycle_s(1843514586, 99), 803);
        assert_eq!(calc_modulo_cycle_s(-1436296528, 99), 777);
        assert_eq!(calc_modulo_cycle_s(-432991421, 99), 785);

        assert_eq!(calc_modulo_cycle_s(1403756501, 49), 812);
        assert_eq!(calc_modulo_cycle_s(-493429862, 48), 802);
        assert_eq!(calc_modulo_cycle_s(-1001956674, 33), 824);
        assert_eq!(calc_modulo_cycle_s(-321103627, 36), 765);
        assert_eq!(calc_modulo_cycle_s(904862469, 14), 874);
        assert_eq!(calc_modulo_cycle_s(-357004509, 83), 769);
        assert_eq!(calc_modulo_cycle_s(396388959, 21), 785);
        assert_eq!(calc_modulo_cycle_s(-367289968, 12), 843);
        assert_eq!(calc_modulo_cycle_s(771082162, 15), 857);
    }
}
