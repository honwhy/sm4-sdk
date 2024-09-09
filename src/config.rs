use dotenv_codegen::dotenv;

pub const KEY: &str = dotenv!("SM4_SDK_KEY");
pub const IV: &str = dotenv!("SM4_SDK_IV");
