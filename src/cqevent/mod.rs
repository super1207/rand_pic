mod do_group_msg;

// 处理1207号事件
pub fn do_1207_event(onebot_json_str: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let root = json::parse(onebot_json_str)?;
    if root["message_type"] == "group" {
        do_group_msg::do_group_msg(&root)?;
    }
    Ok(0)
}
