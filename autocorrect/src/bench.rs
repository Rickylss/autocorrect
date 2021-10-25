// autocorrect: false
mod tests {
    use crate::*;
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use test::Bencher;

    #[allow(unused)]
    fn fixture(path: &str) -> String {
        let current_dir = env::current_dir().unwrap();
        let path = Path::join(
            &current_dir,
            Path::join(Path::new("../tests/fixtures"), path),
        );
        println!("fixture: {}", path.display());

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    #[bench]
    fn bench_format_50(b: &mut Bencher) {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称"));
    }

    #[bench]
    fn bench_format_100(b: &mut Bencher) {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称，【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称"));
    }

    #[bench]
    fn bench_format_400(b: &mut Bencher) {
        b.iter(|| format("【野村：重申吉利汽车(00175)“买入”评级 上调目标价至17.9港元】智通财经APP获悉，野村发布报告称，上调吉利汽车(00175)目标价12.58%，由15.9港元升至17.9港元，并维持吉汽为行业首选股，重申对其“买入”评级，坚信吉汽长远可成为行业赢家。 该行称，随着公司销量持续复苏及产品组合改善，预计今年销量可达148万辆，同比升9%，较公司原定目标销量141万辆为高。 该行又称称，上调公司今明两年每股盈利预测各13%及升毛利率0.1个百分点，以反映销量较预期高2%及产品组合改善，主要是由领克品牌带动。公司自去年8月开始已持续投资领克品牌及进行市场推广，带动领克销量环比有所改变，预期今明两年领克将占整体销量的11%及14%。 该行表示，由于低端国产车品牌在欠缺新车款及科技下，行业整合度将提升。另外，公司从去年第二季到12月为止，一直都积极推动经销商去库存，这将有利公司今年利润率复苏。"));
    }

    #[bench]
    fn bench_format_html(b: &mut Bencher) {
        let raw = r###"
bad html
<% a = 1 %>
{% hello = a %}
<!DOCTYPE html>
<!-- html的注释 -->
<html xmlns=http://www.w3.org/1999/xhtml>
<title><%= title %>或者{{ title }}</title>
<article>
<h1>编译Rust为WebAssembly</h1>
<style type="text/css" nofollow>
/* 在css里面的注释会转换 */
.body { font-size: 14px; } /* 后面个comment注释 */
</style>
<script type="text/javascript">
// 这个script也会转换
// 按照javascript的方式来处理
const a = "hello你好";
/**
 * 多行comment测试
 * 多行第2行
 */
</script>
<script async src=https://dlswbr.baidu.com/heicha/mw/abclite-2036-s.js></script>
<div class="content">
    <p>如果你写了一些Rust代码，你可以把它编译成WebAssembly！这份教程将带你编译Rust项目为wasm并在一个现存的web应用中使用它。</p>
    <a href="#rust_和_webassembly_用例" title="Permalink to Rust 和 WebAssembly 用例">Rust和WebAssembly用例</a>
    <h2>Rust和WebAssembly用例</h2>
    <div @click.prevent="hello" :name="foo" #bar="dar"><p>Rust 和 WebAssembly 有两大主要用例：</p>
    <ul>
    <li>构建完整应用——整个Web应用都基于Rust开发！</li>
    <li>构建应用的组成部分——在现存的JavaScript前端中使用Rust。</li>
    <%= link_to "FTP管理", "/", class: "subnav-item #{(params[:title_tab].blank? || params[:title_tab] == 'sftp_index') ? 'active' : ''}" %>
    </ul>
    <p>目前，Rust团队正专注于第二种用例，因此我们也将着重介绍它。对于第一种用例，可以参阅&nbsp;<code><a href="https://github.com/DenisKolodin/yew" class="external" rel=" noopener">yew</a></code>&nbsp;这类项目。</p>
    <p>在本教程中，我们将使用Rust的npm包构建工具<code>wasm-pack</code>来构建一个npm包。这个包只包含WebAssembly和JavaScript代码，以便包的用户无需安装Rust就能使用。他们甚至不需要知道这里包含WebAssembly！</p></div>
</div>
</article>
</html>
"###;

        b.iter(|| format_for(raw, "html"));
    }

    #[bench]
    fn bench_format_json(b: &mut Bencher) {
        let raw = r###"
{
  "name": "你好hello世界",
  "displayName": "JSON格式测试",
  "publisher": "huacnlee",
  "meta": {
    // 第1行注释
    "title": "第1个meta", 
    /** 
     * 第2行注释
     * 第3行注释
     */
    "description": "第2个meta", 
    "测试key不格式化": false
  }
}
"###;

        b.iter(|| format_for(raw, "json"));
    }

    #[bench]
    fn bench_format_javascript(b: &mut Bencher) {
        let raw = r###"
// 第1行注释
// 第2行注释
function helloWorld(a) {
    const a = '第1个';
    const b = "第2个" + "第3个";
    const re = /包含#regexp测试/;
    const re1 = new RegExp("RegExp不处理");
    const re2 = new RegExp('不处理RegExp');
    const str_literal = `这个${foo}不会处理`;

    /**
     * Hello你好
     * 这是第2行
     */
    const c = `这是string第1行
    这是string第2行`;

    // autocorrect-disable
    const disable_1 = "这行将会disable掉";
    const disable_2 = "这行将也会disable掉";
    // autocorrect-enable

    return <>
    <div className="react-name">
        <List renderItem={(item) => (
            <Item className="list-item">
            <span>nested项</span>
            <span>{item}</span>
            </Item>
        )} />
        <h1>Hello你好<strong>你好foo世界</strong></h1>
        外部HTML结果
        <div>{ a && t("这里string也要处理")}</div>
    </div>
    </>
}
"###;

        b.iter(|| format_for(raw, "javascript"));
    }
}