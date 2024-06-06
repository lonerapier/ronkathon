pub const ALPHA: usize = 3;
pub const WIDTH: usize = 16;
pub const NUM_F: usize = 8;
pub const NUM_P: usize = 11;
pub fn constants() -> (Vec<usize>, Vec<Vec<usize>>) {
  let rc16: Vec<usize> = vec![
    10, 69, 45, 20, 69, 36, 74, 82, 77, 24, 23, 26, 19, 54, 69, 88, 27, 58, 58, 33, 13, 58, 56, 2,
    74, 72, 33, 83, 2, 49, 29, 8, 41, 30, 47, 82, 83, 30, 73, 94, 68, 81, 16, 51, 33, 93, 42, 96,
    3, 67, 14, 84, 13, 73, 35, 57, 40, 56, 66, 2, 60, 87, 63, 64, 42, 10, 43, 60, 65, 34, 74, 74,
    99, 3, 77, 71, 70, 96, 10, 9, 18, 14, 24, 93, 97, 54, 16, 70, 81, 35, 35, 75, 100, 81, 88, 26,
    22, 29, 90, 12, 50, 65, 82, 89, 48, 23, 22, 8, 42, 62, 5, 52, 33, 17, 13, 40, 37, 2, 27, 85,
    25, 58, 29, 77, 53, 37, 76, 54, 75, 1, 79, 86, 13, 64, 71, 100, 74, 48, 50, 94, 19, 29, 91, 33,
    9, 0, 16, 25, 19, 76, 38, 50, 43, 52, 85, 84, 4, 7, 90, 0, 24, 42, 37, 53, 41, 71, 34, 7, 13,
    65, 3, 79, 46, 38, 66, 12, 14, 81, 79, 31, 51, 83, 33, 24, 75, 47, 77, 9, 65, 37, 30, 90, 60,
    78, 56, 98, 92, 47, 67, 5, 74, 71, 73, 14, 16, 5, 84, 32, 82, 41, 11, 5, 99, 22, 86, 70, 46,
    36, 78, 14, 66, 19, 45, 77, 65, 97, 35, 46, 10, 28, 6, 10, 64, 95, 35, 97, 66, 65, 84, 52, 30,
    40, 69, 32, 5, 29, 70, 77, 24, 18, 97, 75, 24, 53, 45, 94, 17, 78, 9, 19, 67, 53, 66, 69, 93,
    61, 42, 78, 53, 36, 93, 99, 67, 5, 22, 43, 13, 65, 97, 11, 70, 61, 65, 84, 54, 25, 55, 71, 32,
    70, 83, 9, 61, 36, 1, 69, 40, 41, 47, 82, 33, 48, 71, 84,
  ];

  let mds16: Vec<Vec<usize>> = vec![
    vec![72, 34, 99, 22, 24, 11, 70, 84, 77, 1, 59, 58, 26, 81, 80, 55],
    vec![86, 95, 64, 34, 32, 35, 72, 66, 1, 85, 63, 3, 27, 74, 76, 13],
    vec![60, 89, 42, 44, 73, 82, 68, 49, 32, 48, 2, 63, 86, 62, 93, 83],
    vec![90, 14, 46, 82, 69, 29, 66, 92, 28, 93, 86, 27, 25, 98, 74, 2],
    vec![75, 4, 23, 20, 68, 73, 74, 98, 72, 86, 82, 35, 29, 79, 25, 44],
    vec![80, 3, 5, 58, 91, 21, 62, 97, 68, 60, 47, 82, 75, 87, 90, 96],
    vec![43, 39, 34, 21, 49, 100, 98, 40, 66, 90, 75, 29, 42, 12, 79, 47],
    vec![58, 30, 91, 95, 48, 17, 86, 90, 85, 44, 18, 65, 20, 75, 82, 99],
    vec![4, 74, 60, 81, 44, 83, 20, 21, 34, 95, 84, 87, 6, 31, 17, 94],
    vec![70, 22, 13, 47, 100, 75, 49, 65, 69, 77, 60, 86, 90, 97, 62, 45],
    vec![67, 29, 58, 15, 95, 99, 6, 50, 97, 81, 19, 54, 57, 45, 83, 72],
    vec![27, 97, 94, 9, 1, 52, 26, 19, 47, 22, 3, 4, 39, 15, 11, 46],
    vec![74, 20, 89, 27, 94, 8, 81, 36, 70, 72, 76, 11, 15, 67, 19, 85],
    vec![38, 73, 14, 8, 84, 53, 83, 45, 87, 19, 15, 41, 99, 96, 57, 76],
    vec![9, 90, 1, 66, 88, 67, 14, 11, 18, 61, 30, 81, 36, 39, 4, 69],
    vec![95, 72, 48, 70, 13, 87, 34, 82, 46, 56, 51, 62, 97, 20, 65, 24],
  ];
  (rc16, mds16)
}
