// autocorrect: false
use super::*;

use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/jira.pest"]
struct JiraParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_jira() {
        let example = r###"
        h1. 这是Heading 1大标题

        {*}加粗{*}
        {_}倾斜{_}
        {-}删除线{-}
        这是{*}Bold加粗{*}在1个段落中，这端会correct掉，如果是inline code，例如{{Rust语言}}，也可以应该处理。
        
        bq. 引用文本：Quote也是可以的。
        
        {code:rust}
        // Codeblock里面也会处理
        let a = "你好hello";
        {code}
        
        - !https://google.com/a/b/url不处理!
        - [this不处理|https://google.com/a/b/url不处理]
        - {anchor:anchorname不处理}
    "###;

        let expected = r###"
        h1. 这是 Heading 1 大标题

        {*}加粗{*}
        {_}倾斜{_}
        {-}删除线{-}
        这是{*}Bold 加粗{*}在 1 个段落中，这端会 correct 掉，如果是 inline code，例如 {{Rust 语言}}，也可以应该处理。
        
        bq. 引用文本：Quote 也是可以的。
        
        {code:rust}
        // Codeblock 里面也会处理
        let a = "你好 hello";
        {code}
        
        - !https://google.com/a/b/url不处理!
        - [this不处理|https://google.com/a/b/url不处理]
        - {anchor:anchorname不处理}
    "###;

        assert_eq!(expected, format_jira(example).to_string())
    }
}
