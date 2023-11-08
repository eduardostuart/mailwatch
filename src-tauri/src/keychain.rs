use keyring::Entry;

pub struct Keychain;

/// Keychain service name
const SERVICE: &str = "mailwatch";

pub struct KeychainEntryKey<'a> {
    pub id: i64,
    pub username: &'a str,
}

impl<'a> KeychainEntryKey<'a> {
    pub fn new(id: i64, username: &'a str) -> Self {
        Self { id, username }
    }
}

impl std::fmt::Display for KeychainEntryKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.id, self.username)
    }
}

impl Default for Keychain {
    fn default() -> Self {
        Self::new()
    }
}

impl Keychain {
    pub fn new() -> Self {
        Self {}
    }

    /// Get an entry from the keychain
    pub fn get_entry(&self, identification: &KeychainEntryKey) -> anyhow::Result<Entry> {
        Ok(Entry::new(SERVICE, &identification.to_string())?)
    }

    /// Create a new entry in the keychain
    /// This will also update the password if the entry already exists
    pub fn new_entry(
        &self,
        identification: &KeychainEntryKey,
        password: &str,
    ) -> anyhow::Result<Entry> {
        let entry = self.get_entry(identification)?;
        entry.set_password(password)?;
        Ok(entry)
    }

    /// Get the password for a given entry
    pub fn get_password(&self, identification: KeychainEntryKey) -> anyhow::Result<String> {
        let entry = Entry::new(SERVICE, &identification.to_string())?;
        Ok(entry.get_password()?)
    }

    /// Delete an entry from the keychain
    pub fn delete_entry(&self, entry: Entry) -> anyhow::Result<()> {
        entry.delete_password()?;
        Ok(())
    }
}
