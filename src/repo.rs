use git2::{Error, ErrorCode, Repository, StatusOptions, SubmoduleIgnore};

pub struct Repo {
  repo: Repository,
}

impl Repo {
  pub fn new(path: &str) -> Repo {
    let repo = Repository::open(path).unwrap();
    Repo {
      repo,
    }
  }

  pub fn has_local_changes() -> bool {
    true
  }

  pub fn get_status(&self) -> Result<(), Error> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);

    let statuses = &self.repo.statuses(Some(&mut opts))?;

   for status in statuses.iter() {
     let path = status.path().unwrap_or("end");
     let mode = status.status();
     println!("path: {}, {:?}", path, mode);
   }

    Ok(())
  }
}