//! `wordcount` はシンプルな文字、単語、行の出現頻度の集計機能を提供します。
//! 詳しくは[`count`](fn.count.html)関数のドキュメントをご確認ください。
#![warn(missing_docs)]


use regex::Regex;
use std::{io::BufRead, collections::HashMap};

/// inputから1行ずつUTF-8文字列を読み込み、頻度を数える
/// 
/// 頻度を数える対象はオプションによって制御される
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicodeの1文字ごと
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現 \w+ にマッチする単語ごと
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): \n または \r\n で区切られた1行ごと
/// 
/// # Examples
/// 入力中の単語の出現頻度を集計する例
/// 
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
/// 
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
/// 
/// # Panics
/// 
/// 入力がUTF-8でフォーマットされてない場合はパニック
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();

        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字ごとに頻度カウント
    Char,
    /// 単語ごとに頻度カウント
    Word,
    /// 行ごとに頻度カウント
    Line,
}

/// オプションのデフォルト値は[`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

#[test]
fn word_count_works() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
fn word_count_works2() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);

    assert_eq!(count(Cursor::new("aa"), CountOption::Word), exp);
}

#[test]
#[should_panic]
fn word_count_contain_unknown_words() {
    use std::io::Cursor;

    count(
        Cursor::new([
            b'a',
            0xf0, 0x90, 0x80,
            0xe3, 0x81, 0x82,
        ]),
        CountOption::Word,
    );
}
