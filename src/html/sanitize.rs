use aho_corasick::AhoCorasick;
pub fn sanitize(inp: &str) -> String {
    /*
    & -> &amp;
    < -> &lt;
    > -> &gt;

    " -> &quot;
    ' -> &#39;
    \n -> <br>
    */
    let germs = &["&", "<", ">", "\"", "'", "\n"];
    let target = &["&amp;", "&lt;", "&gt;", "&quot;", "&#39;", "<br>"];

    let replacer = AhoCorasick::new(germs).unwrap();
    replacer.replace_all(inp, target)
}
