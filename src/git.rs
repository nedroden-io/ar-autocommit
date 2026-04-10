pub struct GitClient {
    repository: git2::Repository,
    path: String,
}

impl GitClient {
    pub fn new(path: &str) -> Self {
        GitClient {
            repository: git2::Repository::open(path).unwrap(),
            path: path.to_string(),
        }
    }

    pub fn stage_changes(&self) -> anyhow::Result<()> {
        let mut index = self.repository.index()?;

        index.add_all([&self.path].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        Ok(())
    }

    pub fn get_diff(&self) -> anyhow::Result<String> {
        let head = self.repository.head()?.peel_to_tree()?;
        let diff = self
            .repository
            .diff_tree_to_index(Some(&head), None, None)?;

        let mut diff_aggr = String::new();

        let _ = diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let origin = line.origin();
            let content = std::str::from_utf8(line.content()).unwrap_or("failure");

            diff_aggr.push_str(&match origin {
                '+' => format!("+ {}", content),
                '-' => format!("- {}", content),
                _ => format!(" {}", content),
            });
            true
        });

        Ok(diff_aggr)
    }

    pub fn commit(&self, message: &str) -> anyhow::Result<()> {
        let mut index = self.repository.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repository.find_tree(tree_id)?;

        let signature = git2::Signature::now("Robert Monden", "robert.monden@iodigital.com")?;
        let parent_commit = self.repository.head()?.peel_to_commit()?;

        let commit_id = self.repository.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )?;

        println!("[{}] {}", commit_id, message);

        Ok(())
    }
}
