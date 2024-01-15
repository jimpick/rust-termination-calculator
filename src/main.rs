use clap::Parser;
use rust_termination_calculator::terminate_sectors;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    epoch: i64,

    #[arg(long)]
    sector_size: String,

    #[arg(long)]
    qap_position: String,

    #[arg(long)]
    qap_velocity: String,

    #[arg(long)]
    reward_position: String,

    #[arg(long)]
    reward_velocity: String,

    #[arg(long)]
    activation: i64,

    #[arg(long)]
    expiration: i64,

    #[arg(long)]
    deal_weight: String,

    #[arg(long)]
    verified_deal_weight: String,

    #[arg(long)]
    expected_day_reward: String,

    #[arg(long)]
    expected_storage_pledge: String,

    #[arg(long)]
    power_base_epoch: i64,

    #[arg(long)]
    replaced_day_reward: String,
}

fn main() {
    let args = Args::parse();

    println!("Epoch: {}", args.epoch);
    println!("Sector Size: {}", args.sector_size);
    println!("QAP Position: {}", args.qap_position);
    println!("QAP Velocity: {}", args.qap_velocity);
    println!("Reward Position: {}", args.reward_position);
    println!("Reward Velocity: {}", args.reward_velocity);
    println!("Activation: {}", args.activation);
    println!("Expiration: {}", args.expiration);
    println!("Deal Weight: {}", args.deal_weight);
    println!("Verified Deal Weight: {}", args.verified_deal_weight);
    println!("Expected Day Reward: {}", args.expected_day_reward);
    println!("Expected Storage Pledge: {}", args.expected_storage_pledge);
    println!("Power Base Epoch: {}", args.power_base_epoch);
    println!("Replaced Day Reward: {}", args.replaced_day_reward);

    terminate_sectors(
        epoch,
        sector_size,
        qap_position,
        qap_velocity,
        reward_position,
        reward_velocity,
        activation,
        expiration,
        deal_weight,
        verified_deal_weight,
        expected_day_reward,
        expected_storage_pledge,
        power_base_epoch,
        replaced_day_reward
    );
}
