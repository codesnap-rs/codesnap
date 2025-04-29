use std::{fs, path};
use url::Url;

use anyhow::bail;
use mime_guess::{get_mime_extensions, mime};
use serde_json::{Map, Value};

pub struct Assets {
    destination: String,
}

impl Assets {
    pub fn from(destination: &str) -> Self {
        Assets {
            destination: destination.to_string(),
        }
    }

    fn get_assets_map_path(&self) -> String {
        path::Path::new(&self.destination)
            .join("assets_map.json")
            .to_str()
            .unwrap()
            .to_string()
    }

    fn get_assets_map(&self) -> anyhow::Result<Map<String, Value>> {
        let assets_map_path = self.get_assets_map_path();

        if !fs::metadata(&assets_map_path).is_ok() {
            fs::write(&assets_map_path, "{}")?;
            return Ok(Map::new());
        }

        let assets_map: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&assets_map_path)?)?;

        Ok(assets_map.as_object().unwrap().clone())
    }

    fn upsert_assets_map(&self, name: &str, url: &str, path: &str) -> anyhow::Result<()> {
        let mut assets_map = self.get_assets_map()?;
        let assets_map_path = self.get_assets_map_path();
        let mut map = Map::new();

        map.insert("url".to_string(), Value::String(url.to_string()));
        map.insert("path".to_string(), Value::String(path.to_string()));

        assets_map.insert(name.to_string(), Value::Object(map));
        fs::write(assets_map_path, serde_json::to_string(&assets_map)?)?;

        Ok(())
    }

    pub async fn download(&self, assets_url: &str) -> anyhow::Result<String> {
        let assets_url = AssetsURL::from_url(assets_url)?;
        let assets_map = self.get_assets_map()?;
        let get_value = |key: &str| -> anyhow::Result<Value> {
            let value = assets_map
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("Failed to get existing URL from assets map"))?;

            Ok(value.clone())
        };

        // Read cache if the assets already exists
        if assets_map.contains_key(&assets_url.name) {
            let value = get_value(&assets_url.name)?;
            let info = value.as_object().unwrap();
            let url = info.get("url").unwrap().as_str().unwrap();
            let path = info.get("path").unwrap().as_str().unwrap();

            if url == &assets_url.url {
                return Ok(path.to_string());
            }

            fs::remove_file(path)?;
        }

        let response = reqwest::get(&assets_url.url).await?;
        let headers = response.headers();
        let content_type = headers
            .get("content-type")
            .ok_or_else(|| anyhow::anyhow!("Failed to get content type from response headers"))?
            .to_str()?;
        let url =
            Url::parse(&assets_url.url).map_err(|_| anyhow::anyhow!("Failed to parse URL"))?;
        let extension = url
            .path_segments()
            .and_then(|segments| {
                path::Path::new(segments.last().unwrap())
                    .extension()
                    .and_then(|ext| ext.to_str())
            })
            .unwrap_or_else(|| {
                let mime_type = content_type.parse::<mime::Mime>().unwrap();
                let extension =
                    get_mime_extensions(&mime_type).expect("Failed to get MIME type")[0];

                extension
            });
        let full_file_path = path::Path::new(&self.destination)
            .join(format!("{}.{}", &assets_url.name, extension))
            .to_str()
            .unwrap()
            .to_string();
        let body = response.bytes().await?;

        fs::create_dir_all(&self.destination)?;
        fs::write(&full_file_path, body).expect("Unable to write file");
        self.upsert_assets_map(&assets_url.name, &assets_url.url, &full_file_path)?;

        Ok(full_file_path)
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        fs::remove_dir_all(&self.destination)?;
        fs::create_dir_all(&self.destination)?;

        Ok(())
    }

    pub fn clear_cache(&self) -> anyhow::Result<()> {
        fs::remove_file(self.get_assets_map_path())?;

        Ok(())
    }
}

pub struct AssetsURL {
    pub name: String,
    pub url: String,
}

impl AssetsURL {
    pub fn from_url(assets_url: &str) -> anyhow::Result<Self> {
        let assets_info = assets_url.split("@").collect::<Vec<&str>>();

        if assets_info.len() != 2 {
            bail!("Invalid assets URL format");
        }

        Ok(AssetsURL {
            name: assets_info[0].to_string(),
            url: assets_info[1].to_string(),
        })
    }
}
