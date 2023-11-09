use keyring::Entry;

/// Keychain service name
const SERVICE: &str = "mailwatch";

#[derive(Debug)]
pub struct Keychain<'a> {
    pub id: i64,
    pub username: &'a str,
}

impl<'a> Keychain<'a> {
    pub fn new(id: i64, username: &'a str) -> Self {
        Self { id, username }
    }

    fn entry_key(&self) -> String {
        format!("{}.{}", self.id, self.username)
    }

    /// Get an entry from the keychain
    pub fn get_entry(&self) -> anyhow::Result<Entry> {
        Ok(Entry::new(SERVICE, &self.entry_key())?)
    }

    /// Create a new entry in the keychain
    /// This will also update the password if the entry already exists
    pub fn new_entry(&self, password: &str) -> anyhow::Result<Entry> {
        let entry = self.get_entry()?;
        entry.set_password(password)?;
        Ok(entry)
    }

    /// Get the password for a given entry
    pub fn get_password(&self) -> anyhow::Result<String> {
        Ok(self.get_entry()?.get_password()?)
    }

    /// Delete an entry from the keychain
    pub fn delete_entry(&self) -> anyhow::Result<()> {
        Ok(self.get_entry()?.delete_password()?)
    }
}
