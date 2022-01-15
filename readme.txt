介绍：
	随机图片插件，MiraiCQ拓展API/事件的示例插件（非CQ兼容插件）

触发方法：
	群聊发送："随机图片"
	群聊发送："来点惠惠的图片"("惠惠"可以换成其它)

编译：
	静态链接：
		$ENV:RUSTFLAGS='-C target-feature=+crt-static'
	镜像加速（大陆可能需要这个）：
		$ENV:RUSTUP_UPDATE_ROOT='https://mirrors.ustc.edu.cn/rust-static/rustup'
		$ENV:RUSTUP_DIST_SERVER='https://mirrors.ustc.edu.cn/rust-static'
	编译windows,32位：
		rustup target add i686-pc-windows-msvc
		cargo build --target=i686-pc-windows-msvc

相关rust学习资料：
	https://docs.microsoft.com/zh-cn/learn/browse/?terms=rust
	https://docs.rs/

MiraiCQ项目地址：
	https://github.com/super1207/MiraiCQ

随机图片网址（不知道是谁做的，但表示感谢）：
	https://iw233.cn/api/Random.php

搜图网址：
	百度图片

开源协议：
	MIT