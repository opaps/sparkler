use git2::{Error, ErrorCode, Repository, StatusOptions, SubmoduleIgnore};

struct Repo {
  repo: Repository,
}

impl Repo {
  pub fn new(path: &str) -> Repo {
    let repo = Repository::open(path).unwrap();
    Repo {
      repo,
    }
  }

  pub fn HasLocalChanges() -> bool {
    true
  }
}