use mockall::automock;

#[automock]
pub trait Hash {
    fn run(&self, plain_text: String) -> Result<String, String>;
    fn verify(&self, plain_text: String, hash: String) -> Result<bool, String>;
}