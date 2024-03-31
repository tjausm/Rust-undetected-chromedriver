#[cfg(test)]
mod tests {
    use undetected_chromedriver::chrome;

    #[tokio::test]
    async fn test_chrome() {
        let driver = chrome().await.unwrap().1;
        assert!(driver.title().await.is_ok());
        driver.quit().await.unwrap();
    }
}
