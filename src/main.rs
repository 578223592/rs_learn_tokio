use mini_redis::Connection;
use tokio::net::TcpListener;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:6379").await.unwrap();;
    loop {
        let (socket, _) = tcp_listener.accept().await?;
        tokio::spawn(async move {
            proceess_socket(socket).await;
        });
    }



    Ok(())
    
}

async fn proceess_socket(socket: tokio::net::TcpStream)  {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap(){
        println!("Got frame: {:?}", frame);

         // 回复一个错误
         let response = mini_redis::Frame::Error("unimplemented".to_string());
         connection.write_frame(&response).await.unwrap();
    }
}