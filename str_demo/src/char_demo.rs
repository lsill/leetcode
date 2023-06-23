/// 2496. 数组中字符串的最大值
/// 一个由字母和数字组成的字符串的 值定义如下：
///
/// 如果字符串 只 包含数字，那么值为该字符串在 10进制下的所表示的数字。
/// 否则，值为字符串的 长度。
/// 给你一个字符串数组strs，每个字符串都只由字母和数字组成，请你返回 strs中字符串的 最大值。
/// 输入：strs = ["alic3","bob","3","4","00000"]
/// 输出：5
/// 解释：
/// - "alic3" 包含字母和数字，所以值为长度 5 。
/// - "bob" 只包含字母，所以值为长度 3 。
/// - "3" 只包含数字，所以值为 3 。
/// - "4" 只包含数字，所以值为 4 。
/// - "00000" 只包含数字，所以值为 0 。
/// 所以最大的值为 5 ，是字符串 "alic3" 的值。

// 自己做的
pub fn maximum_value(strs: Vec<String>) -> i32 {
    let mut ans = 0;
    for str in &strs {
        match str.parse() {
            Ok(val) => {
                if ans < val {
                    ans = val;
                }
            },
            Err(_)=>{
                if ans < str.len() {
                    ans = str.len();
                }
            }

        }
    }
    ans as i32
}

// 力扣最佳
fn parse(dat: &[u8]) -> i32 {
    let mut v = 0;
    for &b in dat.iter() {
        if b.is_ascii_digit() {
            v *= 10;
            v += (b - b'0') as i32;
        } else {
            return dat.len() as i32;
        }
    }
    v
}

pub fn maximum_value_lc(strs: Vec<String>) -> i32 {
    strs.into_iter().map(|s| parse(s.as_bytes())).max().unwrap()
}

