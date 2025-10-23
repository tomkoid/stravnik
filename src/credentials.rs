use crate::args::Args;

#[derive(Debug)]
pub struct MatrixCredentials {
    pub homeserver: String,
    pub username: String,
    pub password: String,
}

pub fn init_matrix_credentials(args: &Args) -> anyhow::Result<MatrixCredentials> {
    // get username and password from args
    let homeserver = args.matrix_homeserver.clone().unwrap();
    let username = args.matrix_username.clone().unwrap();
    let password = args.matrix_password.clone().unwrap();

    Ok(MatrixCredentials {
        homeserver,
        username,
        password,
    })
}
