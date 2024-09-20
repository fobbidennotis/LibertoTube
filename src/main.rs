mod checker;

#[tokio::main]
async fn main() {
    checker::server().await;
    // checker::client().await;
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // fn test{}
}
