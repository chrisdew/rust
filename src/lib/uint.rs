/*
Module: uint
*/

/*
Function: min_value

Return the minimal value for an uint.

This is always 0
*/
pure fn min_value() -> uint { ret 0u; }

/*
Function: max_value

Return the maximal value for an uint.

This is 2^wordsize - 1
*/
pure fn max_value() -> uint {
     ret 0u - 1u;
}

/* Function: add */
pure fn add(x: uint, y: uint) -> uint { ret x + y; }

/* Function: sub */
pure fn sub(x: uint, y: uint) -> uint { ret x - y; }

/* Function: mul */
pure fn mul(x: uint, y: uint) -> uint { ret x * y; }

/* Function: div */
pure fn div(x: uint, y: uint) -> uint { ret x / y; }

/* Function: rem */
pure fn rem(x: uint, y: uint) -> uint { ret x % y; }

/* Predicate: lt */
pure fn lt(x: uint, y: uint) -> bool { ret x < y; }

/* Predicate: le */
pure fn le(x: uint, y: uint) -> bool { ret x <= y; }

/* Predicate: eq */
pure fn eq(x: uint, y: uint) -> bool { ret x == y; }

/* Predicate: ne */
pure fn ne(x: uint, y: uint) -> bool { ret x != y; }

/* Predicate: ge */
pure fn ge(x: uint, y: uint) -> bool { ret x >= y; }

/* Predicate: gt */
pure fn gt(x: uint, y: uint) -> bool { ret x > y; }

/*
Function: range

Iterate over the range [`lo`..`hi`)
*/
fn range(lo: uint, hi: uint, it: block(uint)) {
    while lo < hi { it(lo); lo += 1u; }
}

/*
Function: next_power_of_two

Returns the smallest power of 2 greater than or equal to `n`
*/
fn next_power_of_two(n: uint) -> uint {
    let halfbits: uint = sys::size_of::<uint>() * 4u;
    let tmp: uint = n - 1u;
    let shift: uint = 1u;
    while shift <= halfbits { tmp |= tmp >> shift; shift <<= 1u; }
    ret tmp + 1u;
}

/*
Function: parse_buf

Parse a buffer of bytes

Parameters:

buf - A byte buffer
radix - The base of the number

Failure:

buf must not be empty
*/
fn parse_buf(buf: [u8], radix: uint) -> uint {
    if vec::len::<u8>(buf) == 0u {
        log_err "parse_buf(): buf is empty";
        fail;
    }
    let i = vec::len::<u8>(buf) - 1u;
    let power = 1u;
    let n = 0u;
    while true {
        let digit = char::to_digit(buf[i] as char);
        if (digit as uint) >= radix {
            fail;
        }
        n += (digit as uint) * power;
        power *= radix;
        if i == 0u { ret n; }
        i -= 1u;
    }
    fail;
}

/*
Function: from_str

Parse a string to an int

Failure:

s must not be empty
*/
fn from_str(s: str) -> uint { parse_buf(str::bytes(s), 10u) }

/*
Function: to_str

Convert to a string in a given base
*/
fn to_str(num: uint, radix: uint) -> str {
    let n = num;
    assert (0u < radix && radix <= 16u);
    fn digit(n: uint) -> char {
        ret alt n {
              0u { '0' }
              1u { '1' }
              2u { '2' }
              3u { '3' }
              4u { '4' }
              5u { '5' }
              6u { '6' }
              7u { '7' }
              8u { '8' }
              9u { '9' }
              10u { 'a' }
              11u { 'b' }
              12u { 'c' }
              13u { 'd' }
              14u { 'e' }
              15u { 'f' }
              _ { fail }
            };
    }
    if n == 0u { ret "0"; }
    let s: str = "";
    while n != 0u {
        s += str::unsafe_from_byte(digit(n % radix) as u8);
        n /= radix;
    }
    let s1: str = "";
    let len: uint = str::byte_len(s);
    while len != 0u { len -= 1u; s1 += str::unsafe_from_byte(s[len]); }
    ret s1;
}

/*
Function: str

Convert to a string
*/
fn str(i: uint) -> str { ret to_str(i, 10u); }

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C $RBUILD 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End:
