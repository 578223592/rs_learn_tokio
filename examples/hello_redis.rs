

#[tokio::main]
async fn main() -> Result<(),mini_redis::Error> {
    // 建立与mini-redis服务器的连接
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值: "world"
    client.set("hello", "world".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await;
    
    if let Err(e) = result {
        // 处理错误
        eprintln!("请求失败: {}", e); //todo ：很奇怪，返回错误并不会答应请求失败，感觉还是直接panic了诶
        return Ok(()); // 返回
    }

    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}