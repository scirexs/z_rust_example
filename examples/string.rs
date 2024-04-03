fn main() {
    let str1: &str = "a©あ😀";
    let string1: String = String::from("str-ing ");
    let mut string2: String = String::from("1234");
    let vec: Vec<String> = Vec::from(["abc".to_owned(), "789".into(), "@#?".to_string()]);

    let _: String = str1.to_string(); // &strからStringにキャスト
    let _: &str = &string1[..];       // Stringから&strにキャスト &string1は&strか&Stringか型推論が必要
    // -----&strでもStringでも使用可能-----
    if let Ok(int) = string2.parse::<i32>() {        // 文字列から数値キャスト
        string2 = int.to_string();                   // 数値から文字列キャスト
    }
    assert!(String::new().is_empty());               // 空判定
    assert_eq!(string1.chars().count(), 8);          // 文字数カウント .len()はバイト数のため使えない
    assert_eq!(string1.replace("-", ""), "string "); // 置換 (返り値String)
    assert!(string1.starts_with("str"));             // 指定文字列から始まる判定 (ends_withもある)
    assert_eq!(string1.split("-").collect::<Vec<_>>(), ["str","ing "]); // 文字列を分割
    assert_eq!(string1.trim(), "str-ing");           // 前後の空白文字除去 (返り値&str)
    assert!(string1.contains("r-i"));                // 指定文字列を含む判定
    assert!(string1.chars().all(|x| x.is_ascii_alphanumeric())); // [A-Z,a-z,0-9]のみで文字列構成しているか判定
    assert!(!string1.chars().all(|x| x.is_ascii_alphabetic()));  // [A-Z,a-z]のみで文字列構成しているか判定
    assert!(string2.chars().all(|x| x.is_ascii_digit()));        // [0-9]のみで文字列構成しているか判定
    // 数字だけ取得
    assert_eq!("12ab48".chars().filter(|&x| x.is_ascii_digit()).collect::<String>(), "1248");
    // ascii文字だけ取得
    assert_eq!("あ1aい@😀 ".chars().filter(|&x| x.is_ascii()).collect::<String>(), "1a@ ");
    // 部分文字列取得 (返り値String)
    assert_eq!(str1.chars().enumerate().filter(|&(x,_)| x>=1 && x<3).map(|(_,x)| x).collect::<String>(), "©あ");
    // 部分文字列取得 (返り値&str)
    assert_eq!(&str1[str1.char_indices().map(|(x,_)| x).nth(1).unwrap_or(0)..
                    str1.char_indices().map(|(x,_)| x).nth(3).unwrap_or(0)], "©あ");
    // -----------------------------------

    string2.push_str(str1);  // 文字列結合 Stringのみ使用可能
    string2 += &string1[..]; // 文字列結合 内部でpush_strが使用されている
    assert_eq!(string2, "1234a©あ😀str-ing ");
    assert_eq!(vec.concat(), "abc789@#?");    // 文字列要素を結合 (Vec<&str>でも使用可能)
    assert_eq!(vec.join(","), "abc,789,@#?"); // 文字列要素を結合 (Vec<&str>でも使用可能)

    let bar = Foo::new(str1);
    let baz = Foo::new(string1);
    assert_eq!(format!("{}{}",bar.f1,baz.f1), "a©あ😀str-ing "); // String生成

    let foo = "あstrいう";
    let bar = foo.to_string();
    let baz = "";

    assert_eq!(foo.get_slice(0, 0), "あ");
    assert_eq!(foo.get_slice(2, 4), "trい");
    assert_eq!(bar.get_slice(3, 7), "rいう");
    assert_eq!(bar.get_slice(5, 3), "");
    assert_eq!(baz.get_slice(0, 1), "");

    assert_eq!(foo.get_substring(0, 0), "あ");
    assert_eq!(foo.get_substring(2, 4), "trい");
    assert_eq!(bar.get_substring(3, 7), "rいう");
    assert_eq!(bar.get_substring(5, 3), "");
    assert_eq!(baz.get_substring(0, 1), "");
}

struct Foo { f1: String }
impl Foo {
    fn new(str1: impl Into<String>) -> Self {
        Self{ f1: str1.into() } // &str,String両対応可能な定義
    }
}

trait CustomMethods {
    fn get_slice(&self, start: usize, end: usize) -> &str;
    fn get_substring(&self, start: usize, end: usize) -> String;
}
impl CustomMethods for str {
    fn get_slice(&self, start: usize, end: usize) -> &str {
        if self.is_empty() { return self; }
        if start > end { return &self[..0]; }
        let (a, b) = self.char_indices()
            .map(|(x,_)| x)
            .enumerate()
            .filter(|&(i, _)| i==start || i==end+1)
            .fold((usize::MAX, usize::MAX), |(acc_bs,_acc_be),(_, x)| if acc_bs==usize::MAX {(x,x)} else {(acc_bs, x)});
        if self.chars().count() <= end+1 { &self[a..] } else { &self[a..b] }
    }
    fn get_substring(&self, start: usize, end: usize) -> String {
        self.chars().enumerate().filter(|&(x,_)| x>=start && x<=end).map(|(_,x)| x).collect::<String>()
    }
}