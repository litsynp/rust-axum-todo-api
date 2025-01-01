use clap::Parser;
use rust_todo_api::infrastructure::database;
use rust_todo_api::user::views::NewUserRequest;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = database::get_postgres_pool(db_url.as_str())
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to connect to Postgres with provided URL: {}",
                db_url
            )
        });

    let user_service = rust_todo_api::user::service::UserService::new(pool);
    match user_service
        .register_user(NewUserRequest {
            email: args.email,
            nickname: args.nickname,
            password: args.password,
        })
        .await
    {
        Ok(user) => println!("Successfully registered user {:?}", user),
        Err(e) => println!("Failed to create user {}", e),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Email of the user to be created
    #[arg(short, long)]
    email: String,

    // Nickname of the user to be created
    #[arg(short, long)]
    nickname: String,

    // Password of the user to be created
    #[arg(short, long)]
    password: String,
}
