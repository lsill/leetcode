/// 1401. 圆和矩形是否有重叠
/// 给你一个以 (radius, xCenter, yCenter) 表示的圆和一个与坐标轴平行的矩形 (x1, y1, x2, y2) ，其中 (x1, y1) 是矩形左下角的坐标，而 (x2, y2) 是右上角的坐标。
/// 如果圆和矩形有重叠的部分，请你返回 true ，否则返回 false。
/// 换句话说，请你检测是否 存在 点 (xi, yi) ，它既在圆上也在矩形上（两者都包括点落在边界上的情况）。
/// 超级有用的题解
// 力扣简单题解1
pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let dx = if x1 > x_center {x1 - x_center} else if x_center > x2 {x_center - x2} else {0};
    let dy = if y1 > y_center {y1 - y_center} else if y_center > y2 {y_center - y2} else {0};
    return dx*dx + dy*dy <= radius*radius;
}
// 力扣比较rust的题解2
pub fn check_overlap_1(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let get_val = |center:i32, v1:i32, v2:i32|->i32 {
        let min = v1.min(v2);
        let max = v1.max(v2);
        if min <= center && center <= max {
            return center;
        }
        if (center - min).abs() > (center - max).abs() {
            return max;
        }
        min
    };
    let x = get_val(x_center, x1, y1);
    let y = get_val(y_center, y1, y2);
    (x - x_center).pow(2) + (y - y_center).pow(2) <= radius * radius
}