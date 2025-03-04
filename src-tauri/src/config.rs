use serde::{Serialize, Deserialize};
use dirs::config_dir;
use std::{fs, path::PathBuf};

/// 设置选项结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// 工作时间，单位分钟
    pub work_time: f64,
    /// 休息时间，单位分钟
    pub rest_time: f64,
    /// 严格模式
    pub strict: bool,
}

impl Config {
    /// 创建默认设置
    pub fn new() -> Self {
        Self {
            work_time: 45f64,
            rest_time: 5f64,
            strict: false,
        }
    }
}

/// IO 模块
impl Config {
    // 获取存储路径
    fn get_path() -> Option<PathBuf> {
        config_dir().map(|mut path| {
            path.push("pomodoro");
            path.push("config.json");
            path
        })
    }

    /// 保存
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Self::get_path();

        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path.expect("保存路径未找到"))?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    /// 加载
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = Self::get_path().expect("读取未找到指定路径");
        let res;
        
        // 若为第一次创建,创建过程文件夹并保存当前结构体
        if !file_path.exists() {
            fs::create_dir_all(file_path.parent().unwrap())?;
            res = Self::new();
            res.save()?;
        } else {
            // 否则，读取
            let file = fs::File::open(file_path)?;
            res = serde_json::from_reader(file)?;
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_local() {
        match config_dir() {
            Some(path) => println!("{:?}", path),
            None => println!("Noe"),
        }
    }

    #[test]
    fn save_config() {
        let config = Config::load().unwrap();
        println!("{}", serde_json::to_string_pretty(&config).unwrap());
    }
}