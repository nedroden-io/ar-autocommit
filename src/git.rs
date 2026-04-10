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

        index.add_all([&self.path], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        Ok(())
    }

    pub fn get_diff(&self) -> anyhow::Result<String> {
        let head = self.repository.head()?.peel_to_tree()?;
        let diff = self.repository.diff_tree_to_index(Some(&head), None, None)?;

        let _ = diff.print(git2::DiffFormat::Raw, |delta, _hunk, line| {
            let origin = line.origin(); // '+', '-', ' ', usw.
            let content = std::str::from_utf8(line.content()).unwrap_or("");
            let path = delta.new_file().path().unwrap().display();

            match origin {
                '+' => print!("\x1b[32m+{}\x1b[0m", content), // grün
                '-' => print!("\x1b[31m-{}\x1b[0m", content), // rot
                _   => print!(" {}", content),
            }
            true
        });

        todo!("implement")
    }
}