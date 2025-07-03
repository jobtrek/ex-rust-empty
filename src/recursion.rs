// Don't change the code below
pub enum Repo<'a> {
    File(&'a str),
    Repo((&'a str, &'a [Repo<'a>])),
}
/// Write a function to get the path of a given file in a repository
///
/// ```
/// use rust_ex::recursion::Repo;
/// let repo = &[
///     Repo::Repo(("Langage-source", &[
///         Repo::File("Cargo.lock"),
///         Repo::File("Cargo.toml"),
///         Repo::Repo(("src", &[
///             Repo::File("get_file_content.rs"),
///             Repo::File("lib.rs"),
///             Repo::File("main.rs"),
///             Repo::Repo(("tokenize", &[
///                 Repo::File("mod.rs"),
///                 Repo::File("token_types.rs")
///             ]))
///         ])),
///         Repo::File("test.skrb")
///     ])),
///     Repo::Repo(("pomme", &[
///         Repo::Repo(("supported_languages", &[
///             Repo::File("rs.yaml")
///         ])),
///         Repo::File("Cargo.lock"),
///         Repo::File("Cargo.toml"),
///         Repo::Repo(("src", &[
///             Repo::File("main.rs")
///         ])),
///         Repo::Repo(("toto", &[
///             Repo::File("arrays.json")
///         ]))
///     ]))
/// ];
/// assert_eq!(rust_ex::recursion::get_path_of(repo, ".", "arrays.json", 0), "./pomme/toto/arrays.json");
/// assert_eq!(rust_ex::recursion::get_path_of(repo, ".", "token_types.rs", 0), "./Langage-source/src/tokenize/token_types.rs");
/// assert_eq!(rust_ex::recursion::get_path_of(repo, ".", "lib.rs", 0), "./Langage-source/src/lib.rs");
/// ```
pub fn get_path_of(repo: &[Repo], current_path: &str, file: &str, position: usize) -> String {
    // Write your code here
    if position < repo.len() - 1 {
        let result = get_path_of(repo, current_path, file, position + 1);
        if !result.is_empty() {
            return result;
        }
    }
    let r = &repo[position];
    match r {
        Repo::File(f) => {
            if *f == file {
                return format!("{}/{}", current_path, file);
            }
        }
        Repo::Repo((dir, new_repo)) => {
            let path = format!("{}/{}", current_path, dir);
            let result = get_path_of(new_repo, &path, file, 0);
            if !result.is_empty() {
                return result;
            }
        }
    }
    "".to_string()
}