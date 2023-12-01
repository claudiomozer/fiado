use mockall::automock;

#[automock]
pub trait Uuid {
    fn generate(&self) -> String; 
}