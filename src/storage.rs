use std::{
    collections::{btree_map::Iter, BTreeMap},
    fs::File,
    io::Write,
    time::Duration,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{error::Result, utility};

pub trait Storage
where
    Self: Sized + Default + Serialize + DeserializeOwned,
{
    const FILENAME: &'static str;

    fn open() -> Result<Self> {
        let path = utility::data_dir()?.join(Self::FILENAME);

        let storage: Self = if std::fs::exists(&path)? {
            let file = File::open(&path)?;
            bincode::deserialize_from(file)?
        } else {
            let mut file = File::create(&path)?;
            let lb = Self::default();
            let bytes = bincode::serialize(&lb)?;
            file.write_all(&bytes)?;
            lb
        };

        Ok(storage)
    }
    fn save(&self) -> Result<()> {
        let path = utility::data_dir()?.join(Self::FILENAME);
        let mut file = File::options().write(true).open(path)?;
        let bytes = bincode::serialize(self)?;
        file.write_all(&bytes)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    wallet: usize,
    penalty_debuff_millis: u64,
}

impl PlayerData {
    pub fn wallet(&self) -> usize {
        self.wallet
    }

    pub fn add_to_wallet(&mut self, value: usize) {
        self.wallet += value;
    }

    pub fn write_off_from_wallet(&mut self, value: usize) {
        self.wallet = self.wallet.saturating_sub(value);
    }

    pub fn penalty_debuff_dur(&self) -> Duration {
        Duration::from_millis(self.penalty_debuff_millis)
    }

    pub fn set_penalty_debuff(&mut self, millis: u64) {
        self.penalty_debuff_millis = millis;
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            wallet: 0,
            penalty_debuff_millis: 1000,
        }
    }
}

impl Storage for PlayerData {
    const FILENAME: &'static str = "player_data";
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Leaderboard(pub BTreeMap<String, usize>);

impl Storage for Leaderboard {
    const FILENAME: &'static str = "leaderboard";
}

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
    pub fn insert(&mut self, nickname: &str, score: usize) {
        self.0.insert(nickname.to_string(), score);
    }

    pub fn iter(&self) -> Iter<String, usize> {
        self.0.iter()
    }

    pub fn sorted_vec(&self) -> Vec<(&String, &usize)> {
        let mut v = Vec::from_iter(&self.0);
        v.sort_by(|a, b| a.1.cmp(b.1).reverse());
        v
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Upgrades(Vec<UpgradeItem>);

impl Storage for Upgrades {
    const FILENAME: &'static str = "upgrades";
}

impl std::ops::Index<usize> for Upgrades {
    type Output = UpgradeItem;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl std::ops::IndexMut<usize> for Upgrades {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Default for Upgrades {
    fn default() -> Self {
        Self(vec![
            UpgradeItem::new(
                "Exploding Shrapnel",
                "Increases all strategem rewards by +100 Democracy Points",
                2500,
            ),
            UpgradeItem::new(
                "Liquid-Ventilated Cockpit",
                "Reduces time penalty after failed strategem",
                3000,
            ),
            UpgradeItem::new(
                "Targeting Software Upgrade",
                "Increases time reward after successfully completing strategem by +0.5s",
                5000,
            ),
        ])
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UpgradeItem {
    name: String,
    desc: String,
    price: usize,
    purchased: bool,
}

impl UpgradeItem {
    pub fn new(name: impl Into<String>, desc: impl Into<String>, price: usize) -> Self {
        Self {
            name: name.into(),
            desc: desc.into(),
            price,
            purchased: false,
        }
    }

    // pub fn name(&self) -> &str {
    //     &self.name
    // }
    //
    // pub fn desc(&self) -> &str {
    //     &self.desc
    // }

    pub fn price(&self) -> usize {
        self.price
    }

    pub fn is_purchased(&self) -> bool {
        self.purchased
    }

    pub fn set_purchased(&mut self) {
        self.purchased = true;
    }
}

impl std::fmt::Display for UpgradeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<32}[{}]\n\t{}",
            self.name,
            if self.purchased {
                "Purchased".to_string()
            } else {
                format!("{} DP", self.price)
            },
            self.desc
        )
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
