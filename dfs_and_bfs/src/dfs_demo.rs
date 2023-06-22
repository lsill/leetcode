///你有一个用于表示一片土地的整数矩阵land，该矩阵中每个点的值代表对应地点的海拔高度。
/// 若值为0则表示水域。由垂直、水平或对角连接的水域为池塘。
/// 池塘的大小是指相连接的水域的个数。编写一个方法来计算矩阵中所有池塘的大小，返回值需要从小到大排序。
/// 示例：
/// 输入：
/// [
///   [0,2,1,0],
///   [0,1,0,1],
///   [1,1,0,1],
///   [0,1,0,1]
/// ]
/// 输出： [1,2,4]


pub fn pond_sizes(land: Vec<Vec<i32>>)->Vec<i32> {
    let mut res = vec![];
    let mut land = land.clone();
    for i in 0..land.len() {
        for j in 0..land[0].len() {
            if land[i][j] == 0 {
                res.push(pond_sizes_dfs(&mut land, i, j));
            }
        }
    }
    res.sort();
    res
}

fn pond_sizes_dfs(land:&mut Vec<Vec<i32>>, x:usize, y:usize) ->i32 {
    land[x][y] = 1;
    let mut size = 1;

    let dire = [1,0,-1,0,1,1,-1,-1,1];
    for i in 0..8 {
        let nx = x as i32 + dire[i];
        let ny = y as i32 + dire[i+1];
        if in_area(land, nx, ny) && land[nx as usize][ny as usize] == 0 {
            size += pond_sizes_dfs(land, nx as usize, ny as usize);
        }
    }
    size
}

fn in_area(land:&Vec<Vec<i32>>, i:i32, j :i32)->bool {
    i >= 0 && i < land.len() as i32 && j >= 0 && j < land[0].len() as i32
}