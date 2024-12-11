use reqwest;
use tokio;
use serde::Deserialize;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::env;
use std::error::Error;
use std::io::prelude::*;


#[derive(Debug, Deserialize)]
pub struct Env {
    pub hatena_id: String,
    pub password: String,
    pub blog_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub draft: String,
    pub preview: String,
    pub category: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub body: String,
    pub config: Config,
}

pub fn load_env(path: &str) -> Result<Env, Box<dyn Error>> {
    let path = Path::new(".").join(path);
    let env_data = fs::read_to_string(path)?;
    let env: Env = serde_yaml::from_str(&env_data)?;
    Ok(env)
}

pub fn get_content_dir() -> String {
    // content のパスを受け取る
    let args: Vec<String> = env::args().collect();
    // 引数の数が合っているか
    if args.len() != 2 {
        eprintln!("Usage: {} <article_directory>", args[0]);
        std::process::exit(1);
    }

    args[1].clone()
}

pub fn load_content(dir: &str) -> Result<Content, Box<dyn Error>> {
    // content ディレクトリからcontent を読み取る
    let content_path = Path::new(&dir).join("content.md");
    let mut f = File::open(content_path)?;
    let mut body = String::new();
    f.read_to_string(&mut body)?;
    
    let config_path = Path::new(&dir).join("config.yaml");
    let config_data = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_data)?;

    let content = Content {
        body,
        config,
    };
    Ok(content)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env から hatena_id, password, blog_id を読み取る
    let env_path = "./env.yaml";
    let env: Env = load_env(env_path)?;
    println!("{:?}", env);
   
    let content_dir = get_content_dir();
    println!("content dir: {}", content_dir);

    let content = load_content(&content_dir).unwrap();
    println!("{:?}", content);

    // xml に渡せるようにカテゴリの要素を作る
    let mut category_element = "".to_string();
    for category in content.config.category {
        let tmp = format!("<category term=\"{}\" />", category);
        category_element.push_str(&tmp);
    }


    let body = format!(r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <title>{}</title>
  <author><name>name</name></author>
  <content type="text/x-markdown">
{}
  </content>
  {}
  <app:control>
    <app:draft>{}</app:draft>
    <app:preview>{}</app:preview>
  </app:control>
</entry>"#, content.config.title, content.body, category_element, content.config.draft, content.config.preview);

    let host = format!("https://blog.hatena.ne.jp/{}/{}/atom/entry", env.hatena_id, env.blog_id);
    println!("host: {}", host);

    let client = reqwest::Client::builder()
        .build()?;

    let res = client
        .post(host)
        .header("Content-Type", "application/xml")
        .basic_auth(env.hatena_id, Some(env.password))
        .body(body)
        .send()
        .await?;

    // response から preview のリンクがあれば、それを拾って出力する
    let body = res.text().await?;
    println!("Body: {}", body);
    Ok(())

 }
