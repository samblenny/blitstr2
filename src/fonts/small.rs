// DO NOT MAKE EDITS HERE because this file is automatically generated.
// To make changes, see blitstr2/codegen/main.go
//
// This code includes encoded bitmaps of glyphs from the Geneva typeface which
// was designed by Susan Kare and released by Apple in 1984. Geneva is a
// registered trademark of Apple Inc. The PNG sprite sheets of Geneva glyphs came
// from screenshots taken on Macintosh System 7, with additional Latin Extended
// characters added by Sam Blenny.
//
//! Small Font
#![allow(dead_code)]

/// Maximum height of glyph patterns in this bitmap typeface.
pub const MAX_HEIGHT: u8 = 12;

/// Unicode character codepoints corresponding to glyph sprites in GLYPHS array.
/// Indended use:
///  1. Do binary search on CODEPOINTS to find index of the codepoint corresponding
///     to the glyph you want to locate
///  2. Multiply resulting CODEPOINTS index by 8 (<<3) to get index into GLYPHS for
///     the corresponding glyph sprite (because 16*16px sprite size is 8*u32)
pub const CODEPOINTS: [u32; 207] = [
0x00020,
0x00021,
0x00022,
0x00023,
0x00024,
0x00025,
0x00026,
0x00027,
0x00028,
0x00029,
0x0002A,
0x0002B,
0x0002C,
0x0002D,
0x0002E,
0x0002F,
0x00030,
0x00031,
0x00032,
0x00033,
0x00034,
0x00035,
0x00036,
0x00037,
0x00038,
0x00039,
0x0003A,
0x0003B,
0x0003C,
0x0003D,
0x0003E,
0x0003F,
0x00040,
0x00041,
0x00042,
0x00043,
0x00044,
0x00045,
0x00046,
0x00047,
0x00048,
0x00049,
0x0004A,
0x0004B,
0x0004C,
0x0004D,
0x0004E,
0x0004F,
0x00050,
0x00051,
0x00052,
0x00053,
0x00054,
0x00055,
0x00056,
0x00057,
0x00058,
0x00059,
0x0005A,
0x0005B,
0x0005C,
0x0005D,
0x0005E,
0x0005F,
0x00060,
0x00061,
0x00062,
0x00063,
0x00064,
0x00065,
0x00066,
0x00067,
0x00068,
0x00069,
0x0006A,
0x0006B,
0x0006C,
0x0006D,
0x0006E,
0x0006F,
0x00070,
0x00071,
0x00072,
0x00073,
0x00074,
0x00075,
0x00076,
0x00077,
0x00078,
0x00079,
0x0007A,
0x0007B,
0x0007C,
0x0007D,
0x0007E,
0x000A0,
0x000A1,
0x000A2,
0x000A3,
0x000A4,
0x000A5,
0x000A6,
0x000A7,
0x000A8,
0x000A9,
0x000AA,
0x000AB,
0x000AC,
0x000AD,
0x000AE,
0x000AF,
0x000B0,
0x000B1,
0x000B2,
0x000B3,
0x000B4,
0x000B5,
0x000B6,
0x000B7,
0x000B8,
0x000B9,
0x000BA,
0x000BB,
0x000BC,
0x000BD,
0x000BE,
0x000BF,
0x000C0,
0x000C1,
0x000C2,
0x000C3,
0x000C4,
0x000C5,
0x000C6,
0x000C7,
0x000C8,
0x000C9,
0x000CA,
0x000CB,
0x000CC,
0x000CD,
0x000CE,
0x000CF,
0x000D0,
0x000D1,
0x000D2,
0x000D3,
0x000D4,
0x000D5,
0x000D6,
0x000D7,
0x000D8,
0x000D9,
0x000DA,
0x000DB,
0x000DC,
0x000DD,
0x000DE,
0x000DF,
0x000E0,
0x000E1,
0x000E2,
0x000E3,
0x000E4,
0x000E5,
0x000E6,
0x000E7,
0x000E8,
0x000E9,
0x000EA,
0x000EB,
0x000EC,
0x000ED,
0x000EE,
0x000EF,
0x000F0,
0x000F1,
0x000F2,
0x000F3,
0x000F4,
0x000F5,
0x000F6,
0x000F7,
0x000F8,
0x000F9,
0x000FA,
0x000FB,
0x000FC,
0x000FD,
0x000FE,
0x000FF,
0x00152,
0x00153,
0x02018,
0x02019,
0x0201A,
0x0201B,
0x0201C,
0x0201D,
0x0201E,
0x0201F,
0x02020,
0x02021,
0x02022,
0x02026,
0x020AC,
0x0FFFD,
];

/// Packed 16px * 16px glyph pattern data.
/// Pixels are packed in row-major order with LSB of first pixel word
/// containing the top left pixel. Bit of 0 means clear, 1 means set
pub const GLYPHS: [u32; 1656] = [
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050000, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x001f0014, 0x001f000a, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0004, 0x00050015, 0x0014000e, 0x000e0015, 0x00000004, 0x00000000, 0x00000000,
0x00000000, 0x00fe0000, 0x00290049, 0x00980076, 0x00620094, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0012000c, 0x0004000a, 0x0011002a, 0x00460029, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00010001, 0x00010001, 0x00020001, 0x00000004, 0x00000000, 0x00000000,
0x00000000, 0x00020001, 0x00040004, 0x00040004, 0x00020004, 0x00000001, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x000e0015, 0x0011000a, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00040004, 0x0004001f, 0x00000004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00020000, 0x00010002, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x0000000f, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00010000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00080008, 0x00040004, 0x00020002, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00020003, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00100011, 0x00040008, 0x001f0002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00040008, 0x0010000e, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00080000, 0x000a000c, 0x001f0009, 0x00080008, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x000f0001, 0x00100010, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000c0000, 0x00010002, 0x0011000f, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00080010, 0x00040008, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00110011, 0x0011000e, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00110011, 0x0010001e, 0x00060008, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00010000, 0x00000000, 0x00010000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00020000, 0x00000000, 0x00020000, 0x00010002, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00020004, 0x00020001, 0x00000004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x001f0000, 0x001f0000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00020001, 0x00020004, 0x00000001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00060000, 0x00080009, 0x00020004, 0x00020000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x001c0000, 0x00590022, 0x00550055, 0x00020039, 0x0000001c, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00110011, 0x0011000f, 0x000f0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00010011, 0x00010001, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00070000, 0x00110009, 0x00110011, 0x00070009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00010001, 0x00010007, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00010001, 0x00010007, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00010011, 0x00110019, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x00110011, 0x0011001f, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00100000, 0x00100010, 0x00110010, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x00050009, 0x00050003, 0x00110009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00410000, 0x00550063, 0x00410049, 0x00410041, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00130000, 0x00150013, 0x00190015, 0x00110019, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00110011, 0x0001000f, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00110011, 0x00110011, 0x000e0015, 0x00000008, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00110011, 0x0005000f, 0x00110009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00010011, 0x0010000e, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x001f0000, 0x00040004, 0x00040004, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x00110011, 0x000a000a, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00410000, 0x002a0041, 0x0014002a, 0x00140014, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x000a0011, 0x000a0004, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x000a0011, 0x00040004, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000f0000, 0x00040008, 0x00010002, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010003, 0x00010001, 0x00010001, 0x00010001, 0x00000003, 0x00000000, 0x00000000,
0x00000000, 0x00010001, 0x00020002, 0x00040004, 0x00080008, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020003, 0x00020002, 0x00020002, 0x00020002, 0x00000003, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x003f0000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00000002, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00060000, 0x000e0008, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00070001, 0x00090009, 0x00070009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00060000, 0x00010009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00080000, 0x000e0008, 0x00090009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00060000, 0x000f0009, 0x00060001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000c0000, 0x00070002, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00090009, 0x000e0009, 0x00060008, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00070001, 0x00090009, 0x00090009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010000, 0x00010001, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x00040000, 0x00040004, 0x00040004, 0x00030004, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00090001, 0x00030005, 0x00090005, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00370000, 0x00490049, 0x00490049, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00070000, 0x00090009, 0x00090009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00060000, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00070000, 0x00090009, 0x00070009, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00090009, 0x000e0009, 0x00080008, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000d0000, 0x00010003, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000e0000, 0x00060001, 0x00070008, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00070002, 0x00020002, 0x00040002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00090000, 0x00090009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x000a000a, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00410000, 0x002a002a, 0x00140014, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x0004000a, 0x0011000a, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x000a0011, 0x0004000a, 0x00030004, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x000f0000, 0x00020004, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00020002, 0x00020001, 0x00020002, 0x00000004, 0x00000000, 0x00000000,
0x00000000, 0x00010001, 0x00010001, 0x00010001, 0x00010001, 0x00000001, 0x00000000, 0x00000000,
0x00000000, 0x00020001, 0x00020002, 0x00020004, 0x00020002, 0x00000001, 0x00000000, 0x00000000,
0x00000000, 0x00160000, 0x0000000d, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00010000, 0x00010001, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x0015000e, 0x00150005, 0x0004000e, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000c0000, 0x00020002, 0x00020007, 0x000f0012, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x0011000e, 0x00110011, 0x0011000e, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00110000, 0x001f000a, 0x001f0004, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010001, 0x00010001, 0x00010000, 0x00010001, 0x00000001, 0x00000000, 0x00000000,
0x00000000, 0x0011000e, 0x000e0001, 0x000e0011, 0x00110010, 0x0000000e, 0x00000000, 0x00000000,
0x00000000, 0x00050000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0042003c, 0x00850099, 0x00990085, 0x003c0042, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050002, 0x00050006, 0x00000006, 0x00000007, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00240000, 0x00090012, 0x00240012, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x0008000f, 0x00000008, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x0000000f, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0042003c, 0x00a5009d, 0x00a5009d, 0x003c0042, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00070000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00090006, 0x00060009, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00040004, 0x0004001f, 0x001f0004, 0x00000000, 0x00000000, 0x00000000,
0x00070000, 0x00070004, 0x00070001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00070000, 0x00070004, 0x00070004, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00240000, 0x00120024, 0x002e0012, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x003e0000, 0x00170017, 0x00140016, 0x00140014, 0x00140014, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00010000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00010002, 0x00000000, 0x00000000,
0x00020000, 0x00020003, 0x00070002, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00060000, 0x00090009, 0x00000006, 0x0000000f, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00090000, 0x00240012, 0x00090012, 0x00000000, 0x00000000, 0x00000000,
0x00430042, 0x00220022, 0x01500017, 0x01c80148, 0x01040104, 0x00000000, 0x00000000, 0x00000000,
0x00430042, 0x00220022, 0x01d00017, 0x01c80108, 0x01c40044, 0x00000000, 0x00000000, 0x00000000,
0x00440047, 0x00240027, 0x01500017, 0x01c80148, 0x01040104, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040000, 0x00040000, 0x00010002, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00040002, 0x00040000, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x00040008, 0x00040000, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x000a0004, 0x00040000, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x000d0016, 0x00040000, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x000a0000, 0x00040000, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x0011000e, 0x0004000e, 0x000a0004, 0x001f000a, 0x00110011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00f80000, 0x00140014, 0x0012007e, 0x00f10011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00010011, 0x00010001, 0x000e0011, 0x00020004, 0x00000000, 0x00000000,
0x00040002, 0x000f0000, 0x00010001, 0x00010007, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00020004, 0x000f0000, 0x00010001, 0x00010007, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00050002, 0x000f0000, 0x00010001, 0x00010007, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00050000, 0x000f0000, 0x00010001, 0x00010007, 0x000f0001, 0x00000000, 0x00000000, 0x00000000,
0x00020001, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00010002, 0x00010000, 0x00010001, 0x00010001, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00050002, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00050000, 0x00020000, 0x00020002, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000e0000, 0x00220012, 0x00220027, 0x000e0012, 0x00000000, 0x00000000, 0x00000000,
0x000d0016, 0x00130000, 0x00150013, 0x00190015, 0x00110019, 0x00000000, 0x00000000, 0x00000000,
0x00040002, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00040008, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x000a0004, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x000d0016, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x000a0000, 0x000e0000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00110000, 0x0004000a, 0x0011000a, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x005c0000, 0x00320022, 0x0026002a, 0x001d0022, 0x00000000, 0x00000000, 0x00000000,
0x00040002, 0x00110000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00040008, 0x00110000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x000a0004, 0x00110000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x000a0000, 0x00110000, 0x00110011, 0x00110011, 0x000e0011, 0x00000000, 0x00000000, 0x00000000,
0x00040008, 0x00110000, 0x000a0011, 0x00040004, 0x00040004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00090007, 0x00070009, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00060000, 0x00050009, 0x00090005, 0x000d0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040002, 0x00060000, 0x000e0009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00060000, 0x000e0009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000a0004, 0x00060000, 0x000e0009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0005000a, 0x00060000, 0x000e0009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000a0000, 0x00060000, 0x000e0009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00060000, 0x00060009, 0x00060000, 0x000e0009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00360000, 0x007e0048, 0x00360009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00060000, 0x00010009, 0x00060009, 0x00020004, 0x00000000, 0x00000000,
0x00000000, 0x00040002, 0x00060000, 0x000f0009, 0x00060001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00060000, 0x000f0009, 0x000e0001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050002, 0x00060000, 0x000f0009, 0x00060001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00090000, 0x00060000, 0x000f0009, 0x000e0001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020001, 0x00020000, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010002, 0x00010000, 0x00010001, 0x00010001, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050002, 0x00020000, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050000, 0x00020000, 0x00020002, 0x00020002, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00030005, 0x000a0004, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0005000a, 0x00070000, 0x00090009, 0x00090009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040002, 0x00060000, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00060000, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050002, 0x00060000, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0005000a, 0x00060000, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00090000, 0x00060000, 0x00090009, 0x00060009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000004, 0x0000001f, 0x00000004, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x0012002c, 0x0016001a, 0x000d0012, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040002, 0x00090000, 0x00090009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020004, 0x00090000, 0x00090009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00050002, 0x00090000, 0x00090009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00090000, 0x00090000, 0x00090009, 0x000e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00040008, 0x00110000, 0x000a0011, 0x0004000a, 0x00030004, 0x00000000, 0x00000000,
0x00000000, 0x00010000, 0x00070001, 0x00090009, 0x00070009, 0x00010001, 0x00000000, 0x00000000,
0x00000000, 0x000a0000, 0x00110000, 0x000a0011, 0x0004000a, 0x00030004, 0x00000000, 0x00000000,
0x00000000, 0x00fe0000, 0x00110011, 0x00110071, 0x00fe0011, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00360000, 0x00790049, 0x003e0009, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00010002, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020002, 0x00000001, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00020000, 0x00010002, 0x00000000, 0x00000000,
0x00000000, 0x00010001, 0x00000002, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0005000a, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x000a000a, 0x00000005, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000a0000, 0x0005000a, 0x00000000, 0x00000000,
0x00000000, 0x00050005, 0x0000000a, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00070002, 0x00020002, 0x00000002, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00020000, 0x00070002, 0x00020002, 0x00070002, 0x00020002, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x001f000e, 0x001f001f, 0x0000000e, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00490000, 0x00000000, 0x00000000, 0x00000000,
0x00000000, 0x0022001c, 0x0002000f, 0x0022000f, 0x0000001c, 0x00000000, 0x00000000, 0x00000000,
0x00100000, 0x006c0038, 0x01df00d6, 0x00fe01ef, 0x0038006c, 0x00000010, 0x00000000, 0x00000000,
];

/// Widths for proportional glyphs
pub const WIDTHS: [u8; 207] = [
4,
1,
3,
5,
5,
8,
7,
1,
3,
3,
5,
5,
2,
4,
1,
4,
5,
2,
5,
5,
5,
5,
5,
5,
5,
5,
1,
2,
3,
5,
3,
4,
7,
5,
5,
5,
5,
4,
4,
5,
5,
1,
5,
5,
4,
7,
5,
5,
5,
5,
5,
5,
5,
5,
5,
7,
5,
5,
4,
2,
4,
2,
3,
6,
2,
4,
4,
4,
4,
4,
4,
4,
4,
1,
3,
4,
1,
7,
4,
4,
4,
4,
4,
4,
3,
4,
5,
7,
5,
5,
4,
3,
1,
3,
5,
4,
1,
5,
5,
5,
5,
1,
5,
3,
8,
3,
6,
4,
4,
8,
3,
4,
5,
3,
3,
2,
6,
6,
1,
2,
3,
4,
6,
9,
9,
9,
4,
5,
5,
5,
5,
5,
5,
8,
5,
4,
4,
4,
4,
2,
2,
3,
3,
6,
5,
5,
5,
5,
5,
5,
5,
7,
5,
5,
5,
5,
5,
4,
5,
4,
4,
4,
4,
4,
4,
7,
4,
4,
4,
4,
4,
2,
2,
3,
3,
4,
4,
4,
4,
4,
4,
4,
5,
6,
4,
4,
4,
4,
5,
4,
5,
8,
7,
2,
2,
2,
2,
4,
4,
4,
4,
3,
3,
5,
7,
6,
9,
];
