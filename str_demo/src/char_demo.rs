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
