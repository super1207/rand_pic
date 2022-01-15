use crate::cqapi::*;
use reqwest::header::HeaderMap;
use regex::Regex;

fn do_rand_pic(root: &json::JsonValue) -> Result<i32, Box<dyn std::error::Error>> {
    let msg_json = &root["message"];
    if msg_json.len() != 1 {
        return Ok(0);
    }
    if msg_json[0]["data"]["text"] != "随机图片" {
        return Ok(0);
    }
    let send_json = json::object! {
        "action":"send_group_msg",
        "params":{
            "group_id": root["group_id"].as_u64().ok_or("can't get group_id")?,
            "message":[
                {
                    "type":"text",
                    "data": {
                        "text": "喵喵喵~不准涩涩！"
                    }
                },
                {
                    "type": "image",
                    "data": {
                        "file": "https://iw233.cn/api/Random.php?".to_owned() + &rand::random::<i32>().to_string()
                    }
                }
            ]
        }
    };
    cq_call_api(send_json.dump().as_str())?;
    Ok(0)
}

fn do_req_pic(root: &json::JsonValue) -> Result<i32, Box<dyn std::error::Error>> {
    let msg_json = &root["message"];
    if msg_json.len() != 1 {
        return Ok(0);
    }
    let str0 = msg_json[0]["data"]["text"].as_str().ok_or("json is not str")?.to_string();
    if !str0.starts_with("来点") || !str0.ends_with("的图片") {
        return Ok(0);
    }
    fn substr(s: &str, start: usize, length: usize) -> String {
        s.chars().skip(start).take(length).collect()
    }
    let key_word = substr(str0.as_str(),2,str0.as_str().chars().count() - 5);
    cq_add_log(format!("key_word:{:?}，正在构造链接...", key_word).as_str())?;
    let mut headers = HeaderMap::new();
    let client = reqwest::blocking::Client::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.72 Safari/537.36".parse()?);
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".parse()?);
    let url = "https://image.baidu.com/search/index?tn=baiduimage&word=".to_owned() + &key_word.as_str();
    let http_ret = client.get(url.as_str()).headers(headers).send()?.text()?;
    let http_str = http_ret.as_str();
    let re = Regex::new("\"objURL\" {0,1}: {0,1}\"(.*?)\"")?;
    let mut cap_vec = Vec::new();
    for cap in re.captures_iter(http_str) {
        let retstr = cap[1].to_string();
        cap_vec.push(retstr);
    }
    let index = rand::random::<usize>() % cap_vec.len();
    let pic_url = cap_vec.get(index).ok_or("vec index range out")?;
    let send_json = json::object! {
        "action":"send_group_msg",
        "params":{
            "group_id": root["group_id"].as_u64().ok_or("can't get group_id")?,
            "message":[
                {
                    "type":"text",
                    "data": {
                        "text": format!("喵喵喵~{}的涩涩来啦!", key_word).as_str()
                    }
                },
                {
                    "type": "image",
                    "data": {
                        "file": pic_url.as_str()
                    }
                }
            ]
        }
    };
    cq_add_log(format!("链接构造成功:{:?}，", pic_url).as_str())?;
    cq_call_api(send_json.dump().as_str())?;
    Ok(0)
}

// 处理群聊事件
pub fn do_group_msg(root: &json::JsonValue) -> Result<i32, Box<dyn std::error::Error>> {
    if let Err(e) = do_rand_pic(&root) {
        cq_add_log(format!("err in do_rand_pic:{:?}", e).as_str()).unwrap();
    }
    if let Err(e) = do_req_pic(&root) {
        cq_add_log(format!("err in do_req_pic:{:?}", e).as_str()).unwrap();
    }
    Ok(0)
}
