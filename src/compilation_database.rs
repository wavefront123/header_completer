use std::{cmp::Ordering, path::PathBuf};

fn deserialize_command<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;

    let arguments = v.as_array().and_then(|arguments| {
        arguments
            .iter()
            .map(|argument| argument.as_str())
            .collect::<Option<Vec<&str>>>()
    });
    let command = v.as_str();

    match (arguments, command) {
        (Some(arguments), None) => Ok(arguments.iter().map(|s| s.to_string()).collect()),
        (None, Some(command)) => Ok(command.split(" ").map(|s| s.to_string()).collect()),
        _ => Err(serde::de::Error::custom(
            "Either string array field 'arguments' or string field 'command' must be present",
        )),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, Hash, Ord)]
pub struct CompilationDatabaseEntry {
    pub directory: PathBuf,
    pub file: PathBuf,
    #[serde(deserialize_with = "deserialize_command")]
    pub command: Vec<String>,
}

impl PartialEq for CompilationDatabaseEntry {
    fn eq(&self, other: &Self) -> bool {
        let normalize = |s: &PathBuf| s.to_str().map(|s| s.replace("\\", "/"));

        if normalize(&self.directory) != normalize(&other.directory) {
            return false;
        }
        if normalize(&self.file) != normalize(&other.file) {
            return false;
        }
        if self.command != other.command {
            return false;
        }

        return true;
    }
}

impl PartialOrd for CompilationDatabaseEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let file_order = self.file.partial_cmp(&other.file);
        let command_order = self.command.partial_cmp(&other.command);
        let directory_order = self.directory.partial_cmp(&other.directory);

        let mut result = None;
        result = result.or(file_order);
        result = result.or(command_order);
        result = result.or(directory_order);

        result
    }
}

#[derive(Clone, PartialEq)]
pub struct CompilationDatabase {
    pub entries: Vec<CompilationDatabaseEntry>,
}

impl<'de> serde::Deserialize<'de> for CompilationDatabase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let entries = Vec::<CompilationDatabaseEntry>::deserialize(deserializer)?;
        Ok(CompilationDatabase { entries })
    }
}

impl serde::Serialize for CompilationDatabase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.entries.serialize(serializer)
    }
}
