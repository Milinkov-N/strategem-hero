use std::{
    collections::{btree_map::Iter, BTreeMap},
    fs::File,
    io::{Read, Write},
};

use crate::{error::Result, utility};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Leaderboard(pub BTreeMap<String, usize>);

impl Default for Leaderboard {
    fn default() -> Self {
        let mut map = BTreeMap::new();

        map.insert("John Helldiver".into(), 20000);
        map.insert("Eagle-1".into(), 14500);
        map.insert("Pelican-1".into(), 11200);
        map.insert("Democracy Officer".into(), 8300);
        map.insert("You".into(), 0);

        Self(map)
    }
}

impl Leaderboard {
    pub fn open() -> Result<Self> {
        let path = utility::data_dir()?.join("leaderboard");
        let leaderboard = if std::fs::exists(&path)? {
            let mut file = File::open(&path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            bincode::deserialize(&buf)?
        } else {
            let mut file = File::create(&path)?;
            let lb = Leaderboard::default();
            let bytes = bincode::serialize(&lb)?;
            file.write_all(&bytes)?;
            lb
        };

        Ok(leaderboard)
    }

    pub fn save(&self) -> Result<()> {
        let path = utility::data_dir()?.join("leaderboard");

        let mut file = File::open(&path)?;
        let bytes = bincode::serialize(&self)?;
        file.write_all(&bytes)?;

        Ok(())
    }

    pub fn insert(&mut self, nickname: &str, score: usize) {
        self.0.insert(nickname.to_string(), score);
    }

    pub fn iter(&self) -> Iter<String, usize> {
        self.0.iter()
    }

    pub fn sorted_vec(&self) -> Vec<(&String, &usize)> {
        let mut v = Vec::from_iter(&self.0);
        v.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        v
    }
}

#[cfg(test)]
mod tests {
    use super::Leaderboard;

    #[test]
    fn leaderboard_replace() {
        let mut leaderboard = Leaderboard::default();
        leaderboard.insert("You", 1000);
        assert_eq!(Some(&1000), leaderboard.0.get("You"));
    }
}
