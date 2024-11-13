

#[tokio::main]
async fn main() -> Result<(),mini_redis::Error> {
    // 建立与mini-redis服务器的连接
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值: "world"
    client.set("hello", "1world".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}