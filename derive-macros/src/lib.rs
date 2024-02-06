use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ConfigFile)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    #[cfg(feature = "json")]
    let expanded = quote! {
        impl ConfigFile for #name {
            fn path() -> Result<std::path::PathBuf, derive_config::ConfigError> {
                let path = derive_config::dirs::config_dir().ok_or(derive_config::ConfigError::None)?;
                let name = env!("CARGO_PKG_NAME");
                let file = format!("{name}.json");

                Ok(path.join(file))
            }

            fn save(&self) -> Result<(), derive_config::ConfigError> {
                use std::io::Write;

                let path = Self::path()?;
                let mut file = std::fs::File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?;

                let content = derive_config::json::to_string_pretty(&self)?;
                file.write_all(content.as_bytes())?;

                Ok(())
            }

            fn load() -> Result<Self, derive_config::ConfigError> {
                use std::io::{Read, Seek};

                let path = Self::path()?;
                let mut file = std::fs::File::open(&path)?;
                let mut text = String::new();
                file.read_to_string(&mut text)?;
                file.rewind()?;

                let config = derive_config::json::from_str(&text)?;

                Ok(config)
            }
        }
    };

    #[cfg(feature = "toml")]
    let expanded = quote! {
        impl ConfigFile for #name {
            fn path() -> Result<std::path::PathBuf, derive_config::ConfigError> {
                let path = derive_config::dirs::config_dir().ok_or(derive_config::ConfigError::None)?;
                let name = env!("CARGO_PKG_NAME");
                let file = format!("{name}.toml");

                Ok(path.join(file))
            }

            fn save(&self) -> Result<(), derive_config::ConfigError> {
                use std::io::Write;

                let path = Self::path()?;
                let mut file = std::fs::File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?;

                let content = derive_config::toml::to_string_pretty(&self)?;
                file.write_all(content.as_bytes())?;

                Ok(())
            }

            fn load() -> Result<Self, derive_config::ConfigError> {
                use std::io::{Read, Seek};

                let path = Self::path()?;
                let mut file = std::fs::File::open(&path)?;
                let mut text = String::new();
                file.read_to_string(&mut text)?;
                file.rewind()?;

                let config = derive_config::toml::from_str(&text)?;

                Ok(config)
            }
        }
    };

    TokenStream::from(expanded)
}