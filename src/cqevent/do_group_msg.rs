use crate::cqapi::*;

// 处理群聊事件
pub fn do_group_msg(root: &json::JsonValue) -> Result<i32, Box<dyn std::error::Error>> {
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
