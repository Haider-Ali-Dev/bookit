use api::database::Database;
use api::Config;
use dotenv::dotenv;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "http")]
struct Opt {
    #[structopt(default_value = "postgres://postgres:@localhost:5432/bookit")]
    connection_string: String,
}

fn main() {
    dotenv().ok();
    let opt = Opt::from_args();

    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");
    let database = rt.block_on(async move { Database::new(&opt.connection_string).await });

    let config = Config { database };

    rt.block_on(async move {
        api::rocket(config)
            .launch()
            .await
            .expect("Failed to start the server")
    });
}
