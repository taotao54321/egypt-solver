use std::cmp;
use std::fmt;
use std::str::{ FromStr };

use failure;
use generic_array;

use util;

type BoardArray = generic_array::GenericArray<u8, generic_array::typenum::U64>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub pos: u8,
    pub v:   BoardArray,
}

impl Board {
    const N_KIND: usize = 9;

    const EMPTY: u8 = 0x7F;
    const UP:    u8 = 0x80;
    const DOWN:  u8 = 0x81;
    const LEFT:  u8 = 0x82;
    const RIGHT: u8 = 0x83;
    const WALL:  u8 = 0xFF;

    const ADJACENTS: [&'static[u8]; 64] = [
        &[1, 8], &[0, 2, 9], &[1, 3, 10], &[2, 4, 11], &[3, 5, 12], &[4, 6, 13], &[5, 7, 14], &[6, 15],
        &[0, 9, 16], &[1, 8, 10, 17], &[2, 9, 11, 18], &[3, 10, 12, 19], &[4, 11, 13, 20], &[5, 12, 14, 21], &[6, 13, 15, 22], &[7, 14, 23],
        &[8, 17, 24], &[9, 16, 18, 25], &[10, 17, 19, 26], &[11, 18, 20, 27], &[12, 19, 21, 28], &[13, 20, 22, 29], &[14, 21, 23, 30], &[15, 22, 31],
        &[16, 25, 32], &[17, 24, 26, 33], &[18, 25, 27, 34], &[19, 26, 28, 35], &[20, 27, 29, 36], &[21, 28, 30, 37], &[22, 29, 31, 38], &[23, 30, 39],
        &[24, 33, 40], &[25, 32, 34, 41], &[26, 33, 35, 42], &[27, 34, 36, 43], &[28, 35, 37, 44], &[29, 36, 38, 45], &[30, 37, 39, 46], &[31, 38, 47],
        &[32, 41, 48], &[33, 40, 42, 49], &[34, 41, 43, 50], &[35, 42, 44, 51], &[36, 43, 45, 52], &[37, 44, 46, 53], &[38, 45, 47, 54], &[39, 46, 55],
        &[40, 49, 56], &[41, 48, 50, 57], &[42, 49, 51, 58], &[43, 50, 52, 59], &[44, 51, 53, 60], &[45, 52, 54, 61], &[46, 53, 55, 62], &[47, 54, 63],
        &[48, 57], &[49, 56, 58], &[50, 57, 59], &[51, 58, 60], &[52, 59, 61], &[53, 60, 62], &[54, 61, 63], &[55, 62],
    ];
    const ADJACENTS_H: [&'static[u8]; 64] = [
        &[1], &[0, 2], &[1, 3], &[2, 4], &[3, 5], &[4, 6], &[5, 7], &[6],
        &[9], &[8, 10], &[9, 11], &[10, 12], &[11, 13], &[12, 14], &[13, 15], &[14],
        &[17], &[16, 18], &[17, 19], &[18, 20], &[19, 21], &[20, 22], &[21, 23], &[22],
        &[25], &[24, 26], &[25, 27], &[26, 28], &[27, 29], &[28, 30], &[29, 31], &[30],
        &[33], &[32, 34], &[33, 35], &[34, 36], &[35, 37], &[36, 38], &[37, 39], &[38],
        &[41], &[40, 42], &[41, 43], &[42, 44], &[43, 45], &[44, 46], &[45, 47], &[46],
        &[49], &[48, 50], &[49, 51], &[50, 52], &[51, 53], &[52, 54], &[53, 55], &[54],
        &[57], &[56, 58], &[57, 59], &[58, 60], &[59, 61], &[60, 62], &[61, 63], &[62],
    ];
    const ADJACENTS_V: [&'static[u8]; 64] = [
        &[8], &[9], &[10], &[11], &[12], &[13], &[14], &[15],
        &[0, 16], &[1, 17], &[2, 18], &[3, 19], &[4, 20], &[5, 21], &[6, 22], &[7, 23],
        &[8, 24], &[9, 25], &[10, 26], &[11, 27], &[12, 28], &[13, 29], &[14, 30], &[15, 31],
        &[16, 32], &[17, 33], &[18, 34], &[19, 35], &[20, 36], &[21, 37], &[22, 38], &[23, 39],
        &[24, 40], &[25, 41], &[26, 42], &[27, 43], &[28, 44], &[29, 45], &[30, 46], &[31, 47],
        &[32, 48], &[33, 49], &[34, 50], &[35, 51], &[36, 52], &[37, 53], &[38, 54], &[39, 55],
        &[40, 56], &[41, 57], &[42, 58], &[43, 59], &[44, 60], &[45, 61], &[46, 62], &[47, 63],
        &[48], &[49], &[50], &[51], &[52], &[53], &[54], &[55],
    ];

    const DIST: [[u32; 64]; 64] = [
        [0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2],
        [1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3],
        [2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4],
        [3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5],
        [4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4],
        [3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3],
        [2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2],
        [1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1],
        [1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3],
        [2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4],
        [3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5],
        [4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6],
        [5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5],
        [4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4],
        [3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3],
        [2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2],
        [2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4],
        [3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5],
        [4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6],
        [5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7],
        [6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6],
        [5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5],
        [4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4],
        [3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3],
        [3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5],
        [4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6],
        [5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7],
        [6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8],
        [7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7],
        [6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6],
        [5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5],
        [4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4],
        [4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4],
        [5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5],
        [6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6],
        [7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7],
        [8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6],
        [7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5],
        [6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4],
        [5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3],
        [3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3],
        [4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4],
        [5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5],
        [6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6],
        [7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5],
        [6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4],
        [5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3],
        [4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2],
        [2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1,1,2,3,4,5,4,3,2],
        [3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2,2,1,2,3,4,5,4,3],
        [4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3,3,2,1,2,3,4,5,4],
        [5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4,4,3,2,1,2,3,4,5],
        [6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3,5,4,3,2,1,2,3,4],
        [5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2,4,5,4,3,2,1,2,3],
        [4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1,3,4,5,4,3,2,1,2],
        [3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0,2,3,4,5,4,3,2,1],
        [1,2,3,4,5,4,3,2,2,3,4,5,6,5,4,3,3,4,5,6,7,6,5,4,4,5,6,7,8,7,6,5,3,4,5,6,7,6,5,4,2,3,4,5,6,5,4,3,1,2,3,4,5,4,3,2,0,1,2,3,4,3,2,1],
        [2,1,2,3,4,5,4,3,3,2,3,4,5,6,5,4,4,3,4,5,6,7,6,5,5,4,5,6,7,8,7,6,4,3,4,5,6,7,6,5,3,2,3,4,5,6,5,4,2,1,2,3,4,5,4,3,1,0,1,2,3,4,3,2],
        [3,2,1,2,3,4,5,4,4,3,2,3,4,5,6,5,5,4,3,4,5,6,7,6,6,5,4,5,6,7,8,7,5,4,3,4,5,6,7,6,4,3,2,3,4,5,6,5,3,2,1,2,3,4,5,4,2,1,0,1,2,3,4,3],
        [4,3,2,1,2,3,4,5,5,4,3,2,3,4,5,6,6,5,4,3,4,5,6,7,7,6,5,4,5,6,7,8,6,5,4,3,4,5,6,7,5,4,3,2,3,4,5,6,4,3,2,1,2,3,4,5,3,2,1,0,1,2,3,4],
        [5,4,3,2,1,2,3,4,6,5,4,3,2,3,4,5,7,6,5,4,3,4,5,6,8,7,6,5,4,5,6,7,7,6,5,4,3,4,5,6,6,5,4,3,2,3,4,5,5,4,3,2,1,2,3,4,4,3,2,1,0,1,2,3],
        [4,5,4,3,2,1,2,3,5,6,5,4,3,2,3,4,6,7,6,5,4,3,4,5,7,8,7,6,5,4,5,6,6,7,6,5,4,3,4,5,5,6,5,4,3,2,3,4,4,5,4,3,2,1,2,3,3,4,3,2,1,0,1,2],
        [3,4,5,4,3,2,1,2,4,5,6,5,4,3,2,3,5,6,7,6,5,4,3,4,6,7,8,7,6,5,4,5,5,6,7,6,5,4,3,4,4,5,6,5,4,3,2,3,3,4,5,4,3,2,1,2,2,3,4,3,2,1,0,1],
        [2,3,4,5,4,3,2,1,3,4,5,6,5,4,3,2,4,5,6,7,6,5,4,3,5,6,7,8,7,6,5,4,4,5,6,7,6,5,4,3,3,4,5,6,5,4,3,2,2,3,4,5,4,3,2,1,1,2,3,4,3,2,1,0],
    ];

    pub fn new(pos: u8, v: &[u8]) -> Self {
        Self {
            pos,
            v: BoardArray::clone_from_slice(v),
        }
    }

    pub fn idx2xy(pos: u8) -> (u8, u8) {
        (pos%8, pos/8)
    }

    pub fn xy2idx(x: u8, y: u8) -> u8 {
        8*y + x
    }

    fn adjacent(pos: u8) -> &'static[u8] {
        Board::ADJACENTS[pos as usize]
    }

    fn adjacent_h(pos: u8) -> &'static[u8] {
        Board::ADJACENTS_H[pos as usize]
    }

    fn adjacent_v(pos: u8) -> &'static[u8] {
        Board::ADJACENTS_V[pos as usize]
    }

    pub fn moves(&self) -> Vec<u8> {
        fn is_direction(cell: u8) -> bool {
            cell == Board::UP ||
            cell == Board::DOWN ||
            cell == Board::LEFT ||
            cell == Board::RIGHT
        }

        let mut res = vec![];

        let mut que     = util::Queue::<u8>::new();
        let mut visited = [false; 64];
        // 始点を踏むには一度出てから戻らないといけないことに注意
        for &to in Board::adjacent(self.pos) {
            if self.v[to as usize] == Board::WALL { continue; }
            que.push(to);
            visited[to as usize] = true;
        }

        while !que.is_empty() {
            let i = que.pop().unwrap();

            if is_direction(self.v[i as usize]) {
                res.push(i);
                continue;
            }

            for &to in Board::adjacent(i) {
                if visited[to as usize] { continue; }
                if self.v[to as usize] == Board::WALL { continue; }
                que.push(to);
                visited[to as usize] = true;
            }
        }

        res
    }

    pub fn move_(&mut self, pos: u8) {
        match self.v[pos as usize] {
            Board::UP    => self.rotate_up(pos),
            Board::DOWN  => self.rotate_down(pos),
            Board::LEFT  => self.rotate_left(pos),
            Board::RIGHT => self.rotate_right(pos),
            _            => panic!("not direction"),
        };
        self.pos = pos;
    }

    fn rotate_up(&mut self, pos: u8) {
        debug_assert_eq!(self.v[pos as usize], Board::UP);
        let (x, _) = Board::idx2xy(pos);
        let tmp = self.v[Board::xy2idx(x,0) as usize];
        for y in 0..7 {
            self.v[Board::xy2idx(x,y) as usize] = self.v[Board::xy2idx(x,y+1) as usize];
        }
        self.v[Board::xy2idx(x,7) as usize] = tmp;

        let mut erase_edge = false;
        let i = Board::xy2idx(x,6) as usize;
        let j = Board::xy2idx(x,7) as usize;
        if self.v[i] < Board::N_KIND as u8 && self.v[i] == self.v[j] {
            erase_edge = true;
        }

        for i in (0..8).map(|y| Board::xy2idx(x,y)) {
            self.erase_h(i);
        }

        if erase_edge {
            self.v[i] = Board::EMPTY;
            self.v[j] = Board::EMPTY;
        }
    }

    fn rotate_down(&mut self, pos: u8) {
        debug_assert_eq!(self.v[pos as usize], Board::DOWN);
        let (x, _) = Board::idx2xy(pos);
        let tmp = self.v[Board::xy2idx(x,7) as usize];
        for y in (1..8).rev() {
            self.v[Board::xy2idx(x,y) as usize] = self.v[Board::xy2idx(x,y-1) as usize];
        }
        self.v[Board::xy2idx(x,0) as usize] = tmp;

        let mut erase_edge = false;
        let i = Board::xy2idx(x,0) as usize;
        let j = Board::xy2idx(x,1) as usize;
        if self.v[i] < Board::N_KIND as u8 && self.v[i] == self.v[j] {
            erase_edge = true;
        }

        for i in (0..8).map(|y| Board::xy2idx(x,y)) {
            self.erase_h(i);
        }

        if erase_edge {
            self.v[i] = Board::EMPTY;
            self.v[j] = Board::EMPTY;
        }
    }

    fn rotate_left(&mut self, pos: u8) {
        debug_assert_eq!(self.v[pos as usize], Board::LEFT);
        let (_, y) = Board::idx2xy(pos);
        let tmp = self.v[Board::xy2idx(0,y) as usize];
        for x in 0..7 {
            self.v[Board::xy2idx(x,y) as usize] = self.v[Board::xy2idx(x+1,y) as usize];
        }
        self.v[Board::xy2idx(7,y) as usize] = tmp;

        let mut erase_edge = false;
        let i = Board::xy2idx(6,y) as usize;
        let j = Board::xy2idx(7,y) as usize;
        if self.v[i] < Board::N_KIND as u8 && self.v[i] == self.v[j] {
            erase_edge = true;
        }

        for i in (0..8).map(|x| Board::xy2idx(x,y)) {
            self.erase_v(i);
        }

        if erase_edge {
            self.v[i] = Board::EMPTY;
            self.v[j] = Board::EMPTY;
        }
    }

    fn rotate_right(&mut self, pos: u8) {
        debug_assert_eq!(self.v[pos as usize], Board::RIGHT);
        let (_, y) = Board::idx2xy(pos);
        let tmp = self.v[Board::xy2idx(7,y) as usize];
        for x in (1..8).rev() {
            self.v[Board::xy2idx(x,y) as usize] = self.v[Board::xy2idx(x-1,y) as usize];
        }
        self.v[Board::xy2idx(0,y) as usize] = tmp;

        let mut erase_edge = false;
        let i = Board::xy2idx(0,y) as usize;
        let j = Board::xy2idx(1,y) as usize;
        if self.v[i] < Board::N_KIND as u8 && self.v[i] == self.v[j] {
            erase_edge = true;
        }

        for i in (0..8).map(|x| Board::xy2idx(x,y)) {
            self.erase_v(i);
        }

        if erase_edge {
            self.v[i] = Board::EMPTY;
            self.v[j] = Board::EMPTY;
        }
    }

    fn erase_h(&mut self, pos: u8) {
        let kind = self.v[pos as usize];
        if kind >= Board::N_KIND as u8 { return; }

        let mut found = false;
        for &to in Board::adjacent_h(pos) {
            if self.v[to as usize] != kind { continue; }
            self.v[to as usize] = Board::EMPTY;
            found = true;
        }
        if found {
            self.v[pos as usize] = Board::EMPTY;
        }
    }

    fn erase_v(&mut self, pos: u8) {
        let kind = self.v[pos as usize];
        if kind >= Board::N_KIND as u8 { return; }

        let mut found = false;
        for &to in Board::adjacent_v(pos) {
            if self.v[to as usize] != kind { continue; }
            self.v[to as usize] = Board::EMPTY;
            found = true;
        }
        if found {
            self.v[pos as usize] = Board::EMPTY;
        }
    }

    pub fn counts(&self) -> [u8; Board::N_KIND] {
        let mut res = [0; Board::N_KIND];
        self.v.iter()
            .filter(|&&e| e < Board::N_KIND as u8)
            .for_each(|&e| res[e as usize] += 1);
        res
    }

    pub fn is_solved(&self) -> bool {
        self.counts().iter().all(|&n| n == 0)
    }

    pub fn is_stuck(&self) -> bool {
        //if self.moves().is_empty() { return true; }
        if self.counts().iter().any(|&n| n == 1) { return true; }
        false
    }

    pub fn calc_step(&self, src: u8, dst: u8) -> Option<u32> {
        fn is_direction(cell: u8) -> bool {
            cell == Board::UP ||
            cell == Board::DOWN ||
            cell == Board::LEFT ||
            cell == Board::RIGHT
        }

        let mut que  = util::Queue::<u8>::new();
        let mut dist = [None; 64];
        // 始点を踏むには一度出てから戻らないといけないことに注意
        for &to in Board::adjacent(src) {
            if self.v[to as usize] == Board::WALL { continue; }
            que.push(to);
            dist[to as usize] = Some(1);
        }

        while !que.is_empty() {
            let i = que.pop().unwrap();

            if i == dst { break; }
            if is_direction(self.v[i as usize]) { continue; }

            for &to in Board::adjacent(i) {
                if dist[to as usize].is_some() { continue; }
                if self.v[to as usize] == Board::WALL { continue; }
                que.push(to);
                dist[to as usize] = Some(dist[i as usize].unwrap() + 1);
            }
        }

        dist[dst as usize]
    }

    // 解くまでに最低限必要な手数を求める
    // 盤面はまだ解かれておらず、解が存在するものとする(先に is_stuck() チェックが必要)
    // 盤面は正しいものとする(消えるはずのピースが消えていないとかはナシ)
    // ピース間の距離で判断(15パズルのマンハッタン距離枝刈りみたいな感じ)
    pub fn least_to_solve(&self) -> u32 {
        let mut v = [None; Board::N_KIND];
        for i in 0..64 {
            let pi = self.v[i];
            if pi >= Board::N_KIND as u8 { continue; }
            for j in i+1..64 {
                let pj = self.v[j];
                if pi == pj {
                    // "1..<...1" みたいな状態を考慮
                    let d_new = cmp::max(2, Board::DIST[i][j]);
                    v[pi as usize] = match v[pi as usize] {
                        Some(d) => Some(cmp::min(d, d_new)),
                        None    => Some(d_new),
                    }
                }
            }
        }
        v.iter().flat_map(|&e| e).max().unwrap() - 1
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (pos_x, pos_y) = Board::idx2xy(self.pos);
        writeln!(f, "{} {}", pos_x, pos_y)?;
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "{}", match self.v[Board::xy2idx(x,y) as usize] {
                    Board::EMPTY => '.',
                    Board::UP    => '^',
                    Board::DOWN  => 'v',
                    Board::LEFT  => '<',
                    Board::RIGHT => '>',
                    Board::WALL  => '#',
                    n @ 0 ... 9  => char::from(b'0' + n),
                    x            => panic!("unexpected cell: {}", x),
                })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Board {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut it = lines.next().unwrap()
            .split_whitespace()
            .map(|token| token.parse::<u8>().unwrap());
        let pos_x = it.next().unwrap();
        let pos_y = it.next().unwrap();

        let v: Vec<_> = lines
            .take(8)
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '.'         => Board::EMPTY,
                '^'         => Board::UP,
                'v'         => Board::DOWN,
                '<'         => Board::LEFT,
                '>'         => Board::RIGHT,
                '#'         => Board::WALL,
                '0' ... '9' => c as u8 - b'0',
                _           => panic!("unexpected char: {}", c),
            })
            .collect();

        Ok(Self {
            pos: Board::xy2idx(pos_x, pos_y),
            v: BoardArray::clone_from_slice(&v),
        })
    }
}

#[test]
fn test_board() {
    assert_eq!(27, Board::xy2idx(3,3));
    assert_eq!((1,5), Board::idx2xy(41));

    let board_str = "\
1 1
#######4
#010...#
#.<1.^2#
#.2.1..#
3......#
#.v..>.3
#......v
#######4
";
    let board: Board = board_str.parse().unwrap();
    assert_eq!(board_str, format!("{}", board));

    assert_eq!(9, board.pos);
    assert_eq!(vec![18,21,42,45,55], board.moves());
    assert_eq!([2,3,2,2,2,0,0,0,0], board.counts());
    assert!(!board.is_solved());
    assert!(!board.is_stuck());
    assert_eq!(Some(7), board.calc_step(17,45));
    assert_eq!(4, board.least_to_solve());

    {
        let mut board = board.clone();
        board.move_(Board::xy2idx(5,2));
        assert_eq!(Board::from_str("\
5 2
#####.#4
#010.^.#
#.<1..2#
#.2.1..#
3....>.#
#.v....3
#....#.v
#######4
").unwrap(), board);
        assert!(!board.is_stuck());
    }

    {
        let mut board = board.clone();
        board.move_(Board::xy2idx(2,5));
        println!("{}", board);
        assert_eq!(Board::from_str("\
2 5
#######4
#0#0...#
#....^2#
#.<.1..#
3.2....#
#....>.3
#.v....v
##.####4
").unwrap(), board);
        assert!(board.is_stuck());
    }

    {
        let mut board = board.clone();
        board.move_(Board::xy2idx(2,2));
        println!("{}", board);
        assert_eq!(Board::from_str("\
2 2
#######4
#0.0...#
.<..^2##
#.2.1..#
3......#
#.v..>.3
#......v
#######4
").unwrap(), board);
        assert!(board.is_stuck());
    }

    {
        let mut board = board.clone();
        board.move_(Board::xy2idx(5,5));
        println!("{}", board);
        assert_eq!(Board::from_str("\
5 5
#######4
#010...#
#.<1.^2#
#.2.1..#
.......#
.#.v..>.
#......v
#######4
").unwrap(), board);
        assert!(!board.is_stuck());
    }

    {
        let mut board = board.clone();
        board.move_(Board::xy2idx(7,6));
        println!("{}", board);
        assert_eq!(Board::from_str("\
7 6
#######.
#010....
#.<1.^2#
#.2.1..#
3......#
#.v..>.#
#......3
#######v
").unwrap(), board);
    }

    let board = Board::from_str("\
1 1
########
#......#
#......#
#......#
0..<...0
#......#
#......#
########
").unwrap();
    assert_eq!(1, board.least_to_solve());
}
