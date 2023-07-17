use std::io::Bytes;

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

/// *** 2490. 回环句
/// 句子 是由单个空格分隔的一组单词，且不含前导或尾随空格。
///  例如，"Hello World"、"HELLO"、"hello world hello world" 都是符合要求的句子。
///  单词 仅 由大写和小写英文字母组成。且大写和小写字母会视作不同字符。
///
/// 如果句子满足下述全部条件，则认为它是一个 回环句 ：
///
/// 单词的最后一个字符和下一个单词的第一个字符相等。
/// 最后一个单词的最后一个字符和第一个单词的第一个字符相等。
/// 例如，"leetcode exercises sound delightful"、"eetcode"、"leetcode eats soul" 都是回环句。
/// 然而，"Leetcode is cool"、"happy Leetcode"、"Leetcode" 和 "I like Leetcode" 都 不 是回环句。
/// 给你一个字符串 sentence ，请你判断它是不是一个回环句。如果是，返回 true ；否则，返回 false 。

// lc比较优秀做法
pub fn is_circular_sentence_0(sentence: String) -> bool {
    sentence.chars().enumerate().all(|(i,c)| {
        if c == ' ' && sentence.chars().nth(i+1) != sentence.chars().nth(i-1) {
            false
        } else {
            true
        }
    }) && sentence.chars().next() == sentence.chars().last()
}

// lc 非常秀的做法
pub fn is_circular_sentence_1(sentence: String) -> bool {
   let (N, s) = (sentence.len(), sentence.as_bytes());
    s[0] == s[N-1] && (0..N).filter(|&i| s[i] == b' ').all(|i| s[i-1] == s[i+1])
}

/// 415. 字符串相加
/// 给定两个字符串形式的非负整数num1 和num2，计算它们的和并同样以字符串形式返回。
/// 你不能使用任何內建的用于处理大整数的库（比如 BigInteger），也不能直接将输入的字符串转换为整数形式。

// lc 抄
pub fn add_strings(num1: String, num2: String) -> String {
    let mut res = Vec::new();
    let mut i = num1.len();
    let mut j = num2.len();
    let mut carry = 0;

    while(i > 0 || j > 0){
        let n1 = { if i > 0 {num1.as_bytes()[i-1] - '0' as u8} else { 0 }  };
        let n2 = { if j > 0 {num2.as_bytes()[j-1] - '0' as u8} else { 0 }  };

        let tmp = n1 + n2 + carry;
        carry = tmp / 10;
        res.push(tmp % 10 + '0' as u8);
        if i > 0 {i -= 1;}
        if j > 0 {j -= 1;}
    }
    if carry == 1 {
        res.push('1' as u8);
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

// lc best
pub fn add_strings_1(num1: String, num2: String) -> String {
    let mut number1 = num1.as_bytes().iter().rev();
    let mut number2 = num2.as_bytes().iter().rev();

    let mut result = String::new();

    let max_len = std::cmp::max(number1.len(), number2.len());

    let mut over_10 = false;
    for _ in 0..max_len {
        let number1 = *number1.next().unwrap_or(&('0' as u8)) - ('0' as u8);
        let number2 = *number2.next().unwrap_or(&('0' as u8)) - ('0' as u8);

        let sum = number1 + number2 + if over_10 { 1 } else { 0 };
        over_10 = sum >= 10;
        result.insert(0, ((sum % 10) + '0' as u8) as char)
    }

    if over_10 {
        result.insert(0, '1');
    }
    result
}