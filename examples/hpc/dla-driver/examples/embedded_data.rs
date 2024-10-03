#![no_std]
#![no_main]

extern crate alloc;

use core::arch::asm;

use alloc::vec::Vec;
use dla_driver::tensor3::{Order3, Tensor3};
use dla_driver::tensor4::{Order4, Tensor4};
use dla_driver::*;
use headsail_bsp::apb_uart::ApbUart0;
use headsail_bsp::{init_alloc, rt::entry, sprint, sprintln, unmask_u32};
use panic_halt as _;

const DATA: &[i8] = &[
    115, -98, 74, -63, -38, -101, -106, 116, -15, -27, -56, 59, -57, 115, -65, -120, 104, -40,
    -117, -54, -19, 123, 109, -91, -54, -109, -64, -10, 80, -91, 67, 24, 106, -16, -63, 97, -107,
    -15, 20, -39, 63, 102, -119, -57, -125, 93, 59, 66, 59, -103, 70, -24, 125, 45, 125, -63, -9,
    121, 16, -116, 48, 23, 14, 109, 19, 112, 75, -120, -90, 24, 94, -125, 38, 67, -34, -125, 37,
    16, -60, -5, -44, 119, -128, -69, -24, 69, -3, 42, 38, -54, -119, -9, -55, 38, 77, -9, 68, -70,
    -71, -57, -94, 25, 17, 1, 41, -58, 48, 0, -110, -80, 105, 116, -84, 102, -88, -80, 17, 110,
    -90, 110, 25, 56, 7, -14, 15, 79, 11, 60, -109, 28, -67, -75, -64, 78, -77, -127, -5, -90,
    -116, 109, -103, 88, 20, 25, 63, 33, -37, -36, -43, 60, 34, -53, -93, 69, 56, 21, 115, -122,
    49, 9, 52, 87, -34, -113, 29, 35, -71, -46, 47, 6, -106, -12, -47, -20, 23, 14, -25, 114, -117,
    -118, -79, 119, 111, 41, 72, 84, 74, 37, 121, 9, 20, 86, -41, 51, 56, -101, -26, -90, -96, -25,
    -33, -62, 29, -29, 85, 17, 13, -58, 109, -89, 17, -45, 37, -64, 103, 28, 92, -92, 101, -69, 13,
    -34, 22, -108, 77, 110, 99, -20, -63, 84, -49, -41, -107, 42, 92, -48, 11, -35, 96, 95, 16, 5,
    113, 116, 114, 68, 63, 51, -108, -10, 86, -33, 117, -23, -90, -43, -15, 51, 43, 60, -91, 33,
    -66, 123, -79, 37, -19, 16, 43, -126, 23, 18, 96, -79, -3, -120, -13, 27, 107, 94, 37, 54, -77,
    9, 16, -49, -122, 3, -126, -107, -15, 30, -29, -95, -101, -67, 117, 0, -46, 1, -67, 119, 57,
    27, -80, -46, 113, 88, -96, -116, -14, -70, 57, 82, 84, 25, -128, 66, 55, -5, -33, 49, 120,
    115, 45, 19, 49, -65, -23, -12, 36, -98, 43, -107, -12, -88, 57, 3, -51, 47, 60, -69, -25, 1,
    -117, 56, 47, 66, -124, 84, -43, 0, 1, -53, 29, -55, 66, -128, 45, -91, 45, 84, 100, -124, 11,
    80, -54, 122, -30, 99, 42, -28, -94, -40, -120, -113, 95, -87, -35, 94, 24, -88, -118, 39,
    -128, 111, -13, 99, 111, -40, -27, -93, -76, 24, 13, -10, -69, 63, -11, -106, 117, -31, -99,
    -82, 73, 114, 92, 2, 111, 126, 64, -65, -14, 66, -72, 16, 32, 16, -7, 12, -33, -62, -74, 85,
    -9, 107, -82, 107, 30, 62, 89, 51, -45, -30, 50, -26, -88, 112, -65, -106, 46, -58, 27, -56,
    -70, 36, 10, 119, -63, 113, -111, 0, 61, 88, 117, -108, -92, 84, -93, -54, -13, 18, -20, -37,
    -7, 33, -33, -51, -81, 51, 46, -84, -108, -63, -43, -53, 3, 49, 12, 87, -90, 2, -77, -93, -89,
    -76, 59, 9, -123, 76, 80, -44, -81, -41, -80, -124, 97, -103, 110, 66, -44, -91, -83, -2, -125,
    40, 116, -78, -7, -58, -45, -8, -46, 61, -120, -39, -46, -17, -14, -85, 116, 21, -32, 58, 2,
    17, 3, 94, -68, -29, 57, -45, -68, 39, -25, 4, 108, 103, 95, 114, 51, -45, -104, -9, 61, -106,
    29, 55, 122, -30, 110, -6, 94, -112, 41, -124, 4, 6, 98, 32, 30, 38, 122, -88, -13, -4, -112,
    -5, 13, 30, -80, 47, -11, 55, -20, -30, -34, 17, -9, 119, 15, -66, -50, 126, 48, -20, -99, 43,
    38, 54, 29, 14, 15, 4, 120, 67, -76, 1, 67, 15, 92, -82, 99, -48, 60, -124, -12, 43, 116, -91,
    95, 97, -52, -19, 88, 13, 108, -79, -67, 78, 89, -100, -98, 90, -63, -117, 20, -12, -42, 67,
    -120, 71, -32, 119, -33, -15, -116, -54, -106, 62, -104, -84, -122, 121, -93, 120, -119, 92,
    20, -96, -38, 58, 64, -68, -67, -66, -87, -81, 113, -66, 124, -28, -16, 7, -127, -67, 101,
    -122, -56, 118, 107, 40, -15, 78, -118, -11, 12, -57, 7, -71, 26, 14, -88, 17, -60, 54, 85, 42,
    -102, -25, -4, -81, 25, -122, -18, -41, -71, -122, -122, 84, -34, 89, 65, 36, 73, -10, 62, 122,
    121, 59, 1, 79, -54, -69, 52, 97, -35, 79, -93, 8, -107, 58, -53, 28, -101, -10, -40, -5, -49,
    -80, -53, 0, 62, -103, -110, -63, 23, 30, -100, -64, 93, 42, 67, -95, -26, -31, -94, -11, 69,
    3, 122, -53, -106, -99, -102, 110, -93, -92, -52, -31, -59, -1, 87, 4, 1, -52, -73, -47, -107,
    41, -10, -125, 42, -39, -107, -110, -107, 40, -80, -94, 123, 41, -83, -19, -73, -35, -45, -67,
    -9, 126, -67, -123, -106, -63, 121, -41, -101, -1, -99, 106, 35, 9, 104, 109, -110, -80, -77,
    7, -28, 75, 103, 86, -102, -75, -5, 74, 60, 39, 5, -26, -36, 27, -65, 108, 101, -15, 87, 9,
    -25, -6, -69, 103, -78, -37, -126, 72, -60, -69, -99, -30, -75, -92, -62, -65, -49, 83, 73, 28,
    -24, 2, 38, -49, -124, -7, -16, 109, -121, 68, -45, 72, -63, -114, -123, 63, -101, 15, -58, 3,
    99, -128, 82, -22, -86, -84, -36, -21, -97, 18, 30, 70, 122, 81, -74, 5, -76, 52, -102, -24,
    80, 103, -3, 76, -38, 12, 35, -103, 70, 95, 66, 77, -31, 72, 94, 104, 48, 100, -115, -125, 97,
    -81, -45, 20, -99, 66, -50, 24, -92, -89, 32, -125, -69, -112, 45, -35, 52, 117, 101, -59, -52,
    121, 26, 65, -114, 68, 79, 126, 66, -124, 41, 74, 3, 3, 77, -12, 86, 30, -51, -112, -43, 74,
    -25, -106, 12, 76, -13, 11, 62, 55, 72, 90, 97, -116, 36, 88, -45, -33, -78, -72, -17, 18, 87,
    -6, -10, 118, -74, -4, -10, 4, -17, -25, 91, 20, -124, 98, 40, 41, -62, 22, -102, 85, 65, -10,
    -68, 63, 100, -111, 117, 101, -63, -61, -16, 121, -9, -18, -110, -42, -97, 41, 63, 63, 101, 25,
    121, 33, -39, 48, 19, 31, 30, -24, 76, -11, -38, -90, -106, -114, 22, -112, 81, 42, -114, 30,
    -74, -54, -98, -6, -106, -47, 59, 85, 99, -117, 106, -89, 3, 65, -44, -51, -26, -127, -42, 106,
    110, 59, -113, 99, -85, -16, 117, 96, 11, 38, 121, -91, -3, -3, -55, -118, -97, -16, -106, 21,
    122, -44, 83, 71, -35, -70, 93, -6, 76, 73, -24, 39, -23, -126, -9, 14, 73, -7, 60, -109, -118,
    -62, 44, -94, 103, -66, 70, 86, -26, 29, -117, -53, -43, -35, -27, -78, 104, -77, -7, 46, 25,
    -122, -107, -112, -22, -49, 41, -39, 103, -106, -107, -82, 34, 108, 87, -122, 18, 83, 56, 80,
    -123, -70, 81, 35, 24, -90, 39, -31, 91, 75, 94, 112, 6, 120, 114, 46, -105, 68, 125, 75, 120,
    -38, -61, -73, -113, 122, -108, -77, -29, -123, -71, 10, -60, 81, 58, -44, -78, 75, -119, 118,
    -78, -48, -111, 26, -70, -109, -116, -22, 60, 95, 35, 27, 104, -3, 8, -79, 14, -116, 121, 43,
    -116, -89, -38, -88, -9, -39, -96, -104, -69, 48, 118, 17, 73, -99, -77, 51, 3, 28, -42, 50,
    46, -74, -34, -123, 83, -14, -6, -31, -4, 52, 124, 14, 23, -42, 111, -108, -86, 107, 88, -8,
    56, -71, -52, 40, 20, 1, 72, 6, 29, -1, -24, -128, -127, -113, -94, 51, -15, 126, 90, -80, -24,
    106, -87, -25, 44, -118, 111, -111, 18, -103, -29, 76, 124, 108, 30, 9, -70, -28, 53, -71, 74,
    -51, -115, -26, 92, -115, -28, -103, 21, 89, -15, -48, 58, 87, -104, -15, -29, 55, -32, -22,
    -81, -32, 92, 52, 32, 16, 93, -32, -86, -93, 43, 8, 91, 0, 40, -34, -97, -119, -28, 27, -50,
    -89, -120, 2, 44, -63, 50, -84, -94, 106, -84, 20, -99, -20, 80, 94, -63, 88, 62, -21, 121, 91,
    -19, -62, 41, 41, 37, 46, 70, -126, -126, 21, 103, -49, -107, -33, 80, 73, 69, 101, 112, -49,
    -11, -72, 12, 85, 60, 7, 75, -104, -92, -89, 46, -28, -41, 36, 114, 89, 53, -14, 59, 15, -74,
    54, 94, -21, -55, -25, -82, 123, -72, 44, 53, 80, 121, -32, -64, 16, -79, -109, 40, -95, -104,
    74, 47, -109, 65, 108, -89, 57, 51, 15, -114, 14, 63, -95, -69, 17, 51, -34, -57, 39, -104,
    -101, 42, 118, -8, 49, -59, 120, -27, 113, 59, -39, -48, -29, 56, 39, 48, -17, -55, -35, -57,
    -81, -66, 108, -52, -112, 55, 122, 108, 23, 62, 13, -12, -19, -15, 30, -67, 24, 60, -95, -119,
    57, -24, -51, -10, 48, -98, -113, 78, 19, -81, 123, -107, 98, -3, -1, -66, -31, 16, 59, -122,
    30, 57, -2, -85, 6, 94, 125, -91, -29, -17, -19, -56, -83, 28, -49, -81, -19, -17, -45, 43, 6,
    -42, 104, -115, -108, -120, -52, -70, -82, 82, 114, -47, -117, 25, -121, -8, 62, 1, 114, 54,
    67, -84, 114, -53, 37, 102, -48, 19, 26, -49, -59, -46, 55, 29, 74, 32, -102, -98, 88, 83, 31,
    102, 39, -105, 105, -52, -25, -44, -90, -93, 26, -80, -76, -8, -41, 103, 62, -12, -82, 37, 18,
    -128, 57, -112, -99, 106, 117, -92, -2, 39, 31, 18, -47, -8, 118, -83, 88, -75, 71, 14, -88,
    38, -85, -34, 85, -86, -46, 101, -13, -119, 44, -54, 107, 52, -123, 117, 69, -28, 90, 43, -110,
    28, -104, -110, -98, 120, 54, -85, -95, -96, -79, -68, -121, 122, 84, 27, -127, 50, -77, 39,
    -75, -22, -51, -53, 87, -79, -109, 40, -97, 110, 38, 7, 126, -42, -37, -23, -31, 31, -118, -42,
    54, 71, 48, -87, -13, -8, 82, -33, -128, 70, 57, 111, -51, 118, -85, 41, -12, -32, 86, -81,
    121, 97, -16, -102, -27, 8, 54, -79, -62, -58, 112, -3, -64, 55, -34, 0, -4, -68, -121, -52, 2,
    65, 24, 95, -36, 107, 115, -81, -119, 113, -3, 3, -70, -100, 48, 105, -65, 34, -84, 20, 71,
    -64, 41, -42, 77, 42, -48, 58, -44, 39, -95, 82, 23, 5, 14, -28, -24, 4, -77, 64, -128, -58,
    -46, -45, -17, -43, 0, 117, -45, -127, 72, -127, -25, -108, 5, -116, 24, -48, 110, 6, -3, 56,
    -79, -36, 3, 21, 66, -6, 45, 21, 58, 4, 37, 81, 90, 115, 84, 94, -126, 120, 53, -83, -60, 76,
    -57, 117, -13, 71, 53, -35, 26, -119, -23, -51, -103, -12, 62, -97, 49, -12, -94, 39, 122, -63,
    -9, 22, 21, 118, 88, -29, 9, 56, -123, -127, -20, 46, -33, -33, 47, -79, -127, 38, 4, -5, -76,
    31, 121, 78, 30, 67, -22, -60, 72, 94, -72, -122, 66, 108, 87, 6, 112, 34, -32, -63, 74, 27,
    39, 90, 96, 46, 126, 24, 0, -65, 110, -11, -128, 66, 56, 16, 89, -24, 57, -29, 75, -35, 1, 23,
    -44, -125, 95, 59, 74, -113, -95, 122, 17, -67, -6, 32, -119, -40, -57, 17, 57, 33, 83, 115,
    -75, 20, -87, -106, -121, 69, 115, -61, 1, 78, -80, 94, -50, -99, -128, -87, -82, 70, -109, 56,
    120, 20, -110, 87, -16, -87, 27, -91, 122, -111, 6, -5, -125, 46, -71, -87, -31, -5, -1, -79,
    -59, -87, -93, -22, 90, 112, -73, 39, -50, 85, -69, 113, 107, 33, -17, 36, 83, 45, -44, 124,
    123, -99, 103, 75, 33, 25, -88, 44, -15, -49, -55, -29, -121, -76, 82, 40, 124, -82, -120, -29,
    111, 93, 25, 9, -9, -97, 64, -37, -124, 121, 32, 86, 71, -108, -48, 26, 43, 39, -53, 28, 8,
    -34, 55, -2, -89, 79, -124, -119, 117, -76, 51, -51, -115, 90, -125, -32, -36, -40, 96, 35,
    -79, 10, -126, -70, 82, 6, 10, 74, -102, -28, 109, 5, 90, 43, 120, -85, -97, 18, 33, 80, -80,
    -107, -81, -95, -34, 96, -31, 53, -80, 93, -118, -18, 23, -113, -16, -1, 74, -65, -78, -17, 65,
    112, 62, 95, 111, 95, 96, -74, -80, 100, 16, 79, -119, 126, -13, 79, -116, -21, 120, -36, -5,
    85, -75, -13, 10, -99, -102, -61, 10, -15, -49, -59, 74, -31, 79, 21, -42, -113, 94, 54, -54,
    -52, -31, -22, 93, -83, 26, -11, -56, -90, 111, 103, 119, -65, 111, 117, 88, 28, 26, 60, -121,
    13, 6, -118, 76, 126, 30, 7, 78, 88, 125, 117, -100, 102, -20, -40, -10, -27, -114, 124, 70,
    -18, -107, -103, -84, -50, 17, -104, -30, 11, 34, -66, -80, -9, -81, 85, -18, 11, -3, -106,
    -121, 37, 68, -18, 125, 125, 5, -21, -58, -32, 3, 105, -56, -69, 103, 72, -96, -111, -94, 65,
    -98, -14, 25, -79, 59, 122, 83, -85, -71, 105, -2, 115, -57, 33, 84, -117, 108, 2, -56, 48,
    -55, -108, 120, 82, -25, 30, 64, -38, -80, -95, 18, -57, -38, 74, 36, 49, -46, -115, -127, 119,
    -78, 15, 51, 51, 92, 125, 45, -6, 66, 103, -23, 91, -58, -78, 123, -79, 41, 83, -120, -66, -96,
    -10, -109, -109, -6, 33, 98, 113, 13, 29, 40, -102, 71, 49, -67, 43, 13, -88, 36, -10, -27,
    -17, -76, -101, 113, -55, 27, -69, -126, -52, 18, 18, -87, -114, -113, -113, 37, -59, -90, 27,
    -82, -111, 87, -30, 77, 55, 44, -25, 89, -19, 106, 119, -18, -94, -9, 106, -97, 109, -49, 60,
    71, 54, 24, 86, 43, -121, 121, 37, 85, 117, -55, 18, 4, -24, -109, 80, -107, 54, -85, -42, -49,
    88, -29, 22, -72, -43, 123, -116, 24, 126, -16, 79, 18, 10, 64, -13, -112, -97, 17, -27, -22,
    47, 32, -39, 90, 3, 125, -92, -29, -54, 71, 113, -89, -38, 78, -20, -15, 71, 93, 122, 0, -45,
    15, -25, 85, 119, 30, 16, -35, 27, 2, 22, 107, -55, 69, 60, 34, -89, 7, -26, -95, -109, -95,
    -115, -72, -74, 93, -125, -92, 34, 7, -64, -17, 39, 89, 65, -92, -7, 28, -22, 52, 33, -119,
    -115, 70, 58, -2, 126, 95, 40, -66, 107, -29, 77, 73, 90, 69, 83, -71, 108, 19, 4, 23, -119,
    69, 28, -75, -29, 77, -83, -126, -60, 122, 24, -27, -14, 92, 116, -4, -31, -120, -112, -102,
    -43, -60, -78, -26, -73, 58, -11, 91, -93, 24, 80, -127, 17, 13, -88, 119, 94, 95, 63, -51,
    -67, 42, -41, 53, 34, -23, 12, -53, -104, -37, -30, 54, 89, -107, -87, 18, 19, -20, 117, -126,
    7, 45, -56, -6, -89, -23, -18, -104, 82, -51, -45, 110, 95, -79, -39, -58, -99, -118, 8, -87,
    -126, 61, 61, -66, -96, 40, -125, -30, 12, 99, 122, -90, -2, 63, 79, 66, -29, 30, 116, 101, 86,
    76, -24, -92, -128, 54, 93, 111, -36, -118, -74, 48, -8, -44, 33, 66, 112, 16, -32, 122, 67,
    -38, -60, -99, -103, -2, -32, 99, -45, -91, 56, -19, 71, -49, -38, 15, 71, -123, 35, -55, -53,
    24, 38, -116, 8, 73, -126, 18, 28, -119, -63, -49, -17, -6, 104, 85, -90, 4, -92, -73, -55, 53,
    45, 86, 63, 11, 53, -106, -64, 62, -37, 40, 28, -59, 68, -122, 92, -99, 37, 92, -30, 96, -126,
    -89, 77, -77, -68, 82, 34, -80, 119, -27, 86, -2, -73, -128, -34, 85, -61, -114, -47, -100,
    -111, 58, -47, 75, -85, -42, -15, -20, -6, -67, -47, -32, 87, -10, -72, -49, 28, -97, 44, -30,
    -59, 34, -22, 13, -124, 105, 86, -53, 42, 79, -54, -88, 91, -115, 80, -22, -18, -79, -97, 11,
    11, 117, 77, -79, -15, 43, 126, -108, -51, -59, 105, -63, 78, -47, 44, -125, 62, 76, -1, 113,
    75, -112, 119, -8, -117, -92, 70, -113, -46, 40, -48, 122, 38, 59, 112, 13, -13, 4, -19, -21,
    100, 9, 84, 14, -40, 119, -32, 57, 110, -55, 11, -85, 68, 119, -7, -35, 5, 94, 111, -46, 76,
    95, 61, -21, 79, 125, 116, 85, -26, -102, 122, -34, -120, 123, 64, 25, -96, 67, -88, -65, 73,
    111, -70, 7, 122, -89, 79, -27, -26, -87, -112, 99, -25, 66, 93, 99, 45, -98, -128, 98, -105,
    -35, 1, -94, -71, 102, -19, 90, 88, 93, -75, 113, 33, 117, -27, -66, 61, 51, -56, 92, 102, -34,
    -86, -48, -33, 74, 93, -28, -53, 27, -69, -41, 110, 21, -117, -50, 47, -53, 117, -31, -11,
    -127, -120, 47, -113, 95, 41, 18, 85, -46, 125, -91, 62, -27, -73, 42, -41, -87, -93, -109, 48,
    -39, 107, 14, -57, 14, 95, 105, -66, -15, -1, 79, 39, 121, 78, -69, 107, -100, 25, -128, 126,
    70, 64, -68, -12, -76, 59, -106, 67, 104, -35, -2, -69, 32, 25, 126, 30, -99, -49, 101, -3, 63,
    4, 36, 25, -37, 88, 19, -65, -64, 60, -81, -43, 1, -75, 37, -96, 112, 15, 34, -4, 120, -56, 79,
    73, -84, -125, 123, 1, 91, 105, -92, 1, 6, 84, -54, -89, 73, 115, 113, -120, -70, 96, -97, 7,
    65, 19, -46, -112, -10, -117, 13, 112, -91, 99, 18, -72, 26, -104, -80, -52, -41, -87, 89,
    -112, 66, 121, -63, 101, 47, 41, 41, -19, -128, -113, -22, -46, 93, -72, 119, 11, 104, -125,
    -93, -16, 92, -78, -91, 96, -17, 39, 68, -119, 83, -90, -65, -62, -108, 93, 74, -5, 58, 61,
    124, 87, -7, -48, -87, 116, -39, -68, -36, 119, -82, -89, -93, -110, 65, 3, 57, 20, -22, -27,
    57, -55, -126, -103, 0, 16, 126, -107, -58, 53, -102, 0, 31, -13, 21, 109, 84, 15, -15, -72,
    -29, -74, -92, 92, -25, -94, 73, 40, -58, 120, -45, -31, -60, -82, -90, 113, -87, 14, 73, -56,
    39, 28, 54, 62, 84, -124, -32, 11, 57, 58, 92, 13, -86, 1, -119, 121, -99, -82, 108, -112, -68,
    -16, 48, 90, -89, 7, 77, -33, -127, 73, -79, 34, -85, -38, 56, 124, 59, -114, -81, -88, -61,
    125, -4, -39, -45, -102, 99, -10, -47, -17, 53, -64, 60, -94, 113, -122, -75, 48, -101, -95,
    57, 31, -97, -87, -48, 6, 116, 104, -89, 49, -56, -86, -81, 34, 35, -124, 117, -64, -72, 2,
    114, -110, -46, 69, -28, 1, -13, -62, -103, 35, 3, 111, 37, 109, 6, 31, 104, 11, -15, 100, -33,
    -95, -52, -1, -18, 66, 0, 2, -88, 117, -65, 76, 7, -29, 65, 49, -14, 47, -41, 81, -16, 72, 118,
    -66, -24, 93, 104, 39, -64, -67, 120, -6, -33, -42, 74, -35, -113, -39, 14, 43, -84, 53, -116,
    57, 17, 69, -22, -3, 120, -68, 30, 110, -22, -79, -34, -43, 93, -44, -120, -26, 102, -79, -123,
    -15, 72, 79, 79, -97, -67, -93, 24, -104, 21, 115, -28, 102, 103, -89, 6, 0, 63, -118, -83,
    107, -98, 120, 61, -78, 23, 43, -94, -92, 55, -95, -72, -117, -101, 55, 61, 30, -6, 90, 16,
    -127, -26, 11, -35, -31, -99, -52, 72, -119, -15, -104, -14, -64, -128, -30, 108, 29, 6, 94,
    -14, -122, 112, -15, -67, -63, 108, -82, -84, -126, -71, -113, -86, 43, -72, -66, -37, -55,
    -72, -80, -121, -81, -20, -76, -121, -124, 22, -42, 108, -124, 49, 110, 94, -93, 90, -19, -62,
    114, 58, -55, -4, -41, -51, 84, 46, -36, 74, 102, 8, 111, 65, -98, 117, 97, -99, 44, -27, -107,
    -88, -20, 98, 88, 96, -76, -67, 117, 38, -49, 64, -62, 19, 56, 109, 119, -89, -99, 60, -87, 95,
    76, -91, -16, 104, 25, -111, -110, -60, 18, 110, -27, 37, -30, 43, 58, -30, -38, -28, 109, -17,
    97, -18, -117, 26, 39, 121, -38, 35, 76, -12, 54, -93, -58, -68, 50, 106, -28, 56, -61, 53,
    -71, -112, -120, 34, -39, 83, -55, -107, -54, 35, 19, -82, 6, -56, -125, -46, -109, 66, 8, -2,
    110, 91, 3, 98, 25, -43, -124, -59, 56, 39, -54, 41, -35, -21, -66, 73, -107, -47, 30, 69, 35,
    -106, 10, -67, -98, -5, 63, 31, -111, -114, 52, 73, 26, -29, -77, -73, -93, -114, -52, -58,
    -36, 64, 72, -82, 50, 67, 120, -69, 94, -122, 13, 7, 111, -8, -12, -26, -80, 50, 15, -32, 6,
    -70, -52, -75, -90, 27, -44, -6, 30, 53, -112, -47, 40, -96, -94, 10, 22, -31, 48, 21, 46, 115,
    -72, 89, 14, -78, -43, 63, -125, -126, 100, -93, 35, 74, -8, -16, -72, -118, -18, -121, -116,
    -117, 79, -86, -18, 80, -26, 71, 71, -116, -35, -114, 67, -52, -118, 55, 72, 54, 92, 108, -122,
    -102, -3, 70, -75, -20, 7, 6, -42, -29, -43, -33, -23, -5, 118, 70, 60, 30, -46, -94, -57,
    -120, 59, 115, 122, -121, -87, 48, 80, -91, -91, 20, -85, 91, -89, 6, 5, 28, 29, 59, -124, -78,
    28, -38, 109, -104, -24, 34, 2, 62, -115, -69, -40, -18, 44, 50, 95, -59, -11, 104, 30, 45, 16,
    98, 16, 23, 95, -108, -24, 116, 14, 50, 49, -64, 116, 16, -31, -36, -30, 63, 73, -104, 63, 40,
    -76, 126, -34, 102, 117, 20, -128, 115, -55, 101, 48, 23, -30, -111, -67, -128, 33, 104, -68,
    -128, 12, -93, -94, 111, 51, 42, 32, 76, 3, -58, -93, 23, -7, 15, 117, -50, -50, -19, 53, -25,
    30, -6, 111, 97, 37, 61, 5, 38, -98, -107, -53, 97, -17, -85, -17, -3, -83, -11, 72, -107, -35,
    -71, 88, 43, -28, 106, 78, 114, 67, -108, -22, -49, -50, 59, -93, 71, -89, 60, 15, -3, 14, 21,
    -88, -125, -126, -67, -42, -24, 96, -127, -45, 123, -26, 66, 77, -50, -44, 102, 83, 76, -89,
    -69, 117, 37, 22, 47, 98, -3, 115, -25, 10, 7, 29, 28, 101, 120, 4, -12, 44, 73, -79, -102,
    113, -5, -51, -116, -92, -46, 56, -81, 10, -99, 3, 32, 122, -77, 84, -1, -20, 23, 72, 97, 115,
    105, 119, -126, -96, 13, -71, -35, 67, 7, -127, 10, 7, -21, 36, 109, 90, -46, 61, -48, 82, 84,
    3, 78, 117, -13, -57, -95, -73, -23, -24, -103, -91, 116, -25, -31, -12, 15, 44, -99, -21, 123,
    119, 115, -60, 21, -122, 107, -80, -2, -13, -65, 65, -82, 90, 75, 42, -80, 126, -41, 26, 54,
    -1, -72, 76, -40, -16, 89, -34, -43, -121, 29, -69, 10, 43, -39, -100, 11, 116, -117, 125, -44,
    -106, -59, 27, 48, -51, -75, 80, 9, 102, 29, 19, -80, -115, 67, 26, -84, 115, 101, 21, 125, 91,
    -92, 113, 107, 20, -125, -17, 109, 115, -49, 53, -37, 30, -44, -106, -71, 94, -117, -121, 5,
    55, -10, -95, -116, 36, 83, 112, 57, -6, 31, -107, -22, -21, -56, 45, -93, -40, -6, 29, 121,
    70, -28, -39, -46, 72, 19, -115, 67, 125, 73, -77, 32, -99, 101, -96, -114, 74, -84, 19, -86,
    43, -30, 23, -55, -45, -107, 8, -79, 119, -13, -44, 36, -83, -10, 73, 103, 31, -12, -12, -50,
    55, 96, -108, -73, 19, -119, 6, 82, -116, 110, -106, 125, -42, 121, 126, 67, -13, -118, -20,
    82, 20, 117, -54, -81, -35, 70, -95, 39, 120, 47, 98, -126, -53, 24, 61, -17, -29, 92, -117, 3,
    17, -124, -112, 6, -93, -46, 34, -77, 3, -124, -11, -113, -105, 12, -2, 84, 97, 114, -66, 83,
    -14, 24, 103, 59, -67, 28, -79, 65, -48, -8, 51, 116, 12, 82, 80, 91, 54, 99, -125, 71, 90, 41,
    121, 46, -28, -60, -82, -23, -12, -22, 0, 27, 36, 50, 40, 70, -79, 4, -24, 98, 80, -122, 56,
    -64, 95, -44, 120, 39, 24, -6, -63, -21, 126, -110, 63, -71, -49, 107, -24, -103, 99, -115,
    -107, 22, 71, 63, 66, 87, -11, -126, -57, 91, 83, 53, -117, 114, -115, 94, 59, -22, 19, 75, 75,
    -61, 35, 49, 48, 6, -100, 99, -125, -44, -33, 17, 11, 98, -75, -17, -90, 87, 40, -125, 106,
    -79, 92, 61, -125, 21, -10, -101, 11, 60, 54, -36, 52, -21, 5, -87, -59, -40, -102, 28, -13,
    -61, -111, 116, 3, -112, 22, -106, -9, -65, -78, -20, 38, 31, 60, -104, 29, 48, -21, -32, -5,
    -21, -22, 118, 81, 77, 53, 4, -95, 34, -116, -46, -110, -115,
];

const WGT: &[i8] = &[
    -8, 124, -3, -108, -79, -100, 58, -58, 46, -90, 51, 50, 54, -119, -57, 71, 92, -126, -11, 70,
    26, 102, 27, -63, 60, 75, 108, -113, -59, -51, 65, 88, 88, -92, 2, 121, 110, -65, 42, 119, 4,
    28, -26, 30, -55, -48, -23, 50, -51, -43, -117, 36, -20, 81, -38, -82, -33, -40, 106, -127, 20,
    -114, 45, -44, -66, -63, 66, 123, -114, -109, -44, -110, -68, 51, -61, -58, -54, 77, 37, 45,
    -50, 70, 57, 4, -40, -22, 36, 34, -117, -113, 53, 84, 107, -36, -25, 126, -72, 30, 19, 72, -98,
    -120, -96, 54, -37, 13, -65, -74, 10, 5, -17, 31, -105, -65, -125, 119, -14, 115, 2, 5, 76,
    107, 69, 32, 98, 91, -80, 77, -77, -104, -96, -95, -117, 38, 96, -31, 47, -17, -128, -90, 38,
    84, -125, 123, 33, -76, 83, 71, -61, 76, 73, 113, 7, -125, 38, -30, 34, -105, -65, -62, -114,
    95, 122, -109, -23, -5, 29, -81, -70, -4, -59, 14, 62, -115, -13, 11, 29, 107, 22, 34, 94,
    -121, 40, 73, 14, 27, 68, -118, -27, 27, -22, -111, -11, 68, -60, 69, 63, 70, 103, 110, -42,
    17, 54, 60, 101, -66, -79, -49, 53, 59, 10, -78, 16, -37, 82, 20, 33, -50, 105, -41, -39, -28,
    69, -79, 57, 106, 46, -120, 124, -22, 51, -128, 123, -5, 96, 57, 97, 125, 9, 0, 66, 98, 47,
    -45, -84, 66, -88, -110, -34, -30, -71, -72, -10, -38, 124, 42, -48, -29, -112, -65, -46, -123,
    -125, -55, 6, 48, 109, -126, 112, 85, -116, 66, -47, -77, -10, 5, 124, 83, -28, 95, 8, 8, -31,
    -93, -77, -16, -115, 3, -4, 61, 94, -17, 35, 18, -47, -45, 60, -62, 117, 116, 49, 28, 17, 12,
    60, 62, -14, 47, -53, 125, -69, -12, -76, -61, 80, -69, 79, 99, -25, 44, -35, 102, -120, -36,
    -124, 124, 109, -105, 78, 121, 59, 96, 89, -7, 102, -16, -101, -67, -21, -25, -18, 106, -91,
    26, 6, -68, 52, -43, -106, 37, -102, -86, -101, 49, -90, 98, 82, -50, 49, -109, -74, 55, 112,
    -24, -75, 98, -83, -117, 22, -81, 84, -15, -45, 95, -48, -94, 44, -104, 46, -106, -96, -65,
    -90, 118, -75, 86, -97, -7, -128, -4, -28, -67, 87, 54, -8, -64, 98, 91, 101, -32, 60, -80, 70,
    -99, 33, -54, -40, 95, 49, -43, -1, 61, 66, 107, -103, -57, -65, -120, -35, 29, -119, -12, 118,
    -62, -73, 117, 23, -82, 73, 55, 102, -110, 124, -13, 87, 94, -76, 83, -91, -90, -45, -63, 102,
    -16, 92, -103, -26, -30, -83, -14, 34, -20, -96, 0, 82, 69, -16, 88, 67, 77, 88, -118, -40, 87,
    -110, 102, -127, -21, 8, 6, 62, 35, 93, -49, -122, -77, 42, -52, -121, -88, -117, 106, -110,
    41, 84, -34, -121, 32, -38, 27, -72, 69, 115, -10, -6, -91, -72, -72, 67, 116, -117, 40, 108,
    -21, 37, 102, 5, -33, 84, -27, -128, -11, -106, 71, 114, 125, 24, -51, -18, -128, 53, 34, -84,
    111, 10, 87, -115, -46, -73, 77, -28, 108, 60, 62, -80, 23, -102, -31, -112, 14, 84, -14, 14,
    104, 20, -22, 119, 16, 36, 80, 81, -36, 90, 26, -17, 25, 48, 90, -104, -117, 35, 54, 67, 1, 11,
    72, 23, 54, -61, -108, 67, -24, -42, -36, 64, -16, -61, 72, 121, 72, -74, -63, 68, -4, -118,
    100, 59, 82, 104, -128, -28, -63, 33, -11, 35, -11, -48, -128, -84, 46, -104, 70, -48, -121,
    104, -34, -25, -8, 119, 36, -69, -5, 18, -121, 89, 80, 11, 47, 111, 13, -86, -56, -116, -125,
    74, -111, 46, 58, 119, -31, -28, 36, -78, 16, -62, 69, 67, -67, 79, -12, 116, -34, 102, 112,
    -64, -81, 54, 119, 103, 81, -91, -118, -39, -19, -99, 67, 114, 79, 64, -44, -121, 2, 82, -66,
    93, 77, -112, -51, 71, -80, -55, -38, 35, -32, 56, -62, -11, -86, 29, 63, 38, 24, 73, -59,
    -110, 46, 11, -94, -74, -105, -1, -14, -125, -36, 82, -61, -50, -39, 15, 9, -99, -24, -28, -36,
    -29, -53, -49, 25, -112, -96, -62, -93, 12, -6, -42, 15, -72, 83, -127, 89, 78, -8, -55, -71,
    -121, -90, 20, -79, 12, 22, 25, 4, -78, 100, -72, -16, 26, 121, 62, -27, -4, 108, 43, -43,
    -126, 45, -118, 49, 18, 17, -100, 70, 31, -53, -101, 4, -41, -12, 123, 37, -114, -12, 31, -30,
    75, -81, -65, 88, 28, 21, -6, -54, -7, 64, -61, 96, -107, 65, 119, 42, 102, 66, -49, -78, -84,
    -34, 33, 58, 95, -59, -14, -59, 105, -93, 109, -2, -79, 115, -48, -61, 54, 61, -97, -33, 83,
    111, 7, -128, 55, 100, 120, 45, -5, 83, -77, 69, -47, -38, 43, -20, 60, 33, 14, -26, -101, 40,
    -61, 25, 71, 107, -126, -46, -62, 104, 52, 10, -5, 22, 99, -116, -55, -62, 18, -34, 62, 99,
    -118, 20, 13, 79, 103, 52, 89, -64, 33, -34, -9, -23, 5, 63, -128, -102, 54, -54, 0, 103, 4,
    -58, 2, 17, -42, 13, 114, -49, -84, 69, -122, -47, 45, 67, -2, -78, 33, 17, -7, 118, -32, -94,
    -91, 37, 107, -41, -21, 82, -100, -127, 61, -51, 117, -1, -105, -63, -49, -39, -128, -5, -82,
    78, -1, 37, 79, 77, -65, -97, -94, -56, 96, 73, 113, -43, -61, -90, 80, 53, 4, -13, 101, 1, -9,
    -110, 76, 49, 108, -76, -44, 24, 62, -107, -68, -82, -125, 6, 20, -100, -10, 2, -30, -59, 43,
    8, 62, -124, 30, -43, 83, 13, 12, 45, -76, -5, -28, 2, 90, 43, -74, 98, 54, -74, 83, -16, -103,
    -128, -6, 76, 122, -43, -49, 94, 109, 9, 78, -108, 52, -70, 103, -100, -100, 94, -79, 62, -93,
    -27, -44, 73, 63, 19, 19, -37, 51, -5, -81, -60, -38, -98, -21, 63, -4, 4, 112, -47, 92, -11,
    -21, 65, 102, 107, 42, -107, -21, 48, -94, 27, -83, -27, 112, 76, -17, 35, -7, -84, 120, 61,
    44, 17, 115, -53, -23, 40, -18, -42, -19, 30, 87, 57, -117, 122, 54, 125, -77, 62, -34, -44,
    -37, 89, 66, 11, 89, 89, -56, 71, 95, -26, -76, -1, -122, -24, -37, -73, 3, 52, -51, -65, -44,
    85, -1, -20, -119, 80, -50, -84, -38, -8, 7, 126, 99, 106, 103, -16, 119, 4, 49, -12, 1, 97,
    74, -99, -76, -80, -10, -71, 64, -116, 2, -100, 113, 2, -62, -90, 75, -66, 47, -75, 53, -105,
    -2, -78, -70, 108, -21, 65, 45, -32, 118, 27, -76, -71, -23, -61, -70, 5, -45, -7, 117, 112,
    57, 24, 35, 30, 71, -45, -38, 46, 60, -3, 58, 80, -24, -125, 114, -86, -35, 67, -68, -92, 49,
    123, -61, 2, 78, -103, -69, -42, -60, 123, 51, 33, -80, 38, -14, -67, 89, -105, 15, 125, -123,
    23, -15, -5, 29, 122, -32, 123, -22, -99, -89, -46, -108, 53, 49, 50, -84, -117, 123, -77, 109,
    71, -62, -84, -34, 78, 73, 87, 39, -3, 28, -76, -76, -27, -41, -76, 94, 4, 121, 109, 61, -74,
    18, -32, -114, -49, 26, 62, 37, 12, 9, 105, -90, -62, -40, 59, -68, 81, -96, 68, -4, -79, -87,
    83, 117, -82, 126, 84, -101, 4, -16, -2, -33, -36, -97, -99, 114, -105, -29, 126, 104, 123,
    -28, 14, -53, 40, -37, 63, 37, 49, 124, -113, 42, 118, -62, -4, 44, 68, 109, -91, 69, -90, -17,
    -64, -95, 116, -107, -119, 121, -99, 16, -38, -113, 34, -106, -61, 65, 99, 42, -38, 9, -123,
    -123, -97, 49, 111, -22, 10, 16, -67, 49, 22, -60, 120, -104, -127, 88, 95, 32, -55, -67, 124,
    55, -94, 91, -55, 18, -100, -22, 62, -59, -22, 20, -60, -82, 0, -5, 98, -114, -101, -97, -24,
    -125, -117, 125, 1, -126, 97, 78, 63, -105, 20, -84, 88, -10, 29, -24, -90, 43, -10, 52, -48,
    62, -42, -120, -82, 51, 7, 92, -3, 68, 20, -2, -38, 104, 102, -58, 71, -71, -112, 36, -47,
    -120, 9, -124, 114, 116, -102, 55, -101, 45, 19, 5, 62, 65, 79, 115, -53, 73, 101, -42, 93,
    -51, 46, 65, -6, -12, 36, 75, 110, -1, 84, 55, -39, -91, 1, -66, 9, 58, 13, 111, 84, 52, -27,
    -25, -72, -80, -74, 103, -39, -67, -96, 125, 53, 7, 71, 6, 53, -27, 44, -47, -114, 107, 73,
    -114, 0, 8, -62, -8, -7, -80, 38, -39, -81, 114, -105, -11, -17, 70, 62, -13, -113, -10, -51,
    50, 68, -100, 114, -56, -36, -73, 19, -31, -98, 27, -105, 21, 35, 69, 89, 7, 36, 75, 0, -96,
    -116, -100, 16, 83, 99, 40, -100, -95, 49, -37, -31, 55, -73, -72, -100, 81, -111, 36, 62, 27,
    -74, -76, -127, 100, 96, 0, 98, -37, -33, 103, -86, 104, -114, 52, 28, -3, -103, -55, -8, 4,
    -46, 81, -84, 23, 10, 44, 54, -110, 37, 53, 82, 42, -61, -59, -40, -111, -39, 95, -76, -35,
    -117, -56, 8, -13, -118, -17, 43, 47, -39, -46, 60, 103, 34, 126, 126, 6, 89, 5, 40, -19, -17,
    119, 64, 124, -94, -114, -75, -45, -32, 20, 93, -59, -118, -60, 40, 122, -4, -28, 25, 52, 44,
    -70, -43, -33, -61, -71, -104, -72, 53, 9, -46, -31, 62, 3, 93, 121, -116, -22, -106, 118, 118,
    80, -52, -76, -93, 31, -66, -2, -95, -116, -64, 111, -107, -84, -53, 53, -65, 98, 92, -60, 27,
    80, 10, 107, -37, 49, -21, -50, -24, -82, 26, -48, -83, -59, 53, -3, -71, -52, 5, -27, 79, 92,
    -100, 32, 67, 123, -70, 110, 71, 95, -13, -77, -63, 94, -30, -14, 94, 21, -119, -43, 126, -90,
    46, 38, 124, 15, -89, 82, -57, 109, -21, -14, 114, -31, 124, 108, -109, -111, 15, 119, 111, 95,
    51, -14, 53, 85, -123, -8, -14, -29, 69, 111, -52, -82, 1, 92, 27, -45, -1, 63, 8, 39, 67, -87,
    66, -64, 125, -62, -67, 22, 83, -39, 14, -39, -4, 96, -82, -42, 39, -78, -91, -76, 76, -127,
    122, 81, 49, -90, -26, 100, -111, 102, -108, 98, -79, 25, 23, 121, 123, -73, -42, 52, -52,
    -128, -55, 2, -102, 88, -78, -9, -57, 9, 96, -109, -108, -1, 54, -7, -39, -78, 42, -12, 41, 24,
    -106, 63, 105, -83, 100, -124, -55, -98, -36, 36, -16, 37, -5, 21, -50, 4, -28, 100, -37, 5,
    -56, -50, -17, 65, -16, -104, 11, 47, -33, -60, -70, 48, -100, -53, -6, 66, 61, 99, 23, 116,
    20, -114, 9, -126, 18, -20, -110, 31, 113, -127, 100, -27, -44, 60, -126, -97, 50, -107, 12,
    64, 53, 76, -10, 65, 15, -12, -34, -33, 69, 40, -96, 39, -36, 68, 81, 123, -14, -28, -38, 42,
    -90, 24, 0, -84, -117, 0, -69, -82, 34, 41, -124, 54, -20, 94, 11, 46, -4, -86, 114, 110, -103,
    -95, 100, -88, 102, -45, -39, 16, 63, 100, -67, -103, 11, 93, 35, 4, 44, 25, 38, -103, 20, 69,
    -32, 83, -81, -59, 116, -12, 40, 73, 104, -37, 112, -49, 2, -31, -3, 37, -20, -67, -75, 52,
    -128, -97, -117, -115, -46, -3, 49, -66, 98, -1, 78, 10, 55, -65, 33, 21, -16, 39, -58, 5, -66,
    68, 100, -60, -117, -43, 8, 17, -57, -53, -46, -40, 93, 56, 61, -42, -89, -11, 116, -126, -80,
    -23, -72, -82, -24, 63, -15, -68, -28, -59, 85, -34, -47, -105, -117, 97, 31, 4, -105, 43, 19,
    -127, -33, -72, 68, -45, -64, -77, 24, -43, 73, 45, 30, -47, 53, 47, 59, -96, -23, -36, -69,
    -64, 96, 22, 124, -83, -18, -125, 78, -109, -67, -18, -51, 31, 108, -24, -97, 55, -35, 108,
    -95, -87, 81, -81, 118, 51, 90, -100, -52, 112, 108, 97, 124, -26, -118, -41, -29, 106, -115,
    -121, 1, 48, 32, -126, -40, 32, 14, 119, 126, -6, -94, 57, -76, -82, -38, -125, -33, 87, -119,
    30, 96, -6, -127, 92, 34, -56, -60, 86, -30, -34, 102, 109, 81, 32, 85, -63, 56, 123, 6, 88,
    -47, -93, -113, -72, 116, 50, 101, 26, 61, 77, 26, 56, -124, -33, 112, -51, -29, 72, -65, -34,
    38, 89, -105, 53, 98, -51, 64, -51, -115, -46, -49, -122, -44, -44, -18, -8, 103, 55, -37, 13,
    -19, 74, -35, -101, -50, -40, -67, 27, -99, 37, 35, -76, 12, -8, -55, 117, -77, -22, -104, 95,
    -3, 22, 111, -42, -86, 71, 63, 50, -31, -54, 4, -54, -69, 69, 21, -14, -13, -119, -34, 112, 92,
    -109, 61, 1, 112, 84, 90, -41, 54, -25, 60, -39, -64, -4, 64, -100, -79, 117, -22, 5, -65, -87,
    6, -123, 38, -50, 2, -13, -124, 114, 87, 13, -40, -81, -42, -14, 123, -49, 85, -104, 45, 96,
    36, 26, 75, -74, 120, -28, 81, -82, -32, -114, -78, -5, 64, 1, -47, 9, -56, 77, -74, -75, 25,
    53, -6, 79, 100, 45, 53, 60, 98, 103, -20, 92, -39, -114, -94, 43, -127, -63, 6, -128, -82, 42,
    104, -31, -103, -5, 2, 97, 79, -3, -56, 94, 124, -122, -34, -22, 113, -31, 59, 83, 67, -33,
    -55, -64, -53, -51, -65, -87, 54, 101, 88, -85, 61, -103, -62, -87, 120, -66, -41, -6, 55, 17,
    -99, 12, -111, -82, 28, -52, -83, -21, -13, -7, -66, 100, 47, 86, 72, 34, 49, -86, 111, -117,
    -99, -126, 56, -68, 36, 38, 0, 27, -10, -26, -105, 44, 115, -41, 3,
];

pub const NOPS_PER_SEC: usize = match () {
    // These are experimentally found values
    #[cfg(debug_assertions)]
    () => 6_000,
    #[cfg(not(debug_assertions))]
    () => 120_000,
};

fn run_embedded_data() {
    let din_tensor: Tensor3<i8> =
        Tensor3::from_data_buffer(16, 16, 16, DATA.to_vec(), Order3::HWC).unwrap();
    let wgt_tensor: Tensor4<i8> =
        Tensor4::from_data_buffer(16, 16, 3, 3, WGT.to_vec(), Order4::HWKC).unwrap();
    let output: Tensor3<i8> =
        dla_driver::layers::conv2d(din_tensor, wgt_tensor, None, None, None, None, None);

    for x in output.to_buffer() {
        sprint!(" {:?}", x);
    }
}

#[entry]
fn main() -> ! {
    for _ in 0..NOPS_PER_SEC {
        unsafe { asm!("nop") };
    }

    // Disable GPIO behavior for UART pins
    const PAD_CONF_UART0_TX: usize = 0x1_fff0_7064;
    unmask_u32(PAD_CONF_UART0_TX, (0b1 << 5) | (0b1 << 10));

    let mut _uart = ApbUart0::init(30_000_000, 9600);
    sprintln!("Start!");
    init_alloc();
    sprintln!("Allocator initialized!");

    run_embedded_data();
    sprintln!("Done!");
    loop {}
}
