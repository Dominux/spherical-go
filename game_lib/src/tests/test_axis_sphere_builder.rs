use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    field::{AxisSphereField, AxisSphereFieldBuilder, Field},
    point::{Point, PointWrapper},
};

const COMPRESSED_FIELD: [[usize; 4]; 54] = [
    [20, 9, 1, 3],
    [19, 0, 2, 4],
    [18, 1, 17, 5],
    [0, 10, 4, 6],
    [1, 3, 5, 7],
    [2, 4, 16, 8],
    [3, 11, 7, 12],
    [4, 6, 8, 13],
    [5, 7, 15, 14],
    [0, 20, 10, 21],
    [3, 9, 11, 22],
    [6, 10, 12, 23],
    [6, 11, 13, 24],
    [7, 12, 14, 25],
    [8, 13, 15, 26],
    [8, 14, 16, 27],
    [5, 15, 17, 28],
    [2, 16, 18, 29],
    [2, 17, 19, 30],
    [1, 18, 20, 31],
    [0, 19, 9, 32],
    [9, 31, 22, 33],
    [10, 21, 23, 34],
    [11, 22, 24, 35],
    [12, 23, 25, 36],
    [13, 24, 26, 37],
    [14, 25, 27, 38],
    [15, 26, 28, 39],
    [16, 27, 29, 40],
    [17, 28, 30, 41],
    [18, 29, 31, 42],
    [19, 30, 32, 43],
    [20, 31, 21, 44],
    [21, 43, 34, 45],
    [22, 33, 35, 46],
    [23, 34, 36, 47],
    [24, 35, 37, 48],
    [25, 36, 38, 49],
    [26, 37, 39, 50],
    [27, 38, 40, 51],
    [28, 39, 41, 52],
    [29, 40, 42, 53],
    [30, 41, 43, 54],
    [31, 42, 44, 55],
    [32, 43, 33, 56],
    [39, 46, 38, 48],
    [40, 47, 45, 49],
    [41, 42, 46, 50],
    [45, 49, 37, 51],
    [46, 50, 48, 52],
    [47, 43, 49, 53],
    [48, 52, 36, 35],
    [49, 53, 51, 34],
    [50, 44, 52, 33],
];

#[test]
fn test_axis_sphere_builder_with_size_7() {
    let expected_field = {
        let points = COMPRESSED_FIELD
            .iter()
            .enumerate()
            .map(|(id, p)| {
                Rc::new(RefCell::new(PointWrapper::new(
                    Point::new(id),
                    Some(p[0]),
                    Some(p[1]),
                    Some(p[2]),
                    Some(p[3]),
                )))
            })
            .collect();
        AxisSphereField::new(points, &7)
    };

    let real = AxisSphereFieldBuilder::default().with_size(&3);

    for id in 0..real.len() {
        assert_eq!(
            *expected_field.get_point(&id).borrow(),
            *real.get_point(&id).borrow()
        )
    }
}