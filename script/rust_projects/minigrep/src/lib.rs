use std::error::Error;
use std::{fmt, process};
use std::fs::File;
use std::io::Read;

// 关注点分离(Separation of Concerns)。简而言之，main.rs 负责启动程序，lib.rs 负责逻辑代码的运行
pub fn run(search: SearchInfo) {
    let res = search_str_in_file(search);
    if let Err(e) = res {
        println!("Application error: {e}");
    } else {
        println!("Search res:{:?}", res.unwrap());
    }
}

// 默认最小参数长度
const DEFAULT_MIN_PARAMETERS_LEN: usize = 3;

// 搜索信息
#[derive(Debug)]
pub struct SearchInfo {
    filepath: String,
    query_string: String,
}

impl SearchInfo {
    // 1.构建搜索信息
    pub fn build(args: &[String]) -> Result<SearchInfo, &'static str> {
        if args.len() < DEFAULT_MIN_PARAMETERS_LEN {
            return Err("not enough arguments!");
        }
        let arg1 = args[1].clone();
        let arg2 = args[2].clone();
        Ok(SearchInfo { filepath: arg1, query_string: arg2 })
    }
}

// 搜索结果
#[derive(Debug)]
struct SearchRes {
    context: String,
    position: Vec<usize>,
}

impl SearchRes {
    // 构建搜索结果
    fn build(context: String, position: Vec<usize>) -> Result<SearchRes, Box<dyn Error>> {
        if position.len() == 0 {
            // 真丑啊！！！
            return Err(Box::new(CustomError(String::from("str found,but res is illegal!"))));
        }
        Ok(SearchRes { context, position })
    }
}

// 自定义错误类型，实现Error特征！方便Error特征对象的"多态"处理
#[derive(Debug)]
struct CustomError(String);

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 2.读取文件内容，借助？的链式调用 + 错误处理
fn read_file_content(search: &SearchInfo) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();
    File::open(&search.filepath)?.read_to_string(&mut res)?;
    Ok(res)
}


//3.搜索字符串，仍然借助？的链式调用 + 错误处理，同时借助字符串迭代器匹配
fn search_str_in_file(search_info: SearchInfo) -> Result<SearchRes, Box<dyn Error>> {
    let content = read_file_content(&search_info)?;
    match content.contains(&search_info.query_string) {
        true => {
            let mut position = Vec::new();
            for (index, _) in content.match_indices(&search_info.query_string) {
                position.push(index);
            }
            let search_res = SearchRes::build(content, position)?;
            Ok(search_res)
        }
        false => Err(Box::new(CustomError(String::from("no str found!"))))
    }
}

