use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day24)]
fn parse(_input: &str) -> () {}

/*
 * The input can be split up in 14 nearly identical sections, only differing on the literal value for
 * three instructions.
 *
 *              00   01   02   03   04   05   06   07   08   09   10   11   12   13
 *     inp  w
 *     mul  x    0    0    0    0    0    0    0    0    0    0    0    0    0    0
 *     add  x    z    z    z    z    z    z    z    z    z    z    z    z    z    z
 *     mod  x   26   26   26   26   26   26   26   26   26   26   26   26   26   26
 *     div  z    1    1    1   26    1   26    1    1    1   26   26   26   26   26
 *     add  x   14   15   15   -6   14   -4   15   15   11    0    0   -3   -9   -9
 *     eql  x    w    w    w    w    w    w    w    w    w    w    w    w    w    w
 *     eql  x    0    0    0    0    0    0    0    0    0    0    0    0    0    0
 *     mul  y    0    0    0    0    0    0    0    0    0    0    0    0    0    0
 *     add  y   25   25   25   25   25   25   25   25   25   25   25   25   25   25
 *     mul  y    x    x    x    x    x    x    x    x    x    x    x    x    x    x
 *     add  y    1    1    1    1    1    1    1    1    1    1    1    1    1    1
 *     mul  z    y    y    y    y    y    y    y    y    y    y    y    y    y    y
 *     mul  y    0    0    0    0    0    0    0    0    0    0    0    0    0    0
 *     add  y    w    w    w    w    w    w    w    w    w    w    w    w    w    w
 *     add  y    1    7   13   10    0   13   11    6    1    7   11   14    4   10
 *     mul  y    x    x    x    x    x    x    x    x    x    x    x    x    x    x
 *     add  z    y    y    y    y    y    y    y    y    y    y    y    y    y    y
 *
 * Substituting `a`, `b`, and `c` for the differing values, every section is instead
 *
 *     inp w
 *     mul x 0
 *     add x z
 *     mod x 26
 *     div z a
 *     add x b
 *     eql x w
 *     eql x 0
 *     mul y 0
 *     add y 25
 *     mul y x
 *     add y 1
 *     mul z y
 *     mul y 0
 *     add y w
 *     add y c
 *     mul y x
 *     add z y
 *
 * which can be translated to
 *
 *     w = input()
 *     x *= 0
 *     x += z
 *     x %= 26
 *     z /= a
 *     x += b
 *     x = (int) x == w
 *     x = (int) x == 0
 *     y *= 0
 *     y += 25
 *     y *= x
 *     y += 1
 *     z *= y
 *     y *= 0
 *     y += w
 *     y += c
 *     y *= x
 *     z += y
 *
 * and simplified as
 *
 *     w = input()
 *
 *     if (z % 26) + b == w {
 *         z = z / a
 *     } else {
 *         z = (z / a) * 26 + w + c
 *     }
 *
 * Together with the different values for `a`, `b`, and `c` tabulated
 *
 *         00   01   02   03   04   05   06   07   08   09   10   11   12   13
 *     a =  1    1    1   26    1   26    1    1    1   26   26   26   26   26
 *     b = 14   15   15   -6   14   -4   15   15   11    0    0   -3   -9   -9
 *     c =  1    7   13   10    0   13   11    6    1    7   11   14    4   10
 *
 * we can see that `(z % 26) + b == w` can only be `true` if `a == 26` given the corresponding
 * values of `b`, and otherwise `a` is `1`. This gives something like
 *
 *     w = input()
 *
 *     if z.peek() + b == w {
 *         z.pop()
 *     } else {
 *         z.push(w + c)
 *     }
 *
 * with the goal that z should be an empty array at the end for the input number to be valid. Given
 * the values for `a`, `b`, and `c` above, this gives the requirements for the input as follows.
 *
 *     input[13] = input[0] + 1 - 9
 *     input[12] = input[1] + 7 - 9
 *     input[11] = input[6] + 11 - 3
 *     input[10] = input[7] + 6 - 0
 *     input[ 9] = input[8] + 1 - 0
 *     input[ 5] = input[4] + 0 - 4
 *     input[ 3] = input[2] + 13 - 6
 */

#[aoc(day24, part1)]
fn part1(_: &()) -> i64 {
    /*
     * input[ 0] = 9
     * input[ 1] = 9
     * input[ 2] = 2
     * input[ 3] = 9
     * input[ 4] = 9
     * input[ 5] = 5
     * input[ 6] = 1
     * input[ 7] = 3
     * input[ 8] = 8
     * input[ 9] = 9
     * input[10] = 9
     * input[11] = 9
     * input[12] = 7
     * input[13] = 1
     */

    99299513899971
}

#[aoc(day24, part2)]
fn part2(_: &()) -> i64 {
    /*
     * input[ 0] = 9
     * input[ 1] = 3
     * input[ 2] = 1
     * input[ 3] = 8
     * input[ 4] = 5
     * input[ 5] = 1
     * input[ 6] = 1
     * input[ 7] = 1
     * input[ 8] = 1
     * input[ 9] = 2
     * input[10] = 7
     * input[11] = 9
     * input[12] = 1
     * input[13] = 1
     */

    93185111127911
}
