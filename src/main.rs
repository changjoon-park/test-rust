use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 채널 생성
    let (tx, mut rx) = mpsc::channel(32);
    
    // 송신 태스크
    tokio::spawn(async move {
        tx.send("Hello").await.unwrap();
        tx.send("from").await.unwrap();
        tx.send("Tokio!").await.unwrap();
    });
    
    // 수신
    while let Some(msg) = rx.recv().await {
        println!("받음: {}", msg);
    }
}
