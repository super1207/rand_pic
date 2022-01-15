use std::ffi::CStr;
use std::os::raw::c_char;

mod cqapi;
mod cqevent;

// 这是插件第一个被调用的函数，不要在这里调用任何CQ的API,也不要在此处阻塞
#[no_mangle]
pub extern "system" fn Initialize(ac: i32) -> i32 {
    cqapi::set_auth_code(ac);
    // 要使CQ正常启动，请一定返回0
    return 0;
}

// 1207号事件，用于接收OneBotv11格式的原始数据，utf8编码
#[no_mangle]
pub extern "system" fn _event1207(msg: *const c_char) -> i32 {
    let onebot_json: String = unsafe {
        CStr::from_ptr(msg)
            .to_str()
            .expect("get error msg ptr from event1207")
            .to_string()
    };

    if let Err(e) = cqevent::do_1207_event(onebot_json.as_str()) {
        cqapi::cq_add_log(format!("{:?}", e).as_str()).unwrap();
    }
    return 0;
}

