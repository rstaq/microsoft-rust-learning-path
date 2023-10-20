use pwhash::bcrypt;

pub struct User {
    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(password),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password_hash(&self) -> &String {
        &self.password_hash
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(new_password)
    }
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password.to_string()).unwrap()
}