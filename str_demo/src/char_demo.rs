use std::arch::aarch64::{int32x2_t, int32x2x2_t, vsubw_u32};
use std::collections::{HashMap, HashSet};
use std::io::Bytes;
use std::ops::Sub;
use std::os::unix::raw::off_t;

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
            }
            Err(_) => {
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
    sentence.chars().enumerate().all(|(i, c)| {
        if c == ' ' && sentence.chars().nth(i + 1) != sentence.chars().nth(i - 1) {
            false
        } else {
            true
        }
    }) && sentence.chars().next() == sentence.chars().last()
}

// lc 非常秀的做法
pub fn is_circular_sentence_1(sentence: String) -> bool {
    let (N, s) = (sentence.len(), sentence.as_bytes());
    s[0] == s[N - 1]
        && (0..N)
            .filter(|&i| s[i] == b' ')
            .all(|i| s[i - 1] == s[i + 1])
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

    while (i > 0 || j > 0) {
        let n1 = {
            if i > 0 {
                num1.as_bytes()[i - 1] - '0' as u8
            } else {
                0
            }
        };
        let n2 = {
            if j > 0 {
                num2.as_bytes()[j - 1] - '0' as u8
            } else {
                0
            }
        };

        let tmp = n1 + n2 + carry;
        carry = tmp / 10;
        res.push(tmp % 10 + '0' as u8);
        if i > 0 {
            i -= 1;
        }
        if j > 0 {
            j -= 1;
        }
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

/// 771. 宝石与石头
/// 给你一个字符串 jewels代表石头中宝石的类型，另有一个字符串 stones 代表你拥有的石头。
/// stones中每个字符代表了一种你拥有的石头的类型，你想知道你拥有的石头中有多少是宝石。
/// 字母区分大小写，因此 "a" 和 "A" 是不同类型的石头。

// lc写法1
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones.matches(|c| jewels.contains(c)).count() as i32
}

// lc写法2
pub fn num_jewels_in_stones_0(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .into_iter()
        .filter(|&c| jewels.contains(c))
        .count() as i32
}

// lc写法3
pub fn num_jewels_in_stones_1(jewels: String, stones: String) -> i32 {
    let mut a = [0; 128];
    stones.bytes().for_each(|x| a[x as usize] += 1);
    jewels.bytes().fold(0, |s, x| s + a[x as usize])
}

/// 722. 删除注释
/// 给一个 C++ 程序，删除程序中的注释。这个程序source是一个数组，其中source[i]表示第 i 行源码。 这表示每行源码由 '\n' 分隔。
/// 在 C++ 中有两种注释风格，行内注释和块注释。
/// 字符串// 表示行注释，表示//和其右侧的其余字符应该被忽略。
/// 字符串/* 表示一个块注释，它表示直到下一个（非重叠）出现的*/之间的所有字符都应该被忽略。（阅读顺序为从左到右）非重叠是指，字符串/*/并没有结束块注释，因为注释的结尾与开头相重叠。
/// 第一个有效注释优先于其他注释。
/// 如果字符串//出现在块注释中会被忽略。
/// 同样，如果字符串 /*出现在行或块注释中也会被忽略。
/// 如果一行在删除注释之后变为空字符串，那么不要输出该行。即，答案列表中的每个字符串都是非空的。
/// 样例中没有控制字符，单引号或双引号字符。
/// 比如，source = "string s = "/* Not a comment. */";" 不会出现在测试样例里。
/// 此外，没有其他内容（如定义或宏）会干扰注释。
/// 我们保证每一个块注释最终都会被闭合， 所以在行或块注释之外的/*总是开始新的注释。
/// 最后，隐式换行符可以通过块注释删除。 有关详细信息，请参阅下面的示例。
/// 从源代码中删除注释后，需要以相同的格式返回源代码。

// lc 题解

pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut ans = vec![];
    let mut is_comment = false;
    let mut line = String::new();

    for s in source {
        let mut it = s.chars().peekable();

        while let Some(c) = it.next() {
            if is_comment {
                match (c, it.peek()) {
                    ('*', Some(&'/')) => {
                        it.next();
                        is_comment = false;
                    }
                    _ => {}
                }
            } else {
                match (c, it.peek()) {
                    ('/', Some(&'/')) => {
                        break;
                    }
                    ('/', Some(&'*')) => {
                        it.next();
                        is_comment = true;
                        continue;
                    }
                    _ => {}
                }

                line.push(c);
            }
        }

        if !is_comment && !line.is_empty() {
            ans.push(line);
            line = String::new();
        }
    }

    ans
}

// lc 内存比较好的解
pub fn remove_comments_1(source: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    let mut t = Vec::new();
    let mut block = false;
    for s in source {
        let mut _s = s.as_bytes().iter().peekable();
        while let Some(&c) = _s.next() {
            if block {
                if c == b'*' && _s.peek() == Some(&&b'/') {
                    _s.next();
                    block = false;
                }
            } else {
                if c == b'/' {
                    let peek = _s.peek();
                    if peek == Some(&&b'/') {
                        break;
                    } else if peek == Some(&&b'*') {
                        _s.next();
                        block = true;
                        continue;
                    }
                }
                t.push(c);
            }
        }
        if !block && !t.is_empty() {
            res.push(unsafe { String::from_utf8_unchecked(t.clone()) });
            t.clear();
        }
    }
    res
}

/// 344. 反转字符串
/// 编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 s 的形式给出。
/// 不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外空间解决这一问题。

// lc自己做的
pub fn reverse_string(s: &mut Vec<char>) {
    let n = s.len();
    let mid = n / 2;
    for i in 0..mid {
        let tmp = s[i];
        s[i] = s[n - i -1];
        s[n-i-1] = tmp;
    }
}
// LC 比较好的写法
pub fn reverse_string_0(s: &mut Vec<char>) {
    // let (mut l, mut r) = (0, s.len() - 1);
    // while l < r {
    //     s.swap(l, r);
    //     l += 1;
    //     r -= 1;
    // }
    s.reverse();
}

/// 2578. 最小和分割
/// 给你一个正整数 num ，请你将它分割成两个非负整数 num1 和 num2 ，满足：
/// num1 和 num2 直接连起来，得到 num 各数位的一个排列。
/// 换句话说，num1 和 num2 中所有数字出现的次数之和等于 num 中所有数字出现的次数。
/// num1 和 num2 可以包含前导 0 。
/// 请你返回 num1 和 num2 可以得到的和的 最小 值。
/// 注意：
/// num 保证没有前导 0 。
/// num1 和 num2 中数位顺序可以与 num 中数位顺序不同。
/// 示例 1：
/// 输入：num = 4325
/// 输出：59
/// 解释：我们可以将 4325 分割成 num1 = 24 和 num2 = 35 ，和为 59 ，59 是最小和。
/// 示例 2：
/// 输入：num = 687
/// 输出：75
/// 解释：我们可以将 687 分割成 num1 = 68 和 num2 = 7 ，和为最优值 75 。

// 力扣优秀解
pub fn split_num(num: i32) -> i32 {
    let mut s:Vec<u8> = num.to_string().bytes().collect();
    s.sort_unstable();
    let mut a = [0,0];
    for (i, &c) in s.iter().enumerate() {
        a[i % 2] = a[i % 2] * 10 + c as i32 - '0' as i32;
    }
    a[0] + a[1]
}

/// 2512. 奖励最顶尖的 K 名学生
/// 给你两个字符串数组 positive_feedback 和 negative_feedback ，分别包含表示正面的和负面的词汇。不会 有单词同时是正面的和负面的。
/// 一开始，每位学生分数为 0 。每个正面的单词会给学生的分数 加 3 分，每个负面的词会给学生的分数 减  1 分。
/// 给你 n 个学生的评语，用一个下标从 0 开始的字符串数组 report 和一个下标从 0 开始的整数数组
/// student_id 表示，其中 student_id[i] 表示这名学生的 ID ，这名学生的评语是 report[i] 。每名学生的 ID 互不相同。
/// 给你一个整数 k ，请你返回按照得分 从高到低 最顶尖的 k 名学生。如果有多名学生分数相同，ID 越小排名越前。
/// 示例 1：
/// 输入：positive_feedback = ["smart","brilliant","studious"], negative_feedback = ["not"], report = ["this student is studious","the student is smart"], student_id = [1,2], k = 2
/// 输出：[1,2]
/// 解释：
/// 两名学生都有 1 个正面词汇，都得到 3 分，学生 1 的 ID 更小所以排名更前。
/// 示例 2：
/// 输入：positive_feedback = ["smart","brilliant","studious"], negative_feedback = ["not"], report = ["this student is not studious","the student is smart"], student_id = [1,2], k = 2
/// 输出：[2,1]
/// 解释：
/// - ID 为 1 的学生有 1 个正面词汇和 1 个负面词汇，所以得分为 3-1=2 分。
/// - ID 为 2 的学生有 1 个正面词汇，得分为 3 分。
/// 学生 2 分数更高，所以返回 [2,1] 。
pub fn top_students(positive_feedback: Vec<String>, negative_feedback: Vec<String>, report: Vec<String>, student_id: Vec<i32>, k: i32) -> Vec<i32> {
    let positive_feedback_set = positive_feedback.iter().collect::<HashSet<_>>();
    let negative_feedback_set = negative_feedback.iter().collect::<HashSet<_>>();

    let mut score = vec![0; student_id.len()];

    let report = report
        .into_iter()
        .map(|x| {x
            .to_string()
            .split_whitespace() // Using split_whitespace to handle potential spaces
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>();

    for i in 0..report.len() {
        for word in &report[i] {
            if positive_feedback_set.contains(word) {
                score[i] += 3;
            } else if negative_feedback_set.contains(word) {
                score[i] -= 1;
            }
        }
    }

    let mut ans: Vec<_> = score.iter().zip(&student_id).collect();

    ans.sort_by(|(s1, i1), (s2, i2)| {
        match s2.cmp(s1) {
            std::cmp::Ordering::Equal => i1.cmp(i2),
            order => order,
        }
    });

    ans.into_iter()
        .take(k as usize)
        .map(|(_, b)| *b)
        .collect()
}

/// 2520. 统计能整除数字的位数
/// 给你一个整数 num ，返回 num 中能整除 num 的数位的数目。
/// 如果满足 nums % val == 0 ，则认为整数 val 可以整除 nums 。
/// 输入：num = 7
/// 输出：1
/// 解释：7 被自己整除，因此答案是 1 。
/// 示例 2：
/// 输入：num = 121
/// 输出：2
/// 解释：121 可以被 1 整除，但无法被 2 整除。由于 1 出现两次，所以返回 2 。
/// 示例 3：
/// 输入：num = 1248
/// 输出：4
/// 解释：1248 可以被它每一位上的数字整除，因此答案是 4 。
pub fn count_digits(num: i32) -> i32 {
    let num_str = num.to_string();
     let num_char =   num_str.as_bytes();
    let mut ans:i32 = 0;
    for i in 0..num_char.len() {
       if num % (num_char[i] - '0' as u8) as i32  == 0 {
           ans+=1;
       }
    }
    ans
}

// 力扣比较好的写法
pub fn count_digits_1(num: i32) -> i32 {
    let mut ans = 0;
    let mut x = num;
    while  x != 0 {
        if num % (x % 10) == 0 {
            ans += 1;
        }
        x /= 10;
    }
    ans
}

/// 2103. 环和杆
/// 总计有 n 个环，环的颜色可以是红、绿、蓝中的一种。这些环分别穿在 10 根编号为 0 到 9 的杆上。
/// 给你一个长度为 2n 的字符串 rings ，表示这 n 个环在杆上的分布。rings 中每两个字符形成一个 颜色位置对 ，用于描述每个环：
/// 第 i 对中的 第一个 字符表示第 i 个环的 颜色（'R'、'G'、'B'）。
/// 第 i 对中的 第二个 字符表示第 i 个环的 位置，也就是位于哪根杆上（'0' 到 '9'）。
/// 例如，"R3G2B1" 表示：共有 n == 3 个环，红色的环在编号为 3 的杆上，绿色的环在编号为 2 的杆上，蓝色的环在编号为 1 的杆上。
/// 找出所有集齐 全部三种颜色 环的杆，并返回这种杆的数量。
/// 示例 1：
/// 输入：rings = "B0B6G0R6R0R6G9"
/// 输出：1
/// 解释：
/// - 编号 0 的杆上有 3 个环，集齐全部颜色：红、绿、蓝。
/// - 编号 6 的杆上有 3 个环，但只有红、蓝两种颜色。
/// - 编号 9 的杆上只有 1 个绿色环。
/// 因此，集齐全部三种颜色环的杆的数目为 1 。
///
// 自己做的垃圾
pub fn count_points(rings: String) -> i32 {
    let mut red = vec![0;10];
    let mut green = vec![0;10];
    let mut blue = vec![0;10];
    let chars = rings.as_bytes();
    let mut index = 0;
    for &char in chars {
        if char == b'R' {
            index = 0;
        } else if char == b'G'{
            index = 1;
        } else if char == b'B' {
            index = 2;
        } else {
            let arr_index = (char - b'0') as usize;
            if index == 0 {
                red[arr_index] += 1;
            } else if index == 1 {
                green[arr_index] += 1;
            } else {
                blue[arr_index] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..10 {
        if red[i] > 0 && green[i] > 0 && blue[i] > 0 {
            ans += 1;
        }
    }
    ans
}

// 力扣符合rust的写法
pub fn count_points_1(rings: String) -> i32 {
    let mut d:[i32;90] = [0;90];
    d['R' as usize] = 1;
    d['G' as usize] = 2;
    d['B' as usize] = 4;
    let mut mask = [0;10];
    let cs:Vec<char> = rings.chars().collect();
    for i in (0..cs.len()).step_by(2) {
        let c = cs[i] as usize;
        let j = cs[i+1] as usize - '0' as usize;
        mask[j] |= d[c];
    }
    mask.iter().filter(|&&x| x == 7).count() as i32
}

/// 318. 最大单词长度乘积
/// 给你一个字符串数组 words ，找出并返回 length(words[i]) * length(words[j]) 的最大值，并且这两个单词不含有公共字母。如果不存在这样的两个单词，返回 0 。
/// 示例 1：
/// 输入：words = ["abcw","baz","foo","bar","xtfn","abcdef"]
/// 输出：16
/// 解释：这两个单词为 "abcw", "xtfn"。
/// 示例 2：
///
/// 输入：words = ["a","ab","abc","d","cd","bcd","abcd"]
/// 输出：4
/// 解释：这两个单词为 "ab", "cd"。
/// 示例 3：
///
/// 输入：words = ["a","aa","aaa","aaaa"]
/// 输出：0
/// 解释：不存在这样的两个单词。
/// 提示：
///
/// 2 <= words.length <= 1000
/// 1 <= words[i].length <= 1000
/// words[i] 仅包含小写字母

// 自己做的 垃圾
pub fn max_product(mut words: Vec<String>) -> i32 {
    if words.len() < 2 {
        return 0
    }
    let mut record:Vec<u32> = Vec::new();
    for i in 0..words.len() {
        let bytes = words[i].chars();
        let mut re = 0;
        for by in bytes {
            let index = by as i32- 'a' as i32;
            re |=  1u32 << index;
        }
        println!("{:b}", re);
        record.push(re);
    }
    let mut ans = 0;
    for i in 0..record.len()-1 {
        for i1 in i+1..record.len() {
            if record[i] & record[i1] == 0 {
               ans = ans.max((words[i].len() * words[i1].len()) as i32)
            }
        }
    }
    return ans
}

// 符合rust的写法
pub fn max_product_1(mut words: Vec<String>) -> i32 {
    let mask: Vec<i32> = words
        .iter()
        .map(|word| {
            word.chars()
                .fold(0, |acc, c| acc | 1 << (c as u8 - 'a' as u8))
        })
        .collect();
    let mut ans = 0;
    for i in 0..mask.len() {
        for j in i + 1..mask.len() {
            if mask[i] & mask[j] == 0 {
                ans = ans.max(words[i].len() * words[j].len());
            }
        }
    }
    ans as i32
}

/// 2586. 统计范围内的元音字符串数
/// 给你一个下标从 0 开始的字符串数组 words 和两个整数：left 和 right 。
/// 如果字符串以元音字母开头并以元音字母结尾，那么该字符串就是一个 元音字符串 ，其中元音字母是 'a'、'e'、'i'、'o'、'u' 。
/// 返回 words[i] 是元音字符串的数目，其中 i 在闭区间 [left, right] 内。
/// 示例 1：
/// 输入：words = ["are","amy","u"], left = 0, right = 2
/// 输出：2
/// 解释：
/// - "are" 是一个元音字符串，因为它以 'a' 开头并以 'e' 结尾。
/// - "amy" 不是元音字符串，因为它没有以元音字母结尾。
/// - "u" 是一个元音字符串，因为它以 'u' 开头并以 'u' 结尾。
/// 在上述范围中的元音字符串数目为 2 。
/// 示例 2：
/// 输入：words = ["hey","aeo","mu","ooo","artro"], left = 1, right = 4
/// 输出：3
/// 解释：
/// - "aeo" 是一个元音字符串，因为它以 'a' 开头并以 'o' 结尾。
/// - "mu" 不是元音字符串，因为它没有以元音字母开头。
/// - "ooo" 是一个元音字符串，因为它以 'o' 开头并以 'o' 结尾。
/// - "artro" 是一个元音字符串，因为它以 'a' 开头并以 'o' 结尾。
/// 在上述范围中的元音字符串数目为 3 。
/// 提示：
/// 1 <= words.length <= 1000
/// 1 <= words[i].length <= 10
/// words[i] 仅由小写英文字母组成
/// 0 <= left <= right < words.length
// 自己做的真垃圾
pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let mut ans = 0;
    for i  in left as usize..(right+1) as usize {
        let word = words[i].as_bytes();
        if word[0] != 'a' as u8 && word[0] != 'e' as u8&& word[0] != 'i' as u8 && word[0] != 'o' as u8&& word[0] != 'u' as u8{
            continue;
        }
        if word[word.len()-1] != 'a' as u8&& word[word.len()-1] != 'e' as u8&& word[word.len()-1] != 'i' as u8&& word[word.len()-1] != 'o' as u8 && word[word.len()-1] != 'u' as u8{
            continue;
        }
        ans +=1 ;
    }
    ans
}

// 符合rust写法
pub fn vowel_strings_1(words: Vec<String>, left: i32, right: i32) -> i32 {
    let mut ans = 0;
    for i in left..=right {
       let s = &words[i as usize];
        if "aeiou".contains(s.chars().next().unwrap()) &&
            "aeiou".contains(s.chars().last().unwrap()) {
            ans += 1;
        }
    }
    ans
}