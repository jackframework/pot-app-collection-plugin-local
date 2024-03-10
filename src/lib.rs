use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
struct ErrorMsg {
    message: String,
}

impl Display for ErrorMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ErrorMsg {}

#[no_mangle]
pub fn collection(
    source: &str,                   // 原文
    target: &str,                   // 译文
    from: &str,                     // 原文语言代码
    to: &str,                       // 译文语言代码
    needs: HashMap<String, String>, // 插件需要的其他配置信息,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    // 获取生词本文件存储位置
    let path = match needs.get("path") {
        Some(port) => port.to_string(),
        None => {
            return Err(Box::new(ErrorMsg {
                message: "请设置生词本文件位置".to_string(),
            }));
        }
    };

    let mut file = OpenOptions::new()
        .append(true) // 设置为追加模式
        .create(true) // 如果文件不存在，则创建新文件
        .open(path)?; // 打开文件，如果出错则返回Err

    // 写入内容到文件，注意这里使用了`write_all`来确保整个内容都被写入
    file.write_all(format!("{}\n", source.to_lowercase()).as_bytes())?;

    Ok(Value::Bool(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("path".to_string(), "./collection-test.txt".to_string());
        let result = collection("Hello", "你好", "en", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
